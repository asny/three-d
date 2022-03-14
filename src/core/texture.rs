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

/// The basic data type used for each channel of each pixel in a texture.
pub trait TextureDataType: std::fmt::Debug + crate::core::internal::DataType {}
impl TextureDataType for u8 {}
impl TextureDataType for f16 {}
impl TextureDataType for f32 {}

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

    pub(super) fn as_const(&self) -> u32 {
        match self {
            Format::R => glow::RED,
            Format::RG => glow::RG,
            Format::RGB => glow::RGB,
            Format::RGBA => glow::RGBA,
        }
    }
}

/// See [CpuTexture]
#[deprecated = "Renamed to CpuTexture"]
pub type CPUTexture<T> = CpuTexture<T>;

///
/// A CPU-side version of a [Texture2D].
/// Can be constructed manually or loaded via [Loader](crate::Loader).
///
#[derive(Clone)]
pub struct CpuTexture<T: TextureDataType> {
    /// The pixel data for the image
    pub data: Vec<T>,
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

impl<T: TextureDataType> CpuTexture<T> {
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

    /// Returns whether or not this texture is transparent by looking at its alpha values.
    pub fn is_transparent(&self) -> bool {
        if self.format == Format::RGBA {
            for i in 0..self.width as usize * self.height as usize {
                if !self.data[i * 4 + 3].is_max() {
                    return true;
                }
            }
        }
        false
    }
}

impl<T: TextureDataType> Default for CpuTexture<T> {
    fn default() -> Self {
        Self {
            data: [T::default(), T::default(), T::default(), T::default()].into(),
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

impl<T: TextureDataType> std::fmt::Debug for CpuTexture<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CpuTexture")
            .field("format", &self.format)
            .field("data length", &self.data.len())
            .field("width", &self.width)
            .field("height", &self.height)
            .field("min_filter", &self.min_filter)
            .field("mag_filter", &self.mag_filter)
            .field("mip_map_filter", &self.mip_map_filter)
            .field("wrap_s", &self.wrap_s)
            .field("wrap_t", &self.wrap_t)
            .finish()
    }
}

///
/// A CPU-side version of a [Texture3D].
///
#[derive(Clone)]
pub struct CpuTexture3D<T: TextureDataType> {
    /// The pixel data for the image
    pub data: Vec<T>,
    /// The width of the image
    pub width: u32,
    /// The height of the image
    pub height: u32,
    /// The depth of the image
    pub depth: u32,
    /// The format of the image
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

impl<T: TextureDataType> Default for CpuTexture3D<T> {
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

impl<T: TextureDataType> std::fmt::Debug for CpuTexture3D<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CpuTexture3D")
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

/// See [CpuTextureCube]
#[deprecated = "Renamed to CpuTextureCube"]
pub type CPUTextureCube<T> = CpuTextureCube<T>;

///
/// A CPU-side version of a [TextureCubeMap]. All 6 images must have the same dimensions.
/// Can be constructed manually or loaded via [Loader](crate::Loader).
///
pub struct CpuTextureCube<T: TextureDataType> {
    /// The pixel data for the right image
    pub right_data: Vec<T>,
    /// The pixel data for the left image
    pub left_data: Vec<T>,
    /// The pixel data for the top image
    pub top_data: Vec<T>,
    /// The pixel data for the bottom image
    pub bottom_data: Vec<T>,
    /// The pixel data for the front image
    pub front_data: Vec<T>,
    /// The pixel data for the back image
    pub back_data: Vec<T>,
    /// The width of each of the 6 images
    pub width: u32,
    /// The height of each of the 6 images
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
    /// Determines how the texture is sampled outside the [0..1] s coordinate range.
    pub wrap_s: Wrapping,
    /// Determines how the texture is sampled outside the [0..1] t coordinate range.
    pub wrap_t: Wrapping,
    /// Determines how the texture is sampled outside the [0..1] r coordinate range.
    pub wrap_r: Wrapping,
}

impl<T: TextureDataType> Default for CpuTextureCube<T> {
    fn default() -> Self {
        Self {
            right_data: vec![],
            left_data: vec![],
            top_data: vec![],
            bottom_data: vec![],
            front_data: vec![],
            back_data: vec![],
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

impl<T: TextureDataType> std::fmt::Debug for CpuTextureCube<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CpuTexture")
            .field("format", &self.format)
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

mod internal {
    pub trait TextureExtensions {
        fn bind(&self);
    }
    impl<T: TextureExtensions + ?Sized> TextureExtensions for &T {
        fn bind(&self) {
            (*self).bind()
        }
    }
    impl<T: TextureExtensions + ?Sized> TextureExtensions for &mut T {
        fn bind(&self) {
            (**self).bind()
        }
    }
    impl<T: TextureExtensions> TextureExtensions for Box<T> {
        fn bind(&self) {
            self.as_ref().bind()
        }
    }
    impl<T: TextureExtensions> TextureExtensions for std::rc::Rc<T> {
        fn bind(&self) {
            self.as_ref().bind()
        }
    }
    impl<T: TextureExtensions> TextureExtensions for std::rc::Rc<std::cell::RefCell<T>> {
        fn bind(&self) {
            self.borrow().bind()
        }
    }
}

use crate::core::*;
use glow::HasContext;

///
/// A texture that can be sampled in a fragment shader (see [Program::use_texture].
///
pub trait Texture: internal::TextureExtensions {}

impl<T: Texture + ?Sized> Texture for &T {}
impl<T: Texture + ?Sized> Texture for &mut T {}
impl<T: Texture> Texture for Box<T> {}
impl<T: Texture> Texture for std::rc::Rc<T> {}
impl<T: Texture> Texture for std::rc::Rc<std::cell::RefCell<T>> {}

// COMMON TEXTURE FUNCTIONS
fn generate(context: &Context) -> ThreeDResult<glow::Texture> {
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
                glow::TEXTURE_MIN_FILTER,
                interpolation_from(min_filter),
            ),
            Some(Interpolation::Nearest) => {
                if min_filter == Interpolation::Nearest {
                    context.tex_parameter_i32(
                        target,
                        glow::TEXTURE_MIN_FILTER,
                        glow::NEAREST_MIPMAP_NEAREST as i32,
                    );
                } else {
                    context.tex_parameter_i32(
                        target,
                        glow::TEXTURE_MIN_FILTER,
                        glow::LINEAR_MIPMAP_NEAREST as i32,
                    )
                }
            }
            Some(Interpolation::Linear) => {
                if min_filter == Interpolation::Nearest {
                    context.tex_parameter_i32(
                        target,
                        glow::TEXTURE_MIN_FILTER,
                        glow::NEAREST_MIPMAP_LINEAR as i32,
                    );
                } else {
                    context.tex_parameter_i32(
                        target,
                        glow::TEXTURE_MIN_FILTER,
                        glow::LINEAR_MIPMAP_LINEAR as i32,
                    )
                }
            }
        }
        context.tex_parameter_i32(
            target,
            glow::TEXTURE_MAG_FILTER,
            interpolation_from(mag_filter),
        );
        context.tex_parameter_i32(target, glow::TEXTURE_WRAP_S, wrapping_from(wrap_s));
        context.tex_parameter_i32(target, glow::TEXTURE_WRAP_T, wrapping_from(wrap_t));
        if let Some(r) = wrap_r {
            context.tex_parameter_i32(target, glow::TEXTURE_WRAP_R, wrapping_from(r));
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
        DepthFormat::Depth16 => glow::DEPTH_COMPONENT16,
        DepthFormat::Depth24 => glow::DEPTH_COMPONENT24,
        DepthFormat::Depth32F => glow::DEPTH_COMPONENT32F,
    }
}

fn wrapping_from(wrapping: Wrapping) -> i32 {
    (match wrapping {
        Wrapping::Repeat => glow::REPEAT,
        Wrapping::MirroredRepeat => glow::MIRRORED_REPEAT,
        Wrapping::ClampToEdge => glow::CLAMP_TO_EDGE,
    }) as i32
}

fn interpolation_from(interpolation: Interpolation) -> i32 {
    (match interpolation {
        Interpolation::Nearest => glow::NEAREST,
        Interpolation::Linear => glow::LINEAR,
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
