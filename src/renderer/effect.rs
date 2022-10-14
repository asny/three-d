//!
//! A collection of image based effects, ie. effects applied to each pixel of a rendered image.
//!

mod fxaa;
#[doc(inline)]
pub use fxaa::*;

mod copy;
#[doc(inline)]
pub use copy::*;

use crate::renderer::*;

pub trait PostMaterial {
    ///
    /// Returns the fragment shader source for this material. Should output the final fragment color.
    ///
    fn fragment_shader_source(
        &self,
        lights: &[&dyn Light],
        color_texture: Option<&Texture2D>,
        depth_texture: Option<&DepthTargetTexture2D>,
    ) -> String;

    ///
    /// Sends the uniform data needed for this material to the fragment shader.
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
    /// Returns the render states needed to render with this material.
    ///
    fn render_states(&self) -> RenderStates;
}
