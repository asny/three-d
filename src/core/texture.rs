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
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Interpolation {
    Nearest,
    Linear,
}

///
/// Possible wrapping modes for a texture which determines how the texture is applied outside of the
/// [0..1] uv coordinate range.
///
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Wrapping {
    Repeat,
    MirroredRepeat,
    ClampToEdge,
}

pub trait TextureDataType:
    Default + std::fmt::Debug + Clone + crate::core::internal::TextureDataTypeExtension
{
}
impl TextureDataType for u8 {}
impl TextureDataType for f32 {}
impl TextureDataType for u32 {}

///
/// Possible formats for pixels in a texture.
///
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Format {
    R,
    RG,
    RGB,
    RGBA,
}

impl Format {
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
    pub fn add_padding(&mut self, x0: u32, x1: u32, y0: u32, y1: u32) {
        let channels = self.format.color_channel_count();
        let width = x0 + self.width + x1;
        let height = y0 + self.height + y1;
        let mut new_data = vec![T::default(); width as usize * height as usize * channels as usize];
        for x in 0..self.width {
            for y in 0..self.height {
                let x_ = x + x0;
                let y_ = y + y0;
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

use crate::io::*;
use std::path::Path;

#[cfg(feature = "image-io")]
impl Loaded {
    ///
    /// Deserialize the loaded image resource at the given path into a [CPUTexture] using
    /// the [image](https://crates.io/crates/image/main.rs) crate.
    /// The CPUTexture can then be used to create a [Texture2D].
    ///
    /// # Feature
    /// Only available when the `image-io` feature is enabled.
    ///
    pub fn image<P: AsRef<Path>>(&mut self, path: P) -> Result<CPUTexture<u8>, IOError> {
        image_from_bytes(&self.get_bytes(path)?)
    }

    ///
    /// Deserialize the 6 loaded image resources at the given paths into a [CPUTexture] using
    /// the [image](https://crates.io/crates/image/main.rs) crate.
    /// The CPUTexture can then be used to create a [TextureCubeMap].
    ///
    /// # Feature
    /// Only available when the `image-io` feature is enabled.
    ///
    pub fn cube_image<P: AsRef<Path>>(
        &mut self,
        right_path: P,
        left_path: P,
        top_path: P,
        bottom_path: P,
        front_path: P,
        back_path: P,
    ) -> Result<CPUTexture<u8>, IOError> {
        let mut right = self.image(right_path)?;
        let left = self.image(left_path)?;
        let top = self.image(top_path)?;
        let bottom = self.image(bottom_path)?;
        let front = self.image(front_path)?;
        let back = self.image(back_path)?;

        right.data.extend(left.data);
        right.data.extend(top.data);
        right.data.extend(bottom.data);
        right.data.extend(front.data);
        right.data.extend(back.data);
        Ok(right)
    }
}

#[cfg(all(feature = "image-io", not(target_arch = "wasm32")))]
impl Saver {
    ///
    /// Saves the given RGB pixels as an image.
    ///
    /// # Feature
    /// Only available when the `image-io` feature is enabled.
    ///
    pub fn save_pixels<P: AsRef<Path>>(
        path: P,
        pixels: &[u8],
        width: u32,
        height: u32,
    ) -> Result<(), IOError> {
        let mut pixels_out = vec![0u8; width as usize * height as usize * 4];
        for row in 0..height as usize {
            for col in 0..width as usize {
                for i in 0..4 {
                    pixels_out[4 * width as usize * (height as usize - row - 1) + 4 * col + i] =
                        pixels[4 * width as usize * row + 4 * col + i];
                }
            }
        }

        image::save_buffer(
            path,
            &pixels_out,
            width as u32,
            height as u32,
            image::ColorType::Rgba8,
        )?;
        Ok(())
    }
}
