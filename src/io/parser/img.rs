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
pub fn image_from_bytes(bytes: &[u8]) -> ThreeDResult<CpuTexture> {
    use image::DynamicImage;
    use image::GenericImageView;
    let img = image::load_from_memory(bytes)?;
    let bytes = img.to_bytes();
    let width = img.width();
    let height = img.height();
    let data = match img {
        DynamicImage::ImageLuma8(_) => TextureData::RU8(bytes),
        DynamicImage::ImageLumaA8(_) => {
            let mut data = Vec::new();
            for i in 0..bytes.len() / 2 {
                data.push([bytes[i * 2], bytes[i * 2 + 1]]);
            }
            TextureData::RgU8(data)
        }
        DynamicImage::ImageRgb8(_) => {
            let mut data = Vec::new();
            for i in 0..bytes.len() / 3 {
                data.push([bytes[i * 3], bytes[i * 3 + 1], bytes[i * 3 + 2]]);
            }
            TextureData::RgbU8(data)
        }
        DynamicImage::ImageRgba8(_) => {
            let mut data = Vec::new();
            for i in 0..bytes.len() / 4 {
                data.push([
                    bytes[i * 4],
                    bytes[i * 4 + 1],
                    bytes[i * 4 + 2],
                    bytes[i * 4 + 3],
                ]);
            }
            TextureData::RgbaU8(data)
        }
        DynamicImage::ImageBgr8(_) => unimplemented!(),
        DynamicImage::ImageBgra8(_) => unimplemented!(),
        DynamicImage::ImageLuma16(_) => unimplemented!(),
        DynamicImage::ImageLumaA16(_) => unimplemented!(),
        DynamicImage::ImageRgb16(_) => unimplemented!(),
        DynamicImage::ImageRgba16(_) => unimplemented!(),
    };
    Ok(CpuTexture {
        data,
        width,
        height,
        ..Default::default()
    })
}

