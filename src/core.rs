//!
//! Mid-level modular abstractions of common graphics concepts such as buffer, texture, program, render target and so on.
//! Can be combined with low-level calls in the `context` module as long as any graphics state changes are reset.
//!

mod context;
#[doc(inline)]
pub use context::*;

pub mod buffer;
pub use buffer::*;

pub mod math;
pub use math::*;

pub mod texture;
pub use texture::*;

mod cpu_mesh;
#[doc(inline)]
pub use cpu_mesh::*;

pub mod render_states;
pub use render_states::*;

pub mod render_target;
pub use render_target::*;

mod uniform;
#[doc(inline)]
pub use uniform::*;

mod cpu_material;
#[doc(inline)]
pub use cpu_material::*;

mod cpu_volume;
#[doc(inline)]
pub use cpu_volume::*;

mod camera;
#[doc(inline)]
pub use camera::*;

mod image_effect;
#[doc(inline)]
pub use image_effect::*;

mod image_cube_effect;
#[doc(inline)]
pub use image_cube_effect::*;

mod program;
#[doc(inline)]
pub use program::*;

mod aabb;
#[doc(inline)]
pub use aabb::*;

mod color;
#[doc(inline)]
pub use color::*;

mod viewport;
#[doc(inline)]
pub use viewport::*;

pub use crate::ThreeDResult;
use thiserror::Error;
///
/// Error in the [core](crate::core) module.
///
#[derive(Debug, Error)]
#[allow(missing_docs)]
pub enum CoreError {
    #[error("failed creating context with error: {0}")]
    ContextCreation(String),
    #[error("failed rendering with error: {0}")]
    ContextError(String),
    #[error("failed creating shader: {0}")]
    ShaderCreation(String),
    #[error("failed creating program: {0}")]
    ProgramCreation(String),
    #[error("failed creating buffer: {0}")]
    BufferCreation(String),
    #[error("failed compiling {0} shader: {1}")]
    ShaderCompilation(String, String),
    #[error("failed to link shader program: {0}")]
    ShaderLink(String),
    #[error("the uniform {0} is sent to the shader but not defined or never used")]
    UnusedUniform(String),
    #[error("the attribute {0} is sent to the shader but not defined or never used")]
    UnusedAttribute(String),
    #[error("failed creating a new render target: {0}")]
    RenderTargetCreation(String),
    #[error("cannot copy {0} from a {1} texture")]
    RenderTargetCopy(String, String),
    #[error("cannot read color from anything else but an RGBA texture")]
    ReadWrongFormat,
    #[error("failed creating a new texture: {0}")]
    TextureCreation(String),
    #[error("invalid size of texture data (got {0} pixels but expected {1} pixels)")]
    InvalidTextureLength(usize, usize),
    #[error("the render call requires the {0} vertex buffer which is missing on the given mesh")]
    MissingMeshBuffer(String),
    #[error(
        "if the fragment shader defined 'in vec3 tang' it also needs to define 'in vec3 bitang'"
    )]
    MissingBitangent,
    #[error("{0} buffer length must be {1}, actual length is {2}")]
    InvalidBufferLength(String, usize, usize),
    #[error("mesh must have both normals and uv coordinates to be able to compute tangents")]
    FailedComputingTangents,
    #[error("the number of vertices must be divisable by 3, actual count is {0}")]
    InvalidNumberOfVertices(usize),
    #[error("data for element at index {0} has length {1} but a length of {2} was expected")]
    InvalidUniformBufferElementLength(u32, usize, usize),
    #[error("the index {0} is outside the expected range [0, {1}]")]
    IndexOutOfRange(usize, usize),
    #[error("cannot take as input a negative minimum distance")]
    NegativeDistance,
    #[error("a minimum must be smaller than a maximum")]
    MinimumLargerThanMaximum,
}

mod internal {
    use crate::core::*;

