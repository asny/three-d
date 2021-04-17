use crate::core::*;
use crate::effect::*;
use crate::math::*;

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
    pub fn new(gl: &Context) -> Result<Self, Error> {
        Ok(Self {
            color: vec3(0.8, 0.8, 0.8),
            density: 0.2,
            animation: 0.1,
            image_effect: ImageEffect::new(gl, include_str!("shaders/fxaa.frag"))?,
        })
    }

    pub fn apply(&self, viewport: Viewport, color_texture: &dyn Texture) -> Result<(), Error> {
        let render_states = RenderStates {
            write_mask: WriteMask::COLOR,
            depth_test: DepthTestType::Always,
            ..Default::default()
        };

        self.image_effect.use_texture(color_texture, "colorMap")?;
        self.image_effect.use_uniform_vec2(
            "resolution",
            &vec2(color_texture.width() as f32, color_texture.height() as f32),
        )?;

        self.image_effect.apply(render_states, viewport)?;
        Ok(())
    }
}
