
use crate::*;

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Program(program::Error),
    Rendertarget(rendertarget::Error),
    Texture(texture::Error),
    Buffer(buffer::Error),
    Light(light::Error),
    LightExtendsMaxLimit {message: String}
}

impl From<std::io::Error> for Error {
    fn from(other: std::io::Error) -> Self {
        Error::IO(other)
    }
}

impl From<program::Error> for Error {
    fn from(other: program::Error) -> Self {
        Error::Program(other)
    }
}

impl From<rendertarget::Error> for Error {
    fn from(other: rendertarget::Error) -> Self {
        Error::Rendertarget(other)
    }
}

impl From<texture::Error> for Error {
    fn from(other: texture::Error) -> Self {
        Error::Texture(other)
    }
}

impl From<buffer::Error> for Error {
    fn from(other: buffer::Error) -> Self {
        Error::Buffer(other)
    }
}

impl From<light::Error> for Error {
    fn from(other: light::Error) -> Self {
        Error::Light(other)
    }
}

pub struct DeferredPipeline {
    gl: Gl,
    buffer_index: usize,
    light_pass_program: program::Program,
    geometry_pass_rendertarget: rendertarget::RenderTarget,
    geometry_pass_textures: [Texture2DArray; 2],
    geometry_pass_depth_textures: [Texture2DArray; 2],
    full_screen: VertexBuffer,
    ambient_light: AmbientLight,
    directional_lights: DirectionalLight,
    point_lights: PointLight,
    spot_lights: SpotLight,
    pub background_color: Vec4
}


impl DeferredPipeline
{
    pub fn new(gl: &Gl, screen_width: usize, screen_height: usize, background_color: Vec4) -> Result<DeferredPipeline, Error>
    {
        let light_pass_program = program::Program::from_source(gl,
                                                               include_str!("shaders/light_pass.vert"),
                                                               include_str!("shaders/light_pass.frag"))?;
        let geometry_pass_rendertarget = rendertarget::RenderTarget::new(gl, 2)?;
        let geometry_pass_textures =
            [Texture2DArray::new_as_color_targets(gl, screen_width, screen_height, 2)?,
            Texture2DArray::new_as_color_targets(gl, screen_width, screen_height, 2)?];
        let geometry_pass_depth_textures =
            [Texture2DArray::new_as_depth_targets(gl, screen_width, screen_height, 1)?,
            Texture2DArray::new_as_depth_targets(gl, screen_width, screen_height, 1)?];

        let positions = vec![
            -3.0, -1.0, 0.0,
            3.0, -1.0, 0.0,
            0.0, 2.0, 0.0
        ];
        let uvs = vec![
            -1.0, 0.0,
            2.0, 0.0,
            0.5, 1.5
        ];
        let full_screen = VertexBuffer::new_with_two_static_attributes(&gl, &positions, &uvs).unwrap();

        Ok(DeferredPipeline {
            buffer_index: 0,
            gl: gl.clone(),
            light_pass_program,
            full_screen,
            geometry_pass_rendertarget,
            geometry_pass_textures,
            geometry_pass_depth_textures,
            ambient_light: AmbientLight::new(),
            directional_lights: DirectionalLight::new(gl)?,
            point_lights: PointLight::new(gl)?,
            spot_lights: SpotLight::new(gl)?,
            background_color })
    }

    pub fn resize(&mut self, screen_width: usize, screen_height: usize) -> Result<(), Error>
    {
        for i in 0..self.geometry_pass_textures.len()
        {
            self.geometry_pass_textures[i] = Texture2DArray::new_as_color_targets(&self.gl, screen_width, screen_height, 2)?;
            self.geometry_pass_depth_textures[i] = Texture2DArray::new_as_depth_targets(&self.gl, screen_width, screen_height, 1)?;
        }
        Ok(())
    }

    pub fn shadow_pass<F>(&self, render_scene: &F)
        where F: Fn(&Camera)
    {
        self.directional_lights.shadow_pass(render_scene);
        self.spot_lights.shadow_pass(render_scene);
    }

