
use crate::buffer::*;
use crate::core::Gl;
use crate::core::types::*;

pub struct Mesh {
    position_buffer: StaticVertexBuffer,
    normal_buffer: StaticVertexBuffer,
    index_buffer: ElementBuffer
}

impl Mesh
{
    pub fn new(gl: &Gl, indices: &[u32], positions: &[f32], normals: &[f32]) -> Result<Mesh, Error>
    {
        let position_buffer = StaticVertexBuffer::new_with_vec3(gl, positions)?;
        let normal_buffer = StaticVertexBuffer::new_with_vec3(gl, normals)?;
        let index_buffer = ElementBuffer::new_with(gl, indices)?;

        Ok(Mesh { index_buffer, position_buffer, normal_buffer})
    }

    pub fn new_with_computed_normals(gl: &Gl, indices: &[u32], positions: &[f32]) -> Result<Mesh, Error>
    {
        let position_buffer = StaticVertexBuffer::new_with_vec3(gl, positions)?;
        let normal_buffer = StaticVertexBuffer::new_with_vec3(gl, &compute_normals(indices, positions))?;
        let index_buffer = ElementBuffer::new_with(gl, indices)?;

        Ok(Mesh { index_buffer, position_buffer, normal_buffer})
    }

    pub fn update_positions(&mut self, positions: &[f32]) -> Result<(), Error>
    {
        self.position_buffer.clear();
        self.position_buffer.add(positions, 3);
        self.position_buffer.send_data();
        Ok(())
    }

    pub fn update_normals(&mut self, normals: &[f32]) -> Result<(), Error>
    {
        self.normal_buffer.clear();
        self.normal_buffer.add(normals, 3);
        self.normal_buffer.send_data();
        Ok(())
    }

    pub fn position_buffer(&self) -> &VertexBuffer
    {
        &self.position_buffer
    }

    pub fn normal_buffer(&self) -> &VertexBuffer
    {
        &self.normal_buffer
    }

    pub fn index_buffer(&self) -> &ElementBuffer
    {
        &self.index_buffer
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