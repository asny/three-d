//!
//! Different types of textures used by the GPU to read from and write to.
//!
mod texture2d;
#[doc(inline)]
pub use texture2d::*;

mod texture_cube_map;
#[doc(inline)]
pub use texture_cube_map::*;

mod depth_texture2d;
#[doc(inline)]
pub use depth_texture2d::*;

mod texture2d_array;
#[doc(inline)]
pub use texture2d_array::*;

mod texture2d_multisample;
#[doc(inline)]
pub(in crate::core) use texture2d_multisample::*;

mod texture3d;
#[doc(inline)]
pub use texture3d::*;

mod depth_texture2d_array;
#[doc(inline)]
pub use depth_texture2d_array::*;

mod depth_texture_cube_map;
#[doc(inline)]
pub use depth_texture_cube_map::*;

mod depth_texture2d_multisample;
#[doc(inline)]
pub(in crate::core) use depth_texture2d_multisample::*;

use data_type::*;
pub use three_d_asset::texture::{
    Interpolation, Texture2D as CpuTexture, Texture3D as CpuTexture3D, TextureData, Wrapping,
};

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

/// The basic data type used for each pixel in a depth texture.
pub trait DepthTextureDataType: DepthDataType {}

/// 24 bit float which can be used as [DepthTextureDataType].
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Default, Debug)]
pub struct f24 {}

impl DepthTextureDataType for f16 {}
impl DepthTextureDataType for f24 {}
impl DepthTextureDataType for f32 {}

///
/// A reference to some type of texture containing colors.
///
#[derive(Clone, Copy)]
#[allow(missing_docs)]
pub enum ColorTexture<'a> {
    /// A single 2D texture.
    Single(&'a Texture2D),
    /// An array of 2D textures and a set of indices into the array.
    Array {
        texture: &'a Texture2DArray,
        layers: &'a [u32],
    },
    /// A cube map texture and a set of [CubeMapSide]s indicating the sides to use.
    CubeMap {
        texture: &'a TextureCubeMap,
        sides: &'a [CubeMapSide],
    },
}

impl ColorTexture<'_> {
    ///
    /// Returns the width of the color texture in texels.
    ///
    pub fn width(&self) -> u32 {
        match self {
            ColorTexture::Single(texture) => texture.width(),
            ColorTexture::Array { texture, .. } => texture.width(),
            ColorTexture::CubeMap { texture, .. } => texture.width(),
        }
    }

    ///
    /// Returns the height of the color texture in texels.
    ///
    pub fn height(&self) -> u32 {
        match self {
            ColorTexture::Single(texture) => texture.height(),
            ColorTexture::Array { texture, .. } => texture.height(),
            ColorTexture::CubeMap { texture, .. } => texture.height(),
        }
    }
}

///
/// A reference to some type of texture containing depths.
///
#[derive(Clone, Copy)]
#[allow(missing_docs)]
pub enum DepthTexture<'a> {
    /// A single 2D texture.
    Single(&'a DepthTexture2D),
    /// An array of 2D textures and an index into the array.
    Array {
        texture: &'a DepthTexture2DArray,
        layer: u32,
    },
    /// A cube map texture and a [CubeMapSide] indicating the side to use.
    CubeMap {
        texture: &'a DepthTextureCubeMap,
        side: CubeMapSide,
    },
}

impl DepthTexture<'_> {
    ///
    /// Returns the width of the depth texture in texels.
    ///
    pub fn width(&self) -> u32 {
        match self {
            DepthTexture::Single(texture) => texture.width(),
            DepthTexture::Array { texture, .. } => texture.width(),
            DepthTexture::CubeMap { texture, .. } => texture.width(),
        }
    }

    ///
    /// Returns the height of the depth texture in texels.
    ///
    pub fn height(&self) -> u32 {
        match self {
            DepthTexture::Single(texture) => texture.height(),
            DepthTexture::Array { texture, .. } => texture.height(),
            DepthTexture::CubeMap { texture, .. } => texture.height(),
        }
    }
}

