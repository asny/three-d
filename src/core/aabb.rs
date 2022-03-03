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
    ///
    pub fn new_with_positions(positions: &[Vec3]) -> Self {
        let mut aabb = Self::EMPTY;
        aabb.expand(positions);
        aabb
    }

    ///
    /// Constructs a new bounding box and expands it such that all of the given positions transformed with the given transformation are contained inside the bounding box.
    /// A position consisting of an x, y and z coordinate corresponds to three consecutive value in the positions array.
    ///
    pub fn new_with_transformed_positions(positions: &[Vec3], transformation: &Mat4) -> Self {
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
    pub fn min(&self) -> Vec3 {
        self.min
    }

    ///
    /// Get the maximum coordinate of the bounding box.
    ///
    pub fn max(&self) -> Vec3 {
        self.max
    }

    ///
    /// Get the center of the bounding box.
    ///
    pub fn center(&self) -> Vec3 {
        if self.is_infinite() {
            vec3(0.0, 0.0, 0.0)
        } else {
            0.5 * self.max + 0.5 * self.min
        }
    }

    ///
    /// Get the size of the bounding box.
    ///
    pub fn size(&self) -> Vec3 {
        self.max - self.min
    }

    ///
    /// Expands the bounding box such that all of the given positions are contained inside the bounding box.
    /// A position consisting of an x, y and z coordinate corresponds to three consecutive value in the positions array.
    ///
    pub fn expand(&mut self, positions: &[Vec3]) {
        for p in positions {
            self.min.x = self.min.x.min(p.x);
            self.min.y = self.min.y.min(p.y);
            self.min.z = self.min.z.min(p.z);

            self.max.x = self.max.x.max(p.x);
            self.max.y = self.max.y.max(p.y);
            self.max.z = self.max.z.max(p.z);
        }
    }

    ///
    /// Expands the bounding box such that all of the given positions transformed with the given transformation are contained inside the bounding box.
    /// A position consisting of an x, y and z coordinate corresponds to three consecutive value in the positions array.
    ///
    pub fn expand_with_transformation(&mut self, positions: &[Vec3], transformation: &Mat4) {
        self.expand(
            &positions
                .iter()
                .map(|p| (transformation * p.extend(1.0)).truncate())
                .collect::<Vec<_>>(),
        )
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
                self.min,
                vec3(self.max.x, self.min.y, self.min.z),
                vec3(self.min.x, self.max.y, self.min.z),
                vec3(self.min.x, self.min.y, self.max.z),
                vec3(self.min.x, self.max.y, self.max.z),
                vec3(self.max.x, self.min.y, self.max.z),
                vec3(self.max.x, self.max.y, self.min.z),
                self.max,
            ],
            transformation,
        );
        self.min = aabb.min;
        self.max = aabb.max;
    }

    ///
    /// The distance from position to the point in this bounding box that is closest to position.
    ///
    pub fn distance(&self, position: &Vec3) -> f32 {
        let x = (self.min.x - position.x)
            .max(position.x - self.max.x)
            .max(0.0);
        let y = (self.min.y - position.y)
            .max(position.y - self.max.y)
            .max(0.0);
        let z = (self.min.z - position.z)
            .max(position.z - self.max.z)
            .max(0.0);
        let d2 = x * x + y * y + z * z;
        if d2 > 0.001 {
            d2.sqrt()
        } else {
            d2
        }
    }

    ///
    /// The distance from position to the point in this bounding box that is furthest away from position.
    ///
    pub fn distance_max(&self, position: &Vec3) -> f32 {
        let x = (position.x - self.min.x)
            .abs()
            .max((self.max.x - position.x).abs());
        let y = (position.y - self.min.y)
            .abs()
            .max((self.max.y - position.y).abs());
        let z = (position.z - self.min.z)
            .abs()
            .max((self.max.z - position.z).abs());
        let d2 = x * x + y * y + z * z;
        if d2 > 0.001 {
            d2.sqrt()
        } else {
            d2
        }
    }
}
