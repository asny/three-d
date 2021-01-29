
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Viewport {
    pub x: i32,
    pub y: i32,
    pub width: usize,
    pub height: usize
}

impl Viewport {
    pub fn new(width: usize, height: usize) -> Self {
        Self {x: 0, y: 0, width, height}
    }

    pub fn aspect(&self) -> f32 {
        self.width as f32 / self.height as f32
    }
}