
use crate::core::*;

#[derive(Default, Debug)]
pub struct CPUMesh {
    pub name: String,
    pub material_name: Option<String>,
    pub positions: Vec<f32>,
    pub indices: Option<Vec<u32>>,
    pub normals: Option<Vec<f32>>,
    pub uvs: Option<Vec<f32>>
}

impl CPUMesh {
    pub fn sphere(radius: f32) -> Self {
        let x = radius*0.525731112119133606f32;
        let z = radius*0.850650808352039932f32;
        let positions = vec!(
           -x, 0.0, z, x, 0.0, z, -x, 0.0, -z, x, 0.0, -z,
           0.0, z, x, 0.0, z, -x, 0.0, -z, x, 0.0, -z, -x,
           z, x, 0.0, -z, x, 0.0, z, -x, 0.0, -z, -x, 0.0
        );
        let indices = vec!(
           0,1,4, 0,4,9, 9,4,5, 4,8,5, 4,1,8,
           8,1,10, 8,10,3, 5,8,3, 5,3,2, 2,3,7,
           7,3,10, 7,10,6, 7,6,11, 11,6,0, 0,6,1,
           6,10,1, 9,11,0, 9,2,11, 9,5,2, 7,11,2
        );
        let normals = Some(compute_normals_with_indices(&indices, &positions));
        CPUMesh {name: "sphere".to_string(), indices: Some(indices), positions, normals, ..Default::default() }
    }

    pub fn compute_normals(&mut self) {
        if let Some(ref ind) = self.indices {
            self.normals = Some(compute_normals_with_indices(ind, &self.positions));
        } else {
            self.normals = Some(compute_normals(&self.positions));
        }
    }

    pub fn compute_aabb(&self) -> AxisAlignedBoundingBox {
        AxisAlignedBoundingBox::new_from_positions(&self.positions)
    }
}

fn compute_normals_with_indices(indices: &[u32], positions: &[f32]) -> Vec<f32> {
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


fn compute_normals(positions: &[f32]) -> Vec<f32> {
    let mut normals = vec![0.0f32; positions.len()];
    for face in 0..positions.len()/9 {
        let index0 = face*3 as usize;
        let p0 = vec3(positions[index0*3], positions[index0*3+1], positions[index0*3+2]);
        let index1 = face*3 + 1 as usize;
        let p1 = vec3(positions[index1*3], positions[index1*3+1], positions[index1*3+2]);
        let index2 = face*3 + 2 as usize;
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