use crate::core::*;

// COMMON TEXTURE FUNCTIONS

fn generate(context: &Context) -> crate::context::Texture {
    unsafe { context.create_texture().expect("Failed creating texture") }
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
) {
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
            _ => panic!("Can only sample textures using 'NEAREST' or 'LINEAR' interpolation"),
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
        _ => panic!("Can only sample textures using 'NEAREST' or 'LINEAR' interpolation"),
    }) as i32
}

fn check_data_length<T: TextureDataType>(
    width: u32,
    height: u32,
    depth: u32,
    data_byte_size: usize,
    data_len: usize,
) {
    let expected_bytes = width as usize * height as usize * depth as usize * data_byte_size;
    let actual_bytes = data_len * std::mem::size_of::<T>();
    if expected_bytes != actual_bytes {
        panic!(
            "invalid size of texture data (expected {} bytes but got {} bytes)",
            expected_bytes, actual_bytes
        )
    }
}

fn ru8_data(t: &CpuTexture) -> &[u8] {
    if let TextureData::RU8(data) = &t.data {
        data
    } else {
        panic!("all of the images used for cube map sides must have the same texture data type")
    }
}

fn rgu8_data(t: &CpuTexture) -> &[[u8; 2]] {
    if let TextureData::RgU8(data) = &t.data {
        data
    } else {
        panic!("all of the images used for cube map sides must have the same texture data type")
    }
}

fn rgbu8_data(t: &CpuTexture) -> &[[u8; 3]] {
    if let TextureData::RgbU8(data) = &t.data {
        data
    } else {
        panic!("all of the images used for cube map sides must have the same texture data type")
    }
}

fn rgbau8_data(t: &CpuTexture) -> &[[u8; 4]] {
    if let TextureData::RgbaU8(data) = &t.data {
        data
    } else {
        panic!("all of the images used for cube map sides must have the same texture data type")
    }
}

fn rf16_data(t: &CpuTexture) -> &[f16] {
    if let TextureData::RF16(data) = &t.data {
        data
    } else {
        panic!("all of the images used for cube map sides must have the same texture data type")
    }
}

fn rgf16_data(t: &CpuTexture) -> &[[f16; 2]] {
    if let TextureData::RgF16(data) = &t.data {
        data
    } else {
        panic!("all of the images used for cube map sides must have the same texture data type")
    }
}

fn rgbf16_data(t: &CpuTexture) -> &[[f16; 3]] {
    if let TextureData::RgbF16(data) = &t.data {
        data
    } else {
        panic!("all of the images used for cube map sides must have the same texture data type")
    }
}

fn rgbaf16_data(t: &CpuTexture) -> &[[f16; 4]] {
    if let TextureData::RgbaF16(data) = &t.data {
        data
    } else {
        panic!("all of the images used for cube map sides must have the same texture data type")
    }
}

fn rf32_data(t: &CpuTexture) -> &[f32] {
    if let TextureData::RF32(data) = &t.data {
        data
    } else {
        panic!("all of the images used for cube map sides must have the same texture data type")
    }
}

fn rgf32_data(t: &CpuTexture) -> &[[f32; 2]] {
    if let TextureData::RgF32(data) = &t.data {
        data
    } else {
        panic!("all of the images used for cube map sides must have the same texture data type")
    }
}

fn rgbf32_data(t: &CpuTexture) -> &[[f32; 3]] {
    if let TextureData::RgbF32(data) = &t.data {
        data
    } else {
        panic!("all of the images used for cube map sides must have the same texture data type")
    }
}

fn rgbaf32_data(t: &CpuTexture) -> &[[f32; 4]] {
    if let TextureData::RgbaF32(data) = &t.data {
        data
    } else {
        panic!("all of the images used for cube map sides must have the same texture data type")
    }
}
