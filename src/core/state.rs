use gl;

#[derive(PartialEq)]
pub enum PolygonType {
    Point,
    Line,
    Fill
}

pub fn polygon_mode(gl: &gl::Gl, polygon_type: PolygonType)
{
    unsafe {
        static mut CURRENT: PolygonType = PolygonType::Fill;
        if polygon_type != CURRENT
        {
            match polygon_type {
                PolygonType::Point => {
                    gl.PolygonMode(gl::FRONT_AND_BACK, gl::POINT);
                },
                PolygonType::Line => {
                    gl.PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
                },
                PolygonType::Fill => {
                    gl.PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
                }
            }
            CURRENT = polygon_type;
        }
    }
}

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
                    gl.Disable(gl::BLEND);
                },
                BlendType::SRC_ALPHA__ONE_MINUS_SRC_ALPHA => {
                    gl.Enable(gl::BLEND);
                    gl.BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
                },
                BlendType::DST_ALPHA__ONE_MINUS_DST_ALPHA => {
                    gl.Enable(gl::BLEND);
                    gl.BlendFunc(gl::DST_ALPHA, gl::ONE_MINUS_DST_ALPHA);
                },
                BlendType::ONE__ONE => {
                    gl.Enable(gl::BLEND);
                    gl.BlendFunc(gl::ONE, gl::ONE);
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
                    gl.Disable(gl::CULL_FACE);
                },
                CullType::BACK => {
                    gl.Enable(gl::CULL_FACE);
                    gl.CullFace(gl::BACK);
                },
                CullType::FRONT => {
                    gl.Enable(gl::CULL_FACE);
                    gl.CullFace(gl::FRONT);
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
                    gl.Disable(gl::DEPTH_TEST);
                },
                DepthTestType::LEQUAL => {
                    gl.Enable(gl::DEPTH_TEST);
                    gl.DepthFunc(gl::LEQUAL);
                },
                DepthTestType::LESS => {
                    gl.Enable(gl::DEPTH_TEST);
                    gl.DepthFunc(gl::LESS);
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