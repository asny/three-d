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

    pub fn rasterize(&self, effect: TextEffect, context: &Context) -> CpuTexture {
        // Compute a bitmap for each letter
        let results: Vec<_> = effect
            .text
            .chars()
            .map(|letter| self.font.rasterize(letter, effect.size))
            .collect();

        // Get the size of the `Texture2D`.
        let width: usize = results.iter().map(|(m, _)| m.width).sum();
        let height: Option<usize> = results.iter().map(|(m, _)| m.height).max();
        let max_height = match height {
            Some(height) => height,
            None => todo!(),
        };

        // Transform the letters from single textures into a union `Texture2D`.
        //
        // Three things are to be considered.
        // 1. Not each letter has the same height. Letters with smaller height
        // than the maximum height of all letters have transparent texel, where the
        // texture height outreachs their own height.
        // 2. The new canvas for the texture of all unified letters has the size
        // (max height of all letters) * sum over all letters width.
        // 3. Each letter follows its own unique column-first ordering of values.
        // To fill in the values of each letter into the unified texture,
        // we go through each row of the vector and manually compute the slice
        // of values of each letter and append to the texture.
        let mut data: Vec<[u8; 4]> = Vec::new();
        for row in 0..max_height {
            let cur_height = max_height - row;
            for (metrics, letter) in &results {
                // Region of texture, where individual letters are smaller than canvas.
                if cur_height > metrics.height {
                    data.extend_from_slice(&vec![[0u8; 4]; metrics.width])
                }
                // Region of texture, where each letter has values to fill into texture.
                else {
                    // Compute offseted index into letter's bitmap data.
                    let row = row - (max_height - metrics.height);
                    let start = row * metrics.width;
                    let end = row * metrics.width + metrics.width;
                    data.extend_from_slice(
                        &letter[start..end]
                            .iter()
                            .map(|u| [0u8, 0u8, 0u8, *u])
                            .collect::<Vec<[u8; 4]>>(),
                    );
                }
            }
        }

        CpuTexture {
            name: "text".to_string(), // not necessary for text rendering?
            data: TextureData::RgbaU8(data),
            width: width as u32,
            height: max_height as u32,
            min_filter: Interpolation::Linear,
            mag_filter: Interpolation::Linear,
            mip_map_filter: Some(Interpolation::Linear),
            wrap_s: Wrapping::ClampToEdge,
            wrap_t: Wrapping::ClampToEdge,
        }
    }
}

///
/// An texture atlas with all UTF-8 symbols. Use the hashmap to find the index of a specific character.
///

///
/// A text effect contains a string to be rendered. Use `Font::rasterize` to transform the effect into
/// a `Texture2D`.
///
#[derive(Clone, Debug)]
pub struct TextEffect {
    pub text: String,
    pub size: f32,
}
