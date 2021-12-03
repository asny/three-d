use crate::core::*;
use crate::io::*;
use ::gltf::Gltf;
use std::path::Path;

impl Loaded {
    ///
    /// Deserialize a loaded .gltf file and related .bin resource file and related texture resources or a loaded .glb file into a list of meshes and materials.
    /// It uses the [gltf](https://crates.io/crates/gltf/main.rs) crate.
    ///
    pub fn gltf(
        &mut self,
        path: impl AsRef<Path>,
    ) -> ThreeDResult<(Vec<CPUMesh>, Vec<CPUMaterial>)> {
        let mut cpu_meshes = Vec::new();
        let mut cpu_materials = Vec::new();

        let Gltf { document, mut blob } = Gltf::from_slice(self.get_bytes(path.as_ref())?)?;
        let base_path = path.as_ref().parent().unwrap();
        let mut buffers = Vec::new();
        for buffer in document.buffers() {
            let mut data = match buffer.source() {
                ::gltf::buffer::Source::Uri(uri) => self.remove_bytes(base_path.join(uri))?,
                ::gltf::buffer::Source::Bin => blob.take().ok_or(IOError::GltfMissingData)?,
            };
            if data.len() < buffer.length() {
                Err(IOError::GltfCorruptData)?;
            }
            while data.len() % 4 != 0 {
                data.push(0);
            }
            buffers.push(::gltf::buffer::Data(data));
        }

        for scene in document.scenes() {
            for node in scene.nodes() {
                parse_tree(
                    &node,
                    self,
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
    loaded: &mut Loaded,
    path: &Path,
    buffers: &[::gltf::buffer::Data],
    cpu_meshes: &mut Vec<CPUMesh>,
    cpu_materials: &mut Vec<CPUMaterial>,
) -> ThreeDResult<()> {
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

                let normals = reader
                    .read_normals()
                    .map(|values| values.flatten().collect::<Vec<_>>());

                let tangents = reader
                    .read_tangents()
                    .map(|values| values.flatten().collect::<Vec<_>>());

                let indices = reader.read_indices().map(|values| match values {
                    ::gltf::mesh::util::ReadIndices::U8(iter) => {
                        let mut inds = Vec::new();
                        for value in iter {
                            inds.push(value);
                        }
                        Indices::U8(inds)
                    }
                    ::gltf::mesh::util::ReadIndices::U16(iter) => {
                        let mut inds = Vec::new();
                        for value in iter {
                            inds.push(value);
                        }
                        Indices::U16(inds)
                    }
                    ::gltf::mesh::util::ReadIndices::U32(iter) => {
                        let mut inds = Vec::new();
                        for value in iter {
                            inds.push(value);
                        }
                        Indices::U32(inds)
                    }
                });

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

                if !parsed {
                    let pbr = material.pbr_metallic_roughness();
                    let color = pbr.base_color_factor();
                    let albedo_texture = if let Some(info) = pbr.base_color_texture() {
                        Some(parse_texture(loaded, path, buffers, info.texture())?)
                    } else {
                        None
                    };
                    let metallic_roughness_texture =
                        if let Some(info) = pbr.metallic_roughness_texture() {
                            Some(parse_texture(loaded, path, buffers, info.texture())?)
                        } else {
                            None
                        };
                    let (normal_texture, normal_scale) =
                        if let Some(normal) = material.normal_texture() {
                            (
                                Some(parse_texture(loaded, path, buffers, normal.texture())?),
                                normal.scale(),
                            )
                        } else {
                            (None, 1.0)
                        };
                    let (occlusion_texture, occlusion_strength) =
                        if let Some(occlusion) = material.occlusion_texture() {
                            (
                                Some(parse_texture(loaded, path, buffers, occlusion.texture())?),
                                occlusion.strength(),
                            )
                        } else {
                            (None, 1.0)
                        };
                    let emissive_texture = if let Some(info) = material.emissive_texture() {
                        Some(parse_texture(loaded, path, buffers, info.texture())?)
                    } else {
                        None
                    };
                    cpu_materials.push(CPUMaterial {
                        name: material_name.clone(),
                        albedo: Color::from_rgba_slice(&color),
                        albedo_texture,
                        metallic: pbr.metallic_factor(),
                        roughness: pbr.roughness_factor(),
                        metallic_roughness_texture,
                        normal_texture,
                        normal_scale,
                        occlusion_texture,
                        occlusion_strength,
                        occlusion_metallic_roughness_texture: None,
                        emissive: Color::from_rgb_slice(&material.emissive_factor()),
                        emissive_texture,
                        alpha_cutout: None,
                    });
                }

                let colors = reader.read_colors(0).map(|values| {
                    let mut cols = Vec::new();
                    for value in values.into_rgb_u8() {
                        cols.push(value[0]);
                        cols.push(value[1]);
                        cols.push(value[2]);
                    }
                    cols
                });

                let uvs = reader.read_tex_coords(0).map(|values| {
                    let mut uvs = Vec::new();
                    for value in values.into_f32() {
                        uvs.push(value[0]);
                        uvs.push(value[1]);
                    }
                    uvs
                });

                cpu_meshes.push(CPUMesh {
                    name: name.clone(),
                    positions,
                    normals,
                    tangents,
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

fn parse_texture<'a>(
    loaded: &mut Loaded,
    path: &Path,
    buffers: &[::gltf::buffer::Data],
    gltf_texture: ::gltf::texture::Texture,
) -> ThreeDResult<CPUTexture<u8>> {
    let gltf_image = gltf_texture.source();
    let gltf_source = gltf_image.source();
    let tex = match gltf_source {
        ::gltf::image::Source::Uri { uri, .. } => loaded.image(path.join(Path::new(uri)))?,
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
            image_from_bytes(&bytes)?
        }
    };
    // TODO: Parse sampling parameters
    Ok(tex)
}
