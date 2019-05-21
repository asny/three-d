pub mod buffer;
pub mod program;
pub mod rendertarget;
pub mod full_screen_quad;
mod shader;
pub mod state;
pub mod texture;


mod hidden {
    pub fn init(gl: &gl::Gl)
    {
        unsafe {
            static mut VAO: Option<u32> = None;
            if VAO.is_none()
            {
                VAO = Some(gl.create_vertex_array().unwrap());
                gl.bind_vertex_array(VAO.as_ref().unwrap());
            }
        }
    }
}