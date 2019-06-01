pub mod buffer;
pub mod program;
pub mod rendertarget;
mod shader;
pub mod state;
pub mod texture;

pub type Gl = std::rc::Rc<gl::Gl>;