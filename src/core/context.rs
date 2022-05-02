use super::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[doc(hidden)]
pub use crate::context::HasContext;

///
/// Contains the low-level OpenGL/WebGL graphics context as well as other "global" variables.
/// Implements Deref with the low-level graphics context as target, so you can call low-level functionality
/// directly on this struct. Use the [context](crate::context) module to get access to low-level constants and structs.
///
#[derive(Clone)]
pub struct Context {
    context: Rc<crate::context::Context>,
    pub(super) vao: crate::context::VertexArray,
    programs: Rc<RefCell<HashMap<String, Program>>>,
    effects: Rc<RefCell<HashMap<String, ImageEffect>>>,
    camera2d: Rc<RefCell<Option<Camera>>>,
    #[cfg(all(feature = "glutin-window", not(target_arch = "wasm32")))]
    pub(crate) glutin_context: Option<Rc<glutin::Context<glutin::PossiblyCurrent>>>,
}

impl Context {
    ///
    /// Creates a new mid-level context, used in this [core](crate::core) module, from a low-level OpenGL/WebGL context from the [context](crate::context) module.
    /// This should only be called directly if you are creating a low-level context yourself (ie. not using the features in the [window](crate::window) module).
    /// Since the content in the [context](crate::context) module is just a re-export of [glow](https://crates.io/crates/glow),
    /// you can also call this method with a reference counter to a glow context created using glow and not the re-export in [context](crate::context).
    ///
    pub fn from_gl_context(context: Rc<crate::context::Context>) -> ThreeDResult<Self> {
        #[cfg(not(target_arch = "wasm32"))]
        unsafe {
            // Enable seamless cube map textures
            context.enable(crate::context::TEXTURE_CUBE_MAP_SEAMLESS);
            context.pixel_store_i32(crate::context::UNPACK_ALIGNMENT, 1);
            context.pixel_store_i32(crate::context::PACK_ALIGNMENT, 1);
        };
        let c = unsafe {
            // Create one Vertex Array Object which is then reused all the time.
            let vao = context
                .create_vertex_array()
                .map_err(|e| CoreError::ContextCreation(e))?;
            Self {
                context,
                vao,
                programs: Rc::new(RefCell::new(HashMap::new())),
                effects: Rc::new(RefCell::new(HashMap::new())),
                camera2d: Rc::new(RefCell::new(None)),
                #[cfg(all(feature = "glutin-window", not(target_arch = "wasm32")))]
                glutin_context: None,
            }
        };
        c.error_check()?;
        Ok(c)
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

    pub(super) fn error_check(&self) -> ThreeDResult<()> {
        #[cfg(debug_assertions)]
        unsafe {
            let e = self.get_error();
            if e != crate::context::NO_ERROR {
                Err(CoreError::ContextError(
                    match e {
                        crate::context::INVALID_ENUM => "Invalid enum",
                        crate::context::INVALID_VALUE => "Invalid value",
                        crate::context::INVALID_OPERATION => "Invalid operation",
                        crate::context::INVALID_FRAMEBUFFER_OPERATION => {
                            "Invalid framebuffer operation"
                        }
                        crate::context::OUT_OF_MEMORY => "Out of memory",
                        crate::context::STACK_OVERFLOW => "Stack overflow",
                        crate::context::STACK_UNDERFLOW => "Stack underflow",
                        _ => "Unknown",
                    }
                    .to_string(),
                ))?;
            }
        }
        Ok(())
    }

    pub(super) fn framebuffer_check(&self) -> ThreeDResult<()> {
        #[cfg(debug_assertions)]
        unsafe {
            match self.check_framebuffer_status(crate::context::FRAMEBUFFER) {
                crate::context::FRAMEBUFFER_COMPLETE => Ok(()),
                crate::context::FRAMEBUFFER_INCOMPLETE_ATTACHMENT => {
                    Err(CoreError::RenderTargetCreation(
                        "FRAMEBUFFER_INCOMPLETE_ATTACHMENT".to_string(),
                    ))
                }
                crate::context::FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER => {
                    Err(CoreError::RenderTargetCreation(
                        "FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER".to_string(),
                    ))
                }
                crate::context::FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT => {
                    Err(CoreError::RenderTargetCreation(
                        "FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT".to_string(),
                    ))
                }
                crate::context::FRAMEBUFFER_UNSUPPORTED => Err(CoreError::RenderTargetCreation(
                    "FRAMEBUFFER_UNSUPPORTED".to_string(),
                )),
                crate::context::FRAMEBUFFER_UNDEFINED => Err(CoreError::RenderTargetCreation(
                    "FRAMEBUFFER_UNDEFINED".to_string(),
                )),
                crate::context::FRAMEBUFFER_INCOMPLETE_READ_BUFFER => {
                    Err(CoreError::RenderTargetCreation(
                        "FRAMEBUFFER_INCOMPLETE_READ_BUFFER".to_string(),
                    ))
                }
                crate::context::FRAMEBUFFER_INCOMPLETE_MULTISAMPLE => {
                    Err(CoreError::RenderTargetCreation(
                        "FRAMEBUFFER_INCOMPLETE_MULTISAMPLE".to_string(),
                    ))
                }
                crate::context::FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS => {
                    Err(CoreError::RenderTargetCreation(
                        "FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS".to_string(),
                    ))
                }
                _ => Err(CoreError::RenderTargetCreation(
                    "Unknown framebuffer error".to_string(),
                )),
            }?;
        }
        Ok(())
    }
}

impl std::ops::Deref for Context {
    type Target = crate::context::Context;
    fn deref(&self) -> &Self::Target {
        &self.context
    }
}
