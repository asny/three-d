#![allow(missing_docs)]

//!
//! Thin and low-level graphics abstraction layer which maps one-to-one with the OpenGL graphics API on desktop
//! and WebGL2 bindings provided by the [web-sys](https://rustwasm.github.io/wasm-bindgen/api/web_sys/) crate on web.
//! Can be used in combination with more high-level features or be ignored entirely.
//!

// GL
#[cfg(not(target_arch = "wasm32"))]
mod ogl;

#[doc(inline)]
#[cfg(not(target_arch = "wasm32"))]
pub use ogl::*;

// WEBGL
#[cfg(target_arch = "wasm32")]
mod wgl2;

#[doc(inline)]
#[cfg(target_arch = "wasm32")]
pub use wgl2::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ShaderType {
    Vertex,
    Fragment,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum DataType {
    FLOAT,
    BYTE,
    UNSIGNED_BYTE,
    SHORT,
    UNSIGNED_SHORT,
    INT,
    UNSIGNED_INT,
}

impl DataType {
    fn byte_size(&self) -> u32 {
        match self {
            DataType::FLOAT => std::mem::size_of::<f32>() as u32,
            DataType::UNSIGNED_BYTE => std::mem::size_of::<u8>() as u32,
            DataType::UNSIGNED_SHORT => std::mem::size_of::<u16>() as u32,
            DataType::UNSIGNED_INT => std::mem::size_of::<u32>() as u32,
            DataType::BYTE => std::mem::size_of::<i8>() as u32,
            DataType::SHORT => std::mem::size_of::<i16>() as u32,
            DataType::INT => std::mem::size_of::<i32>() as u32,
        }
    }
}
