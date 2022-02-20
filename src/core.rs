#![warn(missing_docs)]
//!
//! Mid-level modular abstractions of common graphics concepts such as buffer, texture, program, render target and so on.
//! Can be combined with low-level calls in the `context` module as long as any graphics state changes are reset.
//!

///
/// Possible types that can be send as a uniform to a shader (a variable that is uniformly available when processing all vertices and fragments).
///
pub trait UniformDataType: std::fmt::Debug + internal_uniform::UniformDataTypeExtension {}

impl UniformDataType for i32 {}

impl UniformDataType for f32 {}
impl UniformDataType for Vec2 {}
impl UniformDataType for Vec3 {}
impl UniformDataType for Vec4 {}

impl UniformDataType for [f32; 2] {}
impl UniformDataType for [f32; 3] {}
impl UniformDataType for [f32; 4] {}

impl UniformDataType for Quat {}

impl UniformDataType for Mat2 {}
impl UniformDataType for Mat3 {}
impl UniformDataType for Mat4 {}

impl<T: UniformDataType + ?Sized> UniformDataType for &T {}

pub(in crate::core) mod internal_uniform {
    use crate::context::UniformLocation;
    use crate::core::math::*;
    use crate::core::Context;

    pub trait UniformDataTypeExtension: Copy {
        fn send(&self, context: &Context, location: &UniformLocation);
        fn send_array(data: &[Self], context: &Context, location: &UniformLocation);
    }
    impl<T: UniformDataTypeExtension + ?Sized> UniformDataTypeExtension for &T {
        fn send(&self, context: &Context, location: &UniformLocation) {
            (*self).send(context, location)
        }
        fn send_array(data: &[Self], context: &Context, location: &UniformLocation) {
            Self::send_array(
                &data.iter().map(|v| *v).collect::<Vec<_>>(),
                context,
                location,
            )
        }
    }

    impl UniformDataTypeExtension for i32 {
        fn send(&self, context: &Context, location: &UniformLocation) {
            context.uniform1i(location, *self);
        }
        fn send_array(data: &[Self], context: &Context, location: &UniformLocation) {
            context.uniform1iv(location, &data);
        }
    }

    impl UniformDataTypeExtension for f32 {
        fn send(&self, context: &Context, location: &UniformLocation) {
            context.uniform1f(location, *self);
        }
        fn send_array(data: &[Self], context: &Context, location: &UniformLocation) {
            context.uniform1fv(location, &data);
        }
    }

    impl UniformDataTypeExtension for Vec2 {
        fn send(&self, context: &Context, location: &UniformLocation) {
            context.uniform2fv(location, &self.as_array());
        }
        fn send_array(data: &[Self], context: &Context, location: &UniformLocation) {
            context.uniform2fv(
                location,
                &data.iter().flat_map(|v| v.as_array()).collect::<Vec<_>>(),
            );
        }
    }

    impl UniformDataTypeExtension for Vec3 {
        fn send(&self, context: &Context, location: &UniformLocation) {
            context.uniform3fv(location, &self.as_array());
        }
        fn send_array(data: &[Self], context: &Context, location: &UniformLocation) {
            context.uniform3fv(
                location,
                &data.iter().flat_map(|v| v.as_array()).collect::<Vec<_>>(),
            );
        }
    }

    impl UniformDataTypeExtension for Vec4 {
        fn send(&self, context: &Context, location: &UniformLocation) {
            context.uniform4fv(location, &self.as_array());
        }
        fn send_array(data: &[Self], context: &Context, location: &UniformLocation) {
            context.uniform4fv(
                location,
                &data.iter().flat_map(|v| v.as_array()).collect::<Vec<_>>(),
            );
        }
    }

    impl UniformDataTypeExtension for [f32; 2] {
        fn send(&self, context: &Context, location: &UniformLocation) {
            context.uniform4fv(location, self);
        }
        fn send_array(data: &[Self], context: &Context, location: &UniformLocation) {
            context.uniform4fv(location, &data.iter().flat_map(|v| *v).collect::<Vec<_>>());
        }
    }

    impl UniformDataTypeExtension for [f32; 3] {
        fn send(&self, context: &Context, location: &UniformLocation) {
            context.uniform4fv(location, self);
        }
        fn send_array(data: &[Self], context: &Context, location: &UniformLocation) {
            context.uniform4fv(location, &data.iter().flat_map(|v| *v).collect::<Vec<_>>());
        }
    }

