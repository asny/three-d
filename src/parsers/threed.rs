use crate::*;

pub struct ThreeD {

}

impl ThreeD {
    pub fn parse(bytes: &[u8]) -> Result<CPUMesh, bincode::Error>
    {
        let decoded = bincode::deserialize::<ThreeDMesh>(bytes)
            .or_else(|_| bincode::deserialize::<ThreeDMeshV1>(bytes).map(|m| ThreeDMesh {
                magic_number: m.magic_number,
                version: 2,
                indices: m.indices,
                positions: m.positions,
                normals: m.normals,
                uvs: vec![]
            }))?;
        if decoded.magic_number != 61 {
            Err(bincode::Error::new(bincode::ErrorKind::Custom("Corrupt file!".to_string())))?;
        }
        Ok(CPUMesh{indices: decoded.indices, positions: decoded.positions, normals: decoded.normals, uvs: decoded.uvs, texture: None })
    }

    pub fn serialize(mesh: &CPUMesh) -> Result<Vec<u8>, bincode::Error>
    {
        Ok(bincode::serialize(&ThreeDMesh {magic_number: 61, version: 2, indices: mesh.indices.to_owned(), positions: mesh.positions.to_owned(), normals: mesh.normals.to_owned(), uvs: mesh.uvs.to_owned()})?)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct ThreeDMesh {
    pub magic_number: u8,
    pub version: u8,
    pub indices: Vec<u32>,
    pub positions: Vec<f32>,
    pub normals: Vec<f32>,
    pub uvs: Vec<f32>
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct ThreeDMeshV1 {
    pub magic_number: u8,
    pub version: u8,
    pub indices: Vec<u32>,
    pub positions: Vec<f32>,
    pub normals: Vec<f32>
}