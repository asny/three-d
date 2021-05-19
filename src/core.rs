//!
//! Modular abstractions of common graphics concepts such as GPU shader program, buffer (vertex buffer, uniform buffer, element buffer),
//! texture (2D texture, cube texture, ..) and render target.
//! They are higher level than [context](crate::context) but lower level than other features.
//!

pub use crate::context::Context;

mod render_states;
#[doc(inline)]
pub use render_states::*;

mod texture;
#[doc(inline)]
pub use texture::*;

mod element_buffer;
#[doc(inline)]
pub use element_buffer::*;

mod vertex_buffer;
#[doc(inline)]
pub use vertex_buffer::*;

mod uniform_buffer;
#[doc(inline)]
pub use uniform_buffer::*;

mod render_target;
#[doc(inline)]
pub use render_target::*;

mod program;
#[doc(inline)]
pub use program::*;

///
/// Error in some part of the render engine.
///
#[derive(Debug)]
pub enum Error {
    /// An error in a shader program.
    ProgramError {
        /// Error message
        message: String,
    },
    /// An error when using a render target.
    RenderTargetError {
        /// Error message
        message: String,
    },
    /// An error when using a texture.
    TextureError {
        /// Error message
        message: String,
    },
    /// An error when using a buffer.
    BufferError {
        /// Error message
        message: String,
    },
    /// An error when using a mesh.
    MeshError {
        /// Error message
        message: String,
    },
    /// An error when using a camera.
    CameraError {
        /// Error message
        message: String,
    },
}

pub trait VertexBufferDataType:
    Default + std::fmt::Debug + Clone + internal::BufferDataTypeExtension
{
}
impl VertexBufferDataType for u8 {}
impl VertexBufferDataType for u16 {}
impl VertexBufferDataType for f32 {}

pub trait ElementBufferDataType:
    Default + std::fmt::Debug + Clone + internal::BufferDataTypeExtension
{
}
impl ElementBufferDataType for u8 {}
impl ElementBufferDataType for u16 {}
impl ElementBufferDataType for u32 {}

pub(crate) mod internal {

    use crate::context::{consts, Context};
    use crate::definition::*;
    use crate::math::*;

    pub trait BufferDataTypeExtension: Clone {
        fn buffer_data(context: &Context, target: u32, data: &[Self], usage: u32);
        fn data_type() -> u32;
    }

    impl BufferDataTypeExtension for u8 {
        fn buffer_data(context: &Context, target: u32, data: &[Self], usage: u32) {
            context.buffer_data_u8(target, data, usage);
        }
        fn data_type() -> u32 {
            crate::context::consts::UNSIGNED_BYTE
        }
    }

    impl BufferDataTypeExtension for u16 {
        fn buffer_data(context: &Context, target: u32, data: &[Self], usage: u32) {
            context.buffer_data_u16(target, data, usage);
        }
        fn data_type() -> u32 {
            crate::context::consts::UNSIGNED_SHORT
        }
    }

    impl BufferDataTypeExtension for f32 {
        fn buffer_data(context: &Context, target: u32, data: &[Self], usage: u32) {
            context.buffer_data_f32(target, data, usage);
        }
        fn data_type() -> u32 {
            crate::context::consts::FLOAT
        }
    }

    impl BufferDataTypeExtension for u32 {
        fn buffer_data(context: &Context, target: u32, data: &[Self], usage: u32) {
            context.buffer_data_u32(target, data, usage);
        }
        fn data_type() -> u32 {
            crate::context::consts::UNSIGNED_INT
        }
    }

    pub trait TextureDataTypeExtension: Clone {
        fn internal_format(format: Format) -> Result<u32, crate::Error>;
        fn fill(
            context: &Context,
            target: u32,
            width: usize,
            height: usize,
            format: Format,
            data: &[Self],
        );
        fn read(context: &Context, viewport: Viewport, format: Format, pixels: &mut [Self]);
    }

