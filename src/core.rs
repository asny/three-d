//!
//! Mid-level modular abstractions of common graphics concepts such as buffer, texture, program, render target and so on.
//! Can be combined with low-level calls in the [context](crate::context) module as well as high-level functionality in the [renderer](crate::renderer) module.
//!
#![allow(unsafe_code)]

mod context;
#[doc(inline)]
pub use context::*;

pub mod buffer;
pub use buffer::*;

pub mod texture;
pub use texture::*;

pub mod render_states;
pub use render_states::*;

pub mod render_target;
pub use render_target::*;

mod uniform;
#[doc(inline)]
pub use uniform::*;

mod program;
#[doc(inline)]
pub use program::*;

mod scissor_box;
#[doc(inline)]
pub use scissor_box::*;

pub mod prelude {

    //!
    //! Basic types used throughout this crate, mostly basic math.
    //!
    pub use three_d_asset::prelude::*;
}
pub use prelude::*;
pub use three_d_asset::Viewport;

/// A result for this crate.
use thiserror::Error;

///
/// Error in the [core](crate::core) module.
///
#[derive(Debug, Error)]
#[allow(missing_docs)]
pub enum CoreError {
    #[error("failed creating context with error: {0}")]
    ContextCreation(String),
    #[error("failed rendering with error: {0}")]
    ContextError(String),
    #[error("failed compiling {0} shader: {1}\n{2}")]
    ShaderCompilation(String, String, String),
    #[error("failed to link shader program: {0}")]
    ShaderLink(String),
}

pub(crate) fn full_screen_draw(
    context: &Context,
    program: &Program,
    render_states: RenderStates,
    viewport: Viewport,
) {
    let position_buffer = VertexBuffer::new_with_data(
        context,
        &[
            vec3(-3.0, -1.0, 0.0),
            vec3(3.0, -1.0, 0.0),
            vec3(0.0, 2.0, 0.0),
        ],
    );
    program.use_vertex_attribute("position", &position_buffer);
    program.draw_arrays(render_states, viewport, 3);
}

pub(crate) fn full_screen_vertex_shader_source() -> &'static str {
    "
        in vec3 position;
        out vec2 uvs;
        out vec4 col;
        void main()
        {
            uvs = 0.5 * position.xy + 0.5;
            col = vec4(1.0);
            gl_Position = vec4(position, 1.0);
        }
    "
}

mod data_type;
use data_type::DataType;
fn to_byte_slice<T: DataType>(data: &[T]) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(
            data.as_ptr() as *const _,
            data.len() * std::mem::size_of::<T>(),
        )
    }
}

fn from_byte_slice<T: DataType>(data: &[u8]) -> &[T] {
    unsafe {
        let (_prefix, values, _suffix) = data.align_to::<T>();
        values
    }
}

fn format_from_data_type<T: DataType>() -> u32 {
    match T::size() {
        1 => crate::context::RED,
        2 => crate::context::RG,
        3 => crate::context::RGB,
        4 => crate::context::RGBA,
        _ => unreachable!(),
    }
}

fn flip_y<T: TextureDataType>(pixels: &mut [T], width: usize, height: usize) {
    for row in 0..height / 2 {
        for col in 0..width {
            let index0 = width * row + col;
            let index1 = width * (height - row - 1) + col;
            pixels.swap(index0, index1);
        }
    }
}
