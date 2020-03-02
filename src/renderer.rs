
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
    light_pass_program: program::Program,
    geometry_pass_rendertarget: rendertarget::RenderTarget,
    geometry_pass_texture: Texture2DArray,
    geometry_pass_depth_texture: Texture2DArray,
    full_screen: VertexBuffer,
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
        let geometry_pass_texture = Texture2DArray::new_as_color_targets(gl, screen_width, screen_height, 2)?;
        let geometry_pass_depth_texture = Texture2DArray::new_as_depth_targets(gl, screen_width, screen_height, 1)?;

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
            gl: gl.clone(),
            light_pass_program,
            full_screen,
            geometry_pass_rendertarget,
            geometry_pass_texture,
            geometry_pass_depth_texture,
            background_color })
    }

    pub fn geometry_pass<F>(&mut self, render_scene: &F) -> Result<(), Error>
        where F: Fn()
    {
        self.geometry_pass_texture = Texture2DArray::new_as_color_targets(&self.gl, self.geometry_pass_texture.width, self.geometry_pass_texture.height, 2)?;
        self.geometry_pass_depth_texture = Texture2DArray::new_as_depth_targets(&self.gl, self.geometry_pass_depth_texture.width, self.geometry_pass_depth_texture.height, 1)?;

        self.geometry_pass_rendertarget.write_to_color_array_and_depth_array(&self.geometry_pass_texture,
                                                                             &self.geometry_pass_depth_texture,
                                                                             &|channel| {channel}, 0)?;
        self.geometry_pass_rendertarget.clear_color_and_depth(&self.background_color);

        state::depth_write(&self.gl, true);
        state::depth_test(&self.gl, state::DepthTestType::LessOrEqual);
        state::cull(&self.gl, state::CullType::None);
        state::blend(&self.gl, state::BlendType::None);

        render_scene();
        Ok(())
    }

    /*pub fn light_pass(&self, camera: &Camera) -> Result<(), Error>
    {
        ScreenRendertarget::write(&self.gl, self.geometry_pass_texture.width, self.geometry_pass_texture.height);
        ScreenRendertarget::clear_color_and_depth(&self.gl, &vec4(0.0, 0.0, 0.0, 0.0));
        self.light_pass_render_to_rendertarget(camera)?;
        Ok(())
    }*/

    pub fn light_pass_render_to_rendertarget(&self, camera: &Camera, ambient_light: &AmbientLight, directional_lights: &[&DirectionalLight], spot_lights: &[&SpotLight], point_lights: &[&PointLight]) -> Result<(), Error>
    {
        state::depth_write(&self.gl,false);
        state::depth_test(&self.gl, state::DepthTestType::None);
        state::cull(&self.gl,state::CullType::Back);
        state::blend(&self.gl, state::BlendType::OneOne);

        self.light_pass_program.use_texture(self.geometry_pass_texture(), "gbuffer")?;
        self.light_pass_program.use_texture(self.geometry_pass_depth_texture(), "depthMap")?;

        self.light_pass_program.add_uniform_vec3("eyePosition", &camera.position())?;
        self.light_pass_program.add_uniform_mat4("viewProjectionInverse", &(camera.get_projection() * camera.get_view()).invert().unwrap())?;

        // Ambient light
        self.light_pass_program.add_uniform_int("light_type", &0)?;
        self.light_pass_program.add_uniform_vec3("ambientLight.base.color", &ambient_light.color())?;
        self.light_pass_program.add_uniform_float("ambientLight.base.intensity", &ambient_light.intensity())?;

        self.light_pass_program.use_attribute_vec3_float(&self.full_screen, "position", 0).unwrap();
        self.light_pass_program.use_attribute_vec2_float(&self.full_screen, "uv_coordinate", 1).unwrap();
        self.light_pass_program.draw_arrays(3);

        // Directional light
        for light in directional_lights {
            if let Some(texture) = light.shadow_map() {
                self.light_pass_program.use_texture(texture, "shadowMap")?;
                self.light_pass_program.add_uniform_int("light_type", &2)?;
            }
            else {
                self.light_pass_program.add_uniform_int("light_type", &1)?;
            }
            self.light_pass_program.use_uniform_block(light.buffer(), "DirectionalLight");

            self.light_pass_program.use_attribute_vec3_float(&self.full_screen, "position", 0).unwrap();
            self.light_pass_program.use_attribute_vec2_float(&self.full_screen, "uv_coordinate", 1).unwrap();
            self.light_pass_program.draw_arrays(3);
        }

        // Spot lights
        for light in spot_lights {
            if let Some(texture) = light.shadow_map() {
                self.light_pass_program.use_texture(texture, "shadowMap")?;
                self.light_pass_program.add_uniform_int("light_type", &4)?;
            }
            else {
                self.light_pass_program.add_uniform_int("light_type", &3)?;
            }
            self.light_pass_program.use_uniform_block(light.buffer(), "SpotLight");

            self.light_pass_program.use_attribute_vec3_float(&self.full_screen, "position", 0).unwrap();
            self.light_pass_program.use_attribute_vec2_float(&self.full_screen, "uv_coordinate", 1).unwrap();
            self.light_pass_program.draw_arrays(3);
        }

        // Point lights
        for light in point_lights {
            self.light_pass_program.add_uniform_int("light_type", &5)?;
            self.light_pass_program.use_uniform_block(light.buffer(), "PointLight");

            self.light_pass_program.use_attribute_vec3_float(&self.full_screen, "position", 0).unwrap();
            self.light_pass_program.use_attribute_vec2_float(&self.full_screen, "uv_coordinate", 1).unwrap();
            self.light_pass_program.draw_arrays(3);
        }

        Ok(())
    }

    pub fn geometry_pass_texture(&self) -> &Texture2DArray
    {
        &self.geometry_pass_texture
    }
    pub fn geometry_pass_depth_texture(&self) -> &Texture2DArray
    {
        &self.geometry_pass_depth_texture
    }
}