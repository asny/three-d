use gl;

pub fn cull_back_faces(gl: &gl::Gl, enable: bool)
{
    unsafe {
        static mut CURRENTLY_ENABLED: bool = false;
        if enable != CURRENTLY_ENABLED
        {
            if enable
            {
                gl.Enable(gl::CULL_FACE);
                gl.CullFace(gl::BACK);
            }
            else {
                gl.Disable(gl::CULL_FACE);
            }
            CURRENTLY_ENABLED = enable;
        }
    }
}