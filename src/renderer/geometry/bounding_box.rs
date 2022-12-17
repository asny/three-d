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

        let mesh = InstancedMesh::new(
            context,
            &Instances {
                transformations: (0..12)
                    .map(|i| {
                        Mat4::from_translation(translations[i])
                            * match i {
                                0..=3 => Mat4::from_nonuniform_scale(size.x, thickness, thickness),
                                4..=7 => {
                                    Mat4::from_angle_z(degrees(90.0))
                                        * Mat4::from_nonuniform_scale(size.y, thickness, thickness)
                                }
                                8..=11 => {
                                    Mat4::from_angle_y(degrees(-90.0))
                                        * Mat4::from_nonuniform_scale(size.z, thickness, thickness)
                                }
                                _ => unreachable!(),
                            }
                    })
                    .collect(),
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
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) {
        self.mesh
            .render_with_post_material(material, camera, lights, color_texture, depth_texture)
    }
}