///
/// Deserialize the given bytes representing a hdr image into a [CpuTexture] using
/// the [image](https://crates.io/crates/image/main.rs) crate.
/// The CpuTexture can then be used to create a [Texture2D] or a [TextureCubeMap] using the `new_from_equirectangular` method.
/// Supported formats: HDR.
///
pub fn hdr_image_from_bytes(bytes: &[u8]) -> ThreeDResult<CpuTexture> {
    use image::codecs::hdr::*;
    use image::*;
    let decoder = HdrDecoder::new(bytes)?;
    let metadata = decoder.metadata();
    let img = decoder.read_image_native()?;
    Ok(CpuTexture {
        data: TextureData::RgbF32(
            img.iter()
                .map(|rgbe| {
                    let Rgb(values) = rgbe.to_hdr();
                    [values[0], values[1], values[2]]
                })
                .collect::<Vec<_>>(),
        ),
        width: metadata.width,
        height: metadata.height,
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
) -> ThreeDResult<CpuTextureCube> {
    let right = image_from_bytes(right_bytes)?;
    let left = image_from_bytes(left_bytes)?;
    let top = image_from_bytes(top_bytes)?;
    let bottom = image_from_bytes(bottom_bytes)?;
    let front = image_from_bytes(front_bytes)?;
    let back = image_from_bytes(back_bytes)?;
    let data = match right.data {
        TextureData::RU8(right) => {
            let left = if let TextureData::RU8(data) = left.data {
                data
            } else {
                unreachable!()
            };
            let top = if let TextureData::RU8(data) = top.data {
                data
            } else {
                unreachable!()
            };
            let bottom = if let TextureData::RU8(data) = bottom.data {
                data
            } else {
                unreachable!()
            };
            let front = if let TextureData::RU8(data) = front.data {
                data
            } else {
                unreachable!()
            };
            let back = if let TextureData::RU8(data) = back.data {
                data
            } else {
                unreachable!()
            };
            TextureCubeData::RU8(right, left, top, bottom, front, back)
        }
        TextureData::RgU8(right) => {
            let left = if let TextureData::RgU8(data) = left.data {
                data
            } else {
                unreachable!()
            };
            let top = if let TextureData::RgU8(data) = top.data {
                data
            } else {
                unreachable!()
            };
            let bottom = if let TextureData::RgU8(data) = bottom.data {
                data
            } else {
                unreachable!()
            };
            let front = if let TextureData::RgU8(data) = front.data {
                data
            } else {
                unreachable!()
            };
            let back = if let TextureData::RgU8(data) = back.data {
                data
            } else {
                unreachable!()
            };
            TextureCubeData::RgU8(right, left, top, bottom, front, back)
        }
        TextureData::RgbU8(right) => {
            let left = if let TextureData::RgbU8(data) = left.data {
                data
            } else {
                unreachable!()
            };
            let top = if let TextureData::RgbU8(data) = top.data {
                data
            } else {
                unreachable!()
            };
            let bottom = if let TextureData::RgbU8(data) = bottom.data {
                data
            } else {
                unreachable!()
            };
            let front = if let TextureData::RgbU8(data) = front.data {
                data
            } else {
                unreachable!()
            };
            let back = if let TextureData::RgbU8(data) = back.data {
                data
            } else {
                unreachable!()
            };
            TextureCubeData::RgbU8(right, left, top, bottom, front, back)
        }
        TextureData::RgbaU8(right) => {
            let left = if let TextureData::RgbaU8(data) = left.data {
                data
            } else {
                unreachable!()
            };
            let top = if let TextureData::RgbaU8(data) = top.data {
                data
            } else {
                unreachable!()
            };
            let bottom = if let TextureData::RgbaU8(data) = bottom.data {
                data
            } else {
                unreachable!()
            };
            let front = if let TextureData::RgbaU8(data) = front.data {
                data
            } else {
                unreachable!()
            };
            let back = if let TextureData::RgbaU8(data) = back.data {
                data
            } else {
                unreachable!()
            };
            TextureCubeData::RgbaU8(right, left, top, bottom, front, back)
        }
        _ => unimplemented!(),
    };

    Ok(CpuTextureCube {
        data,
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
    pub fn image<P: AsRef<Path>>(&mut self, path: P) -> ThreeDResult<CpuTexture> {
        image_from_bytes(&self.get_bytes(path)?)
    }

    ///
    /// Deserialize the loaded image resource with hdr information at the given path into a [CpuTexture] using
    /// the [image](https://crates.io/crates/image/main.rs) crate.
    /// The CpuTexture can then be used to create a [Texture2D] or a [TextureCubeMap] using the `new_from_equirectangular` method.
    /// Supported formats: HDR.
    ///
    pub fn hdr_image(&mut self, path: impl AsRef<Path>) -> ThreeDResult<CpuTexture> {
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
    ) -> ThreeDResult<CpuTextureCube> {
        cube_image_from_bytes(
            self.get_bytes(right_path)?,
            self.get_bytes(left_path)?,
            self.get_bytes(top_path)?,
            self.get_bytes(bottom_path)?,
            self.get_bytes(front_path)?,
            self.get_bytes(back_path)?,
        )
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Saver {
    ///
    /// Saves the given RGBA pixels as an image.
    ///
    pub fn save_pixels<P: AsRef<Path>>(
        path: P,
        pixels: &[[u8; 4]],
        width: u32,
        height: u32,
    ) -> ThreeDResult<()> {
        let mut pixels_out = vec![[0u8; 4]; width as usize * height as usize];
        for row in 0..height as usize {
            for col in 0..width as usize {
                pixels_out[width as usize * (height as usize - row - 1) + col] =
                    pixels[width as usize * row + col];
            }
        }

        image::save_buffer(
            path,
            &pixels_out.iter().flatten().map(|v| *v).collect::<Vec<_>>(),
            width as u32,
            height as u32,
            image::ColorType::Rgba8,
        )?;
        Ok(())
    }
}
