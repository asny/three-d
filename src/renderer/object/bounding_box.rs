use crate::renderer::*;

///
/// A bounding box object used for visualising an [AxisAlignedBoundingBox].
///
pub struct BoundingBox<M: Material> {
    model: Gm<InstancedMesh, M>,
    aabb: AxisAlignedBoundingBox,
}

impl<M: Material> BoundingBox<M> {
    ///
    /// Creates a bounding box object from an axis aligned bounding box.
    ///
    pub fn new_with_material(context: &Context, aabb: AxisAlignedBoundingBox, material: M) -> Self {
        let size = aabb.size();
        let thickness = 0.02 * size.x.max(size.y).max(size.z);

        Self::new_with_material_and_thickness(context, aabb, material, thickness)
    }

    ///
    /// Returns an iterator over a reference to the object which can be used as input to a render function, for example [RenderTarget::render].
    ///
    pub fn obj_iter(&self) -> impl Iterator<Item = &dyn Object> + Clone {
        std::iter::once(self as &dyn Object)
    }

    ///
    /// Returns an iterator over a reference to the geometry which can be used as input to for example [pick], [RenderTarget::render_with_material] or [DirectionalLight::generate_shadow_map].
    ///
    pub fn geo_iter(&self) -> impl Iterator<Item = &dyn Geometry> + Clone {
        std::iter::once(self as &dyn Geometry)
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
    ) -> Self {
        let max = aabb.max();
        let min = aabb.min();
        let size = aabb.size();
        let translations = vec![
            min,
            vec3(min.x, max.y, max.z),
            vec3(min.x, min.y, max.z),
            vec3(min.x, max.y, min.z),
            min,
            vec3(max.x, min.y, max.z),
            vec3(min.x, min.y, max.z),
            vec3(max.x, min.y, min.z),
            min,
            vec3(max.x, max.y, min.z),
            vec3(min.x, max.y, min.z),
            vec3(max.x, min.y, min.z),
        ];

        let rotations = vec![
            Quat::zero(),
            Quat::zero(),
            Quat::zero(),
            Quat::zero(),
            Quat::from_angle_z(degrees(90.0)),
            Quat::from_angle_z(degrees(90.0)),
            Quat::from_angle_z(degrees(90.0)),
            Quat::from_angle_z(degrees(90.0)),
            Quat::from_angle_y(degrees(-90.0)),
            Quat::from_angle_y(degrees(-90.0)),
            Quat::from_angle_y(degrees(-90.0)),
            Quat::from_angle_y(degrees(-90.0)),
        ];

        let scales = vec![
            vec3(size.x, thickness, thickness),
            vec3(size.x, thickness, thickness),
            vec3(size.x, thickness, thickness),
            vec3(size.x, thickness, thickness),
            vec3(size.y, thickness, thickness),
            vec3(size.y, thickness, thickness),
            vec3(size.y, thickness, thickness),
            vec3(size.y, thickness, thickness),
            vec3(size.z, thickness, thickness),
            vec3(size.z, thickness, thickness),
            vec3(size.z, thickness, thickness),
            vec3(size.z, thickness, thickness),
        ];
        let model = Gm::new(
            InstancedMesh::new(
                context,
                &Instances {
                    translations,
                    rotations: Some(rotations),
                    scales: Some(scales),
                    ..Default::default()
                },
                &CpuMesh::cylinder(16),
            ),
            material,
        );
        Self { model, aabb }
    }
}

impl<M: Material> Geometry for BoundingBox<M> {
    fn aabb(&self) -> AxisAlignedBoundingBox {
        self.aabb
    }

    fn render_with_material(
        &self,
        material: &dyn Material,
        camera: &Camera,
        lights: &[&dyn Light],
    ) {
        self.model.render_with_material(material, camera, lights)
    }
}

impl<M: Material> Object for BoundingBox<M> {
    fn render(&self, camera: &Camera, lights: &[&dyn Light]) {
        self.model.render(camera, lights)
    }

    fn material_type(&self) -> MaterialType {
        MaterialType::Opaque
    }
}
