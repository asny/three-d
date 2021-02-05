
use crate::io::*;
use std::collections::HashMap;
use std::path::Path;
use crate::{CPUMesh, CPUMaterial};

pub struct Obj {

}

impl Obj {
    pub fn parse<P: AsRef<Path>>(loaded: &Loaded, path: P) -> Result<(Vec<CPUMesh>, Vec<CPUMaterial>), Error> {
        let obj_bytes = Loader::get(loaded, path.as_ref())?;
        let obj = wavefront_obj::obj::parse(String::from_utf8(obj_bytes.to_owned()).unwrap())?;
        let p = path.as_ref().parent().unwrap();

        // Parse materials
        let mut cpu_materials = Vec::new();
        if let Some(material_library) = obj.material_library {
            let bytes = Loader::get(loaded, p.join(material_library).to_str().unwrap())?.to_owned();
            let materials = wavefront_obj::mtl::parse(String::from_utf8(bytes).unwrap())?.materials;

            for material in materials {
                let color = if material.color_diffuse.r != material.color_diffuse.g || material.color_diffuse.g != material.color_diffuse.b { material.color_diffuse }
                    else if material.color_specular.r != material.color_specular.g || material.color_specular.g != material.color_specular.b { material.color_specular }
                    else if material.color_ambient.r != material.color_ambient.g || material.color_ambient.g != material.color_ambient.b { material.color_ambient }
                    else {material.color_diffuse};
                let diffuse_intensity = (material.color_diffuse.r as f32).max(material.color_diffuse.g as f32).max(material.color_diffuse.b as f32);
                let specular_intensity = (material.color_specular.r as f32).max(material.color_specular.g as f32).max(material.color_specular.b as f32);
                cpu_materials.push(CPUMaterial {
                    name: material.name,
                    color: Some((color.r as f32, color.g as f32, color.b as f32, material.alpha as f32)),
                    diffuse_intensity: Some(diffuse_intensity),
                    specular_intensity: Some(specular_intensity),
                    specular_power: Some(material.specular_coefficient as f32),
                    texture_image: if let Some(path) = material.uv_map.as_ref().map(|texture_name| p.join(texture_name).to_str().unwrap().to_owned())
                    {
                        Some(Loader::get_image(loaded, &path)?)
                    } else {None}
                });
            }
        }

        // Parse meshes
        let mut cpu_meshes = Vec::new();
        for object in obj.objects.iter() { // Objects consisting of several meshes with different materials
            for mesh in object.geometry.iter() { // All meshes with different materials
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
                        let position = object.vertices[i.0];
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

                cpu_meshes.push(CPUMesh {
                    name: object.name.to_string(),
                    material_name: mesh.material_name.clone(),
                    positions,
                    indices: Some(indices),
                    normals: Some(normals),
                    uvs: Some(uvs)
                });
            }
        }
        Ok((cpu_meshes, cpu_materials))
    }
}