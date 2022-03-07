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
    pub fn new(context: &Context) -> ThreeDResult<Self> {
        Ok(Self {
            buffer: Buffer::new(context)?,
        })
    }

    ///
    /// Creates a new vertex buffer and fills it with the given data. The data should be in the same format as specified in the shader.
    /// As an example, if specified as `vec3` in the shader it needs to be specified as an array of `Vector3<T>` where `T` is a primitive type that implements [BufferDataType], for example can be f16 or f32.
    ///
    pub fn new_with_data(context: &Context, data: &[T]) -> ThreeDResult<Self> {
        Ok(Self {
            buffer: Buffer::new_with_data(context, data)?,
        })
    }

    ///
    /// Creates a new vertex buffer and fills it with the given data which must contain between 1 and 4 contiguous values for each vertex.
    /// Use this method instead of [new_with_dynamic](VertexBuffer::new_with_dynamic)
    /// when you do not expect the data to change often.
    ///
    #[deprecated = "use new_with_data() and specify the data in the same format as in the shader (for example an array of Vec3 instead of f32)"]
    pub fn new_with_static(context: &Context, data: &[T]) -> ThreeDResult<Self> {
        Self::new_with_data(context, data)
    }

    ///
    /// Creates a new vertex buffer and fills it with the given data which must contain between 1 and 4 contiguous values for each vertex.
    /// Use this method instead of [new_with_static](VertexBuffer::new_with_static)
    /// when you expect the data to change often.
    ///
    #[deprecated = "use new_with_data() and specify the data in the same format as in the shader (for example an array of Vec3 instead of f32)"]
    pub fn new_with_dynamic(context: &Context, data: &[T]) -> ThreeDResult<Self> {
        Self::new_with_data(context, data)
    }
}

impl<T: BufferDataType> std::ops::Deref for VertexBuffer<T> {
    type Target = Buffer<T>;

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}
impl<T: BufferDataType> std::ops::DerefMut for VertexBuffer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buffer
    }
}
