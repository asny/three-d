use crate::core::*;
use crate::io::*;
use std::path::Path;

impl Loaded {
    ///
    /// Deserialize a loaded .3d file resource (a custom binary format for `three-d`) into a list of meshes and materials.
    ///
    /// # Feature
    /// Only available when the `3d-io` feature is enabled.
    ///
    pub fn three_d<P: AsRef<Path>>(&mut self, path: P) -> Result<(Vec<CPUMesh>, Vec<CPUMaterial>)> {
        let bytes = self.get_bytes(path.as_ref())?;
        let mut decoded = bincode::deserialize::<ThreeDMesh>(bytes)
            .or_else(|_| Self::deserialize_version2(bytes))
            .or_else(|_| Self::deserialize_version1(bytes))?;

        if decoded.meshes.len() == 0 {
            decoded = Self::deserialize_version1(&bytes)?;
        }

        if decoded.magic_number != 61 {
            Err(bincode::Error::new(bincode::ErrorKind::Custom(
                "Corrupt file!".to_string(),
            )))?;
        }

        if decoded.meshes.len() == 0 {
            Err(bincode::Error::new(bincode::ErrorKind::Custom(
                "No mesh data in file!".to_string(),
            )))?;
        }

        let mut cpu_meshes = Vec::new();
        for mesh in decoded.meshes {
            cpu_meshes.push(CPUMesh {
                name: mesh.name,
                material_name: mesh.material_name,
                positions: mesh.positions,
                indices: mesh.indices.map(|i| Indices::U32(i)),
                normals: mesh.normals,
                uvs: mesh.uvs,
                colors: None,
            });
        }

        let mut cpu_materials = Vec::new();
        for material in decoded.materials {
            cpu_materials.push(CPUMaterial {
                name: material.name,
                albedo: material
                    .color
                    .map(|color| Color::new_from_rgba_slice(&[color.0, color.1, color.2, color.3]))
                    .unwrap_or(Color::WHITE),
                albedo_texture: if let Some(filename) = material.texture_path {
                    let texture_path = path
                        .as_ref()
                        .parent()
                        .unwrap_or(&Path::new("./"))
                        .join(filename);
                    Some(self.image(&texture_path)?)
                } else {
                    None
                },
                ..Default::default()
            });
        }
        Ok((cpu_meshes, cpu_materials))
    }

    fn deserialize_version2(bytes: &[u8]) -> Result<ThreeDMesh> {
        Ok(
            bincode::deserialize::<ThreeDMeshV2>(bytes).map(|m| ThreeDMesh {
                magic_number: m.magic_number,
                version: 3,
                meshes: m.meshes,
                materials: m
                    .materials
                    .iter()
                    .map(|mat| ThreeDMaterial {
                        name: mat.name.clone(),
                        color: mat.color,
                        texture_path: mat.texture_path.clone(),
                        metallic: mat.specular_intensity,
                        roughness: mat.specular_power.map(|power| (1.999 / power).sqrt()),
                    })
                    .collect(),
            })?,
        )
    }

