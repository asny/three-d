//!
//! Structs for constructing a CPU-side version of a GPU feature (for example a [triangle mesh](crate::Mesh))
//! before transferring it to the GPU.
//! Can be constructed manually or loaded via [io](crate::io).
//!

#[doc(hidden)]
pub mod cpu_mesh;
#[doc(inline)]
pub use crate::cpu_mesh::*;

#[doc(hidden)]
pub mod cpu_material;
#[doc(inline)]
pub use crate::cpu_material::*;

#[doc(hidden)]
pub mod cpu_texture;
#[doc(inline)]
pub use crate::cpu_texture::*;

#[doc(hidden)]
pub mod geometry;
#[doc(inline)]
pub use crate::geometry::*;
