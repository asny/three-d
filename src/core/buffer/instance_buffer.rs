use crate::context::{consts, DataType};
use crate::core::*;

///
/// A buffer containing per instance data. Can send between 1 and 4 values of [InstanceBufferDataType] to a shader program for each instance.
/// To send this data to a shader, use the [Program::use_attribute_instanced], [Program::use_attribute_vec2_instanced], etc. functionality.
///
pub struct InstanceBuffer {
    context: Context,
    id: crate::context::Buffer,
    count: usize,
    data_type: DataType,
}

impl InstanceBuffer {
    ///
    /// Creates a new empty instance buffer.
    ///
    pub fn new(context: &Context) -> ThreeDResult<Self> {
        Ok(Self {
            context: context.clone(),
            id: context.create_buffer().unwrap(),
            count: 0,
            data_type: DataType::Float,
        })
    }

    ///
    /// Creates a new instance buffer and fills it with the given data.
    /// The given data slice must contain between 1 and 4 contiguous values for each instance.
    /// Use this method instead of [new_with_dynamic](InstanceBuffer::new_with_dynamic)
    /// when you do not expect the data to change often.
    ///
    pub fn new_with_static<T: BufferDataType>(context: &Context, data: &[T]) -> ThreeDResult<Self> {
        let mut buffer = Self::new(context)?;
        if data.len() > 0 {
            buffer.fill_with_static(data);
        }
        Ok(buffer)
    }

    ///
    /// Fills the instance buffer with the given data.
    /// The given data slice must contain between 1 and 4 contiguous values for each instance.
    /// Use this method instead of [fill_with_dynamic](InstanceBuffer::fill_with_dynamic)
    /// when you do not expect the data to change often.
    ///
    pub fn fill_with_static<T: BufferDataType>(&mut self, data: &[T]) {
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
    /// Creates a new instance buffer and fills it with the given data.
    /// The given data slice must contain between 1 and 4 contiguous values for each instance.
    /// Use this method instead of [new_with_static](InstanceBuffer::new_with_static)
    /// when you expect the data to change often.
    ///
    pub fn new_with_dynamic<T: BufferDataType>(
        context: &Context,
        data: &[T],
    ) -> ThreeDResult<Self> {
        let mut buffer = Self::new(context).unwrap();
        if data.len() > 0 {
            buffer.fill_with_dynamic(data);
        }
        Ok(buffer)
    }

    ///
    /// Fills the instance buffer with the given data.
    /// The given data slice must contain between 1 and 4 contiguous values for each instance.
    /// Use this method instead of [fill_with_static](InstanceBuffer::fill_with_static)
    /// when you expect the data to change often.
    ///
    pub fn fill_with_dynamic<T: BufferDataType>(&mut self, data: &[T]) {
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

impl Drop for InstanceBuffer {
    fn drop(&mut self) {
        self.context.delete_buffer(&self.id);
    }
}
