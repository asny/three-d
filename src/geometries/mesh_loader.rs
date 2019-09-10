use crate::geometries::mesh::Mesh;
use crate::buffer::*;
use crate::core::Gl;

impl Mesh {
    pub fn new_from_obj_source(gl: &Gl, source: String) -> Result<Mesh, Error>
    {
        let objs = wavefront_obj::obj::parse(source).unwrap();
        let obj = objs.objects.first().unwrap();

        let mut positions = Vec::new();
        obj.vertices.iter().for_each(|v| {
            positions.push(v.x as f32);
            positions.push(v.y as f32);
            positions.push(v.z as f32);
        });
        let mut normals = vec![0.0f32; obj.vertices.len() * 3];
        let mut indices = Vec::new();
        for shape in obj.geometry.first().unwrap().shapes.iter() {
            match shape.primitive {
                wavefront_obj::obj::Primitive::Triangle(i0, i1, i2) => {
                    indices.push(i0.0 as u32);
                    indices.push(i1.0 as u32);
                    indices.push(i2.0 as u32);

                    let mut normal = obj.normals[i0.2.unwrap()];
                    normals[i0.0 * 3] = normal.x as f32;
                    normals[i0.0 * 3 + 1] = normal.y as f32;
                    normals[i0.0 * 3 + 2] = normal.z as f32;

                    normal = obj.normals[i1.2.unwrap()];
                    normals[i1.0 * 3] = normal.x as f32;
                    normals[i1.0 * 3 + 1] = normal.y as f32;
                    normals[i1.0 * 3 + 2] = normal.z as f32;

                    normal = obj.normals[i2.2.unwrap()];
                    normals[i2.0 * 3] = normal.x as f32;
                    normals[i2.0 * 3 + 1] = normal.y as f32;
                    normals[i2.0 * 3 + 2] = normal.z as f32;
                },
                _ => {}
            }
        }
        Self::new(&gl, &indices, &positions, &normals)
    }
}