mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use bindings::*;

use std::rc::Rc;
use std::ops::Deref;
#[cfg(target_os = "emscripten")]
use bindings::Gles2 as InnerGl;
#[cfg(not(target_os = "emscripten"))]
use bindings::Gl as InnerGl;

#[derive(Clone)]
pub struct Gl {
    inner: Rc<InnerGl>,
}

impl Gl {
    pub fn load_with<F>(loadfn: F) -> Gl
        where for<'r> F: FnMut(&'r str) -> *const types::GLvoid
    {
        Gl {
            inner: Rc::new(InnerGl::load_with(loadfn))
        }
    }
}

impl Deref for Gl {
    type Target = InnerGl;

    fn deref(&self) -> &InnerGl {
        &self.inner
    }
}
