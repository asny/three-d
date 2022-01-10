use crate::core::*;
use crate::io::*;
use std::path::Path;

impl Loaded {
    ///
    /// Deserialize the loaded image resource at the given path into a [CPUTexture] using
    /// the [image](https://crates.io/crates/image/main.rs) crate.
    /// The CPUTexture can then be used to create a [Texture2D].
    ///
    pub fn image<P: AsRef<Path>>(&mut self, path: P) -> ThreeDResult<CPUTexture<u8>> {
        image_from_bytes(&self.get_bytes(path)?)
    }

    pub fn hdr_image(&mut self, path: impl AsRef<Path>) -> ThreeDResult<CPUTexture<f32>> {
        use image::codecs::hdr::*;
        use image::*;
        let bytes = self.get_bytes(path)?;
        let decoder = HdrDecoder::new(bytes)?;
        let metadata = decoder.metadata();
        let img = decoder.read_image_native()?;
        Ok(CPUTexture {
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
    /// Deserialize the 6 loaded image resources at the given paths into a [CPUTextureCube] using
    /// the [image](https://crates.io/crates/image/main.rs) crate.
    /// The CPUTextureCube can then be used to create a [TextureCubeMap].
    ///
    pub fn cube_image<P: AsRef<Path>>(
        &mut self,
        right_path: P,
        left_path: P,
        top_path: P,
        bottom_path: P,
        front_path: P,
        back_path: P,
    ) -> ThreeDResult<CPUTextureCube<u8>> {
        let right = self.image(right_path)?;
        let left = self.image(left_path)?;
        let top = self.image(top_path)?;
        let bottom = self.image(bottom_path)?;
        let front = self.image(front_path)?;
        let back = self.image(back_path)?;

        Ok(CPUTextureCube {
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
    }}
