///
/// Defines the part of the screen or render target that is rendered to.
/// All pixels outside of the scissor box will not be modified.
/// All values should be given in physical pixels.
///
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ScissorBox {
    /// The distance in pixels from the left edge of the target.
    pub x: i32,
    /// The distance in pixels from the bottom edge of the target.
    pub y: i32,
    /// The width of the box.
    pub width: u32,
    /// The height of the box.
    pub height: u32,
}

impl ScissorBox {
    ///
    /// Creates a new scissor box which starts at origo (x and y are both zero).
    ///
    pub fn new_at_origo(width: u32, height: u32) -> Self {
        Self {
            x: 0,
            y: 0,
            width,
            height,
        }
    }

    ///
    /// Returns the intersection between this and the other ScissorBox.
    ///
    pub fn intersection(&self, other: impl Into<Self>) -> Self {
        let other = other.into();
        let x = self.x.max(other.x);
        let y = self.y.max(other.y);
        let width = (self.x + self.width as i32 - x)
            .min(other.x + other.width as i32 - x)
            .max(0) as u32;
        let height = (self.y + self.height as i32 - y)
            .min(other.y + other.height as i32 - y)
            .max(0) as u32;
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

impl From<crate::core::Viewport> for ScissorBox {
    fn from(viewport: crate::core::Viewport) -> Self {
        Self {
            x: viewport.x,
            y: viewport.y,
            width: viewport.width,
            height: viewport.height,
        }
    }
}

impl From<crate::core::ScissorBox> for crate::core::Viewport {
    fn from(viewport: crate::core::ScissorBox) -> Self {
        Self {
            x: viewport.x,
            y: viewport.y,
            width: viewport.width,
            height: viewport.height,
        }
    }
}
