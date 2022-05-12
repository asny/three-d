use crate::{core::CpuPointCloud, renderer::*};

///
/// Similar to [Model], except it is possible to render many instances of the same model efficiently.
///
pub struct PointCloud<M: Material>(Gm<InstancedMesh, M>);

impl PointCloud<ColorMaterial> {
    ///
    /// Creates a new instanced 3D model with a triangle mesh as geometry and a default [ColorMaterial].
    /// The model is rendered in as many instances as there are [Instance] structs given as input.
    /// The transformation and texture transform in [Instance] are applied to each model instance before they are rendered.
    ///
    pub fn new(context: &Context, cpu_point_cloud: &CpuPointCloud) -> ThreeDResult<Self> {
        Self::new_with_material(context, cpu_point_cloud, ColorMaterial::default())
    }
}

impl<M: Material> PointCloud<M> {
    ///
    /// Creates a new instanced 3D model with a triangle mesh as geometry and the given material.
    /// The model is rendered in as many instances as there are [Instance] structs given as input.
    /// The transformation and texture transform in [Instance] are applied to each model instance before they are rendered.
    ///
    pub fn new_with_material(
        context: &Context,
        cpu_point_cloud: &CpuPointCloud,
        material: M,
    ) -> ThreeDResult<Self> {
        let positions = cpu_point_cloud.positions.to_f32();
        let mut instances = Vec::new();
        for p in positions {
            instances.push(Instance {
                geometry_transform: Mat4::from_translation(p),
                ..Default::default()
            });
        }

        let mut point = CpuMesh::sphere(8);
        point.transform(&Mat4::from_scale(0.05)).unwrap();

        Ok(Self(Gm {
            geometry: InstancedMesh::new(context, &instances, &point)?,
            material,
        }))
    }
}

impl<M: Material> Geometry for PointCloud<M> {
    fn aabb(&self) -> AxisAlignedBoundingBox {
        self.0.geometry.aabb()
    }

    fn render_with_material(
        &self,
        material: &dyn Material,
        camera: &Camera,
        lights: &[&dyn Light],
    ) -> ThreeDResult<()> {
        self.0
            .geometry
            .render_with_material(material, camera, lights)
    }
}

impl<M: Material> Object for PointCloud<M> {
    fn render(&self, camera: &Camera, lights: &[&dyn Light]) -> ThreeDResult<()> {
        self.render_with_material(&self.0.material, camera, lights)
    }

    fn is_transparent(&self) -> bool {
        self.0.material.is_transparent()
    }
}
