
use crate::cpu_mesh::CPUMesh;
use std::collections::HashMap;

pub struct Obj {

}

impl Obj {
    pub fn parse(bytes: &[u8]) -> Result<Vec<CPUMesh>, wavefront_obj::ParseError> {
        let objs = wavefront_obj::obj::parse(String::from_utf8(bytes.to_owned()).unwrap())?;
        let mut cpu_meshes = Vec::new();

        for obj in objs.objects.iter() { // Objects consisting of several meshes with different materials
            println!("Object: {:?}", obj.name);

            for mesh in obj.geometry.iter() { // All meshes with different materials
                println!("mesh");
                let mut positions = Vec::new();
                let mut normals = Vec::new();
                let mut uvs = Vec::new();
                let mut indices = Vec::new();

                let mut map: HashMap<usize, usize> = HashMap::new();

                let mut process = |i: wavefront_obj::obj::VTNIndex| {

                    let mut index = map.get(&i.0).map(|v| *v);

                    let uvw = i.1.map(|tex_index| obj.tex_vertices[tex_index]);
                    let normal = i.2.map(|normal_index| obj.normals[normal_index]);

                    if let Some(ind) = index {
                        if let Some(tex) = uvw {
                            if ((uvs[ind*2] - tex.u as f32) as f32).abs() > std::f32::EPSILON ||
                                ((uvs[ind*2+1] - tex.v as f32) as f32).abs() > std::f32::EPSILON {
                                index = None;
                            }
                        }
                        if let Some(n) = normal {
                            if ((normals[ind*3] - n.x as f32) as f32).abs() > std::f32::EPSILON ||
                                ((normals[ind*3+1] - n.y as f32) as f32).abs() > std::f32::EPSILON ||
                                ((normals[ind*3+2] - n.z as f32) as f32).abs() > std::f32::EPSILON {
                                index = None;
                            }
                        }
                    }

                    if index.is_none() {
                        index = Some(positions.len() / 3);
                        map.insert(i.0, index.unwrap());
                        let position = obj.vertices[i.0];
                        positions.push(position.x as f32);
                        positions.push(position.y as f32);
                        positions.push(position.z as f32);

                        if let Some(tex) = uvw {
                            uvs.push(tex.u as f32);
                            uvs.push(tex.v as f32);
                        }
                        if let Some(n) = normal {
                            normals.push(n.x as f32);
                            normals.push(n.y as f32);
                            normals.push(n.z as f32);
                        }
                    }

                    indices.push(index.unwrap() as u32);
                };
                for shape in mesh.shapes.iter() { // All triangles with same material
                    match shape.primitive {
                        wavefront_obj::obj::Primitive::Triangle(i0, i1, i2) => {
                            process(i0);
                            process(i1);
                            process(i2);
                        },
                        _ => {}
                    }
                }

                println!("indices: {:?}", indices.len());
                println!("pos: {}", positions.len());
                println!("nor: {}", normals.len());
                println!("uvs: {}", uvs.len());
                cpu_meshes.push(CPUMesh {
                    positions,
                    indices: Some(indices),
                    normals: Some(normals),
                    uvs: Some(uvs),
                    .. Default::default()
                });
            }
        }
        Ok(cpu_meshes)
    }


}

