use crate::core::Error;
use crate::context::{Context, consts};

pub struct VertexBuffer {
    gl: Context,
    id: crate::context::Buffer,
    count: usize
}

impl VertexBuffer
{
    pub fn new_with_static_f32(gl: &Context, data: &[f32]) -> Result<VertexBuffer, Error>
    {
        let id = gl.create_buffer().unwrap();
        let mut buffer = VertexBuffer { gl: gl.clone(), id, count: 0 };
        if data.len() > 0 {
            buffer.fill_with_static_f32(data);
        }
        Ok(buffer)
    }

    pub fn fill_with_static_f32(&mut self, data: &[f32])
    {
        self.bind();
        self.gl.buffer_data_f32(consts::ARRAY_BUFFER, data, consts::STATIC_DRAW);
        self.gl.unbind_buffer(consts::ARRAY_BUFFER);
        self.count = data.len();
    }

    pub fn new_with_dynamic_f32(gl: &Context, data: &[f32]) -> Result<VertexBuffer, Error>
    {
        let id = gl.create_buffer().unwrap();
        let mut buffer = VertexBuffer { gl: gl.clone(), id, count: 0 };
        if data.len() > 0 {
            buffer.fill_with_dynamic_f32(data);
        }
        Ok(buffer)
    }

    pub fn fill_with_dynamic_f32(&mut self, data: &[f32])
    {
        self.bind();
        self.gl.buffer_data_f32(consts::ARRAY_BUFFER, data, consts::DYNAMIC_DRAW);
        self.gl.unbind_buffer(consts::ARRAY_BUFFER);
        self.count = data.len();
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub(crate) fn bind(&self)
    {
        self.gl.bind_buffer(consts::ARRAY_BUFFER, &self.id);
    }
}

impl Drop for VertexBuffer
{
    fn drop(&mut self)
    {
        self.gl.delete_buffer(&self.id);
    }
}