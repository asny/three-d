use crate::context::{consts, Context};
use crate::core::Error;

///
/// A buffer containing per vertex data, for example positions, normals, uv coordinates or colors
/// (see also [use_attribute](crate::Program::use_attribute), [use_attribute_vec2](crate::Program::use_attribute_vec2), etc.).
///
#[derive(Clone)]
pub struct VertexBuffer {
    context: Context,
    id: crate::context::Buffer,
    count: usize,
    data_type: u32,
}

impl VertexBuffer {
    pub fn new_with_static_u8(context: &Context, data: &[u8]) -> Result<VertexBuffer, Error> {
        let id = context.create_buffer().unwrap();
        let mut buffer = VertexBuffer {
            context: context.clone(),
            id,
            count: 0,
            data_type: consts::UNSIGNED_BYTE,
        };
        if data.len() > 0 {
            buffer.fill_with_static_u8(data);
        }
        Ok(buffer)
    }

    pub fn fill_with_static_u8(&mut self, data: &[u8]) {
        self.bind();
        self.context
            .buffer_data_u8(consts::ARRAY_BUFFER, data, consts::STATIC_DRAW);
        self.context.unbind_buffer(consts::ARRAY_BUFFER);
        self.count = data.len();
    }

    pub fn new_with_dynamic_u8(context: &Context, data: &[u8]) -> Result<VertexBuffer, Error> {
        let id = context.create_buffer().unwrap();
        let mut buffer = VertexBuffer {
            context: context.clone(),
            id,
            count: 0,
            data_type: consts::UNSIGNED_BYTE,
        };
        if data.len() > 0 {
            buffer.fill_with_dynamic_u8(data);
        }
        Ok(buffer)
    }

    pub fn fill_with_dynamic_u8(&mut self, data: &[u8]) {
        self.bind();
        self.context
            .buffer_data_u8(consts::ARRAY_BUFFER, data, consts::DYNAMIC_DRAW);
        self.context.unbind_buffer(consts::ARRAY_BUFFER);
        self.count = data.len();
    }

    pub fn new_with_static_f32(context: &Context, data: &[f32]) -> Result<VertexBuffer, Error> {
        let id = context.create_buffer().unwrap();
        let mut buffer = VertexBuffer {
            context: context.clone(),
            id,
            count: 0,
            data_type: consts::FLOAT,
        };
        if data.len() > 0 {
            buffer.fill_with_static_f32(data);
        }
        Ok(buffer)
    }

    pub fn fill_with_static_f32(&mut self, data: &[f32]) {
        self.bind();
        self.context
            .buffer_data_f32(consts::ARRAY_BUFFER, data, consts::STATIC_DRAW);
        self.context.unbind_buffer(consts::ARRAY_BUFFER);
        self.count = data.len();
    }

    pub fn new_with_dynamic_f32(context: &Context, data: &[f32]) -> Result<VertexBuffer, Error> {
        let id = context.create_buffer().unwrap();
        let mut buffer = VertexBuffer {
            context: context.clone(),
            id,
            count: 0,
            data_type: consts::FLOAT,
        };
        if data.len() > 0 {
            buffer.fill_with_dynamic_f32(data);
        }
        Ok(buffer)
    }

    pub fn fill_with_dynamic_f32(&mut self, data: &[f32]) {
        self.bind();
        self.context
            .buffer_data_f32(consts::ARRAY_BUFFER, data, consts::DYNAMIC_DRAW);
        self.context.unbind_buffer(consts::ARRAY_BUFFER);
        self.count = data.len();
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub(crate) fn bind(&self) {
        self.context.bind_buffer(consts::ARRAY_BUFFER, &self.id);
    }

    pub(crate) fn data_type(&self) -> u32 {
        self.data_type
    }
}

impl Drop for VertexBuffer {
    fn drop(&mut self) {
        self.context.delete_buffer(&self.id);
    }
}
