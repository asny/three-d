use crate::math::*;

///
/// A bounding box that aligns with the x, y and z axes.
///
#[derive(Debug, Copy, Clone)]
pub struct AxisAlignedBoundingBox {
    pub min: Vec3,
    pub max: Vec3
}

impl AxisAlignedBoundingBox {

    pub fn new() -> Self {
        Self {min: vec3(std::f32::INFINITY, std::f32::INFINITY, std::f32::INFINITY),
            max: vec3(std::f32::NEG_INFINITY, std::f32::NEG_INFINITY, std::f32::NEG_INFINITY)}
    }

    pub fn expand(mut self, positions: &[f32]) -> Self {
        for i in 0..positions.len() {
            match i%3 {
                0 => {
                    self.min.x = f32::min(positions[i], self.min.x);
                    self.max.x = f32::max(positions[i], self.max.x);
                },
                1 => {
                    self.min.y = f32::min(positions[i], self.min.y);
                    self.max.y = f32::max(positions[i], self.max.y);
                },
                2 => {
                    self.min.z = f32::min(positions[i], self.min.z);
                    self.max.z = f32::max(positions[i], self.max.z);
                },
                _ => {unreachable!()}
            };
        }
        self
    }

    pub fn expand_with_transformation(mut self, positions: &[f32], transformation: &Mat4) -> Self {
        for i in 0..positions.len()/3 {
            let pos = transformation * vec4(positions[i*3], positions[i*3 + 1], positions[i*3+2], 1.0);
            self.min.x = f32::min(pos.x, self.min.x);
            self.max.x = f32::max(pos.x, self.max.x);
            self.min.y = f32::min(pos.y, self.min.y);
            self.max.y = f32::max(pos.y, self.max.y);
            self.min.z = f32::min(pos.z, self.min.z);
            self.max.z = f32::max(pos.z, self.max.z);
        }
        self
    }

    pub fn add(mut self, other: &AxisAlignedBoundingBox) -> Self {
        self.min = vec3(f32::min(self.min.x, other.min.x), f32::min(self.min.y, other.min.y), f32::min(self.min.z, other.min.z));
        self.max = vec3(f32::max(self.max.x, other.max.x), f32::max(self.max.y, other.max.y), f32::max(self.max.z, other.max.z));
        self
    }
}