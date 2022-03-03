use crate::context::consts;
use crate::core::*;

///
/// A buffer containing per instance data. Can send between 1 and 4 values of [InstanceBufferDataType] to a shader program for each instance.
/// To send this data to a shader, use the [Program::use_attribute_instanced], [Program::use_attribute_vec2_instanced], etc. functionality.
///
pub struct InstanceBuffer<T: BufferDataType> {
    context: Context,
    id: crate::context::Buffer,
    count: usize,
    attribute_count: u32,
    attribute_size: u32,
    buffer_type: BufferType,
    _dummy: T,
}

impl<T: BufferDataType> InstanceBuffer<T> {
    ///
    /// Creates a new empty instance buffer.
    ///
    pub fn new(context: &Context, buffer_type: BufferType) -> ThreeDResult<Self> {
        Ok(Self {
            context: context.clone(),
            id: context.create_buffer().unwrap(),
            count: 0,
            attribute_count: 0,
            attribute_size: 0,
            buffer_type,
            _dummy: T::default(),
        })
    }

    pub fn new_with_data<V: Attribute<T>>(
        context: &Context,
        buffer_type: BufferType,
        data: &[V],
    ) -> ThreeDResult<Self> {
        let mut buffer = Self::new(context, buffer_type)?;
        if data.len() > 0 {
            buffer.fill(data);
        }
        Ok(buffer)
    }

    ///
    /// Fills the vertex buffer with the given data.
    ///
    pub fn fill<V: Attribute<T>>(&mut self, data: &[V]) {
        self.bind();
        T::buffer_data(
            &self.context,
            consts::ARRAY_BUFFER,
            &V::flatten(data),
            match self.buffer_type {
                BufferType::Static => consts::STATIC_DRAW,
                BufferType::Dynamic => consts::DYNAMIC_DRAW,
            },
        );
        self.context.unbind_buffer(consts::ARRAY_BUFFER);
        self.count = data.len() * V::length() as usize;
        self.attribute_count = data.len() as u32;
        self.attribute_size = V::length();
    }

    ///
    /// Creates a new instance buffer and fills it with the given data.
    /// The given data slice must contain between 1 and 4 contiguous values for each instance.
    /// Use this method instead of [new_with_dynamic](InstanceBuffer::new_with_dynamic)
    /// when you do not expect the data to change often.
    ///
    #[deprecated = "use new() or new_with_data()"]
    pub fn new_with_static<V: Attribute<T>>(context: &Context, data: &[V]) -> ThreeDResult<Self> {
        Self::new_with_data(context, BufferType::Static, data)
    }

    ///
    /// Fills the instance buffer with the given data.
    /// The given data slice must contain between 1 and 4 contiguous values for each instance.
    /// Use this method instead of [fill_with_dynamic](InstanceBuffer::fill_with_dynamic)
    /// when you do not expect the data to change often.
    ///
    #[deprecated = "use fill()"]
    pub fn fill_with_static<V: Attribute<T>>(&mut self, data: &[V]) {
        self.fill(data)
    }

    ///
    /// Creates a new instance buffer and fills it with the given data.
    /// The given data slice must contain between 1 and 4 contiguous values for each instance.
    /// Use this method instead of [new_with_static](InstanceBuffer::new_with_static)
    /// when you expect the data to change often.
    ///
    #[deprecated = "use new() or new_with_data()"]
    pub fn new_with_dynamic<V: Attribute<T>>(context: &Context, data: &[V]) -> ThreeDResult<Self> {
        Self::new_with_data(context, BufferType::Dynamic, data)
    }

    ///
    /// Fills the instance buffer with the given data.
    /// The given data slice must contain between 1 and 4 contiguous values for each instance.
    /// Use this method instead of [fill_with_static](InstanceBuffer::fill_with_static)
    /// when you expect the data to change often.
    ///
    #[deprecated = "use fill()"]
    pub fn fill_with_dynamic<V: Attribute<T>>(&mut self, data: &[V]) {
        self.fill(data)
    }

    ///
    /// The number of values in the buffer.
    ///
    pub fn count(&self) -> usize {
        self.count
    }

    ///
    /// The number of instance attributes in the buffer.
    ///
    pub fn attribute_count(&self) -> u32 {
        self.attribute_count
    }

    ///
    /// The size of each instance attributes, for example 3 if the instance attribute is a [Vector3].
    ///
    pub fn attribute_size(&self) -> u32 {
        self.attribute_size
    }

    pub(crate) fn bind(&self) {
        self.context.bind_buffer(consts::ARRAY_BUFFER, &self.id);
    }
}

impl<T: BufferDataType> Drop for InstanceBuffer<T> {
    fn drop(&mut self) {
        self.context.delete_buffer(&self.id);
    }
}