pub fn load_obj2(bytes: &[u8]) -> (Vec<u32>, Vec<f32>, Vec<f32>, Vec<f32>) {

    let objs = wavefront_obj::obj::parse(String::from_utf8(bytes.to_owned()).unwrap()).unwrap();
    let mut positions = Vec::new();
    let mut normals_out = Vec::new();
    let mut uvs_out = Vec::new();
    let indices = Vec::new();

    for obj in objs.objects.iter() { // Objects consisting of several meshes with different materials
        println!("Object: {:?}", obj.name);
        if true {
            let _start_index = positions.len()/3;

            for mesh in obj.geometry.iter() { // All meshes with different materials
                    println!("mesh");
                for shape in mesh.shapes.iter() { // All triangles with same material
                    //println!("shape:  {:?}  {:?}", shape.groups, shape.smoothing_groups);
                    match shape.primitive {
                        wavefront_obj::obj::Primitive::Triangle(i0, i1, i2) => {
                            let mut map = |i: wavefront_obj::obj::VTNIndex| {
                                //indices.push((start_index + i.0) as u32);

                                let position = obj.vertices[i.0];
                                positions.push(position.x as f32);
                                positions.push(position.y as f32);
                                positions.push(position.z as f32);

                                if let Some(index) = i.1 {

                                    //println!("{:?} -> {}",index*2, i.0*2);
                                    //uvs_out[start_index + i.0*2] = uvs[index*3];
                                    //uvs_out[start_index + i.0*2 + 1] = uvs[index*3+1];
                                    let uvw = obj.tex_vertices[index];
                                    uvs_out.push(uvw.u as f32);
                                    uvs_out.push(uvw.v as f32);
                                }
                                if let Some(index) = i.2 {

                                    //println!("{:?} -> {}", index*3 , i.0*3);
                                    let normal = obj.normals[index];
                                    normals_out.push(normal.x as f32);
                                    normals_out.push(normal.y as f32);
                                    normals_out.push(normal.z as f32);
                                }
                            };
                            map(i0);
                            map(i1);
                            map(i2);

                        },
                        _ => {}
                    }
                }
            }
        }
    }

    //println!("indices: {:?}", indices);
    //println!("pos: {} - {:?}", positions.len(), positions);
    //println!("nor: {} - {:?}", normals_out.len(), normals_out);
    //println!("uvs: {} - {:?}", uvs_out.len(), uvs_out);

    (indices, positions, normals_out, uvs_out)
}

pub fn load_obj(source: String) -> (Vec<u32>, Vec<f32>, Vec<f32>, Vec<f32>) {

    let objs = wavefront_obj::obj::parse(source).unwrap();
    let mut positions = Vec::new();
    let mut normals_out = Vec::new();
    let mut uvs_out = Vec::new();
    let mut indices = Vec::new();

    for obj in objs.objects.iter() { // Objects consisting of several meshes with different materials
        println!("Object: {:?}", obj.name);
        if true {
            let start_index = positions.len()/3;
            obj.vertices.iter().for_each(|v| {
                positions.push(v.x as f32);
                positions.push(v.y as f32);
                positions.push(v.z as f32);
            });
            let mut normals = Vec::new();
            obj.normals.iter().for_each(|v| {
                normals.push(v.x as f32);
                normals.push(v.y as f32);
                normals.push(v.z as f32);
            });
            let mut uvs = Vec::new();
            obj.tex_vertices.iter().for_each(|v| {
                uvs.push(v.u as f32);
                uvs.push(v.v as f32);
                uvs.push(v.w as f32);
            });

            normals_out = vec![0.0; positions.len()];
            uvs_out = vec![0.0; 2*positions.len()/3];


            for mesh in obj.geometry.iter() { // All meshes with different materials
                println!("mesh");
                for shape in mesh.shapes.iter() { // All triangles with same material
                    match shape.primitive {
                        wavefront_obj::obj::Primitive::Triangle(i0, i1, i2) => {
                            let mut map = |i: wavefront_obj::obj::VTNIndex| {
                                indices.push((start_index + i.0) as u32);

                                if let Some(index) = i.1 {

                                    //println!("{:?} -> {}",index*2, i.0*2);
                                    //uvs_out[start_index + i.0*2] = uvs[index*3];
                                    //uvs_out[start_index + i.0*2 + 1] = uvs[index*3+1];
                                    let uvw = obj.tex_vertices[index];
                                    uvs_out[start_index + i.0*2] = uvw.u as f32;
                                    uvs_out[start_index + i.0*2 + 1] = uvw.v as f32;
                                }
                                if let Some(index) = i.2 {

                                    //println!("{:?} -> {}", index*3 , i.0*3);
                                    normals_out[start_index + i.0*3] = normals[index*3];
                                    normals_out[start_index + i.0*3 + 1] = normals[index*3+1];
                                    normals_out[start_index + i.0*3 + 2] = normals[index*3+2];
                                }
                            };
                            map(i0);
                            map(i1);
                            map(i2);

                        },
                        _ => {}
                    }
                }
            }
        }
    }

    println!("indices: {:?}", indices.len());
    println!("pos: {}", positions.len());
    println!("nor: {}", normals_out.len());
    println!("uvs: {}", uvs_out.len());

    (indices, positions, normals_out, uvs_out)
}