use crate::core::*;
use crate::io::*;
use std::collections::HashMap;
use std::path::Path;

impl Loaded {
    ///
    /// Deserialize a loaded .obj file resource and .mtl material file resource (if present) into a list of meshes and materials.
    /// It uses the [wavefront-obj](https://crates.io/crates/wavefront_obj/main.rs) crate.
    ///
    pub fn obj<P: AsRef<Path>>(&mut self, path: P) -> Result<(Vec<CPUMesh>, Vec<CPUMaterial>)> {
        let obj_bytes = self.remove_bytes(path.as_ref())?;
        let obj = wavefront_obj::obj::parse(String::from_utf8(obj_bytes).unwrap())?;
        let p = path.as_ref().parent().unwrap();

        // Parse materials
        let mut cpu_materials = Vec::new();
        if let Some(material_library) = obj.material_library {
            let bytes = self.remove_bytes(p.join(material_library).to_str().unwrap())?;
            let materials = wavefront_obj::mtl::parse(String::from_utf8(bytes).unwrap())?.materials;

            for material in materials {
                let color = if material.color_diffuse.r != material.color_diffuse.g
                    || material.color_diffuse.g != material.color_diffuse.b
                {
                    material.color_diffuse
                } else if material.color_specular.r != material.color_specular.g
                    || material.color_specular.g != material.color_specular.b
                {
                    material.color_specular
                } else if material.color_ambient.r != material.color_ambient.g
                    || material.color_ambient.g != material.color_ambient.b
                {
                    material.color_ambient
                } else {
                    material.color_diffuse
                };

                let normal_texture = if let Some(ref texture_name) = material.bump_map {
                    Some(self.image(p.join(texture_name))?)
                } else {
                    None
                };
                let albedo_texture = if let Some(ref texture_name) = material.diffuse_map {
                    Some(self.image(p.join(texture_name))?)
                } else {
                    None
                };

                cpu_materials.push(CPUMaterial {
                    name: material.name,
                    albedo: Color::new_from_rgba_slice(&[
                        color.r as f32,
                        color.g as f32,
                        color.b as f32,
                        material.alpha as f32,
                    ]),
                    albedo_texture,
                    metallic: ((material.color_specular.r
                        + material.color_specular.g
                        + material.color_specular.b)
                        / 3.0) as f32,
                    roughness: if material.specular_coefficient > 0.1 {
                        ((1.999 / material.specular_coefficient).sqrt() as f32).min(1.0)
                    } else {
                        1.0
                    },
                    normal_texture,
                    ..Default::default()
                });
            }
        }

        // Parse meshes
        let mut cpu_meshes = Vec::new();
        for object in obj.objects.iter() {
            // Objects consisting of several meshes with different materials
            for mesh in object.geometry.iter() {
                // All meshes with different materials
                let mut positions = Vec::new();
                let mut normals = Vec::new();
                let mut uvs = Vec::new();
                let mut indices = Vec::new();

                let mut map: HashMap<usize, usize> = HashMap::new();

                let mut process = |i: wavefront_obj::obj::VTNIndex| {
                    let mut index = map.get(&i.0).map(|v| *v);

                    let uvw = i.1.map(|tex_index| object.tex_vertices[tex_index]);
                    let normal = i.2.map(|normal_index| object.normals[normal_index]);

                    if let Some(ind) = index {
                        if let Some(tex) = uvw {
                            if ((uvs[ind * 2] - tex.u as f32) as f32).abs() > std::f32::EPSILON
                                || ((uvs[ind * 2 + 1] - tex.v as f32) as f32).abs()
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
                            uvs.push(tex.u as f32);
                            uvs.push(1.0 - tex.v as f32);
                        }
                        if let Some(n) = normal {
                            normals.push(n.x as f32);
                            normals.push(n.y as f32);
                            normals.push(n.z as f32);
                        }
                    }

                    indices.push(index.unwrap() as u32);
                };
                for shape in mesh.shapes.iter() {
                    // All triangles with same material
                    match shape.primitive {
                        wavefront_obj::obj::Primitive::Triangle(i0, i1, i2) => {
                            process(i0);
                            process(i1);
                            process(i2);
                        }
                        _ => {}
                    }
                }

                cpu_meshes.push(CPUMesh {
                    name: object.name.to_string(),
                    material_name: mesh.material_name.clone(),
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