    pub fn geometry_pass<F>(&mut self, render_scene: &F) -> Result<(), Error>
        where F: Fn()
    {
        // Double buffering is necessary to avoid:
        // Chrome: GL ERROR :GL_INVALID_OPERATION : glDrawElements: Source and destination textures of the draw are the same.
        // Firefox: Error: WebGL warning: drawElements: Texture level 0 would be read by TEXTURE_2D unit 0, but written by framebuffer attachment DEPTH_ATTACHMENT, which would be illegal feedback.
        self.buffer_index = (self.buffer_index + 1) % self.geometry_pass_textures.len();
        self.geometry_pass_rendertarget.write_to_color_array_and_depth_array(&self.geometry_pass_textures[self.buffer_index],
                                                                             &self.geometry_pass_depth_textures[self.buffer_index],
                                                                             &|channel| {channel}, 0)?;
        self.geometry_pass_rendertarget.clear_color_and_depth(&self.background_color);

        state::depth_write(&self.gl, true);
        state::depth_test(&self.gl, state::DepthTestType::LEQUAL);
        state::cull(&self.gl, state::CullType::NONE);
        state::blend(&self.gl, state::BlendType::NONE);

        render_scene();
        Ok(())
    }

    pub fn light_pass(&self, camera: &Camera) -> Result<(), Error>
    {
        ScreenRendertarget::write(&self.gl, self.geometry_pass_textures[0].width, self.geometry_pass_textures[0].height);
        ScreenRendertarget::clear_color_and_depth(&self.gl, &vec4(0.0, 0.0, 0.0, 0.0));
        self.light_pass_render_to_rendertarget(camera)?;
        Ok(())
    }

    pub fn light_pass_render_to_rendertarget(&self, camera: &Camera) -> Result<(), Error>
    {
        state::depth_write(&self.gl,false);
        state::depth_test(&self.gl, state::DepthTestType::NONE);
        state::cull(&self.gl,state::CullType::BACK);
        state::blend(&self.gl, state::BlendType::ONE__ONE);

        self.light_pass_program.use_texture(self.geometry_pass_texture(), "gbuffer")?;
        self.light_pass_program.use_texture(self.geometry_pass_depth_texture(), "depthMap")?;

        self.light_pass_program.add_uniform_vec3("eyePosition", &camera.position())?;
        self.light_pass_program.add_uniform_mat4("viewProjectionInverse", &(camera.get_projection() * camera.get_view()).invert().unwrap())?;

        // Ambient light
        self.light_pass_program.add_uniform_vec3("ambientLight.base.color", &self.ambient_light.color())?;
        self.light_pass_program.add_uniform_float("ambientLight.base.intensity", &self.ambient_light.intensity())?;

        // Directional lights
        self.light_pass_program.use_texture(self.directional_lights.shadow_maps(), "directionalLightShadowMaps")?;
        self.light_pass_program.use_uniform_block(self.directional_lights.buffer(), "DirectionalLights");

        // Point lights
        self.light_pass_program.use_uniform_block(self.point_lights.buffer(), "PointLights");

        // Spot lights
        self.light_pass_program.use_texture(self.spot_lights.shadow_maps(), "spotLightShadowMaps")?;
        self.light_pass_program.use_uniform_block(self.spot_lights.buffer(), "SpotLights");

        // Render
        self.light_pass_program.use_attribute_vec3_float(&self.full_screen, "position", 0).unwrap();
        self.light_pass_program.use_attribute_vec2_float(&self.full_screen, "uv_coordinate", 1).unwrap();
        self.light_pass_program.draw_arrays(3);
        Ok(())
    }

    pub fn ambient_light(&mut self) -> &mut AmbientLight
    {
        &mut self.ambient_light
    }

    pub fn directional_light(&mut self, index: usize) -> Result<&mut DirectionalLight, Error>
    {
        if index >= light::MAX_NO_LIGHTS
        {
            return Err(Error::LightExtendsMaxLimit {message: format!("Tried to get directional light number {}, but the limit is {}.", index, light::MAX_NO_LIGHTS)})
        }
        Ok(self.directional_lights.light_at(index))
    }

    pub fn point_light(&mut self, index: usize) -> Result<&mut PointLight, Error>
    {
        if index >= light::MAX_NO_LIGHTS
        {
            return Err(Error::LightExtendsMaxLimit {message: format!("Tried to get point light number {}, but the limit is {}.", index, light::MAX_NO_LIGHTS)})
        }
        Ok(self.point_lights.light_at(index))
    }

    pub fn spot_light(&mut self, index: usize) -> Result<&mut SpotLight, Error>
    {
        if index >= light::MAX_NO_LIGHTS
        {
            return Err(Error::LightExtendsMaxLimit {message: format!("Tried to get spot light number {}, but the limit is {}.", index, light::MAX_NO_LIGHTS)})
        }
        Ok(self.spot_lights.light_at(index))
    }

    pub fn geometry_pass_texture(&self) -> &Texture2DArray
    {
        &self.geometry_pass_textures[self.buffer_index]
    }
    pub fn geometry_pass_depth_texture(&self) -> &Texture2DArray
    {
        &self.geometry_pass_depth_textures[self.buffer_index]
    }
}