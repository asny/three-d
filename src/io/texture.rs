use crate::definition::*;
use crate::io::*;
use std::path::Path;

impl<'a> Loaded<'a> {
    ///
    /// Deserialize the loaded image resource at the given path into a [CPUTexture](crate::CPUTexture) using
    /// the [image](https://crates.io/crates/image/main.rs) crate.
    /// The CPUTexture can then be used to create a [Texture2D](crate::Texture2D).
    ///
    /// # Feature
    /// Only available when the `image-io` feature is enabled.
    ///
    pub fn image<P: AsRef<Path>>(&'a self, path: P) -> Result<CPUTexture<u8>, IOError> {
        image_from_bytes(self.bytes(path)?)
    }

    ///
    /// Deserialize the 6 loaded image resources at the given paths into a [CPUTexture](crate::CPUTexture) using
    /// the [image](https://crates.io/crates/image/main.rs) crate.
    /// The CPUTexture can then be used to create a [TextureCubeMap](crate::TextureCubeMap).
    ///
    /// # Feature
    /// Only available when the `image-io` feature is enabled.
    ///
    pub fn cube_image<P: AsRef<Path>>(
        &'a self,
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

#[cfg(not(target_arch = "wasm32"))]
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
        width: usize,
        height: usize,
    ) -> Result<(), IOError> {
        let mut pixels_out = vec![0u8; width * height * 4];
        for row in 0..height {
            for col in 0..width {
                for i in 0..4 {
                    pixels_out[4 * width * (height - row - 1) + 4 * col + i] =
                        pixels[4 * width * row + 4 * col + i];
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
