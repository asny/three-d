#[cfg(feature = "3d-io")]
#[cfg_attr(docsrs, doc(cfg(feature = "3d-io")))]
mod threed;
#[doc(inline)]
#[cfg(feature = "3d-io")]
pub use threed::*;

#[cfg(feature = "obj-io")]
#[cfg_attr(docsrs, doc(cfg(feature = "obj-io")))]
mod obj;
#[doc(inline)]
#[cfg(feature = "obj-io")]
pub use obj::*;

#[cfg(feature = "gltf-io")]
#[cfg_attr(docsrs, doc(cfg(feature = "gltf-io")))]
mod gltf;
#[doc(inline)]
#[cfg(feature = "gltf-io")]
pub use self::gltf::*;

#[cfg(feature = "image-io")]
#[doc(inline)]
pub use image_io::*;

#[cfg(feature = "image-io")]
#[cfg_attr(docsrs, doc(cfg(feature = "image-io")))]
mod image_io {
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
            let img = decoder.read_image_hdr()?;
            Ok(CPUTexture {
                data: img
                    .iter()
                    .map(|Rgb(values)| values)
                    .flatten()
                    .map(|v| *v)
                    .collect::<Vec<_>>(),
                width: metadata.width,
                height: metadata.height,
                format: Format::RGB,
                ..Default::default()
            })
        }

        ///
        /// Deserialize the 6 loaded image resources at the given paths into a [CPUTexture] using
        /// the [image](https://crates.io/crates/image/main.rs) crate.
        /// The CPUTexture can then be used to create a [TextureCubeMap].
        ///
        pub fn cube_image<P: AsRef<Path>>(
            &mut self,
            right_path: P,
            left_path: P,
            top_path: P,
            bottom_path: P,
            front_path: P,
            back_path: P,
        ) -> ThreeDResult<CPUTexture<u8>> {
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
                        pixels_out
                            [4 * width as usize * (height as usize - row - 1) + 4 * col + i] =
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
}
