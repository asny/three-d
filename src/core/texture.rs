//!
//! Different types of textures used by the GPU to read from and write to.
//!
mod texture2d;
#[doc(inline)]
pub use texture2d::*;

mod texture_cube_map;
#[doc(inline)]
pub use texture_cube_map::*;

mod color_target_texture2d;
#[doc(inline)]
pub use color_target_texture2d::*;

mod depth_target_texture2d;
#[doc(inline)]
pub use depth_target_texture2d::*;

mod color_target_texture2d_array;
#[doc(inline)]
pub use color_target_texture2d_array::*;

mod depth_target_texture2d_array;
#[doc(inline)]
pub use depth_target_texture2d_array::*;

///
/// Possible modes of interpolation which determines the texture output between texture pixels.
///
#[allow(missing_docs)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Interpolation {
    Nearest,
    Linear,
}

///
/// Possible wrapping modes for a texture which determines how the texture is applied outside of the
/// [0..1] uv coordinate range.
///
#[allow(missing_docs)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Wrapping {
    Repeat,
    MirroredRepeat,
    ClampToEdge,
}

/// The basic data type used for each channel of each pixel in a texture.
pub trait TextureDataType:
    Default + std::fmt::Debug + Clone + Copy + internal::TextureDataTypeExtension
{
}
impl TextureDataType for u8 {}
impl TextureDataType for f32 {}
impl TextureDataType for u32 {}

///
/// Possible formats for pixels in a texture.
///
#[allow(missing_docs)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Format {
    R,
    RG,
    RGB,
    RGBA,
}

impl Format {
    /// Returns the number of channels for the given format.
    pub fn color_channel_count(&self) -> u32 {
        match self {
            Format::R => 1,
            Format::RG => 2,
            Format::RGB => 3,
            Format::RGBA => 4,
        }
    }
}

///
/// A CPU-side version of a texture, for example [Texture2D].
/// Can be constructed manually or loaded via [Loader](crate::Loader).
///
#[allow(missing_docs)]
pub struct CPUTexture<T: TextureDataType> {
    pub data: Vec<T>,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub format: Format,
    pub min_filter: Interpolation,
    pub mag_filter: Interpolation,
    pub mip_map_filter: Option<Interpolation>,
    pub wrap_s: Wrapping,
    pub wrap_t: Wrapping,
    pub wrap_r: Wrapping,
}

impl<T: TextureDataType> CPUTexture<T> {
    ///
    /// Adds a padding of default values to the texture.
    /// 'left' number of pixels are added to the left of the original texture, 'right' number of pixels to the right and so on.
    ///
    pub fn add_padding(&mut self, left: u32, right: u32, top: u32, bottom: u32) {
        let channels = self.format.color_channel_count();
        let width = left + self.width + right;
        let height = top + self.height + bottom;
        let mut new_data = vec![T::default(); width as usize * height as usize * channels as usize];
        for x in 0..self.width {
            for y in 0..self.height {
                let x_ = x + left;
                let y_ = y + top;
                let source_index = (y * self.width + x) * channels;
                let dest_index = (y_ * width + x_) * channels;
                for i in 0..channels {
                    new_data[(dest_index + i) as usize] =
                        self.data[(source_index + i) as usize].clone();
                }
            }
        }
        self.data = new_data;
        self.width = width;
        self.height = height;
    }
}

impl<T: TextureDataType> Default for CPUTexture<T> {
    fn default() -> Self {
        Self {
            data: [T::default(), T::default(), T::default(), T::default()].into(),
            width: 1,
            height: 1,
            depth: 1,
            format: Format::RGBA,
            min_filter: Interpolation::Linear,
            mag_filter: Interpolation::Linear,
            mip_map_filter: Some(Interpolation::Linear),
            wrap_s: Wrapping::Repeat,
            wrap_t: Wrapping::Repeat,
            wrap_r: Wrapping::Repeat,
        }
    }
}

impl<T: TextureDataType> std::fmt::Debug for CPUTexture<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CPUTexture")
            .field("format", &self.format)
            .field("data length", &self.data.len())
            .field("width", &self.width)
            .field("height", &self.height)
            .field("depth", &self.depth)
            .field("min_filter", &self.min_filter)
            .field("mag_filter", &self.mag_filter)
            .field("mip_map_filter", &self.mip_map_filter)
            .field("wrap_s", &self.wrap_s)
            .field("wrap_t", &self.wrap_t)
            .field("wrap_r", &self.wrap_r)
            .finish()
    }
}

pub(in crate::core) mod internal {
    use crate::context::{consts, DataType};
    use crate::core::*;

    pub trait TextureDataTypeExtension: Clone {
        fn internal_format(format: Format) -> ThreeDResult<u32>;
        fn fill(
            context: &Context,
            target: u32,
            width: u32,
            height: u32,
            format: Format,
            data: &[Self],
        );
        fn read(context: &Context, viewport: Viewport, format: Format, pixels: &mut [Self]);
        fn is_max(value: Self) -> bool;
    }

    impl TextureDataTypeExtension for u8 {
        fn internal_format(format: Format) -> ThreeDResult<u32> {
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

        fn is_max(value: Self) -> bool {
            value == 255u8
        }
    }
    impl TextureDataTypeExtension for f32 {
        fn internal_format(format: Format) -> ThreeDResult<u32> {
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

        fn is_max(value: Self) -> bool {
            value > 0.99
        }
    }

    impl TextureDataTypeExtension for u32 {
        fn internal_format(format: Format) -> ThreeDResult<u32> {
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

        fn is_max(_value: Self) -> bool {
            true
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
use crate::core::*;

///
/// A texture that can be sampled in a fragment shader (see [use_texture](crate::core::Program::use_texture)).
///
pub trait Texture {
    /// Binds this texture to the current shader program.
    fn bind(&self, location: u32);
    /// The width of this texture.
    fn width(&self) -> u32;
    /// The height of this texture.
    fn height(&self) -> u32;
    /// The format of this texture.
    fn format(&self) -> Format;
    /// Whether this texture contain pixels with alpha value less than maximum.
    fn is_transparent(&self) -> bool;
}

///
/// A texture array that can be sampled in a fragment shader (see [use_texture_array](crate::core::Program::use_texture_array)).
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
    /// The format of this texture.
    fn format(&self) -> Format;
}

///
/// A texture cube that can be sampled in a fragment shader (see [use_texture_cube](crate::core::Program::use_texture_cube)).
///
pub trait TextureCube {
    /// Binds this texture cube to the current shader program.
    fn bind(&self, location: u32);
    /// The width of one of the sides of this texture.
    fn width(&self) -> u32;
    /// The height of one of the sides of this texture.
    fn height(&self) -> u32;
    /// The format of this texture.
    fn format(&self) -> Format;
}

// COMMON TEXTURE FUNCTIONS
fn generate(context: &Context) -> ThreeDResult<crate::context::Texture> {
    Ok(context
        .create_texture()
        .ok_or_else(|| CoreError::TextureCreation)?)
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

fn check_data_length(
    width: u32,
    height: u32,
    depth: u32,
    format: Format,
    length: usize,
) -> ThreeDResult<()> {
    let expected_pixels = width as usize * height as usize * depth as usize;
    let actual_pixels = length / format.color_channel_count() as usize;

    if expected_pixels != actual_pixels {
        Err(CoreError::InvalidTextureLength(
            actual_pixels,
            expected_pixels,
        ))?;
    }
    Ok(())
}
