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
    /// The time used for the animation.
    pub time: f32,
}

impl Default for FogEffect {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            density: 0.2,
            animation: 1.0,
            time: 0.0,
        }
    }
}

impl FogEffect {
    ///
    /// Apply the fog effect on the current render target based on the given depth texture.
    /// Must be called in the callback given as input to a [RenderTarget], [ColorTarget] or [DepthTarget] write method.
    ///
    #[deprecated = "use `apply_screen_effect` instead"]
    pub fn apply(
        &self,
        context: &Context,
        time: f64,
        camera: &Camera,
        depth_texture: DepthTexture,
    ) {
        let mut effect = self.clone();
        effect.time = time as f32;
        apply_screen_effect(context, effect, camera, &[], None, Some(depth_texture));
    }
}

impl Effect for FogEffect {
    fn fragment_shader_source(
        &self,
        _lights: &[&dyn Light],
        _color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) -> String {
        format!(
            "{}\n{}\n{}",
            include_str!("../../core/shared.frag"),
            depth_texture
                .expect("Must supply a depth texture to apply a fog effect")
                .fragment_shader_source(),
            include_str!("shaders/fog_effect.frag")
        )
    }

    fn id(&self, _color_texture: Option<ColorTexture>, depth_texture: Option<DepthTexture>) -> u16 {
        0b1u16 << 14
            | 0b1u16 << 13
            | 0b1u16 << 12
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
        _color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) {
        depth_texture
            .expect("Must supply a depth texture to apply a fog effect")
            .use_uniforms(program);
        program.use_uniform(
            "viewProjectionInverse",
            (camera.projection() * camera.view()).invert().unwrap(),
        );
        program.use_uniform("fogColor", self.color);
        program.use_uniform("fogDensity", self.density);
        program.use_uniform("animation", self.animation);
        program.use_uniform("time", 0.001 * self.time);
        program.use_uniform("eyePosition", camera.position());
    }

    fn render_states(&self) -> RenderStates {
        RenderStates {
            write_mask: WriteMask::COLOR,
            blend: Blend::TRANSPARENCY,
            cull: Cull::Back,
            ..Default::default()
        }
    }
}
