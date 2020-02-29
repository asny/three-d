
#[cfg(feature = "3d-io")]
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct CPUMesh {
    pub magic_number: u8,
    pub version: u8,
    pub indices: Vec<u32>,
    pub positions: Vec<f32>,
    pub normals: Vec<f32>
}

#[cfg(feature = "3d-io")]
impl CPUMesh {
    pub fn new(bytes: &[u8]) -> Result<CPUMesh, bincode::Error>
    {
        let decoded: CPUMesh = bincode::deserialize(bytes)?;
        if decoded.magic_number != 61 {
            Err(bincode::Error::new(bincode::ErrorKind::Custom("Corrupt file!".to_string())))?;
        }
        Ok(decoded)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, bincode::Error>
    {
        bincode::serialize(self)
    }

    pub fn to_mesh(&self, gl: &crate::Gl) -> Result<crate::Mesh, crate::mesh::Error>
    {
        crate::Mesh::new( &gl, &self.indices, &self.positions, &self.normals)
    }
}