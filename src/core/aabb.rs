use crate::core::types::*;

pub struct AxisAllignedBoundingBox {
    pub min: Vec3,
    pub max: Vec3
}

impl AxisAllignedBoundingBox {

    pub fn new(positions: &[f32]) -> Self {
        let mut aabb = Self {min: vec3(std::f32::INFINITY, std::f32::INFINITY, std::f32::INFINITY),
            max: vec3(std::f32::NEG_INFINITY, std::f32::NEG_INFINITY, std::f32::NEG_INFINITY)};

        for i in 0..positions.len() {
            match i%3 {
                0 => {
                    aabb.min.x = f32::min(positions[i], aabb.min.x);
                    aabb.max.x = f32::max(positions[i], aabb.max.x);
                },
                1 => {
                    aabb.min.y = f32::min(positions[i], aabb.min.y);
                    aabb.max.y = f32::max(positions[i], aabb.max.y);
                },
                2 => {
                    aabb.min.z = f32::min(positions[i], aabb.min.z);
                    aabb.max.z = f32::max(positions[i], aabb.max.z);
                },
                _ => {unreachable!()}
            };
        }
        aabb
    }

    pub fn add(&mut self, other: &AxisAllignedBoundingBox) {
        self.min = vec3(f32::min(self.min.x, other.min.x), f32::min(self.min.y, other.min.y), f32::min(self.min.z, other.min.z));
        self.max = vec3(f32::max(self.max.x, other.max.x), f32::max(self.max.y, other.max.y), f32::max(self.max.z, other.max.z));
    }
}