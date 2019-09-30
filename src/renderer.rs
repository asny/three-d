
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
    rendertarget: rendertarget::ColorRendertarget,
    geometry_pass_rendertargets: [rendertarget::ColorRendertargetArray; 2],
    full_screen: FullScreen,
    ambient_light: AmbientLight,
    directional_lights: DirectionalLight,
    point_lights: PointLight,
    spot_lights: SpotLight,
    pub background_color: Vec4,
    pub camera: Camera
}


impl DeferredPipeline
{
    pub fn new(gl: &Gl, screen_width: usize, screen_height: usize, background_color: Vec4) -> Result<DeferredPipeline, Error>
    {
        let light_pass_program = program::Program::from_source(gl,
                                                               include_str!("shaders/light_pass.vert"),
                                                               include_str!("shaders/light_pass.frag"))?;
        let rendertarget = rendertarget::ColorRendertarget::default(gl, screen_width, screen_height)?;
        let geometry_pass_rendertargets =
            [rendertarget::ColorRendertargetArray::new(gl, screen_width, screen_height, 4, true)?,
            rendertarget::ColorRendertargetArray::new(gl, screen_width, screen_height, 4, true)?];


        let camera = Camera::new_perspective(gl, vec3(5.0, 5.0, 5.0), vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0),
                                                    degrees(45.0), screen_width as f32 / screen_height as f32, 0.1, 1000.0);

