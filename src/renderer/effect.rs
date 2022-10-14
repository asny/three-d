//!
//! A collection of image based effects, ie. effects applied to each pixel of a rendered image.
//!

mod fog;
#[doc(inline)]
pub use fog::*;

mod fxaa;
#[doc(inline)]
pub use fxaa::*;

use crate::renderer::*;

pub trait EffectMaterial {
    ///
    /// Returns the fragment shader source for this effect. Should output the final fragment color.
    ///
    fn fragment_shader_source(&self, lights: &[&dyn Light]) -> String;

    ///
    /// Sends the uniform data needed for this effect to the fragment shader.
    ///
    fn use_uniforms(
        &self,
        program: &Program,
        camera: &Camera,
        lights: &[&dyn Light],
        color_texture: Option<&Texture2D>,
        depth_texture: Option<&DepthTargetTexture2D>,
    );

    ///
    /// Returns the render states needed to render with this effect.
    ///
    fn render_states(&self) -> RenderStates;
}