    impl TextureDataTypeExtension for u8 {
        fn internal_format(format: Format) -> Result<u32, crate::Error> {
            Ok(match format {
                Format::R => crate::context::consts::R8,
                Format::RG => crate::context::consts::RG8,
                Format::RGB => crate::context::consts::RGB8,
                Format::RGBA => crate::context::consts::RGBA8,
                Format::SRGB => crate::context::consts::SRGB8,
                Format::SRGBA => crate::context::consts::SRGB8_ALPHA8,
            })
        }

        fn fill(
            context: &Context,
            target: u32,
            width: usize,
            height: usize,
            format: Format,
            data: &[Self],
        ) {
            context.tex_sub_image_2d_with_u8_data(
                target,
                0,
                0,
                0,
                width as u32,
                height as u32,
                format_from(format),
                consts::UNSIGNED_BYTE,
                data,
            );
        }

        fn read(context: &Context, viewport: Viewport, format: Format, pixels: &mut [Self]) {
            context.read_pixels_with_u8_data(
                viewport.x as u32,
                viewport.y as u32,
                viewport.width as u32,
                viewport.height as u32,
                format_from(format),
                consts::UNSIGNED_BYTE,
                pixels,
            );
        }
    }
    impl TextureDataTypeExtension for f32 {
        fn internal_format(format: Format) -> Result<u32, crate::Error> {
            Ok(match format {
                Format::R => crate::context::consts::R32F,
                Format::RG => crate::context::consts::RG32F,
                Format::RGB => crate::context::consts::RGB32F,
                Format::RGBA => crate::context::consts::RGBA32F,
                _ => {
                    return Err(crate::Error::TextureError {
                        message:
                            "Cannot only use the sRGB(A) format together with a u8 texture value type."
                                .to_string(),
                    });
                }
            })
        }

        fn fill(
            context: &Context,
            target: u32,
            width: usize,
            height: usize,
            format: Format,
            data: &[Self],
        ) {
            context.tex_sub_image_2d_with_f32_data(
                target,
                0,
                0,
                0,
                width as u32,
                height as u32,
                format_from(format),
                consts::FLOAT,
                data,
            );
        }

        fn read(context: &Context, viewport: Viewport, format: Format, pixels: &mut [Self]) {
            context.read_pixels_with_f32_data(
                viewport.x as u32,
                viewport.y as u32,
                viewport.width as u32,
                viewport.height as u32,
                format_from(format),
                consts::FLOAT,
                pixels,
            );
        }
    }

    impl TextureDataTypeExtension for u32 {
        fn internal_format(format: Format) -> Result<u32, crate::Error> {
            Ok(match format {
                Format::R => crate::context::consts::R32UI,
                Format::RG => crate::context::consts::RG32UI,
                Format::RGB => crate::context::consts::RGB32UI,
                Format::RGBA => crate::context::consts::RGBA32UI,
                _ => {
                    return Err(crate::Error::TextureError {
                        message:
                            "Cannot only use the sRGB(A) format together with a u8 texture value type."
                                .to_string(),
                    });
                }
            })
        }

        fn fill(
            context: &Context,
            target: u32,
            width: usize,
            height: usize,
            format: Format,
            data: &[Self],
        ) {
            context.tex_sub_image_2d_with_u32_data(
                target,
                0,
                0,
                0,
                width as u32,
                height as u32,
                format_from(format),
                consts::UNSIGNED_INT,
                data,
            );
        }
        fn read(context: &Context, viewport: Viewport, format: Format, pixels: &mut [Self]) {
            context.read_pixels_with_u32_data(
                viewport.x as u32,
                viewport.y as u32,
                viewport.width as u32,
                viewport.height as u32,
                format_from(format),
                consts::UNSIGNED_INT,
                pixels,
            );
        }
    }

    fn format_from(format: Format) -> u32 {
        match format {
            Format::R => consts::RED,
            Format::RG => consts::RG,
            Format::RGB => consts::RGB,
            Format::SRGB => consts::RGB,
            Format::RGBA => consts::RGBA,
            Format::SRGBA => consts::RGBA,
        }
    }
}
