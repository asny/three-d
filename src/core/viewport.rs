///
/// Defines the part of the screen/render target that is rendered to.
/// All values should be given in physical pixels.
///
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Viewport {
    /// The distance in pixels from the left edge of the screen/render target.
    pub x: i32,
    /// The distance in pixels from the bottom edge of the screen/render target.
    pub y: i32,
    /// The width of the viewport.
    pub width: u32,
    /// The height of the viewport.
    pub height: u32,
}

impl Viewport {
    ///
    /// New viewport which starts at origo (x and y are both zero).
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
    /// Returns the aspect ratio of this viewport.
    ///
    pub fn aspect(&self) -> f32 {
        self.width as f32 / self.height as f32
    }

    ///
    /// Returns the intersection between this and the other Viewport.
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

impl From<crate::core::ScissorBox> for Viewport {
    fn from(viewport: crate::core::ScissorBox) -> Self {
        Self {
            x: viewport.x,
            y: viewport.y,
            width: viewport.width,
            height: viewport.height,
        }
    }
}
