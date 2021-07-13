use crate::definition::*;
use crate::io::*;
use std::collections::HashMap;
use std::path::Path;

type VertexIndex = usize;
type TextureIndex = usize;
type NormalIndex = usize;
type DaeVTNIndex = (VertexIndex, Option<TextureIndex>, Option<NormalIndex>);

impl Loaded {
    ///
    /// Deserialize a loaded .dae file resource
    pub fn dae<P: AsRef<Path>>(
        &mut self,
        path: P,
    ) -> Result<(Vec<CPUMesh>, Vec<CPUMaterial>), IOError> {
        let dae_bytes = self.remove_bytes(path.as_ref())?;
        let dae =
            collada::document::ColladaDocument::from_str(&*String::from_utf8(dae_bytes).unwrap())
                .unwrap();
        let p = path.as_ref().parent().unwrap();

        // Parse materials
        let mut cpu_materials = Vec::new();
        // if let Some(material_library) = dae.material_library {
        //     let bytes = self.remove_bytes(p.join(material_library).to_str().unwrap())?;
        // //     let materials = wavefront_obj::mtl::parse(String::from_utf8(bytes).unwrap())?.materials;

        //     for material in materials {
        //     }
        // }

        // Parse meshes
        let mut cpu_meshes = Vec::new();
        for object in dae.get_obj_set().unwrap().objects.into_iter() {
            // Objects consisting of several meshes with different materials
            for geo in object.geometry.iter() {
                let mut positions = Vec::new();
                let mut normals = Vec::new();
                let mut uvs = Vec::new();
                let mut indices = Vec::new();

                let mut map: HashMap<usize, usize> = HashMap::new();

                let mut process = |i: DaeVTNIndex| {
                    let mut index = map.get(&i.0).map(|v| *v);

                    let uvw = i.1.map(|tex_index| object.tex_vertices[tex_index]);
                    let normal = i.2.map(|normal_index| object.normals[normal_index]);

                    if let Some(ind) = index {
                        if let Some(tex) = uvw {
                            if ((uvs[ind * 2] - tex.x as f32) as f32).abs() > std::f32::EPSILON
                                || ((uvs[ind * 2 + 1] - tex.y as f32) as f32).abs()
                                    > std::f32::EPSILON
                            {
                                index = None;
                            }
                        }
                        if let Some(n) = normal {
                            if ((normals[ind * 3] - n.x as f32) as f32).abs() > std::f32::EPSILON
                                || ((normals[ind * 3 + 1] - n.y as f32) as f32).abs()
                                    > std::f32::EPSILON
                                || ((normals[ind * 3 + 2] - n.z as f32) as f32).abs()
                                    > std::f32::EPSILON
                            {
                                index = None;
                            }
                        }
                    }

                    if index.is_none() {
                        index = Some(positions.len() / 3);
                        map.insert(i.0, index.unwrap());
                        let position = object.vertices[i.0];
                        positions.push(position.x as f32);
                        positions.push(position.y as f32);
                        positions.push(position.z as f32);

                        if let Some(tex) = uvw {
                            uvs.push(tex.x as f32);
                            uvs.push(tex.y as f32);
                        }
                        if let Some(n) = normal {
                            normals.push(n.x as f32);
                            normals.push(n.y as f32);
                            normals.push(n.z as f32);
                        }
                    }

                    indices.push(index.unwrap() as u32);
                };

                for shape in &geo.mesh[..] {
                    match shape {
                        collada::PrimitiveElement::Triangles(tris) => {
                            for (i, tri_vert) in tris.vertices.to_vec().into_iter().enumerate() {
                                let curr_tex_verts = match &tris.tex_vertices {
                                    Some(tex_verts) => Some(tex_verts[i]),
                                    None => None,
                                };
                                let curr_normals = match &tris.normals {
                                    Some(norms) => Some(norms[i]),
                                    None => None,
                                };

                                let dae_vtn0: DaeVTNIndex = (
                                    tri_vert.0,
                                    if let Some(v) = curr_tex_verts {
                                        Some(v.0)
                                    } else {
                                        None
                                    },
                                    if let Some(n) = curr_normals {
                                        Some(n.0)
                                    } else {
                                        None
                                    },
                                );
                                let dae_vtn1: DaeVTNIndex = (
                                    tri_vert.1,
                                    if let Some(v) = curr_tex_verts {
                                        Some(v.1)
                                    } else {
                                        None
                                    },
                                    if let Some(n) = curr_normals {
                                        Some(n.1)
                                    } else {
                                        None
                                    },
                                );
                                let dae_vtn2: DaeVTNIndex = (
                                    tri_vert.2,
                                    if let Some(v) = curr_tex_verts {
                                        Some(v.2)
                                    } else {
                                        None
                                    },
                                    if let Some(n) = curr_normals {
                                        Some(n.2)
                                    } else {
                                        None
                                    },
                                );

                                process(dae_vtn0);
                                process(dae_vtn1);
                                process(dae_vtn2);
                            }
                        }
                        _ => {}
                    }
                }
                cpu_meshes.push(CPUMesh {
                    name: object.name.to_string(),
                    material_name: None,
                    positions,
                    indices: Some(Indices::U32(indices)),
                    normals: Some(normals),
                    uvs: Some(uvs),
                    colors: None,
                });
            }
        }
        Ok((cpu_meshes, cpu_materials))
    }
}
