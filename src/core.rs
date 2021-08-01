//!
//! Modular abstractions of common graphics concepts such as GPU shader program, buffer (vertex buffer, uniform buffer, element buffer),
//! texture (2D texture, cube texture, ..) and render target.
//! They are higher level than [context](crate::context) but lower level than other features.
//!

pub use crate::context::Context;

pub mod buffer;
pub use buffer::*;

pub mod math;
pub use math::*;

pub mod texture;
pub use texture::*;

mod render_states;
#[doc(inline)]
pub use render_states::*;

mod image_effect;
#[doc(inline)]
pub use image_effect::*;

mod render_target;
#[doc(inline)]
pub use render_target::*;

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

impl Error {
    pub fn message(&self) -> &String {
        return match self {
            Error::ProgramError { message } => message,
            Error::RenderTargetError { message } => message,
            Error::TextureError { message } => message,
            Error::BufferError { message } => message,
            Error::MeshError { message } => message,
            Error::CameraError { message } => message,
        };
    }
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
    fn into_u32(&self) -> u32;
}
impl ElementBufferDataType for u8 {
    fn into_u32(&self) -> u32 {
        *self as u32
    }
}
impl ElementBufferDataType for u16 {
    fn into_u32(&self) -> u32 {
        *self as u32
    }
}
impl ElementBufferDataType for u32 {
    fn into_u32(&self) -> u32 {
        *self
    }
}

pub(crate) mod internal {
    use crate::context::{consts, Context, DataType};
    use crate::core::*;

    pub trait BufferDataTypeExtension: Clone {
        fn buffer_data(context: &Context, target: u32, data: &[Self], usage: u32);
        fn data_type() -> DataType;
    }

    impl BufferDataTypeExtension for u8 {
        fn buffer_data(context: &Context, target: u32, data: &[Self], usage: u32) {
            context.buffer_data_u8(target, data, usage);
        }
        fn data_type() -> DataType {
            DataType::UnsignedByte
        }
    }

    impl BufferDataTypeExtension for u16 {
        fn buffer_data(context: &Context, target: u32, data: &[Self], usage: u32) {
            context.buffer_data_u16(target, data, usage);
        }
        fn data_type() -> DataType {
            DataType::UnsignedShort
        }
    }

    impl BufferDataTypeExtension for f32 {
        fn buffer_data(context: &Context, target: u32, data: &[Self], usage: u32) {
            context.buffer_data_f32(target, data, usage);
        }
        fn data_type() -> DataType {
            DataType::Float
        }
    }

    impl BufferDataTypeExtension for u32 {
        fn buffer_data(context: &Context, target: u32, data: &[Self], usage: u32) {
            context.buffer_data_u32(target, data, usage);
        }
        fn data_type() -> DataType {
            DataType::UnsignedInt
        }
    }

    pub trait TextureDataTypeExtension: Clone {
        fn internal_format(format: Format) -> Result<u32, crate::Error>;
        fn fill(
            context: &Context,
            target: u32,
            width: u32,
            height: u32,
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
            })
        }

        fn fill(
            context: &Context,
            target: u32,
            width: u32,
            height: u32,
            format: Format,
            data: &[Self],
        ) {
            context.tex_sub_image_2d_with_u8_data(
                target,
                0,
                0,
                0,
                width,
                height,
                format_from(format),
                DataType::UnsignedByte,
                data,
            );
        }

        fn read(context: &Context, viewport: Viewport, format: Format, pixels: &mut [Self]) {
            context.read_pixels_with_u8_data(
                viewport.x as u32,
                viewport.y as u32,
                viewport.width,
                viewport.height,
                format_from(format),
                DataType::UnsignedByte,
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
            })
        }

        fn fill(
            context: &Context,
            target: u32,
            width: u32,
            height: u32,
            format: Format,
            data: &[Self],
        ) {
            context.tex_sub_image_2d_with_f32_data(
                target,
                0,
                0,
                0,
                width,
                height,
                format_from(format),
                DataType::Float,
                data,
            );
        }

        fn read(context: &Context, viewport: Viewport, format: Format, pixels: &mut [Self]) {
            context.read_pixels_with_f32_data(
                viewport.x as u32,
                viewport.y as u32,
                viewport.width,
                viewport.height,
                format_from(format),
                DataType::Float,
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
            })
        }

        fn fill(
            context: &Context,
            target: u32,
            width: u32,
            height: u32,
            format: Format,
            data: &[Self],
        ) {
            context.tex_sub_image_2d_with_u32_data(
                target,
                0,
                0,
                0,
                width,
                height,
                format_from(format),
                DataType::UnsignedInt,
                data,
            );
        }
        fn read(context: &Context, viewport: Viewport, format: Format, pixels: &mut [Self]) {
            context.read_pixels_with_u32_data(
                viewport.x as u32,
                viewport.y as u32,
                viewport.width,
                viewport.height,
                format_from(format),
                DataType::UnsignedInt,
                pixels,
            );
        }
    }

    fn format_from(format: Format) -> u32 {
        match format {
            Format::R => consts::RED,
            Format::RG => consts::RG,
            Format::RGB => consts::RGB,
            Format::RGBA => consts::RGBA,
        }
    }
}

use crate::context::consts;

