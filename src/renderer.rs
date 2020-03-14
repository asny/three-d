
use crate::*;

#[derive(Debug)]
pub enum Error {
    Core(core::Error),
    LightExtendsMaxLimit {message: String}
}

impl From<core::Error> for Error {
    fn from(other: core::Error) -> Self {
        Error::Core(other)
    }
}

pub struct DeferredPipeline {
    gl: Gl,
    ambient_light_program: program::Program,
    directional_light_program: program::Program,
    point_light_program: program::Program,
    spot_light_program: program::Program,
    geometry_pass_texture: Option<Texture2DArray>,
    geometry_pass_depth_texture: Option<Texture2DArray>,
    full_screen: VertexBuffer
}


impl DeferredPipeline
{
    pub fn new(gl: &Gl) -> Result<DeferredPipeline, Error>
    {
        let ambient_light_program = program::Program::from_source(gl,
                                                               include_str!("shaders/light_pass.vert"),
                                                               include_str!("shaders/ambient_light.frag"))?;
        let directional_light_program = program::Program::from_source(gl,
                                                               include_str!("shaders/light_pass.vert"),
                                                               &format!("{}\n{}\n{}",
                                                                       &include_str!("shaders/light_shared.frag"),
                                                                       &include_str!("shaders/shadow_shared.frag"),
                                                                       &include_str!("shaders/directional_light.frag")))?;
        let point_light_program = program::Program::from_source(gl,
                                                               include_str!("shaders/light_pass.vert"),
                                                               &format!("{}\n{}",
                                                                       &include_str!("shaders/light_shared.frag"),
                                                                       &include_str!("shaders/point_light.frag")))?;
        let spot_light_program = program::Program::from_source(gl,
                                                               include_str!("shaders/light_pass.vert"),
                                                               &format!("{}\n{}\n{}",
                                                                       &include_str!("shaders/light_shared.frag"),
                                                                       &include_str!("shaders/shadow_shared.frag"),
                                                                       &include_str!("shaders/spot_light.frag")))?;

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

        let renderer = DeferredPipeline {
            gl: gl.clone(),
            ambient_light_program,
            directional_light_program,
            point_light_program,
            spot_light_program,
            full_screen,
            geometry_pass_texture: Some(Texture2DArray::new_empty(gl, 1, 1, 2,
                  Interpolation::Nearest, Interpolation::Nearest, Wrapping::ClampToEdge,
                  Wrapping::ClampToEdge, Format::RGBA8)?),
            geometry_pass_depth_texture: Some(Texture2DArray::new_empty(gl, 1, 1, 1,
                    Interpolation::Nearest, Interpolation::Nearest, Wrapping::ClampToEdge,
                    Wrapping::ClampToEdge, Format::Depth32F)?)};

        renderer.ambient_light_program.use_texture(renderer.geometry_pass_texture(), "gbuffer")?;
        renderer.ambient_light_program.use_texture(renderer.geometry_pass_depth_texture(), "depthMap")?;
        Ok(renderer)
    }

    pub fn geometry_pass(&mut self, width: usize, height: usize, render_scene: &dyn Fn()) -> Result<(), Error>
    {
        state::depth_write(&self.gl, true);
        state::depth_test(&self.gl, state::DepthTestType::LessOrEqual);
        state::cull(&self.gl, state::CullType::None);
        state::blend(&self.gl, state::BlendType::None);

        self.geometry_pass_texture = Some(Texture2DArray::new_empty(&self.gl, width, height, 2,
                  Interpolation::Nearest, Interpolation::Nearest, Wrapping::ClampToEdge,
                  Wrapping::ClampToEdge, Format::RGBA8)?);
        self.geometry_pass_depth_texture = Some(Texture2DArray::new_empty(&self.gl, width, height, 1,
                    Interpolation::Nearest, Interpolation::Nearest, Wrapping::ClampToEdge,
                    Wrapping::ClampToEdge, Format::Depth32F)?);
        RenderTarget::write_array(&self.gl,0, 0, width, height,
            Some(&vec4(0.0, 0.0, 0.0, 0.0)), Some(1.0),
            self.geometry_pass_texture.as_ref(), self.geometry_pass_depth_texture.as_ref(),
            2, &|channel| {channel},
            0, render_scene)?;
        Ok(())
    }

