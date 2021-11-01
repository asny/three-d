use crate::core::math::*;

/// Represents a color composed of a red, green and blue component.
/// In addition, the alpha value determines the how transparent the color is (0 is fully transparent and 255 is fully opaque).
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Color {
    /// Red component
    pub r: u8,
    /// Green component
    pub g: u8,
    /// Blue component
    pub b: u8,
    /// Alpha component
    pub a: u8,
}

impl Color {
    ///
    /// Creates a new color with the given values.
    ///
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    ///
    /// Creates a new color with the given r, g and b values and an alpha value of 255.
    ///
    pub const fn new_opaque(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    ///
    /// Creates a new color from three float elements where each element are in the range `0.0..=1.0`.
    ///
    pub fn from_rgb_slice(rgba: &[f32; 3]) -> Self {
        Self {
            r: (rgba[0] * 255.0) as u8,
            g: (rgba[1] * 255.0) as u8,
            b: (rgba[2] * 255.0) as u8,
            ..Default::default()
        }
    }

    ///
    /// Creates a new color from four float elements where each element are in the range `0.0..=1.0`.
    ///
    pub fn from_rgba_slice(rgba: &[f32; 4]) -> Self {
        Self {
            r: (rgba[0] * 255.0) as u8,
            g: (rgba[1] * 255.0) as u8,
            b: (rgba[2] * 255.0) as u8,
            a: (rgba[3] * 255.0) as u8,
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

    /// Convert to [`Vec3`] by mapping the red, green and blue component to the range `0.0..=1.0`.
    pub fn to_vec3(&self) -> Vec3 {
        vec3(
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
        )
    }

    /// Convert to [`Vec4`] by mapping each component to the range `0.0..=1.0`.
    pub fn to_vec4(&self) -> Vec4 {
        vec4(
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            self.a as f32 / 255.0,
        )
    }

    /// Convert to a slice by mapping the red, green and blue component to the range `0.0..=1.0`.
    pub fn to_rgb_slice(&self) -> [f32; 3] {
        [
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
        ]
    }

    /// Convert to a slice by mapping each component to the range `0.0..=1.0`.
    pub fn to_rgba_slice(&self) -> [f32; 4] {
        [
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            self.a as f32 / 255.0,
        ]
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::WHITE
    }
}
