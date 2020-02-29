use crate::core::Gl;

#[derive(PartialEq)]
pub enum BlendType {
    None,
    SrcAlphaOneMinusSrcAlpha,
    DstAlphaOneMinusDstAlpha,
    OneOne
}

pub fn blend(gl: &Gl, blend_type: BlendType)
{
    unsafe {
        static mut CURRENT: BlendType = BlendType::None;
        if blend_type != CURRENT
        {
            match blend_type {
                BlendType::None => {
                    gl.disable(gl::consts::BLEND);
                },
                BlendType::SrcAlphaOneMinusSrcAlpha => {
                    gl.enable(gl::consts::BLEND);
                    gl.blend_func(gl::consts::SRC_ALPHA, gl::consts::ONE_MINUS_SRC_ALPHA);
                },
                BlendType::DstAlphaOneMinusDstAlpha => {
                    gl.enable(gl::consts::BLEND);
                    gl.blend_func(gl::consts::DST_ALPHA, gl::consts::ONE_MINUS_DST_ALPHA);
                },
                BlendType::OneOne => {
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
    None,
    Back,
    Front,
    FrontAndBack
}

pub fn cull(gl: &Gl, cull_type: CullType)
{
    unsafe {
        static mut CURRENT: CullType = CullType::None;
        if cull_type != CURRENT
        {
            match cull_type {
                CullType::None => {
                    gl.disable(gl::consts::CULL_FACE);
                },
                CullType::Back => {
                    gl.enable(gl::consts::CULL_FACE);
                    gl.cull_face(gl::consts::BACK);
                },
                CullType::Front => {
                    gl.enable(gl::consts::CULL_FACE);
                    gl.cull_face(gl::consts::FRONT);
                },
                CullType::FrontAndBack => {
                    gl.enable(gl::consts::CULL_FACE);
                    gl.cull_face(gl::consts::FRONT_AND_BACK);
                }
            }
            CURRENT = cull_type;
        }
    }
}

#[derive(PartialEq)]
pub enum DepthTestType {
    None,
    Never,
    Less,
    Equal,
    LessOrEqual,
    Greater,
    NotEqual,
    GreaterOrEqual,
    Always
}

pub fn depth_test(gl: &Gl, depth_test_type: DepthTestType)
{
    unsafe {
        static mut CURRENT: DepthTestType = DepthTestType::None;
        if depth_test_type != CURRENT
        {
            if depth_test_type == DepthTestType::None {
                gl.disable(gl::consts::DEPTH_TEST);
            }
            else {
                gl.enable(gl::consts::DEPTH_TEST);
            }

            match depth_test_type {
                DepthTestType::Never => {
                    gl.depth_func(gl::consts::NEVER);
                },
                DepthTestType::Less => {
                    gl.depth_func(gl::consts::LESS);
                },
                DepthTestType::Equal => {
                    gl.depth_func(gl::consts::EQUAL);
                },
                DepthTestType::LessOrEqual => {
                    gl.depth_func(gl::consts::LEQUAL);
                },
                DepthTestType::Greater => {
                    gl.depth_func(gl::consts::GREATER);
                },
                DepthTestType::NotEqual => {
                    gl.depth_func(gl::consts::NOTEQUAL);
                },
                DepthTestType::GreaterOrEqual => {
                    gl.depth_func(gl::consts::GEQUAL);
                },
                DepthTestType::Always => {
                    gl.depth_func(gl::consts::ALWAYS);
                },
                DepthTestType::None => {}
            }
            CURRENT = depth_test_type;
        }
    }
}

pub fn depth_write(gl: &Gl, enable: bool)
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