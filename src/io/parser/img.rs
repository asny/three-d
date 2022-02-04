use crate::core::*;
use crate::io::*;
use std::path::Path;

///
/// Deserialize the given bytes representing an image into a [CpuTexture] using
/// the [image](https://crates.io/crates/image/main.rs) crate.
/// The CpuTexture can then be used to create a [Texture2D].
/// Supported formats: PNG, JPEG, GIF, WebP, pnm (pbm, pgm, ppm and pam), TIFF, DDS, BMP, ICO, HDR, farbfeld.
/// **Note:** If the image contains and you want to load high dynamic range (hdr) information, use [hdr_image_from_bytes] instead.
///
pub fn image_from_bytes(bytes: &[u8]) -> ThreeDResult<crate::core::CpuTexture<u8>> {
    use crate::core::*;
    use image::DynamicImage;
    use image::GenericImageView;
    let img = image::load_from_memory(bytes)?;
    let format = match img {
        DynamicImage::ImageLuma8(_) => Format::R,
        DynamicImage::ImageLumaA8(_) => Format::RG,
        DynamicImage::ImageRgb8(_) => Format::RGB,
        DynamicImage::ImageRgba8(_) => Format::RGBA,
        DynamicImage::ImageBgr8(_) => unimplemented!(),
        DynamicImage::ImageBgra8(_) => unimplemented!(),
        DynamicImage::ImageLuma16(_) => unimplemented!(),
        DynamicImage::ImageLumaA16(_) => unimplemented!(),
        DynamicImage::ImageRgb16(_) => unimplemented!(),
        DynamicImage::ImageRgba16(_) => unimplemented!(),
    };

    Ok(CpuTexture {
        data: img.to_bytes(),
        width: img.width(),
        height: img.height(),
        format,
        ..Default::default()
    })
}

///
/// Deserialize the given bytes representing a hdr image into a [CpuTexture] using
/// the [image](https://crates.io/crates/image/main.rs) crate.
/// The CpuTexture can then be used to create a [Texture2D] or a [TextureCubeMap] using the `new_from_equirectangular` method.
/// Supported formats: HDR.
///
pub fn hdr_image_from_bytes(bytes: &[u8]) -> ThreeDResult<CpuTexture<f32>> {
    use image::codecs::hdr::*;
    use image::*;
    let decoder = HdrDecoder::new(bytes)?;
    let metadata = decoder.metadata();
    let img = decoder.read_image_native()?;
    Ok(CpuTexture {
        data: img
            .iter()
            .map(|rgbe| {
                let Rgb(values) = rgbe.to_hdr();
                values
            })
            .flatten()
            .collect::<Vec<_>>(),
        width: metadata.width,
        height: metadata.height,
        format: Format::RGB,
        ..Default::default()
    })
}

///
/// Deserialize the 6 images given as byte arrays into a [CpuTextureCube] using
/// the [image](https://crates.io/crates/image/main.rs) crate.
/// The CpuTextureCube can then be used to create a [TextureCubeMap].
///
pub fn cube_image_from_bytes(
    right_bytes: &[u8],
    left_bytes: &[u8],
    top_bytes: &[u8],
    bottom_bytes: &[u8],
    front_bytes: &[u8],
    back_bytes: &[u8],
) -> ThreeDResult<CpuTextureCube<u8>> {
    let right = image_from_bytes(right_bytes)?;
    let left = image_from_bytes(left_bytes)?;
    let top = image_from_bytes(top_bytes)?;
    let bottom = image_from_bytes(bottom_bytes)?;
    let front = image_from_bytes(front_bytes)?;
    let back = image_from_bytes(back_bytes)?;

    Ok(CpuTextureCube {
        right_data: right.data,
        left_data: left.data,
        top_data: top.data,
        bottom_data: bottom.data,
        front_data: front.data,
        back_data: back.data,
        width: right.width,
        height: right.height,
        format: right.format,
        min_filter: right.min_filter,
        mag_filter: right.mag_filter,
        mip_map_filter: right.mip_map_filter,
        wrap_s: right.wrap_s,
        wrap_t: right.wrap_t,
        wrap_r: right.wrap_s,
    })
}

impl Loaded {
    ///
    /// Deserialize the loaded image resource at the given path into a [CpuTexture] using
    /// the [image](https://crates.io/crates/image/main.rs) crate.
    /// The CpuTexture can then be used to create a [Texture2D].
    /// Supported formats: PNG, JPEG, GIF, WebP, pnm (pbm, pgm, ppm and pam), TIFF, DDS, BMP, ICO, HDR, farbfeld.
    /// **Note:** If the image contains high dynamic range (hdr) information, use [hdr_image](Loaded::hdr_image) instead.
    ///
    pub fn image<P: AsRef<Path>>(&mut self, path: P) -> ThreeDResult<CpuTexture<u8>> {
        image_from_bytes(&self.get_bytes(path)?)
    }

    ///
    /// Deserialize the loaded image resource with hdr information at the given path into a [CpuTexture] using
    /// the [image](https://crates.io/crates/image/main.rs) crate.
    /// The CpuTexture can then be used to create a [Texture2D] or a [TextureCubeMap] using the `new_from_equirectangular` method.
    /// Supported formats: HDR.
    ///
    pub fn hdr_image(&mut self, path: impl AsRef<Path>) -> ThreeDResult<CpuTexture<f32>> {
        hdr_image_from_bytes(&self.get_bytes(path)?)
    }

    ///
    /// Deserialize the 6 loaded image resources at the given paths into a [CpuTextureCube] using
    /// the [image](https://crates.io/crates/image/main.rs) crate.
    /// The CpuTextureCube can then be used to create a [TextureCubeMap].
    ///
    pub fn cube_image<P: AsRef<Path>>(
        &mut self,
        right_path: P,
        left_path: P,
        top_path: P,
        bottom_path: P,
        front_path: P,
        back_path: P,
    ) -> ThreeDResult<CpuTextureCube<u8>> {
        let right = self.image(right_path)?;
        let left = self.image(left_path)?;
        let top = self.image(top_path)?;
        let bottom = self.image(bottom_path)?;
        let front = self.image(front_path)?;
        let back = self.image(back_path)?;

        Ok(CpuTextureCube {
            right_data: right.data,
            left_data: left.data,
            top_data: top.data,
            bottom_data: bottom.data,
            front_data: front.data,
            back_data: back.data,
            width: right.width,
            height: right.height,
            format: right.format,
            min_filter: right.min_filter,
            mag_filter: right.mag_filter,
            mip_map_filter: right.mip_map_filter,
            wrap_s: right.wrap_s,
            wrap_t: right.wrap_t,
            wrap_r: right.wrap_s,
        })
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Saver {
    ///
    /// Saves the given RGB pixels as an image.
    ///
    pub fn save_pixels<P: AsRef<Path>>(
        path: P,
        pixels: &[u8],
        width: u32,
        height: u32,
    ) -> ThreeDResult<()> {
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
