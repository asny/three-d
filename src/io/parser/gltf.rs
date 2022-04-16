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
    ) -> ThreeDResult<(Vec<CpuMesh>, Vec<CpuMaterial>)> {
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
                    &Mat4::identity(),
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
    parent_transform: &Mat4,
    node: &::gltf::Node,
    loaded: &mut Loaded,
    path: &Path,
    buffers: &[::gltf::buffer::Data],
    cpu_meshes: &mut Vec<CpuMesh>,
    cpu_materials: &mut Vec<CpuMaterial>,
) -> ThreeDResult<()> {
    let node_transform = parse_transform(node.transform());
    if node_transform.determinant() == 0.0 {
        return Ok(()); // glTF say that if the scale is all zeroes, the node should be ignored.
    }
    let transform = parent_transform * node_transform;

    if let Some(mesh) = node.mesh() {
        let name: String = mesh
            .name()
            .map(|s| s.to_string())
            .unwrap_or(format!("index {}", mesh.index()));
        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            if let Some(read_positions) = reader.read_positions() {
                let positions = read_positions.map(|p| p.into()).collect();

                let normals = reader
                    .read_normals()
                    .map(|values| values.map(|n| n.into()).collect());

                let tangents = reader
                    .read_tangents()
                    .map(|values| values.map(|t| t.into()).collect());

                let indices = reader.read_indices().map(|values| match values {
                    ::gltf::mesh::util::ReadIndices::U8(iter) => Indices::U8(iter.collect()),
                    ::gltf::mesh::util::ReadIndices::U16(iter) => Indices::U16(iter.collect()),
                    ::gltf::mesh::util::ReadIndices::U32(iter) => Indices::U32(iter.collect()),
                });

                let material = primitive.material();
                let material_name: String = material.name().map(|s| s.to_string()).unwrap_or(
                    material
                        .index()
                        .map(|i| format!("index {}", i))
                        .unwrap_or("default".to_string()),
                );
                let parsed = cpu_materials
                    .iter()
                    .any(|material| material.name == material_name);

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
                    cpu_materials.push(CpuMaterial {
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
                        lighting_model: LightingModel::Cook(
                            NormalDistributionFunction::TrowbridgeReitzGGX,
                            GeometryFunction::SmithSchlickGGX,
                        ),
                    });
                }

                let colors = reader.read_colors(0).map(|values| {
                    values
                        .into_rgba_u8()
                        .map(|c| Color::new(c[0], c[1], c[2], c[3]))
                        .collect()
                });

                let uvs = reader
                    .read_tex_coords(0)
                    .map(|values| values.into_f32().map(|uv| uv.into()).collect());

                let mut cpu_mesh = CpuMesh {
                    name: name.clone(),
                    positions: Positions::F32(positions),
                    normals,
                    tangents,
                    indices,
                    colors,
                    uvs,
                    material_name: Some(material_name),
                };
                if transform != Mat4::identity() {
                    cpu_mesh.transform(&transform)?;
                }
                cpu_meshes.push(cpu_mesh);
            }
        }
    }

    for child in node.children() {
        parse_tree(
            &transform,
            &child,
            loaded,
            path,
            buffers,
            cpu_meshes,
            cpu_materials,
        )?;
    }
    Ok(())
}

fn parse_texture<'a>(
    loaded: &mut Loaded,
    path: &Path,
    buffers: &[::gltf::buffer::Data],
    gltf_texture: ::gltf::texture::Texture,
) -> ThreeDResult<CpuTexture> {
    let gltf_image = gltf_texture.source();
    let gltf_source = gltf_image.source();
    let tex = match gltf_source {
        ::gltf::image::Source::Uri { uri, .. } => loaded.image(path.join(Path::new(uri)))?,
        ::gltf::image::Source::View { view, .. } => {
            if view.stride() != None {
                unimplemented!();
            }
            let buffer = &buffers[view.buffer().index()];
            image_from_bytes(&buffer[view.offset()..view.offset() + view.length()])?
        }
    };
    // TODO: Parse sampling parameters
    Ok(tex)
}

fn parse_transform(transform: gltf::scene::Transform) -> Mat4 {
    let [c0, c1, c2, c3] = transform.matrix();
    Mat4::from_cols(c0.into(), c1.into(), c2.into(), c3.into())
}
