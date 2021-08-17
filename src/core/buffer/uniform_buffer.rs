use crate::context::{consts, Context};
use crate::core::*;

///
/// A buffer for transferring a set of uniform variables to the shader program
/// (see also [use_uniform_block](crate::core::Program::use_uniform_block)).
///
pub struct UniformBuffer {
    context: Context,
    id: crate::context::Buffer,
    offsets: Vec<usize>,
    data: Vec<f32>,
}

impl UniformBuffer {
    pub fn new(context: &Context, sizes: &[u32]) -> Result<UniformBuffer> {
        let id = context.create_buffer().unwrap();

        let mut offsets = Vec::new();
        let mut length = 0;
        for size in sizes {
            offsets.push(length);
            length += *size as usize;
        }
        Ok(UniformBuffer {
            context: context.clone(),
            id,
            offsets,
            data: vec![0.0; length as usize],
        })
    }

    pub(crate) fn bind(&self, id: u32) {
        self.context
            .bind_buffer_base(consts::UNIFORM_BUFFER, id, &self.id);
    }

    pub fn update(&mut self, index: u32, data: &[f32]) -> Result<()> {
        let (offset, length) = self.offset_length(index as usize)?;
        if data.len() != length {
            Err(CoreError::InvalidUniformBufferElementLength(
                index,
                data.len(),
                length,
            ))?;
        }
        self.data
            .splice(offset..offset + length, data.iter().cloned());
        self.send();
        //TODO: Send to GPU (contextBufferSubData)
        Ok(())
    }

    pub fn get(&self, index: u32) -> Result<&[f32]> {
        let (offset, length) = self.offset_length(index as usize)?;
        Ok(&self.data[offset..offset + length])
    }

    fn offset_length(&self, index: usize) -> Result<(usize, usize)> {
        if index >= self.offsets.len() {
            Err(CoreError::IndexOutOfRange(index, self.offsets.len() - 1))?;
        }
        let offset = self.offsets[index];
        let length = if index + 1 == self.offsets.len() {
            self.data.len()
        } else {
            self.offsets[index + 1]
        } - offset;
        Ok((offset, length))
    }

    fn send(&self) {
        self.context.bind_buffer(consts::UNIFORM_BUFFER, &self.id);
        self.context
            .buffer_data_f32(consts::UNIFORM_BUFFER, &self.data, consts::STATIC_DRAW);
        self.context.unbind_buffer(consts::UNIFORM_BUFFER);
    }
}

impl Drop for UniformBuffer {
    fn drop(&mut self) {
        self.context.delete_buffer(&self.id);
    }
}
