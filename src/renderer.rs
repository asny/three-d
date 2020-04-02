
use crate::*;

#[derive(Debug)]
pub enum Error {
    Core(core::Error)
}

impl From<core::Error> for Error {
    fn from(other: core::Error) -> Self {
        Error::Core(other)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DebugType {NONE, POSITION, NORMAL, COLOR, DEPTH, DIFFUSE, SPECULAR, POWER}

pub struct DeferredPipeline {
    gl: Gl,
    ambient_light_effect: ImageEffect,
    directional_light_effect: ImageEffect,
    point_light_effect: ImageEffect,
    spot_light_effect: ImageEffect,
    debug_effect: ImageEffect,
    pub debug_type: DebugType,
    geometry_pass_texture: Option<Texture2DArray>,
    geometry_pass_depth_texture: Option<Texture2DArray>
}


impl DeferredPipeline
{
    pub fn new(gl: &Gl) -> Result<DeferredPipeline, Error>
    {
        let renderer = DeferredPipeline {
            gl: gl.clone(),
            ambient_light_effect: ImageEffect::new(gl, include_str!("shaders/ambient_light.frag"))?,
            directional_light_effect: ImageEffect::new(gl, &format!("{}\n{}\n{}",
                                                                       &include_str!("shaders/light_shared.frag"),
                                                                       &include_str!("shaders/shadow_shared.frag"),
                                                                       &include_str!("shaders/directional_light.frag")))?,
            point_light_effect: ImageEffect::new(gl, &format!("{}\n{}",
                                                                       &include_str!("shaders/light_shared.frag"),
                                                                       &include_str!("shaders/point_light.frag")))?,
            spot_light_effect: ImageEffect::new(gl, &format!("{}\n{}\n{}",
                                                                       &include_str!("shaders/light_shared.frag"),
                                                                       &include_str!("shaders/shadow_shared.frag"),
                                                                       &include_str!("shaders/spot_light.frag")))?,
            debug_effect: ImageEffect::new(gl, include_str!("shaders/debug.frag"))?,
            debug_type: DebugType::NONE,
            geometry_pass_texture: Some(Texture2DArray::new(gl, 1, 1, 2,
                  Interpolation::Nearest, Interpolation::Nearest, None, Wrapping::ClampToEdge,
                  Wrapping::ClampToEdge, Format::RGBA8)?),
            geometry_pass_depth_texture: Some(Texture2DArray::new(gl, 1, 1, 1,
                    Interpolation::Nearest, Interpolation::Nearest, None,Wrapping::ClampToEdge,
                    Wrapping::ClampToEdge, Format::Depth32F)?)
        };

        renderer.ambient_light_effect.program().use_texture(renderer.geometry_pass_texture(), "gbuffer")?;
        renderer.ambient_light_effect.program().use_texture(renderer.geometry_pass_depth_texture(), "depthMap")?;
        Ok(renderer)
    }

    pub fn next_debug_type(&mut self)
    {
        self.debug_type =
            match self.debug_type {
                DebugType::NONE => DebugType::POSITION,
                DebugType::POSITION => DebugType::NORMAL,
                DebugType::NORMAL => DebugType::COLOR,
                DebugType::COLOR => DebugType::DEPTH,
                DebugType::DEPTH => DebugType::DIFFUSE,
                DebugType::DIFFUSE => DebugType::SPECULAR,
                DebugType::SPECULAR => DebugType::POWER,
                DebugType::POWER => DebugType::NONE,
            };
    }

    pub fn geometry_pass(&mut self, width: usize, height: usize, render_scene: &dyn Fn()) -> Result<(), Error>
    {
        state::depth_write(&self.gl, true);
        state::depth_test(&self.gl, state::DepthTestType::LessOrEqual);
        state::cull(&self.gl, state::CullType::None);
        state::blend(&self.gl, state::BlendType::None);

        self.geometry_pass_texture = Some(Texture2DArray::new(&self.gl, width, height, 2,
                  Interpolation::Nearest, Interpolation::Nearest, None, Wrapping::ClampToEdge,
                  Wrapping::ClampToEdge, Format::RGBA8)?);
        self.geometry_pass_depth_texture = Some(Texture2DArray::new(&self.gl, width, height, 1,
                    Interpolation::Nearest, Interpolation::Nearest, None, Wrapping::ClampToEdge,
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
        state::blend(&self.gl, state::BlendType::None);

        if self.debug_type != DebugType::NONE {
            self.debug_effect.program().add_uniform_mat4("viewProjectionInverse", &(camera.get_projection() * camera.get_view()).invert().unwrap())?;
            self.debug_effect.program().use_texture(self.geometry_pass_texture(), "gbuffer")?;
            self.debug_effect.program().use_texture(self.geometry_pass_depth_texture(), "depthMap")?;
            self.debug_effect.program().add_uniform_int("type", &(self.debug_type as i32))?;

            self.debug_effect.apply();
            return Ok(());
        }

        // Ambient light
        if let Some(light) = ambient_light {
            self.ambient_light_effect.program().use_texture(self.geometry_pass_texture(), "gbuffer")?;
            self.ambient_light_effect.program().use_texture(self.geometry_pass_depth_texture(), "depthMap")?;
            self.ambient_light_effect.program().add_uniform_vec3("ambientLight.base.color", &light.color())?;
            self.ambient_light_effect.program().add_uniform_float("ambientLight.base.intensity", &light.intensity())?;
            self.ambient_light_effect.apply();
            state::blend(&self.gl, state::BlendType::OneOne);
        }

        // Directional light
        for light in directional_lights {
            self.directional_light_effect.program().use_texture(self.geometry_pass_texture(), "gbuffer")?;
            self.directional_light_effect.program().use_texture(self.geometry_pass_depth_texture(), "depthMap")?;
            self.directional_light_effect.program().add_uniform_vec3("eyePosition", &camera.position())?;
            self.directional_light_effect.program().add_uniform_mat4("viewProjectionInverse", &(camera.get_projection() * camera.get_view()).invert().unwrap())?;
            if let Some(texture) = light.shadow_map() {
                self.directional_light_effect.program().use_texture(texture, "shadowMap")?;
            }
            else {
                let dummy = Texture2D::new(&self.gl, 1, 1, Interpolation::Nearest, Interpolation::Nearest, None,Wrapping::ClampToEdge, Wrapping::ClampToEdge, Format::Depth32F).unwrap();
                self.directional_light_effect.program().use_texture(&dummy, "shadowMap")?;
            }
            self.directional_light_effect.program().use_uniform_block(light.buffer(), "DirectionalLight");
            self.directional_light_effect.apply();
            state::blend(&self.gl, state::BlendType::OneOne);
        }

        // Spot lights
        for light in spot_lights {
            self.spot_light_effect.program().use_texture(self.geometry_pass_texture(), "gbuffer")?;
            self.spot_light_effect.program().use_texture(self.geometry_pass_depth_texture(), "depthMap")?;
            self.spot_light_effect.program().add_uniform_vec3("eyePosition", &camera.position())?;
            self.spot_light_effect.program().add_uniform_mat4("viewProjectionInverse", &(camera.get_projection() * camera.get_view()).invert().unwrap())?;
            if let Some(texture) = light.shadow_map() {
                self.spot_light_effect.program().use_texture(texture, "shadowMap")?;
            }
            else {
                let dummy = Texture2D::new(&self.gl, 1, 1, Interpolation::Nearest, Interpolation::Nearest, None,Wrapping::ClampToEdge, Wrapping::ClampToEdge, Format::Depth32F).unwrap();
                self.spot_light_effect.program().use_texture(&dummy, "shadowMap")?;
            }
            self.spot_light_effect.program().use_uniform_block(light.buffer(), "SpotLight");
            self.spot_light_effect.apply();
            state::blend(&self.gl, state::BlendType::OneOne);
        }

        // Point lights
        for light in point_lights {
            self.point_light_effect.program().use_texture(self.geometry_pass_texture(), "gbuffer")?;
            self.point_light_effect.program().use_texture(self.geometry_pass_depth_texture(), "depthMap")?;
            self.point_light_effect.program().add_uniform_vec3("eyePosition", &camera.position())?;
            self.point_light_effect.program().add_uniform_mat4("viewProjectionInverse", &(camera.get_projection() * camera.get_view()).invert().unwrap())?;
            self.point_light_effect.program().use_uniform_block(light.buffer(), "PointLight");
            self.point_light_effect.apply();
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