    impl UniformDataTypeExtension for [f32; 4] {
        fn send(&self, context: &Context, location: &UniformLocation) {
            context.uniform4fv(location, self);
        }
        fn send_array(data: &[Self], context: &Context, location: &UniformLocation) {
            context.uniform4fv(location, &data.iter().flat_map(|v| *v).collect::<Vec<_>>());
        }
    }

    impl UniformDataTypeExtension for Quat {
        fn send(&self, context: &Context, location: &UniformLocation) {
            context.uniform4fv(location, &self.as_array());
        }
        fn send_array(data: &[Self], context: &Context, location: &UniformLocation) {
            context.uniform4fv(
                location,
                &data.iter().flat_map(|v| v.as_array()).collect::<Vec<_>>(),
            );
        }
    }

    impl UniformDataTypeExtension for Mat2 {
        fn send(&self, context: &Context, location: &UniformLocation) {
            context.uniform_matrix2fv(location, &self.as_array());
        }
        fn send_array(data: &[Self], context: &Context, location: &UniformLocation) {
            context.uniform_matrix2fv(
                location,
                &data.iter().flat_map(|v| v.as_array()).collect::<Vec<_>>(),
            );
        }
    }

    impl UniformDataTypeExtension for Mat3 {
        fn send(&self, context: &Context, location: &UniformLocation) {
            context.uniform_matrix3fv(location, &self.as_array());
        }
        fn send_array(data: &[Self], context: &Context, location: &UniformLocation) {
            context.uniform_matrix3fv(
                location,
                &data.iter().flat_map(|v| v.as_array()).collect::<Vec<_>>(),
            );
        }
    }

    impl UniformDataTypeExtension for Mat4 {
        fn send(&self, context: &Context, location: &UniformLocation) {
            context.uniform_matrix4fv(location, &self.as_array());
        }
        fn send_array(data: &[Self], context: &Context, location: &UniformLocation) {
            context.uniform_matrix4fv(
                location,
                &data.iter().flat_map(|v| v.as_array()).collect::<Vec<_>>(),
            );
        }
    }
}

use crate::context::GLContext;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

///
/// Contains information about the graphics context to use for rendering and other "global" variables.
///
#[derive(Clone)]
pub struct Context {
    context: GLContext,
    programs: Rc<RefCell<HashMap<String, Program>>>,
    effects: Rc<RefCell<HashMap<String, ImageEffect>>>,
    camera2d: Rc<RefCell<Option<Camera>>>,
}

impl Context {
    ///
    /// Creates a new context from a [OpenGL/WebGL context](GLContext).
    ///
    pub fn from_gl_context(context: GLContext) -> Self {
        Self {
            context,
            programs: Rc::new(RefCell::new(HashMap::new())),
            effects: Rc::new(RefCell::new(HashMap::new())),
            camera2d: Rc::new(RefCell::new(None)),
        }
    }

    ///
    /// Compiles a [Program] with the given vertex and fragment shader source and stores it for later use.
    /// If it has already been created, then it is just returned.
    ///
    pub fn program(
        &self,
        vertex_shader_source: &str,
        fragment_shader_source: &str,
        callback: impl FnOnce(&Program) -> ThreeDResult<()>,
    ) -> ThreeDResult<()> {
        let key = format!("{}{}", vertex_shader_source, fragment_shader_source);
        if !self.programs.borrow().contains_key(&key) {
            self.programs.borrow_mut().insert(
                key.clone(),
                Program::from_source(self, vertex_shader_source, fragment_shader_source)?,
            );
        };
        callback(self.programs.borrow().get(&key).unwrap())
    }

    ///
    /// Compiles an [ImageEffect] with the given fragment shader source and stores it for later use.
    /// If it has already been created, then it is just returned.
    ///
    pub fn effect(
        &self,
        fragment_shader_source: &str,
        callback: impl FnOnce(&ImageEffect) -> ThreeDResult<()>,
    ) -> ThreeDResult<()> {
        if !self.effects.borrow().contains_key(fragment_shader_source) {
            self.effects.borrow_mut().insert(
                fragment_shader_source.to_string(),
                ImageEffect::new(self, fragment_shader_source)?,
            );
        };
        callback(self.effects.borrow().get(fragment_shader_source).unwrap())
    }

