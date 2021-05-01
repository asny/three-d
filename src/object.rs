//!
//! A collection of objects that can be rendered, for example a mesh.
//!

mod mesh;
#[doc(inline)]
pub use mesh::*;

mod instanced_mesh;
#[doc(inline)]
pub use instanced_mesh::*;

mod skybox;
#[doc(inline)]
pub use skybox::*;

mod imposters;
#[doc(inline)]
pub use imposters::*;

mod particles;
#[doc(inline)]
pub use particles::*;

mod axes;
#[doc(inline)]
pub use axes::*;
