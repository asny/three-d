//!
//! Structs for constructing a CPU-side version of a GPU feature (for example a [triangle mesh](crate::Mesh))
//! before transferring it to the GPU.
//! Can be constructed manually or loaded via [io](crate::io).
//!

mod cpu_mesh;
#[doc(inline)]
pub use cpu_mesh::*;

mod cpu_material;
#[doc(inline)]
pub use cpu_material::*;

mod cpu_texture;
#[doc(inline)]
pub use cpu_texture::*;

mod geometry;
#[doc(inline)]
pub use geometry::*;