    fn deserialize_version1(bytes: &[u8]) -> Result<ThreeDMesh> {
        Ok(
            bincode::deserialize::<ThreeDMeshV1>(bytes).map(|m| ThreeDMesh {
                magic_number: m.magic_number,
                version: 3,
                meshes: vec![ThreeDMeshSubMesh {
                    indices: if m.indices.len() > 0 {
                        Some(m.indices)
                    } else {
                        None
                    },
                    positions: m.positions,
                    normals: if m.normals.len() > 0 {
                        Some(m.normals)
                    } else {
                        None
                    },
                    ..Default::default()
                }],
                materials: vec![],
            })?,
        )
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Saver {
    ///
    /// Saves the given meshes and materials as a .3d file.
    ///
    /// # Feature
    /// Only available when the `3d-io` and `image-io` features are enabled.
    ///
    #[cfg(feature = "image-io")]
    pub fn save_3d_file<P: AsRef<Path>>(
        path: P,
        cpu_meshes: Vec<CPUMesh>,
        cpu_materials: Vec<CPUMaterial>,
    ) -> Result<()> {
        let dir = path.as_ref().parent().unwrap();
        let filename = path.as_ref().file_stem().unwrap().to_str().unwrap();
        for cpu_material in cpu_materials.iter() {
            if let Some(ref cpu_texture) = cpu_material.albedo_texture {
                let number_of_channels =
                    cpu_texture.data.len() as u32 / (cpu_texture.width * cpu_texture.height);
                let format = match number_of_channels {
                    1 => image::ColorType::L8,
                    3 => image::ColorType::Rgb8,
                    4 => image::ColorType::Rgba8,
                    _ => unimplemented!(),
                };
                let tex_path = dir.join(format!("{}_{}.png", filename, cpu_material.name));
                image::save_buffer(
                    tex_path,
                    &cpu_texture.data,
                    cpu_texture.width as u32,
                    cpu_texture.height as u32,
                    format,
                )?;
            }
        }
        let bytes = Self::serialize(filename, cpu_meshes, cpu_materials)?;
        Self::save_file(dir.join(format!("{}.3d", filename)), &bytes)?;
        Ok(())
    }

    fn serialize(
        filename: &str,
        cpu_meshes: Vec<CPUMesh>,
        cpu_materials: Vec<CPUMaterial>,
    ) -> Result<Vec<u8>> {
        let mut meshes = Vec::new();
        for cpu_mesh in cpu_meshes {
            let indices = cpu_mesh.indices.map(|indices| match indices {
                Indices::U8(ind) => ind.iter().map(|i| *i as u32).collect(),
                Indices::U16(ind) => ind.iter().map(|i| *i as u32).collect(),
                Indices::U32(ind) => ind,
            });
            meshes.push(ThreeDMeshSubMesh {
                name: cpu_mesh.name,
                material_name: cpu_mesh.material_name,
                indices,
                positions: cpu_mesh.positions,
                normals: cpu_mesh.normals,
                uvs: cpu_mesh.uvs,
            });
        }

        let mut materials = Vec::new();
        for cpu_material in cpu_materials {
            let texture_path = cpu_material
                .albedo_texture
                .as_ref()
                .map(|_| format!("{}_{}.png", filename, cpu_material.name));
            let albedo = cpu_material.albedo.to_rgba_slice();
            materials.push(ThreeDMaterial {
                name: cpu_material.name,
                texture_path,
                color: Some((albedo[0], albedo[1], albedo[2], albedo[3])),
                metallic: Some(cpu_material.metallic),
                roughness: Some(cpu_material.roughness),
            });
        }

        Ok(bincode::serialize::<ThreeDMesh>(&ThreeDMesh {
            magic_number: 61,
            version: 3,
            meshes,
            materials,
        })?)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct ThreeDMesh {
    pub magic_number: u8,
    pub version: u8,
    pub meshes: Vec<ThreeDMeshSubMesh>,
    pub materials: Vec<ThreeDMaterial>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
struct ThreeDMeshSubMesh {
    pub name: String,
    pub material_name: Option<String>,
    pub indices: Option<Vec<u32>>,
    pub positions: Vec<f32>,
    pub normals: Option<Vec<f32>>,
    pub uvs: Option<Vec<f32>>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
struct ThreeDMaterial {
    pub name: String,
    pub texture_path: Option<String>,
    pub color: Option<(f32, f32, f32, f32)>,
    pub roughness: Option<f32>,
    pub metallic: Option<f32>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct ThreeDMeshV2 {
    pub magic_number: u8,
    pub version: u8,
    pub meshes: Vec<ThreeDMeshSubMesh>,
    pub materials: Vec<ThreeDMaterialV1>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
struct ThreeDMaterialV1 {
    pub name: String,
    pub texture_path: Option<String>,
    pub color: Option<(f32, f32, f32, f32)>,
    pub diffuse_intensity: Option<f32>,
    pub specular_intensity: Option<f32>,
    pub specular_power: Option<f32>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct ThreeDMeshV1 {
    pub magic_number: u8,
    pub version: u8,
    pub indices: Vec<u32>,
    pub positions: Vec<f32>,
    pub normals: Vec<f32>,
}
