use crate::core::math::*;
use rgb::RGBA8;

/// Represents a color composed of a red, green and blue component.
/// In addition, the alpha value determines the how transparent the color is (0 is fully transparent and 255 is fully opaque).
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Color {
    inner: RGBA8,
}

impl std::ops::Deref for Color {
    type Target = RGBA8;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::ops::DerefMut for Color {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Color {
    ///
    /// Creates a new color with the given r, g and b values and an alpha value of 255.
    ///
    pub const fn new_opaque(r: u8, g: u8, b: u8) -> Self {
        Self {
            inner: RGBA8 { r, g, b, a: 255 },
        }
    }

    /// Opaque red
    pub const RED: Color = Color::new_opaque(255, 0, 0);
    /// Opaque green
    pub const GREEN: Color = Color::new_opaque(0, 255, 0);
    /// Opaque blue
    pub const BLUE: Color = Color::new_opaque(0, 0, 255);
    /// Opaque white
    pub const WHITE: Color = Color::new_opaque(255, 255, 255);
    /// Opaque black
    pub const BLACK: Color = Color::new_opaque(0, 0, 0);

    ///
    /// Creates a new color from three float elements where each element are in the range `0.0..=1.0`.
    ///
    #[deprecated = "use into() instead"]
    pub fn from_rgb_slice(rgba: &[f32; 3]) -> Self {
        rgba.into()
    }

    ///
    /// Creates a new color from four float elements where each element are in the range `0.0..=1.0`.
    ///
    #[deprecated = "use into() instead"]
    pub fn from_rgba_slice(rgba: &[f32; 4]) -> Self {
        rgba.into()
    }

    /// Convert to [`Vec3`] by mapping the red, green and blue component to the range `0.0..=1.0`.
    #[deprecated = "use into() instead"]
    pub fn to_vec3(&self) -> Vec3 {
        (*self).into()
    }

    /// Convert to [`Vec4`] by mapping each component to the range `0.0..=1.0`.
    #[deprecated = "use into() instead"]
    pub fn to_vec4(&self) -> Vec4 {
        (*self).into()
    }

    /// Convert to a slice by mapping the red, green and blue component to the range `0.0..=1.0`.
    #[deprecated = "use into() instead"]
    pub fn to_rgb_slice(&self) -> [f32; 3] {
        (*self).into()
    }

    /// Convert to a slice by mapping each component to the range `0.0..=1.0`.
    #[deprecated = "use into() instead"]
    pub fn to_rgba_slice(&self) -> [f32; 4] {
        (*self).into()
    }
}

impl From<[f32; 3]> for Color {
    fn from(rgba: [f32; 3]) -> Self {
        Self {
            inner: RGBA8 {
                r: (rgba[0] * 255.0) as u8,
                g: (rgba[1] * 255.0) as u8,
                b: (rgba[2] * 255.0) as u8,
                a: 255,
            },
        }
    }
}

impl From<&[f32; 3]> for Color {
    fn from(rgba: &[f32; 3]) -> Self {
        Self {
            inner: RGBA8 {
                r: (rgba[0] * 255.0) as u8,
                g: (rgba[1] * 255.0) as u8,
                b: (rgba[2] * 255.0) as u8,
                a: 255,
            },
        }
    }
}

impl From<[f32; 4]> for Color {
    fn from(rgba: [f32; 4]) -> Self {
        Self {
            inner: RGBA8 {
                r: (rgba[0] * 255.0) as u8,
                g: (rgba[1] * 255.0) as u8,
                b: (rgba[2] * 255.0) as u8,
                a: (rgba[3] * 255.0) as u8,
            },
        }
    }
}

impl From<&[f32; 4]> for Color {
    fn from(rgba: &[f32; 4]) -> Self {
        Self {
            inner: RGBA8 {
                r: (rgba[0] * 255.0) as u8,
                g: (rgba[1] * 255.0) as u8,
                b: (rgba[2] * 255.0) as u8,
                a: (rgba[3] * 255.0) as u8,
            },
        }
    }
}

impl From<Color> for [f32; 3] {
    fn from(color: Color) -> Self {
        [
            color.r as f32 / 255.0,
            color.g as f32 / 255.0,
            color.b as f32 / 255.0,
        ]
    }
}

impl From<Color> for [f32; 4] {
    fn from(color: Color) -> Self {
        [
            color.r as f32 / 255.0,
            color.g as f32 / 255.0,
            color.b as f32 / 255.0,
            color.a as f32 / 255.0,
        ]
    }
}

impl From<Color> for Vec3 {
    fn from(color: Color) -> Self {
        vec3(
            color.r as f32 / 255.0,
            color.g as f32 / 255.0,
            color.b as f32 / 255.0,
        )
    }
}

impl From<Color> for Vec4 {
    fn from(color: Color) -> Self {
        vec4(
            color.r as f32 / 255.0,
            color.g as f32 / 255.0,
            color.b as f32 / 255.0,
            color.a as f32 / 255.0,
        )
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::WHITE
    }
}
