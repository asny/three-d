use crate::context::consts;
use crate::core::*;

///
/// A buffer containing per vertex data, for example positions, normals, uv coordinates or colors.
/// To send this data to a shader, use the [Program::use_vertex_attribute] method.
///
pub struct Buffer<T: BufferDataType> {
    context: Context,
    id: crate::context::Buffer,
    count: usize,
    attribute_count: u32,
    attribute_size: u32,
    _dummy: T,
}

impl<T: BufferDataType> Buffer<T> {
    ///
    /// Creates a new empty vertex buffer.
    ///
    pub fn new(context: &Context) -> ThreeDResult<Self> {
        Ok(Self {
            context: context.clone(),
            id: context.create_buffer().unwrap(),
            count: 0,
            attribute_count: 0,
            attribute_size: 0,
            _dummy: T::default(),
        })
    }

    ///
    /// Creates a new vertex buffer and fills it with the given data.
    ///
    pub fn new_with_data<V: Attribute<T>>(context: &Context, data: &[V]) -> ThreeDResult<Self> {
        let mut buffer = Self::new(context)?;
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
            if self.attribute_count > 0 {
                consts::DYNAMIC_DRAW
            } else {
                consts::STATIC_DRAW
            },
        );
        self.context.unbind_buffer(consts::ARRAY_BUFFER);
        self.count = data.len() * V::length() as usize;
        self.attribute_count = data.len() as u32;
        self.attribute_size = V::length();
    }

    ///
    /// Creates a new vertex buffer and fills it with the given data which must contain between 1 and 4 contiguous values for each vertex.
    /// Use this method instead of [new_with_dynamic](Buffer::new_with_dynamic)
    /// when you do not expect the data to change often.
    ///
    #[deprecated = "use new() or new_with_data()"]
    pub fn new_with_static<V: Attribute<T>>(context: &Context, data: &[V]) -> ThreeDResult<Self> {
        Self::new_with_data(context, data)
    }

    ///
    /// Fills the vertex buffer with the given data which must contain between 1 and 4 contiguous values for each vertex.
    /// Use this method instead of [fill_with_dynamic](Buffer::fill_with_dynamic)
    /// when you do not expect the data to change often.
    ///
    #[deprecated = "use fill()"]
    pub fn fill_with_static<V: Attribute<T>>(&mut self, data: &[V]) {
        self.fill(data)
    }

    ///
    /// Creates a new vertex buffer and fills it with the given data which must contain between 1 and 4 contiguous values for each vertex.
    /// Use this method instead of [new_with_static](Buffer::new_with_static)
    /// when you expect the data to change often.
    ///
    #[deprecated = "use new() or new_with_data()"]
    pub fn new_with_dynamic<V: Attribute<T>>(context: &Context, data: &[V]) -> ThreeDResult<Self> {
        Self::new_with_data(context, data)
    }

    ///
    /// Fills the vertex buffer with the given data which must contain between 1 and 4 contiguous values for each vertex.
    /// Use this method instead of [fill_with_static](Buffer::fill_with_static)
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
    /// The number of vertex attributes in the buffer.
    ///
    pub fn attribute_count(&self) -> u32 {
        self.attribute_count
    }

    ///
    /// The size of each vertex attributes, for example 3 if the vertex attribute is a [Vector3].
    ///
    pub fn attribute_size(&self) -> u32 {
        self.attribute_size
    }

    pub(crate) fn bind(&self) {
        self.context.bind_buffer(consts::ARRAY_BUFFER, &self.id);
    }
}

impl<T: BufferDataType> Drop for Buffer<T> {
    fn drop(&mut self) {
        self.context.delete_buffer(&self.id);
    }
}
