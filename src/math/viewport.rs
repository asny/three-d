
///
/// Defines the part of the screen/render target that is rendered to.
///
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Viewport {
    pub x: i32,
    pub y: i32,
    pub width: usize,
    pub height: usize
}

impl Viewport {
    ///
    /// New viewport which starts at origo (x and y are both zero).
    ///
    pub fn new_at_origo(width: usize, height: usize) -> Self {
        Self {x: 0, y: 0, width, height}
    }

    ///
    /// Returns the aspect ratio of this viewport.
    ///
    pub fn aspect(&self) -> f32 {
        self.width as f32 / self.height as f32
    }
}