    ///
    /// Returns a camera for viewing 2D content.
    ///
    pub fn camera2d(
        &self,
        viewport: Viewport,
        callback: impl FnOnce(&Camera) -> ThreeDResult<()>,
    ) -> ThreeDResult<()> {
        if self.camera2d.borrow().is_none() {
            *self.camera2d.borrow_mut() = Some(Camera::new_orthographic(
                self,
                viewport,
                vec3(0.0, 0.0, -1.0),
                vec3(0.0, 0.0, 0.0),
                vec3(0.0, -1.0, 0.0),
                1.0,
                0.0,
                10.0,
            )?)
        }
        let mut camera2d = self.camera2d.borrow_mut();
        camera2d.as_mut().unwrap().set_viewport(viewport)?;
        camera2d.as_mut().unwrap().set_orthographic_projection(
            viewport.height as f32,
            0.0,
            10.0,
        )?;
        camera2d.as_mut().unwrap().set_view(
            vec3(
                viewport.width as f32 * 0.5,
                viewport.height as f32 * 0.5,
                -1.0,
            ),
            vec3(
                viewport.width as f32 * 0.5,
                viewport.height as f32 * 0.5,
                0.0,
            ),
            vec3(0.0, -1.0, 0.0),
        )?;
        callback(camera2d.as_ref().unwrap())
    }
}

impl std::ops::Deref for Context {
    type Target = crate::context::GLContext;
    fn deref(&self) -> &Self::Target {
        &self.context
    }
}

pub mod buffer;
pub use buffer::*;

pub mod math;
pub use math::*;

pub mod texture;
pub use texture::*;

mod cpu_mesh;
#[doc(inline)]
pub use cpu_mesh::*;

mod mesh;
#[doc(inline)]
pub use mesh::*;

pub mod render_states;
pub use render_states::*;

pub mod render_target;
pub use render_target::*;

mod cpu_material;
#[doc(inline)]
pub use cpu_material::*;

mod camera;
#[doc(inline)]
pub use camera::*;

mod image_effect;
#[doc(inline)]
pub use image_effect::*;

mod image_cube_effect;
#[doc(inline)]
pub use image_cube_effect::*;

mod program;
#[doc(inline)]
pub use program::*;

mod aabb;
#[doc(inline)]
pub use aabb::*;

mod color;
#[doc(inline)]
pub use color::*;

mod viewport;
#[doc(inline)]
pub use viewport::*;

pub use crate::ThreeDResult;
use thiserror::Error;
///
/// Error in the [core](crate::core) module.
///
#[derive(Debug, Error)]
#[allow(missing_docs)]
pub enum CoreError {
    #[error("failed creating a new shader")]
    ShaderCreation,
    #[error("failed compiling {0} shader: {1}")]
    ShaderCompilation(String, String),
    #[error("failed to link shader program: {0}")]
    ShaderLink(String),
    #[error("the uniform {0} is sent to the shader but never used")]
    UnusedUniform(String),
    #[error("the attribute {0} is sent to the shader but never used")]
    UnusedAttribute(String),
    #[error("failed creating a new render target")]
    RenderTargetCreation,
    #[error("cannot copy {0} from a {1} texture")]
    RenderTargetCopy(String, String),
    #[error("cannot read color from anything else but an RGBA texture")]
    ReadWrongFormat,
    #[error("failed creating a new texture")]
    TextureCreation,
    #[error("invalid size of texture data (got {0} pixels but expected {1} pixels)")]
    InvalidTextureLength(usize, usize),
    #[error("the render call requires the {0} vertex buffer which is missing on the given mesh")]
    MissingMeshBuffer(String),
    #[error(
        "if the fragment shader defined 'in vec3 tang' it also needs to define 'in vec3 bitang'"
    )]
    MissingBitangent,
    #[error("{0} buffer length must be {1}, actual length is {2}")]
    InvalidBufferLength(String, usize, usize),
    #[error("mesh must have both normals and uv coordinates to be able to compute tangents")]
    FailedComputingTangents,
    #[error("the number of vertices must be divisable by 3, actual count is {0}")]
    InvalidNumberOfVertices(usize),
    #[error("data for element at index {0} has length {1} but a length of {2} was expected")]
    InvalidUniformBufferElementLength(u32, usize, usize),
    #[error("the index {0} is outside the expected range [0, {1}]")]
    IndexOutOfRange(usize, usize),
    #[error("cannot take as input a negative minimum distance")]
    NegativeDistance,
    #[error("a minimum must be smaller than a maximum")]
    MinimumLargerThanMaximum,
}
