use crate::renderer::*;

///
/// A bounding box geometry used for visualising an [AxisAlignedBoundingBox].
///
pub struct BoundingBox {
    mesh: InstancedMesh,
    aabb: AxisAlignedBoundingBox,
}

impl BoundingBox {
    ///
    /// Creates a bounding box geometry from an axis aligned bounding box.
    ///
    pub fn new(context: &Context, aabb: AxisAlignedBoundingBox) -> Self {
        let size = aabb.size();
        let thickness = 0.02 * size.x.max(size.y).max(size.z);

        Self::new_with_thickness(context, aabb, thickness)
    }

    ///
    /// Creates a bounding box object from an axis aligned bounding box with a specified line
    /// thickness.
    ///
    pub fn new_with_thickness(
        context: &Context,
        aabb: AxisAlignedBoundingBox,
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
        let mesh = InstancedMesh::new(
            context,
            &Instances {
                translations,
                rotations: Some(rotations),
                scales: Some(scales),
                ..Default::default()
            },
            &CpuMesh::cylinder(16),
        );
        Self { mesh, aabb }
    }
}

impl<'a> IntoIterator for &'a BoundingBox {
    type Item = &'a dyn Geometry;
    type IntoIter = std::iter::Once<&'a dyn Geometry>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self)
    }
}

impl Geometry for BoundingBox {
    fn aabb(&self) -> AxisAlignedBoundingBox {
        self.aabb
    }

    fn render_with_material(
        &self,
        material: &dyn Material,
        camera: &Camera,
        lights: &[&dyn Light],
    ) {
        self.mesh.render_with_material(material, camera, lights)
    }

    fn render_with_post_material(
        &self,
        material: &dyn PostMaterial,
        camera: &Camera,
        lights: &[&dyn Light],
        color_texture: ColorTexture,
        depth_texture: DepthTexture,
    ) {
        self.mesh
            .render_with_post_material(material, camera, lights, color_texture, depth_texture)
    }
}
