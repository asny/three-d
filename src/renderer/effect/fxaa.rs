use crate::core::*;

///
/// A simple anti-aliasing approach which smooths otherwise jagged edges (for example lines) but also
/// smooths the rest of the image.
///
pub struct FXAAEffect {
    pub color: Vec3,
    pub density: f32,
    pub animation: f32,
    image_effect: ImageEffect,
}

impl FXAAEffect {
    pub fn new(gl: &Context) -> Result<Self> {
        Ok(Self {
            color: vec3(0.8, 0.8, 0.8),
            density: 0.2,
            animation: 0.1,
            image_effect: ImageEffect::new(gl, include_str!("shaders/fxaa.frag"))?,
        })
    }

    pub fn apply(&self, viewport: Viewport, color_texture: &impl Texture) -> Result<()> {
        let render_states = RenderStates {
            write_mask: WriteMask::COLOR,
            depth_test: DepthTest::Always,
            cull: Cull::Back,
            ..Default::default()
        };

        self.image_effect.use_texture("colorMap", color_texture)?;
        self.image_effect.use_uniform_vec2(
            "resolution",
            &vec2(color_texture.width() as f32, color_texture.height() as f32),
        )?;

        self.image_effect.apply(render_states, viewport)?;
        Ok(())
    }
}
