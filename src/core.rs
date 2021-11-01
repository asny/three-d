#![warn(missing_docs)]
//!
//! Mid-level modular abstractions of common graphics concepts such as buffer, texture, program, render target and so on.
//! Can be combined with low-level calls in the `context` module as long as any graphics state changes are reset.
//!

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
    dummy_tex: Rc<RefCell<Option<Texture2D>>>,
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
            dummy_tex: Rc::new(RefCell::new(None)),
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

    pub(crate) fn use_texture_dummy(&self, program: &Program, name: &str) -> ThreeDResult<()> {
        if self.dummy_tex.borrow().is_none() {
            *self.dummy_tex.borrow_mut() =
                Some(Texture2D::new(&self, &CPUTexture::<u8>::default())?);
        }
        program.use_texture(name, (*self.dummy_tex.borrow()).as_ref().unwrap())
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

mod instanced_mesh;
#[doc(inline)]
pub use instanced_mesh::*;

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
    #[error("{0} buffer length must be divisible by 3, actual count is {1}")]
    InvalidBufferLength(String, usize),
    #[error("index buffer contains values larger than the length of the buffer which is {0}")]
    InvalidIndexBuffer(usize),
    #[error(
        "when indices unspecified, positions length must be divisible by 9, actual count is {0}"
    )]
    InvalidPositionBuffer(usize),
    #[error("data for element at index {0} has length {1} but a length of {2} was expected")]
    InvalidUniformBufferElementLength(u32, usize, usize),
    #[error("the index {0} is outside the expected range [0, {1}]")]
    IndexOutOfRange(usize, usize),
    #[error("cannot take as input a negative minimum distance")]
    NegativeDistance,
    #[error("a minimum must be smaller than a maximum")]
    MinimumLargerThanMaximum,
}
