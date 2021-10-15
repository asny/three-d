use crate::core::*;

///
/// A bounding box that aligns with the x, y and z axes.
///
#[derive(Debug, Copy, Clone)]
pub struct AxisAlignedBoundingBox {
    min: Vec3,
    max: Vec3,
}

impl AxisAlignedBoundingBox {
    /// An empty bounding box.
    pub const EMPTY: Self = Self {
        min: vec3(std::f32::INFINITY, std::f32::INFINITY, std::f32::INFINITY),
        max: vec3(
            std::f32::NEG_INFINITY,
            std::f32::NEG_INFINITY,
            std::f32::NEG_INFINITY,
        ),
    };

    /// An infinitely large bounding box.
    pub const INFINITE: Self = Self {
        min: vec3(
            std::f32::NEG_INFINITY,
            std::f32::NEG_INFINITY,
            std::f32::NEG_INFINITY,
        ),
        max: vec3(std::f32::INFINITY, std::f32::INFINITY, std::f32::INFINITY),
    };

    ///
    /// Constructs a new bounding box and expands it such that all of the given positions are contained inside the bounding box.
    /// A position consisting of an x, y and z coordinate corresponds to three consecutive value in the positions array.
    ///
    pub fn new_with_positions(positions: &[f32]) -> Self {
        let mut aabb = Self::EMPTY;
        aabb.expand(positions);
        aabb
    }

    ///
    /// Constructs a new bounding box and expands it such that all of the given positions transformed with the given transformation are contained inside the bounding box.
    /// A position consisting of an x, y and z coordinate corresponds to three consecutive value in the positions array.
    ///
    pub fn new_with_transformed_positions(positions: &[f32], transformation: &Mat4) -> Self {
        let mut aabb = Self::EMPTY;
        aabb.expand_with_transformation(positions, transformation);
        aabb
    }

    ///
    /// Returns true if the bounding box is empty (ie. constructed by [AxisAlignedBoundingBox::EMPTY]).
    ///
    pub fn is_empty(&self) -> bool {
        self.max.x == f32::NEG_INFINITY
    }

    ///
    /// Returns true if the bounding box is infinitely large (ie. constructed by [AxisAlignedBoundingBox::INFINITE]).
    ///
    pub fn is_infinite(&self) -> bool {
        self.max.x == f32::INFINITY
    }

    ///
    /// Get the minimum coordinate of the bounding box.
    ///
    pub fn min(&self) -> &Vec3 {
        &self.min
    }

    ///
    /// Get the maximum coordinate of the bounding box.
    ///
    pub fn max(&self) -> &Vec3 {
        &self.max
    }

    ///
    /// Expands the bounding box such that all of the given positions are contained inside the bounding box.
    /// A position consisting of an x, y and z coordinate corresponds to three consecutive value in the positions array.
    ///
    pub fn expand(&mut self, positions: &[f32]) {
        for i in 0..positions.len() {
            match i % 3 {
                0 => {
                    self.min.x = f32::min(positions[i], self.min.x);
                    self.max.x = f32::max(positions[i], self.max.x);
                }
                1 => {
                    self.min.y = f32::min(positions[i], self.min.y);
                    self.max.y = f32::max(positions[i], self.max.y);
                }
                2 => {
                    self.min.z = f32::min(positions[i], self.min.z);
                    self.max.z = f32::max(positions[i], self.max.z);
                }
                _ => {
                    unreachable!()
                }
            };
        }
    }

    ///
    /// Expands the bounding box such that all of the given positions transformed with the given transformation are contained inside the bounding box.
    /// A position consisting of an x, y and z coordinate corresponds to three consecutive value in the positions array.
    ///
    pub fn expand_with_transformation(&mut self, positions: &[f32], transformation: &Mat4) {
        for i in 0..positions.len() / 3 {
            let pos = transformation
                * vec4(
                    positions[i * 3],
                    positions[i * 3 + 1],
                    positions[i * 3 + 2],
                    1.0,
                );
            self.min.x = f32::min(pos.x, self.min.x);
            self.max.x = f32::max(pos.x, self.max.x);
            self.min.y = f32::min(pos.y, self.min.y);
            self.max.y = f32::max(pos.y, self.max.y);
            self.min.z = f32::min(pos.z, self.min.z);
            self.max.z = f32::max(pos.z, self.max.z);
        }
    }

    ///
    /// Expand the bounding box such that it also contains the given other bounding box.
    ///
    pub fn expand_with_aabb(&mut self, other: &AxisAlignedBoundingBox) {
        self.min = vec3(
            f32::min(self.min.x, other.min.x),
            f32::min(self.min.y, other.min.y),
            f32::min(self.min.z, other.min.z),
        );
        self.max = vec3(
            f32::max(self.max.x, other.max.x),
            f32::max(self.max.y, other.max.y),
            f32::max(self.max.z, other.max.z),
        );
    }

    ///
    /// Transforms the bounding box by the given transformation.
    ///
    /// **Note:** Use [new_with_transformed_positions](crate::AxisAlignedBoundingBox::new_with_transformed_positions) instead of
    /// [new_with_positions](crate::AxisAlignedBoundingBox::new_with_positions) followed by this method to create a more tight bounding box.
    ///
    pub fn transform(&mut self, transformation: &Mat4) {
        let aabb = Self::new_with_transformed_positions(
            &[
                self.min.x, self.min.y, self.min.z, self.max.x, self.min.y, self.min.z, self.min.x,
                self.max.y, self.min.z, self.min.x, self.min.y, self.max.z, self.min.x, self.max.y,
                self.max.z, self.max.x, self.min.y, self.max.z, self.max.x, self.max.y, self.min.z,
                self.max.x, self.max.y, self.max.z,
            ],
            transformation,
        );
        self.min = aabb.min;
        self.max = aabb.max;
    }
}
