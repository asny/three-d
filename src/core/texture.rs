//!
//! Different types of textures used by the GPU to read from and write to.
//!
mod texture2d;
#[doc(inline)]
pub use texture2d::*;

mod texture_cube_map;
#[doc(inline)]
pub use texture_cube_map::*;

mod depth_target_texture2d;
#[doc(inline)]
pub use depth_target_texture2d::*;

mod texture2d_array;
#[doc(inline)]
pub use texture2d_array::*;

mod texture3d;
#[doc(inline)]
pub use texture3d::*;

mod depth_target_texture2d_array;
#[doc(inline)]
pub use depth_target_texture2d_array::*;

mod depth_target_texture_cube_map;
#[doc(inline)]
pub use depth_target_texture_cube_map::*;

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

use data_type::*;

/// The basic data type used for each channel of each pixel in a texture.
pub trait TextureDataType: DataType {}
impl TextureDataType for u8 {}
impl TextureDataType for f16 {}
impl TextureDataType for f32 {}

impl<T: TextureDataType + PrimitiveDataType> TextureDataType for Vector2<T> {}
impl<T: TextureDataType + PrimitiveDataType> TextureDataType for Vector3<T> {}
impl<T: TextureDataType + PrimitiveDataType> TextureDataType for Vector4<T> {}
impl<T: TextureDataType + PrimitiveDataType> TextureDataType for [T; 2] {}
impl<T: TextureDataType + PrimitiveDataType> TextureDataType for [T; 3] {}
impl<T: TextureDataType + PrimitiveDataType> TextureDataType for [T; 4] {}

impl TextureDataType for Color {}
impl TextureDataType for Quat {}

impl<T: TextureDataType + ?Sized> TextureDataType for &T {}

///
/// Possible formats for pixels in a texture.
///
#[deprecated = "the texture format is instead specified by the generic parameter, so if you fill the texture with [u8; 4] data, the format is RGBA and the data type is byte"]
#[allow(missing_docs)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Format {
    R,
    RG,
    RGB,
    RGBA,
}

#[allow(deprecated)]
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
/// The pixel/texel data for a [CpuTexture].
///
/// If 2D data, the data array should start with the top left texel and then one row at a time.
/// The indices `(row, column)` into the 2D data would look like
/// ```
/// [
/// (0, 0), (1, 0), .., // First row
/// (0, 1), (1, 1), .., // Second row
/// ..
/// ]
/// ```
/// If 3D data, the data array would look like the 2D data, one layer/image at a time.
/// The indices `(row, column, layer)` into the 3D data would look like
/// ```
/// [
/// (0, 0, 0), (1, 0, 0), .., // First row in first layer
/// (0, 1, 0), (1, 1, 0), .., // Second row in first layer
/// ..
/// (0, 0, 1), (1, 0, 1), .., // First row in second layer
/// (0, 1, 1), (1, 1, 1), ..,  // Second row in second layer
/// ..
/// ]
/// ```
///
#[derive(Clone)]
pub enum TextureData {
    /// One byte in the red channel.
    RU8(Vec<u8>),
    /// One byte in the red and green channel.
    RgU8(Vec<[u8; 2]>),
    /// One byte in the red, green and blue channel.
    RgbU8(Vec<[u8; 3]>),
    /// One byte in the red, green, blue and alpha channel.
    RgbaU8(Vec<[u8; 4]>),

    /// 16-bit float in the red channel.
    RF16(Vec<f16>),
    /// 16-bit float in the red and green channel.
    RgF16(Vec<[f16; 2]>),
    /// 16-bit float in the red, green and blue channel.
    RgbF16(Vec<[f16; 3]>),
    /// 16-bit float in the red, green, blue and alpha channel.
    RgbaF16(Vec<[f16; 4]>),

    /// 32-bit float in the red channel.
    RF32(Vec<f32>),
    /// 32-bit float in the red and green channel.
    RgF32(Vec<[f32; 2]>),
    /// 32-bit float in the red, green and blue channel.
    RgbF32(Vec<[f32; 3]>),
    /// 32-bit float in the red, green, blue and alpha channel.
    RgbaF32(Vec<[f32; 4]>),
}

