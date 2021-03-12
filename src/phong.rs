
//!
//! Lighting functionality based on the phong reflection model.
//!

#[doc(hidden)]
pub mod material;
#[doc(inline)]
pub use crate::material::*;

#[doc(hidden)]
pub mod forward_pipeline;
#[doc(inline)]
pub use crate::forward_pipeline::*;

#[doc(hidden)]
pub mod deferred_pipeline;
#[doc(inline)]
pub use crate::deferred_pipeline::*;

#[doc(hidden)]
pub mod phong_mesh;
#[doc(inline)]
pub use crate::phong_mesh::*;

#[doc(hidden)]
pub mod forward_instanced_mesh;
#[doc(inline)]
pub use crate::forward_instanced_mesh::*;

#[doc(hidden)]
pub mod deferred_instanced_mesh;
#[doc(inline)]
pub use crate::deferred_instanced_mesh::*;