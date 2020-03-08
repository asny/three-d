
use crate::*;

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
    pub fn new(indices: &[u32], positions: &[f32], normals: &[f32]) -> Result<Self, objects::Error>
    {
        Ok(CPUMesh {magic_number: 61, version: 1, indices: indices.to_owned(), positions: positions.to_owned(), normals: normals.to_owned()})
    }

    pub fn new_with_computed_normals(indices: &[u32], positions: &[f32]) -> Result<Self, objects::Error>
    {
        Self::new(indices, positions, &compute_normals(indices, positions))
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<CPUMesh, bincode::Error>
    {
        let decoded: CPUMesh = bincode::deserialize(bytes)?;
        if decoded.magic_number != 61 {
            Err(bincode::Error::new(bincode::ErrorKind::Custom("Corrupt file!".to_string())))?;
        }
        Ok(decoded)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, objects::Error>
    {
        Ok(bincode::serialize(self)?)
    }

    pub fn to_mesh(&self, gl: &crate::Gl) -> Result<Mesh, objects::Error>
    {
        Ok(crate::Mesh::new( &gl, &self.indices, &self.positions, &self.normals)?)
    }
}

fn compute_normals(indices: &[u32], positions: &[f32]) -> Vec<f32> {
    let mut normals = vec![0.0f32; positions.len() * 3];
    for face in 0..indices.len()/3 {
        let index0 = indices[face*3] as usize;
        let p0 = vec3(positions[index0*3], positions[index0*3+1], positions[index0*3+2]);
        let index1 = indices[face*3 + 1] as usize;
        let p1 = vec3(positions[index1*3], positions[index1*3+1], positions[index1*3+2]);
        let index2 = indices[face*3 + 2] as usize;
        let p2 = vec3(positions[index2*3], positions[index2*3+1], positions[index2*3+2]);

        let normal = (p1 - p0).cross(p2 - p0);
        normals[index0*3] += normal.x;
        normals[index0*3+1] += normal.y;
        normals[index0*3+2] += normal.z;
        normals[index1*3] += normal.x;
        normals[index1*3+1] += normal.y;
        normals[index1*3+2] += normal.z;
        normals[index2*3] += normal.x;
        normals[index2*3+1] += normal.y;
        normals[index2*3+2] += normal.z;
    }

    for i in 0..normals.len()/3 {
        let normal = vec3(normals[3*i], normals[3*i+1], normals[3*i+2]).normalize();
        normals[3*i] = normal.x;
        normals[3*i+1] = normal.y;
        normals[3*i+2] = normal.z;
    }
    normals
}