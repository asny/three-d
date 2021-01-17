
use crate::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DebugType {POSITION, NORMAL, COLOR, DEPTH, DIFFUSE, SPECULAR, POWER, NONE}

pub struct PhongDeferredPipeline {
    gl: Gl,
    ambient_light_effect: ImageEffect,
    directional_light_effect: ImageEffect,
    point_light_effect: ImageEffect,
    spot_light_effect: ImageEffect,
    debug_effect: Option<ImageEffect>,
    debug_type: DebugType,
    geometry_pass_texture: Option<Texture2DArray>,
    geometry_pass_depth_texture: Option<Texture2DArray>
}

impl PhongDeferredPipeline
{
    pub fn new(gl: &Gl) -> Result<Self, Error>
    {
        let renderer = Self {
            gl: gl.clone(),
            ambient_light_effect: ImageEffect::new(gl, &format!("{}\n{}\n{}",
                                                                       &include_str!("shaders/light_shared.frag"),
                                                                       &include_str!("shaders/deferred_light_shared.frag"),
                                                                       &include_str!("shaders/ambient_light.frag")))?,
            directional_light_effect: ImageEffect::new(gl, &format!("{}\n{}\n{}",
                                                                       &include_str!("shaders/light_shared.frag"),
                                                                       &include_str!("shaders/deferred_light_shared.frag"),
                                                                       &include_str!("shaders/directional_light.frag")))?,
            point_light_effect: ImageEffect::new(gl, &format!("{}\n{}\n{}",
                                                                       &include_str!("shaders/light_shared.frag"),
                                                                       &include_str!("shaders/deferred_light_shared.frag"),
                                                                       &include_str!("shaders/point_light.frag")))?,
            spot_light_effect: ImageEffect::new(gl, &format!("{}\n{}\n{}",
                                                                       &include_str!("shaders/light_shared.frag"),
                                                                       &include_str!("shaders/deferred_light_shared.frag"),
                                                                       &include_str!("shaders/spot_light.frag")))?,
            debug_effect: None,
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

    pub fn geometry_pass<F: FnOnce() -> Result<(), Error>>(&mut self, width: usize, height: usize, render_scene: F) -> Result<(), Error>
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

    pub fn light_pass(&self, camera: &Camera, ambient_light: Option<&AmbientLight>, directional_lights: &[&DirectionalLight],
                      spot_lights: &[&SpotLight], point_lights: &[&PointLight]) -> Result<(), Error>
    {
        state::depth_write(&self.gl,true);
        state::depth_test(&self.gl, state::DepthTestType::LessOrEqual);
        state::cull(&self.gl, state::CullType::Back);
        state::blend(&self.gl, state::BlendType::None);

        if self.debug_type != DebugType::NONE {
            self.debug_effect.as_ref().unwrap().program().add_uniform_mat4("viewProjectionInverse", &(camera.get_projection() * camera.get_view()).invert().unwrap())?;
            self.debug_effect.as_ref().unwrap().program().use_texture(self.geometry_pass_texture(), "gbuffer")?;
            self.debug_effect.as_ref().unwrap().program().use_texture(self.geometry_pass_depth_texture(), "depthMap")?;
            self.debug_effect.as_ref().unwrap().program().add_uniform_int("type", &(self.debug_type as i32))?;
            self.debug_effect.as_ref().unwrap().apply();
            return Ok(());
        }

        // Ambient light
        if let Some(light) = ambient_light {
            self.ambient_light_effect.program().use_texture(self.geometry_pass_texture(), "gbuffer")?;
            self.ambient_light_effect.program().use_texture(self.geometry_pass_depth_texture(), "depthMap")?;
            self.ambient_light_effect.program().add_uniform_mat4("viewProjectionInverse", &(camera.get_projection() * camera.get_view()).invert().unwrap())?;
            self.ambient_light_effect.program().add_uniform_vec3("ambientLight.color", &light.color())?;
            self.ambient_light_effect.program().add_uniform_float("ambientLight.intensity", &light.intensity())?;
            self.ambient_light_effect.apply();
            state::blend(&self.gl, state::BlendType::OneOne);
        }

        // Directional light
        for light in directional_lights {
            self.directional_light_effect.program().use_texture(self.geometry_pass_texture(), "gbuffer")?;
            self.directional_light_effect.program().use_texture(self.geometry_pass_depth_texture(), "depthMap")?;
            self.directional_light_effect.program().add_uniform_vec3("eyePosition", &camera.position())?;
            self.directional_light_effect.program().add_uniform_mat4("viewProjectionInverse", &(camera.get_projection() * camera.get_view()).invert().unwrap())?;
            self.directional_light_effect.program().use_texture(light.shadow_map(), "shadowMap")?;
            self.directional_light_effect.program().use_uniform_block(light.buffer(), "DirectionalLightUniform");
            self.directional_light_effect.apply();
            state::blend(&self.gl, state::BlendType::OneOne);
        }

        // Spot lights
        for light in spot_lights {
            self.spot_light_effect.program().use_texture(self.geometry_pass_texture(), "gbuffer")?;
            self.spot_light_effect.program().use_texture(self.geometry_pass_depth_texture(), "depthMap")?;
            self.spot_light_effect.program().add_uniform_vec3("eyePosition", &camera.position())?;
            self.spot_light_effect.program().add_uniform_mat4("viewProjectionInverse", &(camera.get_projection() * camera.get_view()).invert().unwrap())?;
            self.spot_light_effect.program().use_texture(light.shadow_map(), "shadowMap")?;
            self.spot_light_effect.program().use_uniform_block(light.buffer(), "SpotLightUniform");
            self.spot_light_effect.apply();
            state::blend(&self.gl, state::BlendType::OneOne);
        }

        // Point lights
        for light in point_lights {
            self.point_light_effect.program().use_texture(self.geometry_pass_texture(), "gbuffer")?;
            self.point_light_effect.program().use_texture(self.geometry_pass_depth_texture(), "depthMap")?;
            self.point_light_effect.program().add_uniform_vec3("eyePosition", &camera.position())?;
            self.point_light_effect.program().add_uniform_mat4("viewProjectionInverse", &(camera.get_projection() * camera.get_view()).invert().unwrap())?;
            self.point_light_effect.program().use_uniform_block(light.buffer(), "PointLightUniform");
            self.point_light_effect.apply();
            state::blend(&self.gl, state::BlendType::OneOne);
        }
        state::blend(&self.gl, state::BlendType::None);

        Ok(())
    }

    pub fn render_to_screen(&self, camera: &Camera, ambient_light: Option<&AmbientLight>, directional_lights: &[&DirectionalLight],
                       spot_lights: &[&SpotLight], point_lights: &[&PointLight], width: usize, height: usize) -> Result<(), Error>
    {
        Ok(self.render_to_screen_with_forward_pass(camera, ambient_light, directional_lights, spot_lights, point_lights, width, height, || {Ok(())})?)
    }

    pub fn render_to_screen_with_forward_pass<F: FnOnce() -> Result<(), Error>>(&self, camera: &Camera,
                       ambient_light: Option<&AmbientLight>, directional_lights: &[&DirectionalLight],
                       spot_lights: &[&SpotLight], point_lights: &[&PointLight], width: usize, height: usize,
                       forward_pass: F) -> Result<(), Error>
    {
        Ok(Screen::write(&self.gl, 0, 0, width, height,
                         Some(&vec4(0.0, 0.0, 0.0, 1.0)),
                         Some(1.0),
                         || {
            self.light_pass(camera, ambient_light, directional_lights, spot_lights, point_lights)?;
            forward_pass()?;
            Ok(())
        })?)
    }

    pub fn geometry_pass_texture(&self) -> &Texture2DArray
    {
        &self.geometry_pass_texture.as_ref().unwrap()
    }
    pub fn geometry_pass_depth_texture(&self) -> &Texture2DArray
    {
        &self.geometry_pass_depth_texture.as_ref().unwrap()
    }

    pub fn debug_type(&self) -> DebugType
    {
        self.debug_type
    }

    pub fn set_debug_type(&mut self, debug_type: DebugType)
    {
        self.debug_type = debug_type;
        if self.debug_effect.is_none() {
            self.debug_effect = Some(ImageEffect::new(&self.gl, include_str!("shaders/debug.frag")).unwrap());
        }
    }

    pub fn next_debug_type(&mut self)
    {
        let debug_type =
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
        self.set_debug_type(debug_type);
    }
}