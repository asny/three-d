
pub mod core;
pub use crate::core::*;

#[cfg(not(feature = "core-only"))]
pub mod renderer;
#[cfg(not(feature = "core-only"))]
pub mod light;
#[cfg(not(feature = "core-only"))]
pub mod objects;
#[cfg(not(feature = "core-only"))]
pub mod effects;

#[cfg(not(feature = "core-only"))]
pub use crate::renderer::*;
#[cfg(not(feature = "core-only"))]
pub use crate::light::*;
#[cfg(not(feature = "core-only"))]
pub use crate::objects::*;
#[cfg(not(feature = "core-only"))]
pub use crate::effects::*;

#[cfg(feature = "desktop")]
pub mod window;
#[cfg(feature = "desktop")]
pub use window::*;

#[cfg(feature = "web")]
pub mod window;
#[cfg(feature = "web")]
pub use window::*;