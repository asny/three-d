use crate::core::*;
use crate::renderer::*;

pub use crate::core::{CPUMaterial, Color};

mod color_material;
#[doc(inline)]
pub use color_material::*;

mod depth_material;
#[doc(inline)]
pub use depth_material::*;

mod normal_material;
#[doc(inline)]
pub use normal_material::*;

mod orm_material;
#[doc(inline)]
pub use orm_material::*;

mod position_material;
#[doc(inline)]
pub use position_material::*;

mod uv_material;
#[doc(inline)]
pub use uv_material::*;

mod physical_material;
#[doc(inline)]
pub use physical_material::*;

mod deferred_physical_material;
#[doc(inline)]
pub use deferred_physical_material::*;

///
/// Represents a material that can be applied to a [Shadable] object.
///
/// The material can use the attributes position (in world space) by adding `in vec3 pos;`,
/// normal by `in vec3 nor;`, uv coordinates by `in vec2 uvs;` and color by `in vec4 col;` to the fragment shader source code.
///
pub trait ForwardMaterial {
    /// Returns the fragment shader source for this material. Should output the final fragment color.
    fn fragment_shader_source(&self, use_vertex_colors: bool, lights: &Lights) -> String;
    /// Sends the uniform data needed for this material to the fragment shader.
    fn use_uniforms(&self, program: &Program, camera: &Camera, lights: &Lights)
        -> ThreeDResult<()>;
    /// Returns the render states needed to render with this material.
    fn render_states(&self) -> RenderStates;
    /// Returns whether or not this material is transparent.
    fn is_transparent(&self) -> bool;
}

impl<T: ForwardMaterial + ?Sized> ForwardMaterial for &T {
    fn fragment_shader_source(&self, use_vertex_colors: bool, lights: &Lights) -> String {
        (*self).fragment_shader_source(use_vertex_colors, lights)
    }
    fn use_uniforms(
        &self,
        program: &Program,
        camera: &Camera,
        lights: &Lights,
    ) -> ThreeDResult<()> {
        (*self).use_uniforms(program, camera, lights)
    }
    fn render_states(&self) -> RenderStates {
        (*self).render_states()
    }
    fn is_transparent(&self) -> bool {
        (*self).is_transparent()
    }
}
