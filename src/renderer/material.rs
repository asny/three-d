//!
//! A collection of common materials implementing the [Material] trait.
//! A material together with a [geometry] can be rendered directly, or combined into an [object] (see [Gm]) that can be used in a render call, for example [render_pass].
//!

use crate::core::*;
use crate::renderer::*;

pub use crate::core::{Color, CpuMaterial};

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

mod skybox_material;
#[doc(inline)]
pub(in crate::renderer) use skybox_material::*;

mod isosurface_material;
#[doc(inline)]
pub use isosurface_material::*;

///
/// Represents a material that, together with a [geometry], can be rendered using [Geometry::render_with_material].
/// Alternatively, a geometry and a material can be combined in a [Gm],
/// thereby creating an [Object] which can be used in a render call, for example [render_pass].
///
/// The material can use an attribute by adding the folowing to the fragment shader source code.
/// - position (in world space): `in vec3 pos;`
/// - normal: `in vec3 nor;`,
/// - tangent: `in vec3 tang;`
/// - bitangent: `in vec3 bitang;`
/// - uv coordinates: `in vec2 uvs;`
/// - color: `in vec4 col;`
/// The rendering will fail if the material requires one of these attributes and the [geometry] does not provide it.
///
pub trait Material {
    /// Returns the fragment shader source for this material. Should output the final fragment color.
    fn fragment_shader_source(&self, use_vertex_colors: bool, lights: &[&dyn Light]) -> String;
    /// Sends the uniform data needed for this material to the fragment shader.
    fn use_uniforms(
        &self,
        program: &Program,
        camera: &Camera,
        lights: &[&dyn Light],
    ) -> ThreeDResult<()>;
    /// Returns the render states needed to render with this material.
    fn render_states(&self) -> RenderStates;
    /// Returns whether or not this material is transparent.
    fn is_transparent(&self) -> bool;
}

impl<T: Material + ?Sized> Material for &T {
    fn fragment_shader_source(&self, use_vertex_colors: bool, lights: &[&dyn Light]) -> String {
        (*self).fragment_shader_source(use_vertex_colors, lights)
    }
    fn use_uniforms(
        &self,
        program: &Program,
        camera: &Camera,
        lights: &[&dyn Light],
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

impl<T: Material + ?Sized> Material for &mut T {
    fn fragment_shader_source(&self, use_vertex_colors: bool, lights: &[&dyn Light]) -> String {
        (**self).fragment_shader_source(use_vertex_colors, lights)
    }
    fn use_uniforms(
        &self,
        program: &Program,
        camera: &Camera,
        lights: &[&dyn Light],
    ) -> ThreeDResult<()> {
        (**self).use_uniforms(program, camera, lights)
    }
    fn render_states(&self) -> RenderStates {
        (**self).render_states()
    }
    fn is_transparent(&self) -> bool {
        (**self).is_transparent()
    }
}

impl<T: Material> Material for Box<T> {
    fn fragment_shader_source(&self, use_vertex_colors: bool, lights: &[&dyn Light]) -> String {
        self.as_ref()
            .fragment_shader_source(use_vertex_colors, lights)
    }
    fn use_uniforms(
        &self,
        program: &Program,
        camera: &Camera,
        lights: &[&dyn Light],
    ) -> ThreeDResult<()> {
        self.as_ref().use_uniforms(program, camera, lights)
    }
    fn render_states(&self) -> RenderStates {
        self.as_ref().render_states()
    }
    fn is_transparent(&self) -> bool {
        self.as_ref().is_transparent()
    }
}

impl<T: Material> Material for std::rc::Rc<T> {
    fn fragment_shader_source(&self, use_vertex_colors: bool, lights: &[&dyn Light]) -> String {
        self.as_ref()
            .fragment_shader_source(use_vertex_colors, lights)
    }
    fn use_uniforms(
        &self,
        program: &Program,
        camera: &Camera,
        lights: &[&dyn Light],
    ) -> ThreeDResult<()> {
        self.as_ref().use_uniforms(program, camera, lights)
    }
    fn render_states(&self) -> RenderStates {
        self.as_ref().render_states()
    }
    fn is_transparent(&self) -> bool {
        self.as_ref().is_transparent()
    }
}

impl<T: Material> Material for std::rc::Rc<std::cell::RefCell<T>> {
    fn fragment_shader_source(&self, use_vertex_colors: bool, lights: &[&dyn Light]) -> String {
        self.borrow()
            .fragment_shader_source(use_vertex_colors, lights)
    }
    fn use_uniforms(
        &self,
        program: &Program,
        camera: &Camera,
        lights: &[&dyn Light],
    ) -> ThreeDResult<()> {
        self.borrow().use_uniforms(program, camera, lights)
    }
    fn render_states(&self) -> RenderStates {
        self.borrow().render_states()
    }
    fn is_transparent(&self) -> bool {
        self.borrow().is_transparent()
    }
}

fn is_transparent(cpu_material: &CpuMaterial) -> bool {
    cpu_material.albedo.a != 255
        || cpu_material
            .albedo_texture
            .as_ref()
            .map(|t| match &t.data {
                TextureData::RgbaU8(data) => data.iter().any(|d| d[3] != 255),
                TextureData::RgbaF16(data) => data.iter().any(|d| d[3] < f16::from_f32(0.99)),
                TextureData::RgbaF32(data) => data.iter().any(|d| d[3] < 0.99),
                _ => false,
            })
            .unwrap_or(false)
}
