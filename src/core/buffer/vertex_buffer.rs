use super::Buffer;
use crate::core::*;

///
/// A buffer containing per vertex data, for example positions, normals, uv coordinates or colors.
/// To send this data to a shader, use the [Program::use_vertex_attribute] method.
///
pub struct VertexBuffer<T: BufferDataType> {
    buffer: Buffer<T>,
}

impl<T: BufferDataType> VertexBuffer<T> {
    ///
    /// Creates a new empty vertex buffer.
    ///
    pub fn new(context: &Context) -> Self {
        Self {
            buffer: Buffer::new(context),
        }
    }

    ///
    /// Creates a new vertex buffer and fills it with the given data. The data should be in the same format as specified in the shader.
    /// As an example, if specified as `vec3` in the shader it needs to be specified as an array of `Vector3<T>` where `T` is a primitive type that implements [BufferDataType], for example can be f16 or f32.
    ///
    pub fn new_with_data(context: &Context, data: &[T]) -> Self {
        Self {
            buffer: Buffer::new_with_data(context, data),
        }
    }

    ///
    /// Fills the vertex buffer with the given data. The data should be in the same format as specified in the shader.
    /// As an example, if specified as `vec3` in the shader it needs to be specified as an array of `Vector3<T>` where `T` is a primitive type that implements [BufferDataType], for example can be f16 or f32.
    /// This function will resize the buffer to have the same size as the data, if that is not desired, use [fill_subset](Self::fill_subset) instead.
    ///
    pub fn fill(&mut self, data: &[T]) {
        self.buffer.fill(data);
    }

    ///
    /// Fills the vertex buffer with the given data starting at the given offset.
    /// This will increase the size of the buffer if there's not enough room. Otherwise, the size will remain unchanged.
    ///
    pub fn fill_subset(&mut self, offset: u32, data: &[T]) {
        self.buffer.fill_subset(offset, data);
    }

    ///
    /// The number of values in the buffer.
    ///
    pub fn count(&self) -> u32 {
        self.buffer.attribute_count() * T::size()
    }

    ///
    /// The number of vertex attributes in the buffer.
    ///
    pub fn vertex_count(&self) -> u32 {
        self.buffer.attribute_count()
    }

    pub(in crate::core) fn bind(&self) {
        self.buffer.bind();
    }
}
