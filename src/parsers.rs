
#[cfg(feature = "3d-io")]
pub mod threed;

#[cfg(feature = "3d-io")]
pub use threed::*;

#[cfg(feature = "obj-io")]
pub mod obj;

#[cfg(feature = "obj-io")]
pub use obj::*;
