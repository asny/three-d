
use crate::context::consts;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Interpolation {
    Nearest = consts::NEAREST as isize,
    Linear = consts::LINEAR as isize
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Wrapping {
    Repeat = consts::REPEAT as isize,
    MirroredRepeat = consts::MIRRORED_REPEAT as isize,
    ClampToEdge = consts::CLAMP_TO_EDGE as isize
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Format {
    R8 = consts::R8 as isize,
    R32F = consts::R32F as isize,
    RGB8 = consts::RGB8 as isize,
    RGB32F = consts::RGB32F as isize,
    RGBA4 = consts::RGBA4 as isize,
    RGBA8 = consts::RGBA8 as isize,
    RGBA32F = consts::RGBA32F as isize,
    Depth16 = consts::DEPTH_COMPONENT16 as isize,
    Depth24 = consts::DEPTH_COMPONENT24 as isize,
    Depth32F = consts::DEPTH_COMPONENT32F as isize
}

pub struct CPUTexture {
    pub bytes: Option<Vec<u8>>,
    pub width: usize,
    pub height: usize,
    pub depth: usize,
    pub format: Format,
    pub min_filter: Interpolation,
    pub mag_filter: Interpolation,
    pub mip_map_filter: Option<Interpolation>,
    pub wrap_s: Wrapping,
    pub wrap_t: Wrapping,
    pub wrap_r: Wrapping
}

impl Default for CPUTexture {
    fn default() -> Self {
        Self {
            bytes: None,
            width: 1,
            height: 1,
            depth: 1,
            format: Format::RGBA8,
            min_filter: Interpolation::Linear,
            mag_filter: Interpolation::Linear,
            mip_map_filter: Some(Interpolation::Linear),
            wrap_s: Wrapping::Repeat,
            wrap_t: Wrapping::Repeat,
            wrap_r: Wrapping::Repeat
        }
    }
}