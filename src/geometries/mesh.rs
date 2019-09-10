
use crate::buffer::*;
use crate::core::Gl;

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
