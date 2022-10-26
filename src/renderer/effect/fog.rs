use crate::renderer::*;

///
/// An effect that simulates fog, ie. the area where it is applied gets hazy when objects are far away.
///
pub struct FogEffect {
    context: Context,
    /// The color of the fog.
    pub color: Color,
    /// The density of the fog.
    pub density: f32,
    /// Determines the variation on the density as a function of time.
    pub animation: f32,
}

impl FogEffect {
    ///
    /// Constructs a new fog effect.
    ///
    pub fn new(context: &Context, color: Color, density: f32, animation: f32) -> Self {
        Self {
            context: context.clone(),
            color,
            density,
            animation,
        }
    }

    ///
    /// Apply the fog effect on the current render target based on the given depth texture.
    /// Must be called in the callback given as input to a [RenderTarget], [ColorTarget] or [DepthTarget] write method.
    ///
    pub fn render(&self, time: f64, camera: &Camera, depth_texture: DepthTexture) {
        self.context.apply_effect(
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
