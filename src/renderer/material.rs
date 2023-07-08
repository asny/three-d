//!
//! A collection of materials implementing the [Material] trait.
//!
//! A material together with a [geometry] can be rendered directly (using [Geometry::render_with_material] or [Geometry::render_with_effect]).
//! A [Material] can also be combined into an [object] (see [Gm]) and be used in a render call, for example [RenderTarget::render].
//!

macro_rules! impl_material_body {
    ($inner:ident) => {
        fn fragment_shader_source(&self, lights: &[&dyn Light]) -> String {
            self.$inner().fragment_shader_source(lights)
        }
        fn fragment_attributes(&self) -> FragmentAttributes {
            self.$inner().fragment_attributes()
        }
        fn use_uniforms(&self, program: &Program, camera: &Camera, lights: &[&dyn Light]) {
            self.$inner().use_uniforms(program, camera, lights)
        }
        fn render_states(&self) -> RenderStates {
            self.$inner().render_states()
        }
        fn material_type(&self) -> MaterialType {
            self.$inner().material_type()
        }
        fn id(&self) -> u16 {
            self.$inner().id()
        }
    };
}

use crate::renderer::*;

pub use three_d_asset::material::{
    GeometryFunction, LightingModel, NormalDistributionFunction, PbrMaterial as CpuMaterial,
};

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

use std::{ops::Deref, sync::Arc};

///
/// A reference to a 2D texture and a texture transformation.
///
#[derive(Clone)]
pub struct Texture2DRef {
    /// A reference to the texture.
    pub texture: Arc<Texture2D>,
    /// A transformation applied to the uv coordinates before reading a texel value at those uv coordinates.
    /// This is primarily used in relation to texture atlasing.
    pub transformation: Mat3,
}

impl Texture2DRef {
    /// Creates a new [Texture2DRef] with an identity transformation from a [CpuTexture].
    pub fn from_cpu_texture(context: &Context, cpu_texture: &CpuTexture) -> Self {
        Self {
            texture: Arc::new(Texture2D::new(context, cpu_texture)),
            transformation: Mat3::identity(),
        }
    }

    /// Creates a new [Texture2DRef] with an identity transformation from a [Texture2D].
    pub fn from_texture(texture: Texture2D) -> Self {
        Self {
            texture: Arc::new(texture),
            transformation: Mat3::identity(),
        }
    }
}

impl std::ops::Deref for Texture2DRef {
    type Target = Texture2D;
    fn deref(&self) -> &Self::Target {
        &self.texture
    }
}

impl std::convert::From<Texture2D> for Texture2DRef {
    fn from(texture: Texture2D) -> Self {
        Self::from_texture(texture)
    }
}

impl std::convert::From<Arc<Texture2D>> for Texture2DRef {
    fn from(texture: Arc<Texture2D>) -> Self {
        Self {
            texture,
            transformation: Mat3::identity(),
        }
    }
}

///
/// Defines the material type which is needed to render the objects in the correct order.
/// For example, transparent objects need to be rendered back to front, whereas opaque objects need to be rendered front to back.
///
#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Debug)]
pub enum MaterialType {
    /// Forward opaque
    Opaque,
    /// Forward transparent
    Transparent,
    /// Deferred opaque
    Deferred,
}

///
/// Describes the set of attributes provided by a [geometry] and consumed by a [Material], ie. calculated in the vertex shader and then sent to the fragment shader.
/// To use an attribute for a material, add the relevant shader code to the fragment shader source (documented for each attribute) and return this struct from [Material::fragment_attributes] with the relevant attribute set to true.
///
#[derive(Clone, Copy, Debug)]
pub struct FragmentAttributes {
    /// Position in world space: `in vec3 pos;`
    pub position: bool,
    /// Normal: `in vec3 nor;`,
    pub normal: bool,
    /// Tangent and bitangent: `in vec3 tang; in vec3 bitang;`
    pub tangents: bool,
    /// UV coordinates: `in vec2 uvs;`
    pub uv: bool,
    /// Color: `in vec4 col;`
    pub color: bool,
}

impl FragmentAttributes {
    /// All attributes
    pub const ALL: Self = Self {
        position: true,
        normal: true,
        tangents: true,
        uv: true,
        color: true,
    };
    /// No attributes
    pub const NONE: Self = Self {
        position: false,
        normal: false,
        tangents: false,
        uv: false,
        color: false,
    };
}

///
/// Represents a material that, together with a [geometry], can be rendered using [Geometry::render_with_material].
/// Alternatively, a geometry and a material can be combined in a [Gm],
/// thereby creating an [Object] which can be used in a render call, for example [RenderTarget::render].
///
pub trait Material {
    ///
    /// Returns the fragment shader source for this material.
    ///
    fn fragment_shader_source(&self, lights: &[&dyn Light]) -> String;

    ///
    /// Returns a unique ID for each variation of the shader source returned from [Material::fragment_shader_source].
    ///
    /// **Note:** The last bit is reserved to internally implemented materials, so if implementing the [Material] trait
    /// outside of this crate, always return an id that is smaller than `0b1u16 << 15`.
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
    fn use_uniforms(&self, program: &Program, camera: &Camera, lights: &[&dyn Light]);

    ///
    /// Returns the render states needed to render with this material.
    ///
    fn render_states(&self) -> RenderStates;

    ///
    /// Returns the type of material.
    ///
    fn material_type(&self) -> MaterialType;
}

///
/// Implement this for a [Material] that can be created from a [CpuMaterial].
///
pub trait FromCpuMaterial: std::marker::Sized {
    ///
    /// Creates a new material that can be used for rendering from a [CpuMaterial].
    ///
    fn from_cpu_material(context: &Context, cpu_material: &CpuMaterial) -> Self;
}

///
/// Implement this for a [Material] that can be created from a [CpuVoxelGrid].
///
pub trait FromCpuVoxelGrid: std::marker::Sized {
    ///
    /// Creates a new material that can be used for rendering from a [CpuVoxelGrid].
    ///
    fn from_cpu_voxel_grid(context: &Context, cpu_voxel_grid: &CpuVoxelGrid) -> Self;
}

impl<T: Material + ?Sized> Material for &T {
    impl_material_body!(deref);
}

impl<T: Material + ?Sized> Material for &mut T {
    impl_material_body!(deref);
}

impl<T: Material> Material for Box<T> {
    impl_material_body!(as_ref);
}

impl<T: Material> Material for std::rc::Rc<T> {
    impl_material_body!(as_ref);
}

impl<T: Material> Material for std::sync::Arc<T> {
    impl_material_body!(as_ref);
}

impl<T: Material> Material for std::cell::RefCell<T> {
    impl_material_body!(borrow);
}

impl<T: Material> Material for std::sync::RwLock<T> {
    fn fragment_shader_source(&self, lights: &[&dyn Light]) -> String {
        self.read().unwrap().fragment_shader_source(lights)
    }
    fn fragment_attributes(&self) -> FragmentAttributes {
        self.read().unwrap().fragment_attributes()
    }
    fn use_uniforms(&self, program: &Program, camera: &Camera, lights: &[&dyn Light]) {
        self.read().unwrap().use_uniforms(program, camera, lights)
    }
    fn render_states(&self) -> RenderStates {
        self.read().unwrap().render_states()
    }
    fn material_type(&self) -> MaterialType {
        self.read().unwrap().material_type()
    }
    fn id(&self) -> u16 {
        self.read().unwrap().id()
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
