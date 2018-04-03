mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use bindings::*;
pub use bindings::Gles2 as Gl;

/*
use std::rc::Rc;
use std::ops::Deref;

pub use bindings::*;
pub use bindings::Gles2 as InnerGl;

#[derive(Clone)]
pub struct Gl {
    inner: Rc<bindings::Gles2>,
}

impl Gl {
    pub fn load_with<F>(loadfn: F) -> Gl
        where F: FnMut(&'static str) -> *const types::GLvoid
    {
        Gl {
            inner: Rc::new(bindings::Gles2::load_with(loadfn))
        }
    }
}

impl Deref for Gl {
    type Target = bindings::Gles2;

    fn deref(&self) -> &bindings::Gles2 {
        &self.inner
    }
}
*/
