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
