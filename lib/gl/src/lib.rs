
// GL

#[cfg(target_arch = "x86_64")]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[cfg(target_arch = "x86_64")]
pub use bindings::*;

#[cfg(target_arch = "x86_64")]
use bindings::Gl as InnerGl;

// WEBGL

#[cfg(target_arch = "wasm32")]
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

//#[cfg(target_arch = "wasm32")]
//pub use WebGlRenderingContext::*; // HOW TO DO THIS!?

#[cfg(target_arch = "wasm32")]
use WebGlRenderingContext as InnerGl;


use std::rc::Rc;
use std::ops::Deref;

#[derive(Clone)]
pub struct Gl {
    inner: Rc<InnerGl>,
}

impl Gl {
    #[cfg(target_arch = "x86_64")]
    pub fn load_with<F>(loadfn: F) -> Gl
        where for<'r> F: FnMut(&'r str) -> *const types::GLvoid
    {
        Gl {
            inner: Rc::new(InnerGl::load_with(loadfn))
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn new(webgl_context: WebGlRenderingContext) -> Gl
    {
        Gl {
            inner: Rc::new(webgl_context)
        }
    }
}

impl Deref for Gl {
    type Target = InnerGl;

    fn deref(&self) -> &InnerGl {
        &self.inner
    }
}
