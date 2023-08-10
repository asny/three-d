use crate::renderer::*;

///
/// An effect that simulates fog, ie. the area where it is applied gets hazy when objects are far away.
///
#[derive(Clone, Debug)]
pub struct FogEffect {
    /// The color of the fog.
    pub color: Srgba,
    /// The density of the fog.
    pub density: f32,
    /// Determines the variation on the density as a function of time.
    pub animation: f32,
    /// The time used for the animation.
    pub time: f32,
}

impl Default for FogEffect {
    fn default() -> Self {
        Self {
            color: Srgba::WHITE,
            density: 0.2,
            animation: 1.0,
            time: 0.0,
        }
    }
}

impl Effect for FogEffect {
    fn fragment_shader_source(
        &self,
        _lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) -> String {
        format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            include_str!("../../core/shared.frag"),
            color_texture
                .expect("Must supply a depth texture to apply a fog effect")
                .fragment_shader_source(),
            depth_texture
                .expect("Must supply a depth texture to apply a fog effect")
                .fragment_shader_source(),
            ToneMapping::fragment_shader_source(),
            ColorMapping::fragment_shader_source(),
            include_str!("shaders/fog_effect.frag")
        )
    }

    fn id(&self, color_texture: Option<ColorTexture>, depth_texture: Option<DepthTexture>) -> u16 {
        0b1u16 << 14
            | 0b1u16 << 13
            | 0b1u16 << 12
            | color_texture
                .expect("Must supply a color texture to apply a fog effect")
                .id()
            | depth_texture
                .expect("Must supply a depth texture to apply a fog effect")
                .id()
    }

    fn fragment_attributes(&self) -> FragmentAttributes {
        FragmentAttributes {
            uv: true,
            ..FragmentAttributes::NONE
        }
    }

    fn use_uniforms(
        &self,
        program: &Program,
        camera: &Camera,
        _lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) {
        camera.tone_mapping.use_uniforms(program);
        camera.color_mapping.use_uniforms(program);
        color_texture
            .expect("Must supply a color texture to apply a fog effect")
            .use_uniforms(program);
        depth_texture
            .expect("Must supply a depth texture to apply a fog effect")
            .use_uniforms(program);
        program.use_uniform(
            "viewProjectionInverse",
            (camera.projection() * camera.view()).invert().unwrap(),
        );
        program.use_uniform("fogColor", Vec4::from(self.color));
        program.use_uniform("fogDensity", self.density);
        program.use_uniform("animation", self.animation);
        program.use_uniform("time", 0.001 * self.time);
        program.use_uniform("eyePosition", camera.position());
    }

    fn render_states(&self) -> RenderStates {
        RenderStates {
            depth_test: DepthTest::Always,
            cull: Cull::Back,
            ..Default::default()
        }
    }
}
