
#[derive(Clone, Copy, Debug, Default)]
pub struct TextureRegion {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl TextureRegion {
    pub fn new() -> TextureRegion {
        TextureRegion {
            x: 0.0,
            y: 0.0,
            w: 1.0,
            h: 1.0,
        }
    }

    pub fn from_slice(subt: [f32;4]) -> TextureRegion {
        TextureRegion {
            x: subt[0],
            y: subt[1],
            w: subt[2],
            h: subt[3],
        }
    }

    pub fn from_vec(subt: Vec<f32>) -> TextureRegion {
        TextureRegion {
            x: subt[0],
            y: subt[1],
            w: subt[2],
            h: subt[3],
        }
    }
}