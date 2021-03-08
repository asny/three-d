
///
/// Possible modes of interpolation which determines the texture output between texture pixels.
///
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Interpolation {
    Nearest,
    Linear
}

///
/// Possible wrapping modes for a texture which determines how the texture is applied outside of the
/// [0..1] uv coordinate range.
///
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Wrapping {
    Repeat,
    MirroredRepeat,
    ClampToEdge
}

///
/// Possible formats for pixels in a texture.
///
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Format {
    R8,
    R32F,
    RGB8,
    RGB32F,
    SRGB8,
    RGBA8,
    SRGBA8,
    RGBA32F
}

///
/// A CPU-side version of a texture (for example [2D texture](crate::Texture2D).
/// Can be constructed manually or loaded via [io](crate::io).
///
pub struct CPUTexture<T> {
    pub data: Vec<T>,
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

impl Default for CPUTexture<u8> {
    fn default() -> Self {
        Self {
            data: [255u8, 255, 0, 255].into(),
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