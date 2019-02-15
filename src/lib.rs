
#[macro_export]
macro_rules! att {
    ($( $name: expr => ($data: expr, $no_components: expr)),*) => {{
         let mut vec = Vec::new();
         $( vec.push(crate::core::buffer::Attribute::new($name, $no_components, $data).unwrap()); )*
         vec
    }}
}

pub mod types;
pub mod core;
pub mod objects;
pub mod effects;

pub mod light;

pub mod camerahandler;
pub mod camera;
pub mod pipelines;

pub use gl;
pub use window;
pub use crate::camera::Camera;
pub use crate::camerahandler::CameraHandler;
pub use crate::texture::Texture;
pub use crate::core::*;
pub use crate::types::*;

pub use crate::pipelines::*;