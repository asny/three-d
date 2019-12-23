use crate::geometries::mesh::Mesh;
use crate::buffer::*;
use crate::core::Gl;
use core::types::*;

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
        let mut normals = vec![0.0f32; positions.len() * 3];
        let mut indices = Vec::new();
        let has_normals = !obj.normals.is_empty();
        for shape in obj.geometry.first().unwrap().shapes.iter() {
            match shape.primitive {
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
        if !has_normals {
            //Compute normals
            println!("{}", indices.len()/3);
            println!("{}", positions.len()/3);
            for face in 0..indices.len()/3 {
                let index0 = indices[face*3] as usize;
                let p0 = vec3(positions[index0*3], positions[index0*3+1], positions[index0*3+2]);
                let index1 = indices[face*3 + 1] as usize;
                let p1 = vec3(positions[index1*3], positions[index1*3+1], positions[index1*3+2]);
                let index2 = indices[face*3 + 2] as usize;
                let p2 = vec3(positions[index2*3], positions[index2*3+1], positions[index2*3+2]);

                let normal = (p1 - p0).cross(p2 - p0).normalize();
                normals[index0*3] += normal.x;
                normals[index0*3+1] += normal.y;
                normals[index0*3+2] += normal.z;
                normals[index1*3] += normal.x;
                normals[index1*3+1] += normal.y;
                normals[index1*3+2] += normal.z;
                normals[index2*3] += normal.x;
                normals[index2*3+1] += normal.y;
                normals[index2*3+2] += normal.z;
            }

            for i in 0..normals.len()/3 {
                let normal = vec3(normals[3*i], normals[3*i+1], normals[3*i+2]).normalize();
                normals[3*i] = normal.x;
                normals[3*i+1] = normal.y;
                normals[3*i+2] = normal.z;
            }
        }

        Self::new(&gl, &indices, &positions, &normals)
    }
}