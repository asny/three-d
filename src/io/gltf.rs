use crate::definition::*;
use crate::io::*;
use ::gltf::Gltf;
use std::path::Path;

impl<'a> Loaded<'a> {
    pub fn gltf(
        &'a self,
        path: impl AsRef<Path>,
    ) -> Result<(Vec<CPUMesh>, Vec<CPUMaterial>), IOError> {
        let mut cpu_meshes = Vec::new();
        let mut cpu_materials = Vec::new();

        let bytes = self.bytes(path.as_ref())?;
        let gltf = Gltf::from_slice(bytes)?;
        let (_, buffers, _) = ::gltf::import(path.as_ref())?;
        let base_path = path.as_ref().parent().unwrap();
        for scene in gltf.scenes() {
            for node in scene.nodes() {
                parse_tree(
                    &node,
                    &self,
                    &base_path,
                    &buffers,
                    &mut cpu_meshes,
                    &mut cpu_materials,
                )?;
            }
        }
        Ok((cpu_meshes, cpu_materials))
    }
}

fn parse_tree<'a>(
    node: &::gltf::Node,
    loaded: &'a Loaded,
    path: &Path,
    buffers: &[::gltf::buffer::Data],
    cpu_meshes: &mut Vec<CPUMesh>,
    cpu_materials: &mut Vec<CPUMaterial>,
) -> Result<(), IOError> {
    if let Some(mesh) = node.mesh() {
        let name: String = mesh
            .name()
            .map(|s| s.to_string())
            .unwrap_or(format!("index {}", mesh.index()));
        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            if let Some(read_positions) = reader.read_positions() {
                let mut positions = Vec::new();
                for value in read_positions {
                    positions.push(value[0]);
                    positions.push(value[1]);
                    positions.push(value[2]);
                }

                let normals = if let Some(values) = reader.read_normals() {
                    let mut nors = Vec::new();
                    for value in values {
                        nors.push(value[0]);
                        nors.push(value[1]);
                        nors.push(value[2]);
                    }
                    Some(nors)
                } else {
                    None
                };

                let indices = if let Some(values) = reader.read_indices() {
                    let mut inds = Vec::new();
                    for value in values.into_u32() {
                        inds.push(value);
                    }
                    Some(inds)
                } else {
                    None
                };

                let material = primitive.material();
                let material_name: String = material.name().map(|s| s.to_string()).unwrap_or(
                    material
                        .index()
                        .map(|i| format!("index {}", i))
                        .unwrap_or("default".to_string()),
                );
                let mut parsed = false;
                for material in cpu_materials.iter() {
                    if material.name == material_name {
                        parsed = true;
                        break;
                    }
                }

                let mut uv_set = None;
                if !parsed {
                    let pbr = material.pbr_metallic_roughness();
                    let color = pbr.base_color_factor();
                    let mut texture_image = None;
                    if let Some(tex_info) = pbr.base_color_texture() {
                        uv_set = Some(tex_info.tex_coord());
                        let gltf_texture = tex_info.texture();
                        let gltf_image = gltf_texture.source();
                        let gltf_source = gltf_image.source();
                        match gltf_source {
                            ::gltf::image::Source::Uri { uri, .. } => {
                                texture_image = Some(loaded.image(path.join(Path::new(uri)))?);
                            }
                            ::gltf::image::Source::View { view, .. } => {
                                let mut bytes = Vec::with_capacity(view.length());
                                bytes.extend(
                                    (0..view.length())
                                        .map(|i| buffers[view.buffer().index()][view.offset() + i])
                                        .into_iter(),
                                );
                                if view.stride() != None {
                                    unimplemented!();
                                }
                                use image::GenericImageView;
                                let img = image::load_from_memory(&bytes)?;
                                bytes = img.to_bytes();

                                let number_of_channels =
                                    bytes.len() / (img.width() * img.height()) as usize;
                                let format = match number_of_channels {
                                    1 => Ok(Format::R),
                                    2 => Ok(Format::RG),
                                    3 => Ok(Format::RGB),
                                    4 => Ok(Format::RGBA),
                                    _ => Err(IOError::FailedToLoad {
                                        message: format!(
                                            "Could not determine the pixel format for the texture."
                                        ),
                                    }),
                                }?;

                                texture_image = Some(CPUTexture {
                                    data: bytes,
                                    width: img.width() as usize,
                                    height: img.height() as usize,
                                    format,
                                    ..Default::default() // TODO: Parse sampling parameters
                                });
                            }
                        }
                    }
                    cpu_materials.push(CPUMaterial {
                        name: material_name.clone(),
                        color: Some((color[0], color[1], color[2], color[3])),
                        texture_image,
                        diffuse_intensity: Some(1.0),
                        specular_intensity: Some(pbr.metallic_factor()),
                        specular_power: Some(pbr.roughness_factor()),
                    });
                }

                let colors = if let Some(values) = reader.read_colors(0) {
                    let mut cols = Vec::new();
                    for value in values.into_rgb_u8() {
                        cols.push(value[0]);
                        cols.push(value[1]);
                        cols.push(value[2]);
                    }
                    Some(cols)
                } else {
                    None
                };

                let uvs = if let Some(values) = reader.read_tex_coords(uv_set.unwrap_or(0)) {
                    let mut uvs = Vec::new();
                    for value in values.into_f32() {
                        uvs.push(value[0]);
                        uvs.push(value[1]);
                    }
                    Some(uvs)
                } else {
                    None
                };

                cpu_meshes.push(CPUMesh {
                    name: name.clone(),
                    positions,
                    normals,
                    indices,
                    colors,
                    uvs,
                    material_name: Some(material_name),
                });
            }
        }
    }

    for child in node.children() {
        parse_tree(&child, loaded, path, buffers, cpu_meshes, cpu_materials)?;
    }
    Ok(())
}
