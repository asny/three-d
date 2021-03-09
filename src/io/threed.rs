use crate::io::*;
use std::path::Path;
use crate::definition::*;

pub struct ThreeD {

}

impl ThreeD {
    pub fn parse<P: AsRef<Path>>(loaded: &Loaded, path: P) -> Result<(Vec<CPUMesh>, Vec<CPUMaterial>), IOError>
    {
        let bytes = Loader::get(loaded, path.as_ref())?;
        let mut decoded = bincode::deserialize::<ThreeDMesh>(bytes)
            .or_else(|_| Self::parse_version1(bytes))?;

        if decoded.meshes.len() == 0 {
            decoded = Self::parse_version1(bytes)?;
        }

        if decoded.magic_number != 61 {
            Err(bincode::Error::new(bincode::ErrorKind::Custom("Corrupt file!".to_string())))?;
        }

        if decoded.meshes.len() == 0 {
            Err(bincode::Error::new(bincode::ErrorKind::Custom("No mesh data in file!".to_string())))?;
        }

        let mut cpu_meshes = Vec::new();
        for mesh in decoded.meshes {
            cpu_meshes.push(CPUMesh {
                name: mesh.name,
                material_name: mesh.material_name,
                positions: mesh.positions,
                indices: mesh.indices,
                normals: mesh.normals,
                uvs: mesh.uvs,
                colors: None
            });
        }

        let mut cpu_materials = Vec::new();
        for material in decoded.materials {
            cpu_materials.push(CPUMaterial {
                name: material.name,
                color: material.color,
                diffuse_intensity: material.diffuse_intensity,
                specular_intensity: material.specular_intensity,
                specular_power: material.specular_power,
                texture_image: if let Some(filename) = material.texture_path {
                    let texture_path = path.as_ref().parent().unwrap_or(&Path::new("./")).join(filename);
                    Some(Loader::get_texture(loaded, &texture_path)?)
                } else {None}
            });
        }
        Ok((cpu_meshes, cpu_materials))
    }

    fn parse_version1(bytes: &[u8]) -> Result<ThreeDMesh, bincode::Error> {
        bincode::deserialize::<ThreeDMeshV1>(bytes).map(|m| ThreeDMesh {
                magic_number: m.magic_number,
                version: 2,
                meshes: vec![ThreeDMeshSubMesh {
                    indices: if m.indices.len() > 0 { Some(m.indices) } else {None},
                    positions: m.positions,
                    normals: if m.normals.len() > 0 { Some(m.normals) } else {None},
                    ..Default::default()
                }],
                materials: vec![]
            })
    }

    pub fn serialize(filename: &str, cpu_meshes: Vec<CPUMesh>, cpu_materials: Vec<CPUMaterial>) -> Result<Vec<u8>, IOError>
    {
        let mut meshes = Vec::new();
        for cpu_mesh in cpu_meshes {
            meshes.push(ThreeDMeshSubMesh {
                    name: cpu_mesh.name,
                    material_name: cpu_mesh.material_name,
                    indices: cpu_mesh.indices,
                    positions: cpu_mesh.positions,
                    normals: cpu_mesh.normals,
                    uvs: cpu_mesh.uvs
                });
        }

        let mut materials = Vec::new();
        for cpu_material in cpu_materials {
            let texture_path = cpu_material.texture_image.as_ref().map(|_| format!("{}_{}.png", filename, cpu_material.name));
            materials.push(ThreeDMaterial {
                    name: cpu_material.name,
                    texture_path,
                    color: cpu_material.color,
                    diffuse_intensity: cpu_material.diffuse_intensity,
                    specular_intensity: cpu_material.specular_intensity,
                    specular_power: cpu_material.specular_power
                });
        }

        Ok(bincode::serialize::<ThreeDMesh>(&ThreeDMesh {
            magic_number: 61,
            version: 2,
            meshes,
            materials
        })?)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct ThreeDMesh {
    pub magic_number: u8,
    pub version: u8,
    pub meshes: Vec<ThreeDMeshSubMesh>,
    pub materials: Vec<ThreeDMaterial>
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
    pub diffuse_intensity: Option<f32>,
    pub specular_intensity: Option<f32>,
    pub specular_power: Option<f32>
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct ThreeDMeshV1 {
    pub magic_number: u8,
    pub version: u8,
    pub indices: Vec<u32>,
    pub positions: Vec<f32>,
    pub normals: Vec<f32>
}