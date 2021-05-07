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

pub trait TextureValueType:
    Default + std::fmt::Debug + Clone + crate::core::internal::TextureValueTypeExtension
{
}
impl TextureValueType for u8 {}
impl TextureValueType for f32 {}

///
/// Possible formats for pixels in a texture.
///
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Format {
    R,
    RG,
    RGB,
    SRGB,
    RGBA,
    SRGBA,
}

impl Format {
    pub fn color_channel_count(&self) -> usize {
        match self {
            Format::R => 1,
            Format::RG => 2,
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
pub struct CPUTexture<T: TextureValueType> {
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

impl<T: TextureValueType> CPUTexture<T> {
    pub fn resize(&mut self, width: usize, height: usize, offset_x: usize, offset_y: usize) {
        let channels = self.format.color_channel_count();
        let mut new_data = vec![T::default(); width * height * channels];
        for x in 0..width {
            for y in 0..height {
                let x_ = x as i32 - offset_x as i32;
                let y_ = y as i32 - offset_y as i32;
                if 0 <= x_ && x_ < self.width as i32 && 0 <= y_ && y_ < self.height as i32 {
                    let source_index = (y_ as usize * self.width + x_ as usize) * channels;
                    let dest_index = (y as usize * width + x as usize) * channels;
                    for i in 0..channels {
                        new_data[dest_index + i] = self.data[source_index + i].clone();
                    }
                }
            }
        }
        self.data = new_data;
        self.width = width;
        self.height = height;
    }

    pub fn resize_to_power_of_2(&mut self, offset_x: usize, offset_y: usize) {
        let w = usize::next_power_of_two(self.width);
        let h = usize::next_power_of_two(self.height);
        self.resize(w, h, offset_x, offset_y);
    }
}

impl<T: TextureValueType> Default for CPUTexture<T> {
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

impl<T: TextureValueType> std::fmt::Debug for CPUTexture<T> {
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
