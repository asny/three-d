
pub mod gl;

pub mod core;
pub use crate::core::*;

pub mod io;
pub use crate::io::*;

pub mod cpu_mesh;
pub use crate::cpu_mesh::*;

pub mod cpu_material;
pub use crate::cpu_material::*;

pub mod phong;
pub use crate::phong::*;

#[cfg(not(feature = "no-renderer"))]
pub mod renderer;
#[cfg(not(feature = "no-renderer"))]
pub mod light;
#[cfg(not(feature = "no-renderer"))]
pub mod objects;
#[cfg(not(feature = "no-renderer"))]
pub mod effects;

#[cfg(not(feature = "no-renderer"))]
pub use crate::renderer::*;
#[cfg(not(feature = "no-renderer"))]
pub use crate::light::*;
#[cfg(not(feature = "no-renderer"))]
pub use crate::objects::*;
#[cfg(not(feature = "no-renderer"))]
pub use crate::effects::*;

#[cfg(any(feature = "glutin-window", feature = "canvas"))]
pub mod window;
#[cfg(any(feature = "glutin-window", feature = "canvas"))]
pub use window::*;
