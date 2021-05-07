///
/// Possible modes of interpolation which determines the texture output between texture pixels.
///
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Interpolation {
    Nearest,
    Linear,
}

///
/// Possible wrapping modes for a texture which determines how the texture is applied outside of the
/// [0..1] uv coordinate range.
///
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Wrapping {
    Repeat,
    MirroredRepeat,
    ClampToEdge,
}

pub trait CPUTextureValueType: Default + std::fmt::Debug + Clone {}
impl CPUTextureValueType for u8 {}
impl CPUTextureValueType for f32 {}

///
/// Possible formats for pixels in a texture.
///
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Format {
    R,
    RGB,
    SRGB,
    RGBA,
    SRGBA,
}

impl Format {
    pub fn color_channel_count(&self) -> usize {
        match self {
            Format::R => 1,
            Format::RGB => 3,
            Format::SRGB => 3,
            Format::RGBA => 4,
            Format::SRGBA => 4,
        }
    }
}

impl Default for Format {
    fn default() -> Self {
        Format::RGBA
    }
}

///
/// A CPU-side version of a texture, for example [2D texture](crate::Texture2D).
/// Can be constructed manually or loaded via [io](crate::io).
///
pub struct CPUTexture<T: CPUTextureValueType> {
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
    pub wrap_r: Wrapping,
}

impl<T: CPUTextureValueType> Default for CPUTexture<T> {
    fn default() -> Self {
        Self {
            data: [T::default(), T::default(), T::default(), T::default()].into(),
            width: 1,
            height: 1,
            depth: 1,
            format: Format::default(),
            min_filter: Interpolation::Linear,
            mag_filter: Interpolation::Linear,
            mip_map_filter: Some(Interpolation::Linear),
            wrap_s: Wrapping::Repeat,
            wrap_t: Wrapping::Repeat,
            wrap_r: Wrapping::Repeat,
        }
    }
}

impl<T: CPUTextureValueType> std::fmt::Debug for CPUTexture<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CPUTexture")
            .field("format", &self.format)
            .field("data length", &self.data.len())
            .field("width", &self.width)
            .field("height", &self.height)
            .field("depth", &self.depth)
            .field("min_filter", &self.min_filter)
            .field("mag_filter", &self.mag_filter)
            .field("mip_map_filter", &self.mip_map_filter)
            .field("wrap_s", &self.wrap_s)
            .field("wrap_t", &self.wrap_t)
            .field("wrap_r", &self.wrap_r)
            .finish()
    }
}
