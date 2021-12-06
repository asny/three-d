#[derive(Clone, Copy, Debug, Default)]
pub struct TextureTransform {
    pub offset_x: f32,
    pub offset_y: f32,
    pub scale_x: f32,
    pub scale_y: f32,
}

impl TextureTransform {
    pub fn new() -> TextureTransform {
        TextureTransform {
            offset_x: 0.0,
            offset_y: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
        }
    }

    pub fn halve(&mut self) {
        self.offset_x /= 2.0;
        self.offset_y /= 2.0;
        self.scale_x /= 2.0;
        self.scale_y /= 2.0;
    }

    pub fn shift(&mut self, dx: f32, dy: f32) {
        self.offset_x += dx;
        self.offset_y += dy;
    }
}
