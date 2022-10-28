use crate::renderer::*;

///
/// An effect that simulates fog, ie. the area where it is applied gets hazy when objects are far away.
///
#[derive(Clone, Debug)]
pub struct FogEffect {
    /// The color of the fog.
    pub color: Color,
    /// The density of the fog.
    pub density: f32,
    /// Determines the variation on the density as a function of time.
    pub animation: f32,
}

impl Default for FogEffect {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            density: 0.2,
            animation: 0.0,
        }
    }
}

impl FogEffect {
    ///
    /// Apply the fog effect on the current render target based on the given depth texture.
    /// Must be called in the callback given as input to a [RenderTarget], [ColorTarget] or [DepthTarget] write method.
    ///
    pub fn apply(
        &self,
        context: &Context,
        time: f64,
        camera: &Camera,
        depth_texture: DepthTexture,
    ) {
        apply_effect(
            context,
            &format!(
                "{}\n{}\n{}",
                include_str!("../../core/shared.frag"),
                depth_texture.fragment_shader_source(),
                include_str!("shaders/fog_effect.frag")
            ),
            RenderStates {
                write_mask: WriteMask::COLOR,
                blend: Blend::TRANSPARENCY,
                cull: Cull::Back,
                ..Default::default()
            },
            camera.viewport(),
            |program| {
                depth_texture.use_uniforms(program);
                program.use_uniform(
                    "viewProjectionInverse",
                    (camera.projection() * camera.view()).invert().unwrap(),
                );
                program.use_uniform("fogColor", self.color);
                program.use_uniform("fogDensity", self.density);
                program.use_uniform("animation", self.animation);
                program.use_uniform("time", 0.001 * time as f32);
                program.use_uniform("eyePosition", camera.position());
            },
        )
    }
}
