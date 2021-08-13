//!
//! A collection of objects that can be rendered with custom fragment shader, for example a mesh.
//!

mod cpu_mesh;
#[doc(inline)]
pub use cpu_mesh::*;

mod mesh;
#[doc(inline)]
pub use mesh::*;

mod instanced_mesh;
#[doc(inline)]
pub use instanced_mesh::*;

mod particles;
#[doc(inline)]
pub use particles::*;
