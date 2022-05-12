use crate::core::*;

///
/// A CPU-side version of a point cloud.
/// Can be constructed manually or loaded via [io](crate::io)
/// or via the utility functions for generating simple point clouds.
///
#[derive(Default)]
pub struct CpuPointCloud {
    /// Name.
    pub name: String,
    /// Name of the associated material, use this to match with [CpuMaterial::name].
    pub material_name: Option<String>,
    /// The positions of the vertices.
    /// If there is no indices associated with this mesh, three contiguous positions defines a triangle, in that case, the length must be divisable by 3.
    pub positions: Positions,
    /// The colors of the vertices.
    /// The colors are assumed to be in linear space.
    pub colors: Option<Vec<Color>>,
}

impl std::fmt::Debug for CpuPointCloud {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("CpuMesh");
        d.field("name", &self.name);
        d.field("material name", &self.material_name);
        d.field("positions", &self.positions.len());
        d.field("colors", &self.colors.as_ref().map(|v| v.len()));
        d.finish()
    }
}

impl CpuPointCloud {
    ///
    /// Returns an axis aligned unconnected cube mesh with positions in the range `[-1..1]` in all axes.
    ///
    pub fn cube() -> Self {
        let positions = vec![
            vec3(1.0, 1.0, -1.0),
            vec3(-1.0, 1.0, -1.0),
            vec3(1.0, 1.0, 1.0),
            vec3(-1.0, 1.0, 1.0),
            vec3(1.0, 1.0, 1.0),
            vec3(-1.0, 1.0, -1.0),
            vec3(-1.0, -1.0, -1.0),
            vec3(1.0, -1.0, -1.0),
            vec3(1.0, -1.0, 1.0),
            vec3(1.0, -1.0, 1.0),
            vec3(-1.0, -1.0, 1.0),
            vec3(-1.0, -1.0, -1.0),
            vec3(1.0, -1.0, -1.0),
            vec3(-1.0, -1.0, -1.0),
            vec3(1.0, 1.0, -1.0),
            vec3(-1.0, 1.0, -1.0),
            vec3(1.0, 1.0, -1.0),
            vec3(-1.0, -1.0, -1.0),
            vec3(-1.0, -1.0, 1.0),
            vec3(1.0, -1.0, 1.0),
            vec3(1.0, 1.0, 1.0),
            vec3(1.0, 1.0, 1.0),
            vec3(-1.0, 1.0, 1.0),
            vec3(-1.0, -1.0, 1.0),
            vec3(1.0, -1.0, -1.0),
            vec3(1.0, 1.0, -1.0),
            vec3(1.0, 1.0, 1.0),
            vec3(1.0, 1.0, 1.0),
            vec3(1.0, -1.0, 1.0),
            vec3(1.0, -1.0, -1.0),
            vec3(-1.0, 1.0, -1.0),
            vec3(-1.0, -1.0, -1.0),
            vec3(-1.0, 1.0, 1.0),
            vec3(-1.0, -1.0, 1.0),
            vec3(-1.0, 1.0, 1.0),
            vec3(-1.0, -1.0, -1.0),
        ];
        let point_cloud = CpuPointCloud {
            positions: Positions::F32(positions),
            ..Default::default()
        };
        point_cloud
    }
}
