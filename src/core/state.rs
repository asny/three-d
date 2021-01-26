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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CullType {
    None,
    Back,
    Front,
    FrontAndBack
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

pub fn cull(gl: &Gl, cull_type: CullType)
{
}

pub fn depth_test(gl: &Gl, depth_test_type: DepthTestType)
{
}

pub fn depth_write(gl: &Gl, enable: bool)
{
}