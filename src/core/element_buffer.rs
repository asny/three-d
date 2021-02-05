use crate::core::Error;
use crate::context::{Context, consts};

pub struct ElementBuffer {
    context: Context,
    id: crate::context::Buffer,
    count: usize
}

impl ElementBuffer
{
    pub fn new_with_u32(context: &Context, data: &[u32]) -> Result<ElementBuffer, Error>
    {
        let id = context.create_buffer().unwrap();
        let mut buffer = ElementBuffer{ context: context.clone(), id, count: 0 };
        if data.len() > 0 {
            buffer.fill_with_u32(data);
        }
        Ok(buffer)
    }

    pub fn fill_with_u32(&mut self, data: &[u32])
    {
        self.bind();
        self.context.buffer_data_u32(consts::ELEMENT_ARRAY_BUFFER, data, consts::STATIC_DRAW);
        self.context.unbind_buffer(consts::ELEMENT_ARRAY_BUFFER);
        self.count = data.len();
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub(crate) fn bind(&self)
    {
        self.context.bind_buffer(consts::ELEMENT_ARRAY_BUFFER, &self.id);
    }
}

impl Drop for ElementBuffer
{
    fn drop(&mut self)
    {
        self.context.delete_buffer(&self.id);
    }
}

