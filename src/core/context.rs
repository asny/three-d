use super::*;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

#[doc(hidden)]
pub use crate::context::HasContext;

///
/// Contains the low-level OpenGL/WebGL graphics context as well as other "global" variables.
/// Implements Deref with the low-level graphics context as target, so you can call low-level functionality
/// directly on this struct. Use the [context](crate::context) module to get access to low-level constants and structs.
///
#[derive(Clone)]
pub struct Context {
    context: Arc<crate::context::Context>,
    pub(super) vao: crate::context::VertexArray,
    programs: Arc<RwLock<HashMap<String, Program>>>,
}

impl Context {
    ///
    /// Creates a new mid-level context, used in this [core](crate::core) module, from a low-level OpenGL/WebGL context from the [context](crate::context) module.
    /// This should only be called directly if you are creating a low-level context yourself (ie. not using the features in the [window](crate::window) module).
    /// Since the content in the [context](crate::context) module is just a re-export of [glow](https://crates.io/crates/glow),
    /// you can also call this method with a reference counter to a glow context created using glow and not the re-export in [context](crate::context).
    ///
    pub fn from_gl_context(context: Arc<crate::context::Context>) -> Result<Self, CoreError> {
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
                programs: Arc::new(RwLock::new(HashMap::new())),
            }
        };
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
        callback: impl FnOnce(&Program),
    ) -> Result<(), CoreError> {
        let key = format!("{}{}", vertex_shader_source, fragment_shader_source);
        if !self.programs.read().unwrap().contains_key(&key) {
            self.programs.write().unwrap().insert(
                key.clone(),
                Program::from_source(self, vertex_shader_source, fragment_shader_source)?,
            );
        };
        callback(self.programs.read().unwrap().get(&key).unwrap());
        Ok(())
    }

    ///
    /// Set the scissor test for this context (see [ScissorBox]).
    ///
    pub fn set_scissor(&self, scissor_box: ScissorBox) {
        unsafe {
            if scissor_box.width > 0 && scissor_box.height > 0 {
                self.enable(crate::context::SCISSOR_TEST);
                self.scissor(
                    scissor_box.x as i32,
                    scissor_box.y as i32,
                    scissor_box.width as i32,
                    scissor_box.height as i32,
                );
            } else {
                self.disable(crate::context::SCISSOR_TEST);
            }
        }
    }

    ///
    /// Set the viewport for this context (See [Viewport]).
    ///
    pub fn set_viewport(&self, viewport: Viewport) {
        unsafe {
            self.viewport(
                viewport.x,
                viewport.y,
                viewport.width as i32,
                viewport.height as i32,
            );
        }
    }

    ///
    /// Set the face culling for this context (see [Cull]).
    ///
    pub fn set_cull(&self, cull: Cull) {
        unsafe {
            match cull {
                Cull::None => {
                    self.disable(crate::context::CULL_FACE);
                }
                Cull::Back => {
                    self.enable(crate::context::CULL_FACE);
                    self.cull_face(crate::context::BACK);
                }
                Cull::Front => {
                    self.enable(crate::context::CULL_FACE);
                    self.cull_face(crate::context::FRONT);
                }
                Cull::FrontAndBack => {
                    self.enable(crate::context::CULL_FACE);
                    self.cull_face(crate::context::FRONT_AND_BACK);
                }
            }
        }
    }

    ///
    /// Set the write mask for this context (see [WriteMask]).
    ///
    pub fn set_write_mask(&self, write_mask: WriteMask) {
        unsafe {
            self.color_mask(
                write_mask.red,
                write_mask.green,
                write_mask.blue,
                write_mask.alpha,
            );
            self.depth_mask(write_mask.depth);
        }
    }

    ///
    /// Set the depth test for this context (see [DepthTest]).
    ///
    pub fn set_depth_test(&self, depth_test: DepthTest) {
        unsafe {
            self.enable(crate::context::DEPTH_TEST);
            match depth_test {
                DepthTest::Never => {
                    self.depth_func(crate::context::NEVER);
                }
                DepthTest::Less => {
                    self.depth_func(crate::context::LESS);
                }
                DepthTest::Equal => {
                    self.depth_func(crate::context::EQUAL);
                }
                DepthTest::LessOrEqual => {
                    self.depth_func(crate::context::LEQUAL);
                }
                DepthTest::Greater => {
                    self.depth_func(crate::context::GREATER);
                }
                DepthTest::NotEqual => {
                    self.depth_func(crate::context::NOTEQUAL);
                }
                DepthTest::GreaterOrEqual => {
                    self.depth_func(crate::context::GEQUAL);
                }
                DepthTest::Always => {
                    self.depth_func(crate::context::ALWAYS);
                }
            }
        }
    }

    ///
    /// Set the blend state for this context (see [Blend]).
    ///
    pub fn set_blend(&self, blend: Blend) {
        unsafe {
            if let Blend::Enabled {
                source_rgb_multiplier,
                source_alpha_multiplier,
                destination_rgb_multiplier,
                destination_alpha_multiplier,
                rgb_equation,
                alpha_equation,
            } = blend
            {
                self.enable(crate::context::BLEND);
                self.blend_func_separate(
                    Self::blend_const_from_multiplier(source_rgb_multiplier),
                    Self::blend_const_from_multiplier(destination_rgb_multiplier),
                    Self::blend_const_from_multiplier(source_alpha_multiplier),
                    Self::blend_const_from_multiplier(destination_alpha_multiplier),
                );
                self.blend_equation_separate(
                    Self::blend_const_from_equation(rgb_equation),
                    Self::blend_const_from_equation(alpha_equation),
                );
            } else {
                self.disable(crate::context::BLEND);
            }
        }
    }

    fn blend_const_from_multiplier(multiplier: BlendMultiplierType) -> u32 {
        match multiplier {
            BlendMultiplierType::Zero => crate::context::ZERO,
            BlendMultiplierType::One => crate::context::ONE,
            BlendMultiplierType::SrcColor => crate::context::SRC_COLOR,
            BlendMultiplierType::OneMinusSrcColor => crate::context::ONE_MINUS_SRC_COLOR,
            BlendMultiplierType::DstColor => crate::context::DST_COLOR,
            BlendMultiplierType::OneMinusDstColor => crate::context::ONE_MINUS_DST_COLOR,
            BlendMultiplierType::SrcAlpha => crate::context::SRC_ALPHA,
            BlendMultiplierType::OneMinusSrcAlpha => crate::context::ONE_MINUS_SRC_ALPHA,
            BlendMultiplierType::DstAlpha => crate::context::DST_ALPHA,
            BlendMultiplierType::OneMinusDstAlpha => crate::context::ONE_MINUS_DST_ALPHA,
            BlendMultiplierType::SrcAlphaSaturate => crate::context::SRC_ALPHA_SATURATE,
        }
    }
    fn blend_const_from_equation(equation: BlendEquationType) -> u32 {
        match equation {
            BlendEquationType::Add => crate::context::FUNC_ADD,
            BlendEquationType::Subtract => crate::context::FUNC_SUBTRACT,
            BlendEquationType::ReverseSubtract => crate::context::FUNC_REVERSE_SUBTRACT,
            BlendEquationType::Min => crate::context::MIN,
            BlendEquationType::Max => crate::context::MAX,
        }
    }

    ///
    /// Set the render states for this context (see [RenderStates]).
    ///
    pub fn set_render_states(&self, render_states: RenderStates) {
        self.set_cull(render_states.cull);
        self.set_write_mask(render_states.write_mask);
        if !render_states.write_mask.depth && render_states.depth_test == DepthTest::Always {
            unsafe { self.disable(crate::context::DEPTH_TEST) }
        } else {
            self.set_depth_test(render_states.depth_test);
        }
        self.set_blend(render_states.blend);
    }

    ///
    /// Returns an error if an GPU-side error has happened while rendering which can be used to check for errors while developing.
    /// Can also be used in production to handle unexpected rendering errors, but do not call it too often to avoid performance problems.
    ///
    pub fn error_check(&self) -> Result<(), CoreError> {
        self.framebuffer_check()?;
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

    fn framebuffer_check(&self) -> Result<(), CoreError> {
        unsafe {
            match self.check_framebuffer_status(crate::context::FRAMEBUFFER) {
                crate::context::FRAMEBUFFER_COMPLETE => Ok(()),
                crate::context::FRAMEBUFFER_INCOMPLETE_ATTACHMENT => Err(CoreError::ContextError(
                    "FRAMEBUFFER_INCOMPLETE_ATTACHMENT".to_string(),
                )),
                crate::context::FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER => Err(CoreError::ContextError(
                    "FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER".to_string(),
                )),
                crate::context::FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT => {
                    Err(CoreError::ContextError(
                        "FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT".to_string(),
                    ))
                }
                crate::context::FRAMEBUFFER_UNSUPPORTED => Err(CoreError::ContextError(
                    "FRAMEBUFFER_UNSUPPORTED".to_string(),
                )),
                crate::context::FRAMEBUFFER_UNDEFINED => {
                    Err(CoreError::ContextError("FRAMEBUFFER_UNDEFINED".to_string()))
                }
                crate::context::FRAMEBUFFER_INCOMPLETE_READ_BUFFER => Err(CoreError::ContextError(
                    "FRAMEBUFFER_INCOMPLETE_READ_BUFFER".to_string(),
                )),
                crate::context::FRAMEBUFFER_INCOMPLETE_MULTISAMPLE => Err(CoreError::ContextError(
                    "FRAMEBUFFER_INCOMPLETE_MULTISAMPLE".to_string(),
                )),
                crate::context::FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS => Err(
                    CoreError::ContextError("FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS".to_string()),
                ),
                _ => Err(CoreError::ContextError(
                    "Unknown framebuffer error".to_string(),
                )),
            }?;
        }
        Ok(())
    }
}

impl std::fmt::Debug for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("Context");
        d.field("programs", &self.programs.read().unwrap().len());
        d.finish()
    }
}

impl std::ops::Deref for Context {
    type Target = Arc<crate::context::Context>;
    fn deref(&self) -> &Self::Target {
        &self.context
    }
}
