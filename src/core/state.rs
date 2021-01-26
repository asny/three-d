use crate::gl::consts;
use crate::gl::Gl;

#[derive(Debug, Copy, Clone)]
pub struct RenderStates {
    pub depth_write: bool,
    pub depth_test: DepthTestType,
    pub cull: CullType
}

impl Default for RenderStates {
    fn default() -> Self {
        Self {
            depth_write: true,
            depth_test: DepthTestType::None,
            cull: CullType::None
        }
     }
}

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
                    gl.disable(consts::BLEND);
                },
                BlendType::SrcAlphaOneMinusSrcAlpha => {
                    gl.enable(consts::BLEND);
                    gl.blend_func(consts::SRC_ALPHA, consts::ONE_MINUS_SRC_ALPHA);
                },
                BlendType::DstAlphaOneMinusDstAlpha => {
                    gl.enable(consts::BLEND);
                    gl.blend_func(consts::DST_ALPHA, consts::ONE_MINUS_DST_ALPHA);
                },
                BlendType::OneOne => {
                    gl.enable(consts::BLEND);
                    gl.blend_func(consts::ONE, consts::ONE);
                }
            }
            CURRENT = blend_type;
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
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
                    gl.disable(consts::CULL_FACE);
                },
                CullType::Back => {
                    gl.enable(consts::CULL_FACE);
                    gl.cull_face(consts::BACK);
                },
                CullType::Front => {
                    gl.enable(consts::CULL_FACE);
                    gl.cull_face(consts::FRONT);
                },
                CullType::FrontAndBack => {
                    gl.enable(consts::CULL_FACE);
                    gl.cull_face(consts::FRONT_AND_BACK);
                }
            }
            CURRENT = cull_type;
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
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
                gl.disable(consts::DEPTH_TEST);
            }
            else {
                gl.enable(consts::DEPTH_TEST);
            }

            match depth_test_type {
                DepthTestType::Never => {
                    gl.depth_func(consts::NEVER);
                },
                DepthTestType::Less => {
                    gl.depth_func(consts::LESS);
                },
                DepthTestType::Equal => {
                    gl.depth_func(consts::EQUAL);
                },
                DepthTestType::LessOrEqual => {
                    gl.depth_func(consts::LEQUAL);
                },
                DepthTestType::Greater => {
                    gl.depth_func(consts::GREATER);
                },
                DepthTestType::NotEqual => {
                    gl.depth_func(consts::NOTEQUAL);
                },
                DepthTestType::GreaterOrEqual => {
                    gl.depth_func(consts::GEQUAL);
                },
                DepthTestType::Always => {
                    gl.depth_func(consts::ALWAYS);
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