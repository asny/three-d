use super::Buffer;
use crate::core::*;

///
/// A buffer containing per vertex data, for example positions, normals, uv coordinates or colors.
/// To send this data to a shader, use the [Program::use_vertex_attribute] method.
///
pub struct VertexBuffer {
    buffer: Buffer,
}

impl VertexBuffer {
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
    pub fn new_with_data<T: BufferDataType>(context: &Context, data: &[T]) -> Self {
        Self {
            buffer: Buffer::new_with_data(context, data),
        }
    }

    ///
    /// Fills the vertex buffer with the given data. The data should be in the same format as specified in the shader.
    /// As an example, if specified as `vec3` in the shader it needs to be specified as an array of `Vector3<T>` where `T` is a primitive type that implements [BufferDataType], for example can be f16 or f32.
    ///
    pub fn fill<T: BufferDataType>(&mut self, data: &[T]) {
        self.buffer.fill(data);
    }

    ///
    /// The number of values in the buffer.
    ///
    pub fn count(&self) -> u32 {
        self.buffer.attribute_count() * self.buffer.data_size
    }

    ///
    /// The number of vertex attributes in the buffer.
    ///
    pub fn vertex_count(&self) -> u32 {
        self.buffer.attribute_count()
    }

    pub(crate) fn bind(&self) {
        self.buffer.bind();
    }

    pub(crate) fn data_type(&self) -> u32 {
        self.buffer.data_type
    }

    pub(crate) fn data_size(&self) -> u32 {
        self.buffer.data_size
    }
}
