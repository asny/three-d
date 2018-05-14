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

pub fn depth_test(gl: &gl::Gl, enable: bool)
{
    unsafe {
        static mut CURRENTLY_ENABLED: bool = false;
        if enable != CURRENTLY_ENABLED
        {
            if enable
            {
                gl.Enable(gl::DEPTH_TEST);
            }
            else {
                gl.Disable(gl::DEPTH_TEST);
            }
            CURRENTLY_ENABLED = enable;
        }
    }
}

pub fn depth_write(gl: &gl::Gl, enable: bool)
{
    unsafe {
        static mut CURRENTLY_ENABLED: bool = true;
        if enable != CURRENTLY_ENABLED
        {
            if enable
            {
                gl.DepthMask(gl::TRUE);
            }
            else {
                gl.DepthMask(gl::FALSE);
            }
            CURRENTLY_ENABLED = enable;
        }
    }
}