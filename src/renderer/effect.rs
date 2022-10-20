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

#[derive(Clone, Copy)]
pub enum ColorTexture<'a> {
    None,
    Single(&'a Texture2D),
    Array(&'a Texture2DArray, u32),
}

impl ColorTexture<'_> {
    ///
    /// Returns the fragment shader source for using this texture in a shader.
    ///
    pub fn fragment_shader_source(&self) -> Option<String> {
        match self {
            Self::None => None,
            Self::Single(_) => Some(
                "
                uniform sampler2D colorMap;
                vec4 sample_color(vec2 uv)
                {
                    return texture(colorMap, uv);
                }"
                .to_owned(),
            ),
            Self::Array(_, _) => Some(
                "
                uniform sampler2DArray colorMap;
                uniform int colorLayer;
                vec4 sample_color(vec2 uv)
                {
                    return texture(colorMap, vec3(uv, colorLayer));
                }"
                .to_owned(),
            ),
        }
    }

    ///
    /// Sends the uniform data needed for this texture to the fragment shader.
    ///
    pub fn use_uniforms(&self, program: &Program) {
        match self {
            Self::None => {}
            Self::Single(tex) => program.use_texture("colorMap", tex),
            Self::Array(tex, layer) => {
                program.use_uniform("colorLayer", layer);
                program.use_texture_array("colorMap", tex);
            }
        }
    }

    pub fn resolution(&self) -> Option<(u32, u32)> {
        match self {
            Self::None => None,
            Self::Single(tex) => Some((tex.width(), tex.height())),
            Self::Array(tex, _) => Some((tex.width(), tex.height())),
        }
    }
}

#[derive(Clone, Copy)]
pub enum DepthTexture<'a> {
    None,
    Single(&'a DepthTargetTexture2D),
    Array(&'a DepthTargetTexture2DArray, u32),
}

impl DepthTexture<'_> {
    ///
    /// Returns the fragment shader source for using this texture in a shader.
    ///
    pub fn fragment_shader_source(&self) -> Option<String> {
        match self {
            Self::None => None,
            Self::Single(_) => Some(
                "
                uniform sampler2D depthMap;
                float sample_depth(vec2 uv)
                {
                    return texture(depthMap, uv).x;
                }"
                .to_owned(),
            ),
            Self::Array(_, _) => Some(
                "
                uniform sampler2DArray depthMap;
                uniform int depthLayer;
                float sample_depth(vec2 uv)
                {
                    return texture(depthMap, vec3(uv, depthLayer)).x;
                }"
                .to_owned(),
            ),
        }
    }

    ///
    /// Sends the uniform data needed for this texture to the fragment shader.
    ///
    pub fn use_uniforms(&self, program: &Program) {
        match self {
            Self::None => {}
            Self::Single(tex) => program.use_depth_texture("depthMap", tex),
            Self::Array(tex, layer) => {
                program.use_uniform("depthLayer", layer);
                program.use_depth_texture_array("depthMap", tex);
            }
        }
    }

    pub fn resolution(&self) -> Option<(u32, u32)> {
        match self {
            Self::None => None,
            Self::Single(tex) => Some((tex.width(), tex.height())),
            Self::Array(tex, _) => Some((tex.width(), tex.height())),
        }
    }
}

pub trait PostMaterial {
    ///
    /// Returns the fragment shader source for this material. Should output the final fragment color.
    ///
    fn fragment_shader_source(
        &self,
        lights: &[&dyn Light],
        color_texture: ColorTexture,
        depth_texture: DepthTexture,
    ) -> String;

    ///
    /// Sends the uniform data needed for this material to the fragment shader.
    ///
    fn use_uniforms(
        &self,
        program: &Program,
        camera: &Camera,
        lights: &[&dyn Light],
        color_texture: ColorTexture,
        depth_texture: DepthTexture,
    );

    ///
    /// Returns the render states needed to render with this material.
    ///
    fn render_states(&self) -> RenderStates;
}