impl std::fmt::Debug for TextureData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RU8(values) => write!(f, "R u8 ({:?})", values.len()),
            Self::RgU8(values) => write!(f, "RG u8 ({:?})", values.len()),
            Self::RgbU8(values) => write!(f, "RGB u8 ({:?})", values.len()),
            Self::RgbaU8(values) => write!(f, "RGBA u8 ({:?})", values.len()),
            Self::RF16(values) => write!(f, "R f16 ({:?})", values.len()),
            Self::RgF16(values) => write!(f, "RG f16 ({:?})", values.len()),
            Self::RgbF16(values) => write!(f, "RGB f16 ({:?})", values.len()),
            Self::RgbaF16(values) => write!(f, "RGBA f16 ({:?})", values.len()),
            Self::RF32(values) => write!(f, "R f32 ({:?})", values.len()),
            Self::RgF32(values) => write!(f, "RG f32 ({:?})", values.len()),
            Self::RgbF32(values) => write!(f, "RGB f32 ({:?})", values.len()),
            Self::RgbaF32(values) => write!(f, "RGBA f32 ({:?})", values.len()),
        }
    }
}

/// See [CpuTexture]
#[deprecated = "Renamed to CpuTexture"]
pub type CPUTexture = CpuTexture;

///
/// A CPU-side version of a [Texture2D].
/// Can be constructed manually or loaded via [Loader](crate::Loader).
///
#[derive(Clone, Debug)]
#[allow(deprecated)]
pub struct CpuTexture {
    /// The pixel data for the image
    pub data: TextureData,
    /// The width of the image
    pub width: u32,
    /// The height of the image
    pub height: u32,
    /// The format of the image
    pub format: Format,
    /// The way the pixel data is interpolated when the texture is far away
    pub min_filter: Interpolation,
    /// The way the pixel data is interpolated when the texture is close
    pub mag_filter: Interpolation,
    /// Specifies whether mipmaps should be created for this texture and what type of interpolation to use between the two closest mipmaps.
    /// Note, however, that the mipmaps only will be created if the width and height of the texture are power of two.
    pub mip_map_filter: Option<Interpolation>,
    /// Determines how the texture is sampled outside the [0..1] s coordinate range (the first value of the uv coordinates).
    pub wrap_s: Wrapping,
    /// Determines how the texture is sampled outside the [0..1] t coordinate range (the second value of the uv coordinates).
    pub wrap_t: Wrapping,
}

#[allow(deprecated)]
impl Default for CpuTexture {
    fn default() -> Self {
        Self {
            data: TextureData::RgbaU8(vec![[0, 0, 0, 0]]),
            width: 1,
            height: 1,
            format: Format::RGBA,
            min_filter: Interpolation::Linear,
            mag_filter: Interpolation::Linear,
            mip_map_filter: Some(Interpolation::Linear),
            wrap_s: Wrapping::Repeat,
            wrap_t: Wrapping::Repeat,
        }
    }
}

///
/// A CPU-side version of a [Texture3D].
///
#[derive(Clone, Debug)]
pub struct CpuTexture3D {
    /// The pixel data for the image
    pub data: TextureData,
    /// The width of the image
    pub width: u32,
    /// The height of the image
    pub height: u32,
    /// The depth of the image
    pub depth: u32,
    /// The format of the image
    #[deprecated = "the texture format is determined by the TextureData"]
    #[allow(deprecated)]
    pub format: Format,
    /// The way the pixel data is interpolated when the texture is far away
    pub min_filter: Interpolation,
    /// The way the pixel data is interpolated when the texture is close
    pub mag_filter: Interpolation,
    /// Specifies whether mipmaps should be created for this texture and what type of interpolation to use between the two closest mipmaps.
    /// Note, however, that the mipmaps only will be created if the width and height of the texture are power of two.
    pub mip_map_filter: Option<Interpolation>,
    /// Determines how the texture is sampled outside the [0..1] s coordinate range (the first value of the uvw coordinates).
    pub wrap_s: Wrapping,
    /// Determines how the texture is sampled outside the [0..1] t coordinate range (the second value of the uvw coordinates).
    pub wrap_t: Wrapping,
    /// Determines how the texture is sampled outside the [0..1] r coordinate range (the third value of the uvw coordinates).
    pub wrap_r: Wrapping,
}