        Ok(DeferredPipeline {
            buffer_index: 0,
            gl: gl.clone(),
            light_pass_program,
            rendertarget,
            geometry_pass_rendertargets,
            full_screen: FullScreen::new(gl),
            ambient_light: AmbientLight::new(),
            directional_lights: DirectionalLight::new(gl)?,
            point_lights: PointLight::new(gl)?,
            spot_lights: SpotLight::new(gl)?,
            background_color,
            camera })
    }

    pub fn resize(&mut self, screen_width: usize, screen_height: usize) -> Result<(), Error>
    {
        self.rendertarget = rendertarget::ColorRendertarget::default(&self.gl, screen_width, screen_height)?;
        for i in 0..self.geometry_pass_rendertargets.len()
        {
            self.geometry_pass_rendertargets[i] = rendertarget::ColorRendertargetArray::new(&self.gl, screen_width, screen_height, 4, true)?;
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
        where F: Fn(&Camera)
    {
        // Double buffering is necessary to avoid:
        // Chrome: GL ERROR :GL_INVALID_OPERATION : glDrawElements: Source and destination textures of the draw are the same.
        // Firefox: Error: WebGL warning: drawElements: Texture level 0 would be read by TEXTURE_2D unit 0, but written by framebuffer attachment DEPTH_ATTACHMENT, which would be illegal feedback.
        self.buffer_index = (self.buffer_index + 1) % self.geometry_pass_rendertargets.len();
        self.geometry_pass_rendertargets[self.buffer_index].bind();
        self.geometry_pass_rendertargets[self.buffer_index].clear(&self.background_color);

        state::depth_write(&self.gl, true);
        state::depth_test(&self.gl, state::DepthTestType::LEQUAL);
        state::cull(&self.gl, state::CullType::NONE);
        state::blend(&self.gl, state::BlendType::NONE);

        render_scene(&self.camera);
        Ok(())
    }

    pub fn light_pass(&self) -> Result<(), Error>
    {
        self.light_pass_render_to(&self.rendertarget)?;
        Ok(())
    }

    pub fn light_pass_render_to(&self, rendertarget: &ColorRendertarget) -> Result<(), Error>
    {
        rendertarget.bind();
        rendertarget.clear(&vec4(0.0, 0.0, 0.0, 0.0));

        state::depth_write(&self.gl,false);
        state::depth_test(&self.gl, state::DepthTestType::NONE);
        state::cull(&self.gl,state::CullType::BACK);
        state::blend(&self.gl, state::BlendType::ONE__ONE);

        self.light_pass_program.use_texture(self.geometry_pass_texture(), "gbuffer")?;
        self.light_pass_program.use_texture(self.geometry_pass_depth_texture(), "depthMap")?;

        self.light_pass_program.add_uniform_vec3("eyePosition", &self.camera.position())?;

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
        self.light_pass_program.use_attribute_vec3_float(&self.full_screen.buffer(), "position", 0).unwrap();
        self.light_pass_program.use_attribute_vec2_float(&self.full_screen.buffer(), "uv_coordinate", 1).unwrap();
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

    /*pub fn shine_ambient_light(&self, light: &light::AmbientLight) -> Result<(), Error>
    {
        self.light_pass_program.add_uniform_int("lightType", &0)?;
        self.light_pass_program.add_uniform_vec3("ambientLight.base.color", &light.base.color)?;
        self.light_pass_program.add_uniform_float("ambientLight.base.intensity", &light.base.intensity)?;

        self.full_screen.render(&self.light_pass_program);
        Ok(())
    }

    pub fn shine_point_light(&self, light: &light::PointLight) -> Result<(), Error>
    {
        //self.light_pass_program.add_uniform_int("shadowMap", &5)?;
        //self.light_pass_program.add_uniform_int("shadowCubeMap", &6)?;

        self.light_pass_program.add_uniform_int("lightType", &2)?;
        self.light_pass_program.add_uniform_vec3("pointLight.position", &light.position)?;
        self.light_pass_program.add_uniform_vec3("pointLight.base.color", &light.base.color)?;
        self.light_pass_program.add_uniform_float("pointLight.base.intensity", &light.base.intensity)?;
        self.light_pass_program.add_uniform_float("pointLight.attenuation.constant", &light.attenuation.constant)?;
        self.light_pass_program.add_uniform_float("pointLight.attenuation.linear", &light.attenuation.linear)?;
        self.light_pass_program.add_uniform_float("pointLight.attenuation.exp", &light.attenuation.exp)?;

        self.full_screen.render(&self.light_pass_program);
        Ok(())
    }

    pub fn shine_spot_light(&self, light: &light::SpotLight) -> Result<(), Error>
    {
        if let Ok(shadow_camera) = light.shadow_camera() {
            let bias_matrix = crate::Mat4::new(
                                 0.5, 0.0, 0.0, 0.0,
                                 0.0, 0.5, 0.0, 0.0,
                                 0.0, 0.0, 0.5, 0.0,
                                 0.5, 0.5, 0.5, 1.0);
            self.light_pass_program.add_uniform_mat4("shadowMVP", &(bias_matrix * *shadow_camera.get_projection() * *shadow_camera.get_view()))?;

            light.shadow_rendertarget.as_ref().unwrap().target.bind(5);
            self.light_pass_program.add_uniform_int("shadowMap", &5)?;
        }

        //self.light_pass_program.add_uniform_int("shadowCubeMap", &6)?;

        self.light_pass_program.add_uniform_int("lightType", &3)?;
        self.light_pass_program.add_uniform_vec3("spotLight.position", &light.position)?;
        self.light_pass_program.add_uniform_vec3("spotLight.direction", &light.direction)?;
        self.light_pass_program.add_uniform_vec3("spotLight.base.color", &light.base.color)?;
        self.light_pass_program.add_uniform_float("spotLight.base.intensity", &light.base.intensity)?;
        self.light_pass_program.add_uniform_float("spotLight.attenuation.constant", &light.attenuation.constant)?;
        self.light_pass_program.add_uniform_float("spotLight.attenuation.linear", &light.attenuation.linear)?;
        self.light_pass_program.add_uniform_float("spotLight.attenuation.exp", &light.attenuation.exp)?;
        self.light_pass_program.add_uniform_float("spotLight.cutoff", &light.cutoff.cos())?;

        self.full_screen.render(&self.light_pass_program);
        Ok(())
    }*/

    pub fn full_screen(&self) -> &FullScreen
    {
        &self.full_screen
    }

    pub fn screen_rendertarget(&self) -> &ColorRendertarget
    {
        &self.rendertarget
    }

    pub fn geometry_pass_rendertarget(&self) -> &ColorRendertargetArray
    {
        &self.geometry_pass_rendertargets[self.buffer_index]
    }

    pub fn geometry_pass_texture(&self) -> &Texture
    {
        &self.geometry_pass_rendertargets[self.buffer_index].targets
    }
    pub fn geometry_pass_depth_texture(&self) -> &Texture
    {
        self.geometry_pass_rendertargets[self.buffer_index].depth_target.as_ref().unwrap()
    }
}