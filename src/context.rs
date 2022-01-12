#![allow(missing_docs)]

//!
//! Low-level graphics abstraction layer which maps one-to-one with the OpenGL graphics API on desktop
//! and WebGL2 bindings provided by the [web-sys](https://rustwasm.github.io/wasm-bindgen/api/web_sys/) crate on web.
//! Use this if you want to have complete control of a feature but be aware that there are no safety checks.
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

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum DataType {
    HalfFloat,
    Float,
    Byte,
    UnsignedByte,
    Short,
    UnsignedShort,
    Int,
    UnsignedInt,
}

impl DataType {
    fn byte_size(&self) -> u32 {
        match self {
            DataType::HalfFloat => 2,
            DataType::Float => std::mem::size_of::<f32>() as u32,
            DataType::UnsignedByte => std::mem::size_of::<u8>() as u32,
            DataType::UnsignedShort => std::mem::size_of::<u16>() as u32,
            DataType::UnsignedInt => std::mem::size_of::<u32>() as u32,
            DataType::Byte => std::mem::size_of::<i8>() as u32,
            DataType::Short => std::mem::size_of::<i16>() as u32,
            DataType::Int => std::mem::size_of::<i32>() as u32,
        }
    }
}