#[allow(deprecated)]
impl Default for CpuTexture3D {
    fn default() -> Self {
        Self {
            data: TextureData::RgbaU8(vec![[0, 0, 0, 0]]),
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

///
/// The pixel data for a [CpuTextureCube].
///
#[derive(Clone)]
pub enum TextureCubeData {
    /// byte in the red channel.
    RU8(Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>),
    /// byte in the red and green channel.
    RgU8(
        Vec<[u8; 2]>,
        Vec<[u8; 2]>,
        Vec<[u8; 2]>,
        Vec<[u8; 2]>,
        Vec<[u8; 2]>,
        Vec<[u8; 2]>,
    ),
    /// byte in the red, green and blue channel.
    RgbU8(
        Vec<[u8; 3]>,
        Vec<[u8; 3]>,
        Vec<[u8; 3]>,
        Vec<[u8; 3]>,
        Vec<[u8; 3]>,
        Vec<[u8; 3]>,
    ),
    /// byte in the red, green, blue and alpha channel.
    RgbaU8(
        Vec<[u8; 4]>,
        Vec<[u8; 4]>,
        Vec<[u8; 4]>,
        Vec<[u8; 4]>,
        Vec<[u8; 4]>,
        Vec<[u8; 4]>,
    ),

    /// 16-bit float in the red channel.
    RF16(Vec<f16>, Vec<f16>, Vec<f16>, Vec<f16>, Vec<f16>, Vec<f16>),
    /// 16-bit float in the red and green channel.
    RgF16(
        Vec<[f16; 2]>,
        Vec<[f16; 2]>,
        Vec<[f16; 2]>,
        Vec<[f16; 2]>,
        Vec<[f16; 2]>,
        Vec<[f16; 2]>,
    ),
    /// 16-bit float in the red, green and blue channel.
    RgbF16(
        Vec<[f16; 3]>,
        Vec<[f16; 3]>,
        Vec<[f16; 3]>,
        Vec<[f16; 3]>,
        Vec<[f16; 3]>,
        Vec<[f16; 3]>,
    ),
    /// 16-bit float in the red, green, blue and alpha channel.
    RgbaF16(
        Vec<[f16; 4]>,
        Vec<[f16; 4]>,
        Vec<[f16; 4]>,
        Vec<[f16; 4]>,
        Vec<[f16; 4]>,
        Vec<[f16; 4]>,
    ),

    /// 32-bit float in the red channel.
    RF32(Vec<f32>, Vec<f32>, Vec<f32>, Vec<f32>, Vec<f32>, Vec<f32>),
    /// 32-bit float in the red and green channel.
    RgF32(
        Vec<[f32; 2]>,
        Vec<[f32; 2]>,
        Vec<[f32; 2]>,
        Vec<[f32; 2]>,
        Vec<[f32; 2]>,
        Vec<[f32; 2]>,
    ),
    /// 32-bit float in the red, green and blue channel.
    RgbF32(
        Vec<[f32; 3]>,
        Vec<[f32; 3]>,
        Vec<[f32; 3]>,
        Vec<[f32; 3]>,
        Vec<[f32; 3]>,
        Vec<[f32; 3]>,
    ),
    /// 32-bit float in the red, green, blue and alpha channel.
    RgbaF32(
        Vec<[f32; 4]>,
        Vec<[f32; 4]>,
        Vec<[f32; 4]>,
        Vec<[f32; 4]>,
        Vec<[f32; 4]>,
        Vec<[f32; 4]>,
    ),
}

/// See [CpuTextureCube]
#[deprecated = "Renamed to CpuTextureCube"]
pub type CPUTextureCube = CpuTextureCube;

///
/// A CPU-side version of a [TextureCubeMap]. All 6 images must have the same dimensions.
/// Can be constructed manually or loaded via [Loader](crate::Loader).
///
pub struct CpuTextureCube {
    /// The pixel data for the cube image
    pub data: TextureCubeData,
    /// The width of each of the 6 images
    pub width: u32,
    /// The height of each of the 6 images
    pub height: u32,
    /// The format of the image
    #[deprecated = "the texture format is determined by the TextureCubeData"]
    #[allow(deprecated)]
    pub format: Format,
    /// The way the pixel data is interpolated when the texture is far away
    pub min_filter: Interpolation,
    /// The way the pixel data is interpolated when the texture is close
    pub mag_filter: Interpolation,
    /// Specifies whether mipmaps should be created for this texture and what type of interpolation to use between the two closest mipmaps.
    /// Note, however, that the mipmaps only will be created if the width and height of the texture are power of two.
    pub mip_map_filter: Option<Interpolation>,
    /// Determines how the texture is sampled outside the [0..1] s coordinate range.
    pub wrap_s: Wrapping,
    /// Determines how the texture is sampled outside the [0..1] t coordinate range.
    pub wrap_t: Wrapping,
    /// Determines how the texture is sampled outside the [0..1] r coordinate range.
    pub wrap_r: Wrapping,
}

#[allow(deprecated)]
impl Default for CpuTextureCube {
    fn default() -> Self {
        Self {
            data: TextureCubeData::RgbaU8(
                vec![[255, 0, 0, 255]],
                vec![[255, 0, 0, 255]],
                vec![[255, 0, 0, 255]],
                vec![[255, 0, 0, 255]],
                vec![[255, 0, 0, 255]],
                vec![[255, 0, 0, 255]],
            ),
            width: 1,
            height: 1,
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

impl std::fmt::Debug for CpuTextureCube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CpuTexture")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("min_filter", &self.min_filter)
            .field("mag_filter", &self.mag_filter)
            .field("mip_map_filter", &self.mip_map_filter)
            .field("wrap_s", &self.wrap_s)
            .field("wrap_t", &self.wrap_t)
            .field("wrap_r", &self.wrap_r)
            .finish()
    }
}

use crate::core::*;

// COMMON TEXTURE FUNCTIONS

fn generate(context: &Context) -> ThreeDResult<crate::context::Texture> {
    unsafe {
        Ok(context
            .create_texture()
            .map_err(|e| CoreError::TextureCreation(e))?)
    }
}

fn set_parameters(
    context: &Context,
    target: u32,
    min_filter: Interpolation,
    mag_filter: Interpolation,
    mip_map_filter: Option<Interpolation>,
    wrap_s: Wrapping,
    wrap_t: Wrapping,
    wrap_r: Option<Wrapping>,
) -> ThreeDResult<()> {
    unsafe {
        match mip_map_filter {
            None => context.tex_parameter_i32(
                target,
                crate::context::TEXTURE_MIN_FILTER,
                interpolation_from(min_filter),
            ),
            Some(Interpolation::Nearest) => {
                if min_filter == Interpolation::Nearest {
                    context.tex_parameter_i32(
                        target,
                        crate::context::TEXTURE_MIN_FILTER,
                        crate::context::NEAREST_MIPMAP_NEAREST as i32,
                    );
                } else {
                    context.tex_parameter_i32(
                        target,
                        crate::context::TEXTURE_MIN_FILTER,
                        crate::context::LINEAR_MIPMAP_NEAREST as i32,
                    )
                }
            }
            Some(Interpolation::Linear) => {
                if min_filter == Interpolation::Nearest {
                    context.tex_parameter_i32(
                        target,
                        crate::context::TEXTURE_MIN_FILTER,
                        crate::context::NEAREST_MIPMAP_LINEAR as i32,
                    );
                } else {
                    context.tex_parameter_i32(
                        target,
                        crate::context::TEXTURE_MIN_FILTER,
                        crate::context::LINEAR_MIPMAP_LINEAR as i32,
                    )
                }
            }
        }
        context.tex_parameter_i32(
            target,
            crate::context::TEXTURE_MAG_FILTER,
            interpolation_from(mag_filter),
        );
        context.tex_parameter_i32(
            target,
            crate::context::TEXTURE_WRAP_S,
            wrapping_from(wrap_s),
        );
        context.tex_parameter_i32(
            target,
            crate::context::TEXTURE_WRAP_T,
            wrapping_from(wrap_t),
        );
        if let Some(r) = wrap_r {
            context.tex_parameter_i32(target, crate::context::TEXTURE_WRAP_R, wrapping_from(r));
        }
    }
    context.error_check()
}

fn calculate_number_of_mip_maps(
    mip_map_filter: Option<Interpolation>,
    width: u32,
    height: u32,
    depth: Option<u32>,
) -> u32 {
    if mip_map_filter.is_some()
        && width == height
        && depth.map(|d| d == width).unwrap_or(true)
        && width.is_power_of_two()
    {
        (width as f64).log2() as u32 + 1
    } else {
        1
    }
}

fn internal_format_from_depth(format: DepthFormat) -> u32 {
    match format {
        DepthFormat::Depth16 => crate::context::DEPTH_COMPONENT16,
        DepthFormat::Depth24 => crate::context::DEPTH_COMPONENT24,
        DepthFormat::Depth32F => crate::context::DEPTH_COMPONENT32F,
    }
}

fn wrapping_from(wrapping: Wrapping) -> i32 {
    (match wrapping {
        Wrapping::Repeat => crate::context::REPEAT,
        Wrapping::MirroredRepeat => crate::context::MIRRORED_REPEAT,
        Wrapping::ClampToEdge => crate::context::CLAMP_TO_EDGE,
    }) as i32
}

fn interpolation_from(interpolation: Interpolation) -> i32 {
    (match interpolation {
        Interpolation::Nearest => crate::context::NEAREST,
        Interpolation::Linear => crate::context::LINEAR,
    }) as i32
}

fn check_data_length<T: TextureDataType>(
    width: u32,
    height: u32,
    depth: u32,
    data_byte_size: usize,
    data: &[T],
) -> ThreeDResult<()> {
    let expected_bytes = width as usize * height as usize * depth as usize * data_byte_size;
    let actual_bytes = data.len() * std::mem::size_of::<T>();
    if expected_bytes != actual_bytes {
        Err(CoreError::InvalidTextureLength(
            actual_bytes,
            expected_bytes,
        ))?;
    }
    Ok(())
}
