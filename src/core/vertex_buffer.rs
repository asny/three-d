use crate::core::Error;
use crate::context::{Context, consts};

pub struct VertexBuffer {
    context: Context,
    id: crate::context::Buffer,
    count: usize
}

impl VertexBuffer
{
    pub fn new_with_static_f32(context: &Context, data: &[f32]) -> Result<VertexBuffer, Error>
    {
        let id = context.create_buffer().unwrap();
        let mut buffer = VertexBuffer { context: context.clone(), id, count: 0 };
        if data.len() > 0 {
            buffer.fill_with_static_f32(data);
        }
        Ok(buffer)
    }

    pub fn fill_with_static_f32(&mut self, data: &[f32])
    {
        self.bind();
        self.context.buffer_data_f32(consts::ARRAY_BUFFER, data, consts::STATIC_DRAW);
        self.context.unbind_buffer(consts::ARRAY_BUFFER);
        self.count = data.len();
    }

    pub fn new_with_dynamic_f32(context: &Context, data: &[f32]) -> Result<VertexBuffer, Error>
    {
        let id = context.create_buffer().unwrap();
        let mut buffer = VertexBuffer { context: context.clone(), id, count: 0 };
        if data.len() > 0 {
            buffer.fill_with_dynamic_f32(data);
        }
        Ok(buffer)
    }

    pub fn fill_with_dynamic_f32(&mut self, data: &[f32])
    {
        self.bind();
        self.context.buffer_data_f32(consts::ARRAY_BUFFER, data, consts::DYNAMIC_DRAW);
        self.context.unbind_buffer(consts::ARRAY_BUFFER);
        self.count = data.len();
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub(crate) fn bind(&self)
    {
        self.context.bind_buffer(consts::ARRAY_BUFFER, &self.id);
    }
}

impl Drop for VertexBuffer
{
    fn drop(&mut self)
    {
        self.context.delete_buffer(&self.id);
    }
}