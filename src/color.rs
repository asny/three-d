use crate::math::*;

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
    /// Opaque red
    pub const RED: Color = Color {
        r: 255,
        g: 0,
        b: 0,
        a: 255,
    };
    /// Opaque green
    pub const GREEN: Color = Color {
        r: 0,
        g: 255,
        b: 0,
        a: 255,
    };
    /// Opaque blue
    pub const BLUE: Color = Color {
        r: 0,
        g: 0,
        b: 255,
        a: 255,
    };

    /// Convert to [`Vec4`] by mapping each component to range `0.0..=1.0`.
    pub fn to_vec4(&self) -> Vec4 {
        vec4(
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            self.a as f32 / 255.0,
        )
    }
}
