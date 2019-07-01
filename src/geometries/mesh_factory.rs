
use crate::geometries::mesh::Mesh;
use crate::buffer::*;
use crate::core::Gl;

impl Mesh {

    pub fn plane(gl: &Gl) -> Result<Mesh, Error>
    {

        let plane_positions: Vec<f32> = vec![
            -1.0, 0.0, -1.0,
            1.0, 0.0, -1.0,
            1.0, 0.0, 1.0,
            -1.0, 0.0, 1.0
        ];
        let plane_normals: Vec<f32> = vec![
            0.0, 1.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 1.0, 0.0
        ];
        let plane_indices: Vec<u32> = vec![
            0, 2, 1,
            0, 3, 2,
        ];
        let mesh = Mesh::new(&gl, &plane_indices, &plane_positions, &plane_normals)?;
        Ok(mesh)
    }
}