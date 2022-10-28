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

mod image_effect;
#[doc(inline)]
pub use image_effect::*;

mod image_cube_effect;
#[doc(inline)]
pub use image_cube_effect::*;

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
pub use three_d_asset::{Camera, Viewport};

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

///
/// Applies a 2D/screen space effect to the given viewport. Can for example be used for adding an effect on top of a rendered image.
/// The fragment shader get the uv coordinates of the viewport (specified by `in vec2 uvs;`),
/// where uv coordinates of `(0, 0)` corresponds to the bottom left corner of the viewport and `(1, 1)` to the top right corner.
///
pub fn apply_effect(
    context: &Context,
    fragment_shader_source: &str,
    render_states: RenderStates,
    viewport: Viewport,
    use_uniforms: impl FnOnce(&Program),
) {
    let position_buffer = full_screen_buffer(context);
    context
        .program(
            "
            in vec3 position;
            out vec2 uvs;
            void main()
            {
                uvs = 0.5 * position.xy + 0.5;
                gl_Position = vec4(position, 1.0);
            }
        ",
            fragment_shader_source,
            |program| {
                use_uniforms(program);
                program.use_vertex_attribute("position", &position_buffer);
                program.draw_arrays(render_states, viewport, 3);
            },
        )
        .expect("Failed compiling shader");
}

///
/// Applies a 2D/screen space effect to the given viewport of the given side of a cube map.
/// The fragment shader get the 3D position (specified by `in vec3 pos;`) of the fragment on the cube with minimum position `(-1, -1, -1)` and maximum position `(1, 1, 1)`.
///
pub fn apply_cube_effect(
    context: &Context,
    side: CubeMapSide,
    fragment_shader_source: &str,
    render_states: RenderStates,
    viewport: Viewport,
    use_uniforms: impl FnOnce(&Program),
) {
    let position_buffer = full_screen_buffer(context);
    context
        .program(
            "
            uniform vec3 direction;
            uniform vec3 up;
            in vec3 position;
            out vec3 pos;
            void main()
            {
                vec3 right = cross(direction, up);
                pos = up * position.y + right * position.x + direction;
                gl_Position = vec4(position, 1.0);
            }
        ",
            fragment_shader_source,
            |program| {
                use_uniforms(program);
                program.use_uniform("direction", side.direction());
                program.use_uniform("up", side.up());
                program.use_vertex_attribute("position", &position_buffer);
                program.draw_arrays(render_states, viewport, 3);
            },
        )
        .expect("Failed compiling shader");
}

fn full_screen_buffer(context: &Context) -> VertexBuffer {
    VertexBuffer::new_with_data(
        context,
        &vec![
            vec3(-3.0, -1.0, 0.0),
            vec3(3.0, -1.0, 0.0),
            vec3(0.0, 2.0, 0.0),
        ],
    )
}

mod data_type;
use data_type::DataType;
fn to_byte_slice<'a, T: DataType>(data: &'a [T]) -> &'a [u8] {
    unsafe {
        std::slice::from_raw_parts(
            data.as_ptr() as *const _,
            data.len() * std::mem::size_of::<T>(),
        )
    }
}

fn from_byte_slice<'a, T: DataType>(data: &'a [u8]) -> &'a [T] {
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
