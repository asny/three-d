#[cfg(feature = "3d-io")]
mod threed;
#[doc(inline)]
#[cfg(feature = "3d-io")]
pub use threed::*;

#[cfg(feature = "obj-io")]
mod obj;
#[doc(inline)]
#[cfg(feature = "obj-io")]
pub use obj::*;

#[cfg(feature = "gltf-io")]
mod gltf;
#[doc(inline)]
#[cfg(feature = "gltf-io")]
pub use self::gltf::*;

#[cfg(feature = "image-io")]
#[doc(inline)]
pub use image_io::*;

#[cfg(feature = "image-io")]
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
        /// # Feature
        /// Only available when the `image-io` feature is enabled.
        ///
        pub fn image<P: AsRef<Path>>(&mut self, path: P) -> Result<CPUTexture<u8>> {
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
        ) -> Result<CPUTexture<u8>> {
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
}
