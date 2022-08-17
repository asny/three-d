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
    ///
    /// Creates a new uniform buffer with room for a set of variables of varying length defined by the `sizes` argument.
    /// So for example if you create a uniform buffer with `&[3, 1, 4, 16]` as the `sizes` argument, you will have a uniform buffer that has four variables:
    /// The first with 3 elements (a [Vec3]), the second with 1 element (a `f32`), the third with four elements (a [Vec4]) and the last with 16 elements (a [Mat4]).
    /// The variables are initialized to 0.
    ///
    pub fn new(context: &Context, sizes: &[u32]) -> UniformBuffer {
        let id = unsafe { context.create_buffer().expect("Failed creating buffer") };

        let mut offsets = Vec::new();
        let mut length = 0;
        for size in sizes {
            offsets.push(length);
            length += *size as usize;
        }
        let buffer = UniformBuffer {
            context: context.clone(),
            id,
            offsets,
            data: vec![0.0; length as usize],
        };
        buffer.send();
        buffer
    }

    pub(crate) fn bind(&self, id: u32) {
        unsafe {
            self.context
                .bind_buffer_base(crate::context::UNIFORM_BUFFER, id, Some(self.id))
        };
    }

    ///
    /// Update the values of the variable at the given index with the given data.
    ///
    /// # Panic
    /// Will panic if the index is not in the range `[0-max]` where `max` is the length of the `sizes` argument given at construction.
    /// Will panic if the data length does not match the element count of the variable (defined at construction) at the given index.
    ///
    pub fn update(&mut self, index: u32, data: &[f32]) {
        if let Some((offset, length)) = self.offset_length(index as usize) {
            if data.len() != length {
                panic!(
                    "data for element at index {0} has length {1} but a length of {2} was expected",
                    index,
                    data.len(),
                    length,
                );
            }
            self.data
                .splice(offset..offset + length, data.iter().cloned());
            self.send();
        } else {
            panic!(
                "the index {} is outside the expected range [0, {}]",
                index,
                self.offsets.len() - 1
            );
        }
        //TODO: Send to GPU (contextBufferSubData)
    }

    ///
    /// Returns the values of the variable at the given index if inside the range of variables, otherwise `None`.
    ///
    pub fn get(&self, index: u32) -> Option<&[f32]> {
        self.offset_length(index as usize)
            .map(|(offset, length)| &self.data[offset..offset + length])
    }

    fn offset_length(&self, index: usize) -> Option<(usize, usize)> {
        if index >= self.offsets.len() {
            None
        } else {
            let offset = self.offsets[index];
            let length = if index + 1 == self.offsets.len() {
                self.data.len()
            } else {
                self.offsets[index + 1]
            } - offset;
            Some((offset, length))
        }
    }

    fn send(&self) {
        unsafe {
            self.context
                .bind_buffer(crate::context::UNIFORM_BUFFER, Some(self.id));
            self.context.buffer_data_u8_slice(
                crate::context::UNIFORM_BUFFER,
                to_byte_slice(&self.data),
                crate::context::STATIC_DRAW,
            );
            self.context
                .bind_buffer(crate::context::UNIFORM_BUFFER, None);
        }
    }
}

impl Drop for UniformBuffer {
    fn drop(&mut self) {
        unsafe {
            self.context.delete_buffer(self.id);
        }
    }
}
