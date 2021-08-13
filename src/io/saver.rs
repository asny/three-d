use crate::io::*;
use std::path::Path;

///
/// Functionality for saving resources. Only available on desktop at the moment.
///
pub struct Saver {}

impl Saver {
    ///
    /// Save the byte array as a file.
    ///
    pub fn save_file<P: AsRef<Path>>(path: P, bytes: &[u8]) -> Result<(), IOError> {
        let mut file = std::fs::File::create(path)?;
        use std::io::prelude::*;
        file.write_all(bytes)?;
        Ok(())
    }
}

#[cfg(feature = "image-io")]
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
