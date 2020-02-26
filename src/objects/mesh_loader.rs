use crate::objects::mesh::Mesh;

#[cfg(feature = "3d-io")]
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub(crate) struct IOMesh {
    pub magic_number: u8,
    pub version: u8,
    pub indices: Vec<u32>,
    pub positions: Vec<f32>,
    pub normals: Vec<f32>
}

impl Mesh {

    #[cfg(feature = "3d-io")]
    pub fn new_from_3d(gl: &crate::Gl, bytes: &[u8]) -> Result<Mesh, bincode::Error>
    {
        let decoded: crate::mesh_loader::IOMesh = bincode::deserialize(bytes)?;
        Ok(Self::new("3d".to_string(), &gl, &decoded.indices, &decoded.positions, &decoded.normals).unwrap())
    }

    #[cfg(feature = "obj-io")]
    pub fn new_from_obj_source(gl: &crate::Gl, source: String) -> Result<Vec<Mesh>, crate::buffer::Error>
    {
        let objs = wavefront_obj::obj::parse(source).unwrap();

        let mut objects = Vec::new();
        for obj in objs.objects.iter() { // Objects consisting of several meshes with different materials
            let mut positions = Vec::new();
            obj.vertices.iter().for_each(|v| {
                positions.push(v.x as f32);
                positions.push(v.y as f32);
                positions.push(v.z as f32);
            });
            let mut normals = vec![0.0f32; positions.len() * 3];
            let mut indices = Vec::new();
            let has_normals = !obj.normals.is_empty();

            for mesh in obj.geometry.iter() { // All meshes with different materials
                for primitive in mesh.shapes.iter() { // All triangles with same material
                    match primitive.primitive {
                        wavefront_obj::obj::Primitive::Triangle(i0, i1, i2) => {
                            indices.push(i0.0 as u32);
                            indices.push(i1.0 as u32);
                            indices.push(i2.0 as u32);

                            if has_normals {
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
                            }
                        },
                        _ => {}
                    }
                }
            }
            let mesh = if !has_normals {
                Self::new_with_computed_normals(obj.name.to_owned(), &gl, &indices, &positions).unwrap()
            } else {
                Self::new(obj.name.to_owned(), &gl, &indices, &positions, &normals).unwrap()
            };

            objects.push(mesh);
        }

        Ok(objects)
    }
}