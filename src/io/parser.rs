#[cfg(feature = "obj-io")]
#[cfg_attr(docsrs, doc(cfg(feature = "obj-io")))]
mod obj;
#[doc(inline)]
#[cfg(feature = "obj-io")]
pub use obj::*;

#[cfg(feature = "ply-io")]
#[cfg_attr(docsrs, doc(cfg(feature = "ply-io")))]
mod ply;
#[doc(inline)]
#[cfg(feature = "ply-io")]
pub use self::ply::*;

#[cfg(feature = "gltf-io")]
#[cfg_attr(docsrs, doc(cfg(feature = "gltf-io")))]
mod gltf;
#[doc(inline)]
#[cfg(feature = "gltf-io")]
pub use self::gltf::*;

#[cfg(feature = "image-io")]
#[cfg_attr(docsrs, doc(cfg(feature = "image-io")))]
mod img;
#[cfg(feature = "image-io")]
#[doc(inline)]
pub use img::*;

mod vol;
#[doc(inline)]
pub use vol::*;
