use gl;

#[derive(PartialEq)]
pub enum BlendType {
    NONE,
    SRC_ALPHA__ONE_MINUS_SRC_ALPHA,
    DST_ALPHA__ONE_MINUS_DST_ALPHA,
    ONE__ONE
}

pub fn blend(gl: &gl::Gl, blend_type: BlendType)
{
    unsafe {
        static mut CURRENT: BlendType = BlendType::NONE;
        if blend_type != CURRENT
        {
            match blend_type {
                BlendType::NONE => {
                    gl.disable(gl::consts::BLEND);
                },
                BlendType::SRC_ALPHA__ONE_MINUS_SRC_ALPHA => {
                    gl.enable(gl::consts::BLEND);
                    gl.blend_func(gl::consts::SRC_ALPHA, gl::consts::ONE_MINUS_SRC_ALPHA);
                },
                BlendType::DST_ALPHA__ONE_MINUS_DST_ALPHA => {
                    gl.enable(gl::consts::BLEND);
                    gl.blend_func(gl::consts::DST_ALPHA, gl::consts::ONE_MINUS_DST_ALPHA);
                },
                BlendType::ONE__ONE => {
                    gl.enable(gl::consts::BLEND);
                    gl.blend_func(gl::consts::ONE, gl::consts::ONE);
                }
            }
            CURRENT = blend_type;
        }
    }
}

#[derive(PartialEq)]
pub enum CullType {
    NONE,
    BACK,
    FRONT
}

pub fn cull(gl: &gl::Gl, cull_type: CullType)
{
    unsafe {
        static mut CURRENT: CullType = CullType::NONE;
        if cull_type != CURRENT
        {
            match cull_type {
                CullType::NONE => {
                    gl.disable(gl::consts::CULL_FACE);
                },
                CullType::BACK => {
                    gl.enable(gl::consts::CULL_FACE);
                    gl.cull_face(gl::consts::BACK);
                },
                CullType::FRONT => {
                    gl.enable(gl::consts::CULL_FACE);
                    gl.cull_face(gl::consts::FRONT);
                }
            }
            CURRENT = cull_type;
        }
    }
}

#[derive(PartialEq)]
pub enum DepthTestType {
    NONE,
    LEQUAL,
    LESS
}

pub fn depth_test(gl: &gl::Gl, depth_test_type: DepthTestType)
{
    unsafe {
        static mut CURRENT: DepthTestType = DepthTestType::NONE;
        if depth_test_type != CURRENT
        {
            match depth_test_type {
                DepthTestType::NONE => {
                    gl.disable(gl::consts::DEPTH_TEST);
                },
                DepthTestType::LEQUAL => {
                    gl.enable(gl::consts::DEPTH_TEST);
                    gl.depth_func(gl::consts::LEQUAL);
                },
                DepthTestType::LESS => {
                    gl.enable(gl::consts::DEPTH_TEST);
                    gl.depth_func(gl::consts::LESS);
                }
            }
            CURRENT = depth_test_type;
        }
    }
}

pub fn depth_write(gl: &gl::Gl, enable: bool)
{
    unsafe {
        static mut CURRENTLY_ENABLED: bool = true;
        if enable != CURRENTLY_ENABLED
        {
            gl.depth_mask(enable);
            CURRENTLY_ENABLED = enable;
        }
    }
}