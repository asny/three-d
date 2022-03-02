use crate::renderer::*;

pub struct Shape<G: Geometry, M: Material> {
    pub geometry: G,
    /// The material applied to the geometry
    pub material: M,
}

impl<G: Geometry, M: Material> Geometry for Shape<G, M> {
    fn aabb(&self) -> AxisAlignedBoundingBox {
        self.geometry.aabb()
    }

    fn transformation(&self) -> Mat4 {
        self.geometry.transformation()
    }

    fn render_with_material(
        &self,
        material: &dyn Material,
        camera: &Camera,
        lights: &[&dyn Light],
    ) -> ThreeDResult<()> {
        self.geometry.render_with_material(material, camera, lights)
    }
}

impl<G: GeometryMut, M: Material> GeometryMut for Shape<G, M> {
    fn set_transformation(&mut self, transformation: Mat4) {
        self.geometry.set_transformation(transformation);
    }
}

impl<G: Geometry, M: Material> Object for Shape<G, M> {
    fn render(&self, camera: &Camera, lights: &[&dyn Light]) -> ThreeDResult<()> {
        self.render_with_material(&self.material, camera, lights)
    }

    fn is_transparent(&self) -> bool {
        self.material.is_transparent()
    }
}

impl<G: Geometry + Clone, M: Material + Clone> Clone for Shape<G, M> {
    fn clone(&self) -> Self {
        Self {
            geometry: self.geometry.clone(),
            material: self.material.clone(),
        }
    }
}
