//!
//! A collection of image based effects, ie. effects applied to each pixel of a rendered image.
//!

macro_rules! impl_effect_body {
    ($inner:ident) => {
        fn fragment_shader_source(
            &self,
            lights: &[&dyn Light],
            color_texture: Option<ColorTexture>,
            depth_texture: Option<DepthTexture>,
        ) -> String {
            self.$inner()
                .fragment_shader_source(lights, color_texture, depth_texture)
        }

        fn id(&self) -> u16 {
            self.$inner().id()
        }

        fn fragment_attributes(&self) -> FragmentAttributes {
            self.$inner().fragment_attributes()
        }

        fn use_uniforms(
            &self,
            program: &Program,
            camera: &Camera,
            lights: &[&dyn Light],
            color_texture: Option<ColorTexture>,
            depth_texture: Option<DepthTexture>,
        ) {
            self.$inner()
                .use_uniforms(program, camera, lights, color_texture, depth_texture)
        }

        fn render_states(&self) -> RenderStates {
            self.$inner().render_states()
        }

        fn material_type(&self) -> MaterialType {
            self.$inner().material_type()
        }
    };
}

mod fog;
#[doc(inline)]
pub use fog::*;

mod fxaa;
#[doc(inline)]
pub use fxaa::*;

mod water;
#[doc(inline)]
pub use water::*;

use crate::renderer::*;
use std::ops::Deref;

impl ColorTexture<'_> {
    ///
    /// Returns the fragment shader source for using this texture in a shader.
    ///
    pub fn fragment_shader_source(&self) -> String {
        match self {
            Self::Single(_) => "
                uniform sampler2D colorMap;
                vec4 sample_color(vec2 uv)
                {
                    return texture(colorMap, uv);
                }"
            .to_owned(),
            Self::Array { .. } => "
                uniform sampler2DArray colorMap;
                uniform int colorLayers[4];
                vec4 sample_color(vec2 uv)
                {
                    return texture(colorMap, vec3(uv, colorLayers[0]));
                }
                vec4 sample_layer(vec2 uv, int index)
                {
                    return texture(colorMap, vec3(uv, colorLayers[index]));
                }"
            .to_owned(),
            Self::CubeMap { .. } => unimplemented!(),
        }
    }

    ///
    /// Sends the uniform data needed for this texture to the fragment shader.
    ///
    pub fn use_uniforms(&self, program: &Program) {
        match self {
            Self::Single(texture) => program.use_texture("colorMap", texture),
            Self::Array { texture, layers } => {
                let mut la: [i32; 4] = [0; 4];
                layers
                    .iter()
                    .enumerate()
                    .for_each(|(i, l)| la[i] = *l as i32);
                program.use_uniform_array("colorLayers", &la);
                program.use_texture_array("colorMap", texture);
            }
            Self::CubeMap { .. } => unimplemented!(),
        }
    }

    ///
    /// The resolution of the underlying texture if there is any.
    ///
    pub fn resolution(&self) -> (u32, u32) {
        match self {
            Self::Single(texture) => (texture.width(), texture.height()),
            Self::Array { texture, .. } => (texture.width(), texture.height()),
            Self::CubeMap { texture, .. } => (texture.width(), texture.height()),
        }
    }
}

impl DepthTexture<'_> {
    ///
    /// Returns the fragment shader source for using this texture in a shader.
    ///
    pub fn fragment_shader_source(&self) -> String {
        match self {
            Self::Single { .. } => "
                uniform sampler2D depthMap;
                float sample_depth(vec2 uv)
                {
                    return texture(depthMap, uv).x;
                }"
            .to_owned(),
            Self::Array { .. } => "
                uniform sampler2DArray depthMap;
                uniform int depthLayer;
                float sample_depth(vec2 uv)
                {
                    return texture(depthMap, vec3(uv, depthLayer)).x;
                }"
            .to_owned(),
            Self::CubeMap { .. } => {
                unimplemented!()
            }
        }
    }

    ///
    /// Sends the uniform data needed for this texture to the fragment shader.
    ///
    pub fn use_uniforms(&self, program: &Program) {
        match self {
            Self::Single(texture) => program.use_depth_texture("depthMap", texture),
            Self::Array { texture, layer } => {
                program.use_uniform("depthLayer", layer);
                program.use_depth_texture_array("depthMap", texture);
            }
            Self::CubeMap { .. } => unimplemented!(),
        }
    }

    ///
    /// The resolution of the underlying texture if there is any.
    ///
    pub fn resolution(&self) -> (u32, u32) {
        match self {
            Self::Single(texture) => (texture.width(), texture.height()),
            Self::Array { texture, .. } => (texture.width(), texture.height()),
            Self::CubeMap { texture, .. } => (texture.width(), texture.height()),
        }
    }
}

///
/// Similar to [Material], the difference is that this type of material needs the rendered color texture and/or depth texture of the scene to be applied.
/// Therefore this type of material is always applied one at a time and after the scene has been rendered with the regular [Material].
///
pub trait Effect {
    ///
    /// Returns the fragment shader source for this material.
    ///
    fn fragment_shader_source(
        &self,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) -> String;

    ///
    /// Returns a unique ID for each variation of the shader source returned from `Effect::fragment_shader_source`.
    ///
    /// **Note:** The first 16 bits are reserved to internally implemented materials, so if implementing the `Effect` trait
    /// outside of this crate, always return an id that is larger than or equal to `0b1u16 << 16`.
    ///
    fn id(&self) -> u16;

    ///
    /// Returns a [FragmentAttributes] struct that describes which fragment attributes,
    /// ie. the input from the vertex shader, are required for rendering with this material.
    ///
    fn fragment_attributes(&self) -> FragmentAttributes;

    ///
    /// Sends the uniform data needed for this material to the fragment shader.
    ///
    fn use_uniforms(
        &self,
        program: &Program,
        camera: &Camera,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    );

    ///
    /// Returns the render states needed to render with this material.
    ///
    fn render_states(&self) -> RenderStates;

    ///
    /// Returns the type of material.
    ///
    fn material_type(&self) -> MaterialType;
}

impl<T: Effect + ?Sized> Effect for &T {
    impl_effect_body!(deref);
}

impl<T: Effect + ?Sized> Effect for &mut T {
    impl_effect_body!(deref);
}

impl<T: Effect> Effect for Box<T> {
    impl_effect_body!(as_ref);
}

impl<T: Effect> Effect for std::rc::Rc<T> {
    impl_effect_body!(as_ref);
}

impl<T: Effect> Effect for std::sync::Arc<T> {
    impl_effect_body!(as_ref);
}

impl<T: Effect> Effect for std::cell::RefCell<T> {
    impl_effect_body!(borrow);
}

impl<T: Effect> Effect for std::sync::RwLock<T> {
    fn fragment_shader_source(
        &self,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) -> String {
        self.read()
            .unwrap()
            .fragment_shader_source(lights, color_texture, depth_texture)
    }

    fn id(&self) -> u16 {
        self.read().unwrap().id()
    }

    fn fragment_attributes(&self) -> FragmentAttributes {
        self.read().unwrap().fragment_attributes()
    }

    fn use_uniforms(
        &self,
        program: &Program,
        camera: &Camera,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) {
        self.read()
            .unwrap()
            .use_uniforms(program, camera, lights, color_texture, depth_texture)
    }
    fn render_states(&self) -> RenderStates {
        self.read().unwrap().render_states()
    }

    fn material_type(&self) -> MaterialType {
        self.read().unwrap().material_type()
    }
}
