//!
//! Support for text.
//!

// This whole file is inspired by: https://github.com/sebcrozet/kiss3d/blob/ff42b1e84dd406c54e698bc56505b78bd50cdfc8/src/text/font.rs
// available under the BSD-3 licence.
// It has been modified to work with three-d

use crate::{Context, Texture2D};
use fontdue;
use std::{fs::File, io::Read};
use three_d_asset::{texture::Texture2D as CpuTexture, Interpolation, TextureData, Wrapping};

/// A ttf font.
#[derive(Clone, Debug)]
pub struct Font {
    /// A `fontdue` ttf font.
    pub font: fontdue::Font,
}

impl Font {
    /// Loads a new ttf font from a file.
    pub fn new(path: &str, scale: f32) -> Font {
        let mut data = Vec::new();
        let mut file = File::open(path).unwrap();
        let _ = file.read_to_end(&mut data).unwrap();

        let settings = fontdue::FontSettings {
            scale,
            ..fontdue::FontSettings::default()
        };

        let font = fontdue::Font::from_bytes(data, settings).unwrap();

        Font { font }
    }

    /// Instanciate a default font.
    pub fn default() -> Font {
        let data = include_bytes!("text/Roboto-Regular.ttf") as &[u8];

        let settings = fontdue::FontSettings {
            scale: 200.0,
            ..fontdue::FontSettings::default()
        };

        let font = fontdue::Font::from_bytes(data, settings).unwrap();

        Font { font }
    }

    /// The underlying rusttype font.
    #[inline]
    pub fn font(&self) -> &fontdue::Font {
        &self.font
    }

    pub fn rasterize(&self, effect: TextEffect, context: &Context) -> Texture2D {
        let (metrics, bitmap) = self.font.rasterize(effect.text, effect.size);

        Texture2D::new(
            context,
            &CpuTexture {
                name: "text".to_string(), // not necessary for text rendering?
                data: TextureData::RU8(bitmap),
                width: metrics.width as u32,
                height: metrics.height as u32,
                min_filter: Interpolation::Linear,
                mag_filter: Interpolation::Linear,
                mip_map_filter: Some(Interpolation::Linear),
                wrap_s: Wrapping::ClampToEdge,
                wrap_t: Wrapping::ClampToEdge,
            },
        )
    }
}

///
/// A text effect contains a character to be rendered. Use `Font::rasterize` to transform the effect into
/// a `Texture2D`.
///
#[derive(Clone, Debug)]
pub struct TextEffect {
    pub text: char,
    pub size: f32,
}
