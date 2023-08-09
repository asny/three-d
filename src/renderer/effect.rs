//!
//! A collection of effects implementing the [Effect] trait.
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

        fn id(
            &self,
            color_texture: Option<ColorTexture>,
            depth_texture: Option<DepthTexture>,
        ) -> u16 {
            self.$inner().id(color_texture, depth_texture)
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
    };
}

mod fog;
#[doc(inline)]
pub use fog::*;

mod copy;
#[doc(inline)]
pub use copy::*;

mod full_screen;
#[doc(inline)]
pub use full_screen::*;

mod fxaa;
#[doc(inline)]
pub use fxaa::*;

mod water;
#[doc(inline)]
pub use water::*;

pub(crate) mod lighting_pass;

use crate::renderer::*;
use std::ops::Deref;

///
/// Similar to [Material], the difference is that an effect needs the rendered color texture and/or depth texture of the scene to be applied.
/// Therefore an effect is always applied one at a time and after the scene has been rendered with the regular [Material].
///
pub trait Effect {
    ///
    /// Returns the fragment shader source for this effect.
    ///
    fn fragment_shader_source(
        &self,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) -> String;

    ///
    /// Returns a unique ID for each variation of the shader source returned from [Effect::fragment_shader_source].
    ///
    /// **Note:** The first 16 bits are reserved to internally implemented effects, so if implementing the [Effect] trait
    /// outside of this crate, always return an id that is larger than or equal to `0b1u16 << 16`.
    ///
    fn id(&self, color_texture: Option<ColorTexture>, depth_texture: Option<DepthTexture>) -> u16;

    ///
    /// Returns a [FragmentAttributes] struct that describes which fragment attributes,
    /// ie. the input from the vertex shader, are required for rendering with this effect.
    ///
    fn fragment_attributes(&self) -> FragmentAttributes;

    ///
    /// Sends the uniform data needed for this effect to the fragment shader.
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
    /// Returns the render states needed to render with this effect.
    ///
    fn render_states(&self) -> RenderStates;
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

    fn id(&self, color_texture: Option<ColorTexture>, depth_texture: Option<DepthTexture>) -> u16 {
        self.read().unwrap().id(color_texture, depth_texture)
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
}
