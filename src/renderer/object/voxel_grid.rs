use super::*;

pub use three_d_asset::VoxelGrid as CpuVoxelGrid;

///
/// A voxel grid inside a cube with a [material] type specified by the generic parameter.
///
pub struct VoxelGrid<M: Material>(Gm<Mesh, M>);

impl<M: Material + FromCpuVoxelGrid> VoxelGrid<M> {
    ///
    /// Constructs a [VoxelGrid] from a [CpuVoxelGrid], ie. constructs a [Gm] with a cube [Mesh] as geometry and
    /// a [material] type specified by the generic parameter which implement [FromCpuVoxelGrid].
    ///
    pub fn new(context: &Context, cpu_voxel_grid: &CpuVoxelGrid) -> Self {
        let mut cube = CpuMesh::cube();
        cube.transform(&Mat4::from_nonuniform_scale(
            0.5 * cpu_voxel_grid.size.x,
            0.5 * cpu_voxel_grid.size.y,
            0.5 * cpu_voxel_grid.size.z,
        ))
        .expect("Invalid size for VoxelGrid");
        let gm = Gm::new(
            Mesh::new(&context, &cube),
            M::from_cpu_voxel_grid(context, cpu_voxel_grid),
        );
        Self(gm)
    }
}

impl<'a, M: Material> IntoIterator for &'a VoxelGrid<M> {
    type Item = &'a dyn Object;
    type IntoIter = std::iter::Once<&'a dyn Object>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self)
    }
}

impl<M: Material> std::ops::Deref for VoxelGrid<M> {
    type Target = Gm<Mesh, M>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<M: Material> std::ops::DerefMut for VoxelGrid<M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<M: Material> Geometry for VoxelGrid<M> {
    fn aabb(&self) -> AxisAlignedBoundingBox {
        self.0.aabb()
    }

    fn render_with_material(
        &self,
        material: &dyn Material,
        camera: &Camera,
        lights: &[&dyn Light],
    ) {
        self.0.render_with_material(material, camera, lights)
    }

    fn render_with_post_material(
        &self,
        material: &dyn PostMaterial,
        camera: &Camera,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) {
        self.0
            .render_with_post_material(material, camera, lights, color_texture, depth_texture)
    }
}

impl<M: Material> Object for VoxelGrid<M> {
    fn render(&self, camera: &Camera, lights: &[&dyn Light]) {
        self.0.render(camera, lights)
    }

    fn material_type(&self) -> MaterialType {
        self.0.material_type()
    }
}
