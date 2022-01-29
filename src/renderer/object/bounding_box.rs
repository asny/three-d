use crate::renderer::*;

pub struct BoundingBox<M: Material> {
    model: InstancedModel<M>,
    aabb: AxisAlignedBoundingBox,
}

impl<M: Material> BoundingBox<M> {
    ///
    /// Creates a bounding box object from an axis aligned bounding box.
    ///
    pub fn new_with_material(
        context: &Context,
        aabb: AxisAlignedBoundingBox,
        material: M,
    ) -> ThreeDResult<Self> {
        let size = aabb.size();
        let thickness = 0.02 * size.x.max(size.y).max(size.z);

        Self::new_with_material_and_thickness(context, aabb, material, thickness)
    }

    ///
    /// Creates a bounding box object from an axis aligned bounding box with a specified line
    /// thickness.
    ///
    pub fn new_with_material_and_thickness(
        context: &Context,
        aabb: AxisAlignedBoundingBox,
        material: M,
        thickness: f32,
    ) -> ThreeDResult<Self> {
        let max = aabb.max();
        let min = aabb.min();
        let size = aabb.size();
        let transformations = [
            Mat4::from_translation(min) * Mat4::from_nonuniform_scale(size.x, thickness, thickness),
            Mat4::from_translation(vec3(min.x, max.y, max.z))
                * Mat4::from_nonuniform_scale(size.x, thickness, thickness),
            Mat4::from_translation(vec3(min.x, min.y, max.z))
                * Mat4::from_nonuniform_scale(size.x, thickness, thickness),
            Mat4::from_translation(vec3(min.x, max.y, min.z))
                * Mat4::from_nonuniform_scale(size.x, thickness, thickness),
            Mat4::from_translation(min)
                * Mat4::from_angle_z(degrees(90.0))
                * Mat4::from_nonuniform_scale(size.y, thickness, thickness),
            Mat4::from_translation(vec3(max.x, min.y, max.z))
                * Mat4::from_angle_z(degrees(90.0))
                * Mat4::from_nonuniform_scale(size.y, thickness, thickness),
            Mat4::from_translation(vec3(min.x, min.y, max.z))
                * Mat4::from_angle_z(degrees(90.0))
                * Mat4::from_nonuniform_scale(size.y, thickness, thickness),
            Mat4::from_translation(vec3(max.x, min.y, min.z))
                * Mat4::from_angle_z(degrees(90.0))
                * Mat4::from_nonuniform_scale(size.y, thickness, thickness),
            Mat4::from_translation(min)
                * Mat4::from_angle_y(degrees(-90.0))
                * Mat4::from_nonuniform_scale(size.z, thickness, thickness),
            Mat4::from_translation(vec3(max.x, max.y, min.z))
                * Mat4::from_angle_y(degrees(-90.0))
                * Mat4::from_nonuniform_scale(size.z, thickness, thickness),
            Mat4::from_translation(vec3(min.x, max.y, min.z))
                * Mat4::from_angle_y(degrees(-90.0))
                * Mat4::from_nonuniform_scale(size.z, thickness, thickness),
            Mat4::from_translation(vec3(max.x, min.y, min.z))
                * Mat4::from_angle_y(degrees(-90.0))
                * Mat4::from_nonuniform_scale(size.z, thickness, thickness),
        ];
        let model = InstancedModel::new_with_material(
            context,
            &transformations
                .iter()
                .map(|t| ModelInstance {
                    geometry_transform: *t,
                    ..Default::default()
                })
                .collect::<Vec<_>>(),
            &CPUMesh::cylinder(16),
            material,
        )?;
        Ok(Self { model, aabb })
    }
}

impl<M: Material> Shadable for BoundingBox<M> {
    fn render_with_material(
        &self,
        material: &dyn Material,
        camera: &Camera,
        lights: &[&dyn Light],
    ) -> ThreeDResult<()> {
        self.model.render_with_material(material, camera, lights)
    }
}

impl<M: Material> Geometry for BoundingBox<M> {
    fn aabb(&self) -> AxisAlignedBoundingBox {
        self.aabb
    }

    fn transformation(&self) -> Mat4 {
        Mat4::identity()
    }
}

impl<M: Material> Object for BoundingBox<M> {
    fn render(&self, camera: &Camera, lights: &[&dyn Light]) -> ThreeDResult<()> {
        self.model.render(camera, lights)
    }

    fn is_transparent(&self) -> bool {
        false
    }
}
