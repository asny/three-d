use crate::core::*;

/// The basic data type used for each index in an element buffer.
pub trait ElementBufferDataType: data_type::DataType {
    ///
    /// Converts the index to `u32`.
    ///
    fn as_u32(&self) -> u32;
}
impl ElementBufferDataType for u8 {
    fn as_u32(&self) -> u32 {
        *self as u32
    }
}
impl ElementBufferDataType for u16 {
    fn as_u32(&self) -> u32 {
        *self as u32
    }
}
impl ElementBufferDataType for u32 {
    fn as_u32(&self) -> u32 {
        *self
    }
}

///
/// A buffer containing 3 indices for each triangle to be rendered, which is why it is also known as an index buffer.
/// The three indices refer to three places in a set of [VertexBuffer] where the data (position, normal etc.) is found for the three vertices of the triangle.
/// See for example [Program::draw_elements] to use this for drawing.
///
pub struct ElementBuffer {
    context: Context,
    id: crate::context::Buffer,
    count: usize,
    data_type: u32,
}

impl ElementBuffer {
    ///
    /// Creates a new empty element buffer.
    ///
    pub fn new(context: &Context) -> Self {
        let id = unsafe { context.create_buffer().expect("Failed creating buffer") };
        Self {
            context: context.clone(),
            id,
            count: 0,
            data_type: 0,
        }
    }

    ///
    /// Creates a new element buffer and fills it with the given indices which must be divisable by 3.
    ///
    pub fn new_with_data<T: ElementBufferDataType>(context: &Context, data: &[T]) -> Self {
        let mut buffer = Self::new(context);
        if data.len() > 0 {
            buffer.fill(data);
        }
        buffer
    }

    ///
    /// Fills the buffer with the given indices which must be divisable by 3.
    ///
    pub fn fill<T: ElementBufferDataType>(&mut self, data: &[T]) {
        self.bind();
        unsafe {
            self.context.buffer_data_u8_slice(
                crate::context::ELEMENT_ARRAY_BUFFER,
                to_byte_slice(data),
                crate::context::STATIC_DRAW,
            );
            self.context
                .bind_buffer(crate::context::ELEMENT_ARRAY_BUFFER, None);
        }
        self.count = data.len();
        self.data_type = T::data_type();
    }

    ///
    /// The number of values in the buffer.
    ///
    pub fn count(&self) -> usize {
        self.count
    }

    ///
    /// The number of triangles in the buffer.
    ///
    pub fn triangle_count(&self) -> usize {
        self.count / 3
    }

    pub(crate) fn bind(&self) {
        unsafe {
            self.context
                .bind_buffer(crate::context::ELEMENT_ARRAY_BUFFER, Some(self.id));
        }
    }

    pub(crate) fn data_type(&self) -> u32 {
        self.data_type
    }
}

impl Drop for ElementBuffer {
    fn drop(&mut self) {
        unsafe {
            self.context.delete_buffer(self.id);
        }
    }
}