    pub fn to_byte_slice<'a, T: DataType>(data: &'a [T]) -> &'a [u8] {
        unsafe {
            std::slice::from_raw_parts(
                data.as_ptr() as *const _,
                data.len() * std::mem::size_of::<T>(),
            )
        }
    }

    pub fn from_byte_slice<'a, T: DataType>(data: &'a [u8]) -> &'a [T] {
        unsafe {
            let (_prefix, values, _suffix) = data.align_to::<T>();
            values
        }
    }

    pub trait DataType: std::fmt::Debug + Clone {
        fn internal_format(format: Format) -> u32;
        fn data_type() -> u32;
        fn is_max(&self) -> bool;
        fn size() -> u32;
        fn default() -> Self;
    }

    impl DataType for u8 {
        fn data_type() -> u32 {
            glow::UNSIGNED_BYTE
        }

        fn size() -> u32 {
            1
        }

        fn internal_format(format: Format) -> u32 {
            match format {
                Format::R => glow::R8,
                Format::RG => glow::RG8,
                Format::RGB => glow::RGB8,
                Format::RGBA => glow::RGBA8,
            }
        }

        fn is_max(&self) -> bool {
            *self == 255u8
        }

        fn default() -> Self {
            0
        }
    }

    impl DataType for u16 {
        fn data_type() -> u32 {
            glow::UNSIGNED_SHORT
        }

        fn size() -> u32 {
            1
        }

        fn internal_format(format: Format) -> u32 {
            match format {
                Format::R => glow::R16UI,
                Format::RG => glow::RG16UI,
                Format::RGB => glow::RGB16UI,
                Format::RGBA => glow::RGBA16UI,
            }
        }

        fn is_max(&self) -> bool {
            *self == u16::MAX
        }

        fn default() -> Self {
            0
        }
    }

    impl DataType for u32 {
        fn data_type() -> u32 {
            glow::UNSIGNED_INT
        }

        fn size() -> u32 {
            1
        }

        fn internal_format(format: Format) -> u32 {
            match format {
                Format::R => glow::R32UI,
                Format::RG => glow::RG32UI,
                Format::RGB => glow::RGB32UI,
                Format::RGBA => glow::RGBA32UI,
            }
        }

        fn is_max(&self) -> bool {
            *self == u32::MAX
        }

        fn default() -> Self {
            0
        }
    }

    impl DataType for f16 {
        fn data_type() -> u32 {
            glow::HALF_FLOAT
        }

        fn size() -> u32 {
            1
        }

        fn internal_format(format: Format) -> u32 {
            match format {
                Format::R => glow::R16F,
                Format::RG => glow::RG16F,
                Format::RGB => glow::RGB16F,
                Format::RGBA => glow::RGBA16F,
            }
        }

        fn is_max(&self) -> bool {
            *self > f16::from_f32(0.99)
        }

        fn default() -> Self {
            f16::from_f32(0.0)
        }
    }

    impl DataType for f32 {
        fn data_type() -> u32 {
            glow::FLOAT
        }

        fn size() -> u32 {
            1
        }

        fn internal_format(format: Format) -> u32 {
            match format {
                Format::R => glow::R32F,
                Format::RG => glow::RG32F,
                Format::RGB => glow::RGB32F,
                Format::RGBA => glow::RGBA32F,
            }
        }

        fn is_max(&self) -> bool {
            *self > 0.99
        }

        fn default() -> Self {
            0.0
        }
    }

    impl<T: DataType> DataType for Vector2<T> {
        fn data_type() -> u32 {
            T::data_type()
        }

        fn size() -> u32 {
            2
        }

        fn internal_format(format: Format) -> u32 {
            T::internal_format(format)
        }

        fn is_max(&self) -> bool {
            self.x.is_max() && self.y.is_max()
        }

        fn default() -> Self {
            Self::new(T::default(), T::default())
        }
    }

    impl<T: DataType> DataType for Vector3<T> {
        fn data_type() -> u32 {
            T::data_type()
        }

        fn size() -> u32 {
            3
        }

        fn internal_format(format: Format) -> u32 {
            T::internal_format(format)
        }

        fn is_max(&self) -> bool {
            self.x.is_max() && self.y.is_max() && self.z.is_max()
        }

        fn default() -> Self {
            Self::new(T::default(), T::default(), T::default())
        }
    }

    impl<T: DataType> DataType for Vector4<T> {
        fn data_type() -> u32 {
            T::data_type()
        }

        fn size() -> u32 {
            4
        }

        fn internal_format(format: Format) -> u32 {
            T::internal_format(format)
        }

        fn is_max(&self) -> bool {
            self.x.is_max() && self.y.is_max() && self.z.is_max() && self.w.is_max()
        }

        fn default() -> Self {
            Self::new(T::default(), T::default(), T::default(), T::default())
        }
    }

    impl DataType for Color {
        fn data_type() -> u32 {
            u8::data_type()
        }

        fn size() -> u32 {
            4
        }

        fn internal_format(format: Format) -> u32 {
            u8::internal_format(format)
        }

        fn is_max(&self) -> bool {
            self.r.is_max() && self.g.is_max() && self.b.is_max() && self.a.is_max()
        }

        fn default() -> Self {
            Color::WHITE
        }
    }
}
