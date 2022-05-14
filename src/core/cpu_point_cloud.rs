use crate::core::*;
use rand::Rng;

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
        let mut d = f.debug_struct("CpuPointCloud");
        d.field("name", &self.name);
        d.field("material name", &self.material_name);
        d.field("positions", &self.positions.len());
        d.field("colors", &self.colors.as_ref().map(|v| v.len()));
        d.finish()
    }
}

impl CpuPointCloud {
    ///
    /// Returns a point cloud whose points lie on the corners of an axis aligned unconnected cube with positions in the range `[-1..1]` in all axes.
    ///
    pub fn cube() -> Self {
        let positions = vec![
            vec3(-1.0, -1.0, -1.0),
            vec3(-1.0, -1.0, 1.0),
            vec3(-1.0, 1.0, -1.0),
            vec3(-1.0, 1.0, 1.0),
            vec3(1.0, -1.0, -1.0),
            vec3(1.0, -1.0, 1.0),
            vec3(1.0, 1.0, -1.0),
            vec3(1.0, 1.0, 1.0),
        ];
        let point_cloud = CpuPointCloud {
            positions: Positions::F32(positions),
            ..Default::default()
        };
        point_cloud
    }

    ///
    /// Returns a random, unaligned point cloud with positions in the range `[-1..1]` in all axes.
    ///
    pub fn random(number_of_points: usize) -> Self {
        let mut rng = rand::thread_rng();
        let positions = Positions::F32(
            (0..number_of_points)
                .map(|_| {
                    vec3(
                        rng.gen_range(-10.0..10.0),
                        rng.gen_range(-10.0..10.0),
                        rng.gen_range(-10.0..10.0),
                    )
                })
                .collect(),
        );

        let colors = (0..number_of_points)
            .map(|_| Color {
                r: rng.gen_range(100..255),
                g: rng.gen_range(100..255),
                b: rng.gen_range(100..255),
                a: 0,
            })
            .collect();

        let point_cloud = CpuPointCloud {
            positions,
            colors: Some(colors),
            ..Default::default()
        };
        point_cloud
    }
}
