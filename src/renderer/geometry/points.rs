use crate::core::*;
use crate::renderer::*;

///
/// Wraps [InstancedMesh] to render many instances of the same point mesh (Sphere) efficiently.
///
pub struct PointCloud(InstancedMesh);

impl PointCloud {
    ///
    /// Creates a new PointCloud from the given [CpuPointCloud].
    /// All data in the [CpuPointCloud] is transfered to the GPU, so make sure to remove all unnecessary data from the [CpuPointCloud] before calling this method.
    ///
    pub fn new(context: &Context, cpu_point_cloud: CpuPointCloud) -> ThreeDResult<Self> {
        let instances = Instances {
            translations: cpu_point_cloud.positions.to_f32(),
            colors: cpu_point_cloud.colors,
            ..Default::default()
        };

        let mut point = CpuMesh::cube();
        point.transform(&Mat4::from_scale(0.001))?;

        Ok(Self(InstancedMesh::new(context, &instances, &point)?))
    }
}

impl Geometry for PointCloud {
    fn aabb(&self) -> AxisAlignedBoundingBox {
        self.0.aabb()
    }

    fn render_with_material(
        &self,
        material: &dyn Material,
        camera: &Camera,
        lights: &[&dyn Light],
    ) -> ThreeDResult<()> {
        self.0.render_with_material(material, camera, lights)
    }
}