///
/// A texture that can be sampled in a fragment shader (see [use_texture](crate::Program::use_texture)).
///
pub trait Texture {
    /// Binds this texture to the current shader program.
    fn bind(&self, location: u32);
    /// The width of this texture.
    fn width(&self) -> u32;
    /// The height of this texture.
    fn height(&self) -> u32;
}

///
/// A texture array that can be sampled in a fragment shader (see [use_texture_array](crate::Program::use_texture_array)).
///
pub trait TextureArray {
    /// Binds this texture array to the current shader program.
    fn bind(&self, location: u32);
    /// The width of this texture.
    fn width(&self) -> u32;
    /// The height of this texture.
    fn height(&self) -> u32;
    /// The depth of this texture, ie. the number of layers.
    fn depth(&self) -> u32;
}

///
/// A texture cube that can be sampled in a fragment shader (see [use_texture_cube](crate::Program::use_texture_cube)).
///
pub trait TextureCube {
    /// Binds this texture cube to the current shader program.
    fn bind(&self, location: u32);
    /// The width of one of the sides of this texture.
    fn width(&self) -> u32;
    /// The height of one of the sides of this texture.
    fn height(&self) -> u32;
}

// COMMON TEXTURE FUNCTIONS
fn generate(context: &Context) -> Result<crate::context::Texture, Error> {
    context.create_texture().ok_or_else(|| Error::TextureError {
        message: "Failed to create texture".to_string(),
    })
}

fn bind_at(context: &Context, id: &crate::context::Texture, target: u32, location: u32) {
    context.active_texture(consts::TEXTURE0 + location);
    context.bind_texture(target, id);
}

fn set_parameters(
    context: &Context,
    id: &crate::context::Texture,
    target: u32,
    min_filter: Interpolation,
    mag_filter: Interpolation,
    mip_map_filter: Option<Interpolation>,
    wrap_s: Wrapping,
    wrap_t: Wrapping,
    wrap_r: Option<Wrapping>,
) {
    context.bind_texture(target, id);
    match mip_map_filter {
        None => context.tex_parameteri(
            target,
            consts::TEXTURE_MIN_FILTER,
            interpolation_from(min_filter),
        ),
        Some(Interpolation::Nearest) => {
            if min_filter == Interpolation::Nearest {
                context.tex_parameteri(
                    target,
                    consts::TEXTURE_MIN_FILTER,
                    consts::NEAREST_MIPMAP_NEAREST as i32,
                );
            } else {
                context.tex_parameteri(
                    target,
                    consts::TEXTURE_MIN_FILTER,
                    consts::LINEAR_MIPMAP_NEAREST as i32,
                )
            }
        }
        Some(Interpolation::Linear) => {
            if min_filter == Interpolation::Nearest {
                context.tex_parameteri(
                    target,
                    consts::TEXTURE_MIN_FILTER,
                    consts::NEAREST_MIPMAP_LINEAR as i32,
                );
            } else {
                context.tex_parameteri(
                    target,
                    consts::TEXTURE_MIN_FILTER,
                    consts::LINEAR_MIPMAP_LINEAR as i32,
                )
            }
        }
    }
    context.tex_parameteri(
        target,
        consts::TEXTURE_MAG_FILTER,
        interpolation_from(mag_filter),
    );
    context.tex_parameteri(target, consts::TEXTURE_WRAP_S, wrapping_from(wrap_s));
    context.tex_parameteri(target, consts::TEXTURE_WRAP_T, wrapping_from(wrap_t));
    if let Some(r) = wrap_r {
        context.tex_parameteri(target, consts::TEXTURE_WRAP_R, wrapping_from(r));
    }
}

fn calculate_number_of_mip_maps(
    mip_map_filter: Option<Interpolation>,
    width: u32,
    height: u32,
    depth: u32,
) -> u32 {
    if mip_map_filter.is_some() {
        let w = (width as f64).log2().ceil();
        let h = (height as f64).log2().ceil();
        let d = (depth as f64).log2().ceil();
        w.max(h).max(d).floor() as u32 + 1
    } else {
        1
    }
}

fn check_data_length(
    width: u32,
    height: u32,
    depth: u32,
    format: Format,
    length: usize,
) -> Result<(), Error> {
    let expected_pixels = width as usize * height as usize * depth as usize;
    let actual_pixels = length / format.color_channel_count() as usize;

    if expected_pixels != actual_pixels {
        Err(Error::TextureError {
            message: format!(
                "Wrong size of data for the texture (got {} pixels but expected {} pixels)",
                actual_pixels, expected_pixels
            ),
        })?;
    }
    Ok(())
}

fn internal_format_from_depth(format: DepthFormat) -> u32 {
    match format {
        DepthFormat::Depth16 => consts::DEPTH_COMPONENT16,
        DepthFormat::Depth24 => consts::DEPTH_COMPONENT24,
        DepthFormat::Depth32F => consts::DEPTH_COMPONENT32F,
    }
}

fn wrapping_from(wrapping: Wrapping) -> i32 {
    (match wrapping {
        Wrapping::Repeat => consts::REPEAT,
        Wrapping::MirroredRepeat => consts::MIRRORED_REPEAT,
        Wrapping::ClampToEdge => consts::CLAMP_TO_EDGE,
    }) as i32
}

fn interpolation_from(interpolation: Interpolation) -> i32 {
    (match interpolation {
        Interpolation::Nearest => consts::NEAREST,
        Interpolation::Linear => consts::LINEAR,
    }) as i32
}