    pub fn light_pass(&self, camera: &Camera, ambient_light: Option<&AmbientLight>, directional_lights: &[&DirectionalLight], spot_lights: &[&SpotLight], point_lights: &[&PointLight]) -> Result<(), Error>
    {
        state::depth_write(&self.gl,false);
        state::depth_test(&self.gl, state::DepthTestType::None);
        state::cull(&self.gl,state::CullType::Back);
        state::blend(&self.gl, state::BlendType::None);

        // Ambient light
        if let Some(light) = ambient_light {

            self.ambient_light_program.use_texture(self.geometry_pass_texture(), "gbuffer")?;
            self.ambient_light_program.use_texture(self.geometry_pass_depth_texture(), "depthMap")?;

            self.ambient_light_program.add_uniform_vec3("ambientLight.base.color", &light.color())?;
            self.ambient_light_program.add_uniform_float("ambientLight.base.intensity", &light.intensity())?;

            self.ambient_light_program.use_attribute_vec3_float(&self.full_screen, "position", 0).unwrap();
            self.ambient_light_program.use_attribute_vec2_float(&self.full_screen, "uv_coordinate", 1).unwrap();
            self.ambient_light_program.draw_arrays(3);
            state::blend(&self.gl, state::BlendType::OneOne);
        }

        // Directional light
        for light in directional_lights {
            self.directional_light_program.use_texture(self.geometry_pass_texture(), "gbuffer")?;
            self.directional_light_program.use_texture(self.geometry_pass_depth_texture(), "depthMap")?;

            self.directional_light_program.add_uniform_vec3("eyePosition", &camera.position())?;
            self.directional_light_program.add_uniform_mat4("viewProjectionInverse", &(camera.get_projection() * camera.get_view()).invert().unwrap())?;

            if let Some(texture) = light.shadow_map() {
                self.directional_light_program.use_texture(texture, "shadowMap")?;
            }
            else {
                let dummy = Texture2D::new_empty(&self.gl, 1, 1, Interpolation::Nearest, Interpolation::Nearest, Wrapping::ClampToEdge, Wrapping::ClampToEdge, Format::Depth32F).unwrap();
                self.directional_light_program.use_texture(&dummy, "shadowMap")?;
            }
            self.directional_light_program.use_uniform_block(light.buffer(), "DirectionalLight");

            self.directional_light_program.use_attribute_vec3_float(&self.full_screen, "position", 0).unwrap();
            self.directional_light_program.use_attribute_vec2_float(&self.full_screen, "uv_coordinate", 1).unwrap();
            self.directional_light_program.draw_arrays(3);
            state::blend(&self.gl, state::BlendType::OneOne);
        }

        // Spot lights
        for light in spot_lights {
            self.spot_light_program.use_texture(self.geometry_pass_texture(), "gbuffer")?;
            self.spot_light_program.use_texture(self.geometry_pass_depth_texture(), "depthMap")?;

            self.spot_light_program.add_uniform_vec3("eyePosition", &camera.position())?;
            self.spot_light_program.add_uniform_mat4("viewProjectionInverse", &(camera.get_projection() * camera.get_view()).invert().unwrap())?;

            if let Some(texture) = light.shadow_map() {
                self.spot_light_program.use_texture(texture, "shadowMap")?;
            }
            else {
                let dummy = Texture2D::new_empty(&self.gl, 1, 1, Interpolation::Nearest, Interpolation::Nearest, Wrapping::ClampToEdge, Wrapping::ClampToEdge, Format::Depth32F).unwrap();
                self.spot_light_program.use_texture(&dummy, "shadowMap")?;
            }
            self.spot_light_program.use_uniform_block(light.buffer(), "SpotLight");

            self.spot_light_program.use_attribute_vec3_float(&self.full_screen, "position", 0).unwrap();
            self.spot_light_program.use_attribute_vec2_float(&self.full_screen, "uv_coordinate", 1).unwrap();
            self.spot_light_program.draw_arrays(3);
            state::blend(&self.gl, state::BlendType::OneOne);
        }

        // Point lights
        for light in point_lights {
            self.point_light_program.use_texture(self.geometry_pass_texture(), "gbuffer")?;
            self.point_light_program.use_texture(self.geometry_pass_depth_texture(), "depthMap")?;

            self.point_light_program.add_uniform_vec3("eyePosition", &camera.position())?;
            self.point_light_program.add_uniform_mat4("viewProjectionInverse", &(camera.get_projection() * camera.get_view()).invert().unwrap())?;

            self.point_light_program.use_uniform_block(light.buffer(), "PointLight");

            self.point_light_program.use_attribute_vec3_float(&self.full_screen, "position", 0).unwrap();
            self.point_light_program.use_attribute_vec2_float(&self.full_screen, "uv_coordinate", 1).unwrap();
            self.point_light_program.draw_arrays(3);
            state::blend(&self.gl, state::BlendType::OneOne);
        }

        Ok(())
    }

    pub fn geometry_pass_texture(&self) -> &Texture2DArray
    {
        &self.geometry_pass_texture.as_ref().unwrap()
    }
    pub fn geometry_pass_depth_texture(&self) -> &Texture2DArray
    {
        &self.geometry_pass_depth_texture.as_ref().unwrap()
    }
}