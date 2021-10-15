use crate::context::{consts, DataType};
use crate::core::*;

/// The basic data type used for each element in a [VertexBuffer].
pub trait VertexBufferDataType:
    Default + std::fmt::Debug + Clone + internal::BufferDataTypeExtension
{
}
impl VertexBufferDataType for u8 {}
impl VertexBufferDataType for u16 {}
impl VertexBufferDataType for f32 {}

///
/// A buffer containing per vertex data, for example positions, normals, uv coordinates or colors.
/// Can send between 1 and 4 values of [InstanceBufferDataType] to a shader program for each vertex.
/// Bind this using the [Program::use_attribute], [Program::use_attribute_vec2], etc. functionality.
///
pub struct VertexBuffer {
    context: Context,
    id: crate::context::Buffer,
    count: usize,
    data_type: DataType,
}

impl VertexBuffer {
    ///
    /// Creates a new empty vertex buffer.
    ///
    pub fn new(context: &Context) -> ThreeDResult<VertexBuffer> {
        Ok(VertexBuffer {
            context: context.clone(),
            id: context.create_buffer().unwrap(),
            count: 0,
            data_type: DataType::Float,
        })
    }

    ///
    /// Creates a new vertex buffer and fills it with the given data which must contain between 1 and 4 contiguous values for each vertex.
    /// Use this method instead of [new_with_dynamic](VertexBuffer::new_with_dynamic)
    /// when you do not expect the data to change often.
    ///
    pub fn new_with_static<T: VertexBufferDataType>(
        context: &Context,
        data: &[T],
    ) -> ThreeDResult<VertexBuffer> {
        let mut buffer = Self::new(context)?;
        if data.len() > 0 {
            buffer.fill_with_static(data);
        }
        Ok(buffer)
    }

    ///
    /// Fills the vertex buffer with the given data which must contain between 1 and 4 contiguous values for each vertex.
    /// Use this method instead of [fill_with_dynamic](VertexBuffer::fill_with_dynamic)
    /// when you do not expect the data to change often.
    ///
    pub fn fill_with_static<T: VertexBufferDataType>(&mut self, data: &[T]) {
        self.bind();
        T::buffer_data(
            &self.context,
            consts::ARRAY_BUFFER,
            data,
            consts::STATIC_DRAW,
        );
        self.data_type = T::data_type();
        self.context.unbind_buffer(consts::ARRAY_BUFFER);
        self.count = data.len();
    }

    ///
    /// Creates a new vertex buffer and fills it with the given data which must contain between 1 and 4 contiguous values for each vertex.
    /// Use this method instead of [new_with_static](VertexBuffer::new_with_static)
    /// when you expect the data to change often.
    ///
    pub fn new_with_dynamic<T: VertexBufferDataType>(
        context: &Context,
        data: &[T],
    ) -> ThreeDResult<VertexBuffer> {
        let mut buffer = Self::new(context).unwrap();
        if data.len() > 0 {
            buffer.fill_with_dynamic(data);
        }
        Ok(buffer)
    }

    ///
    /// Fills the vertex buffer with the given data which must contain between 1 and 4 contiguous values for each vertex.
    /// Use this method instead of [fill_with_static](VertexBuffer::fill_with_static)
    /// when you expect the data to change often.
    ///
    pub fn fill_with_dynamic<T: VertexBufferDataType>(&mut self, data: &[T]) {
        self.bind();
        T::buffer_data(
            &self.context,
            consts::ARRAY_BUFFER,
            data,
            consts::DYNAMIC_DRAW,
        );
        self.data_type = T::data_type();
        self.context.unbind_buffer(consts::ARRAY_BUFFER);
        self.count = data.len();
    }

    ///
    /// The number of elements in the buffer.
    ///
    pub fn count(&self) -> usize {
        self.count
    }

    pub(crate) fn data_type(&self) -> DataType {
        self.data_type
    }

    pub(crate) fn bind(&self) {
        self.context.bind_buffer(consts::ARRAY_BUFFER, &self.id);
    }
}

impl Drop for VertexBuffer {
    fn drop(&mut self) {
        self.context.delete_buffer(&self.id);
    }
}
