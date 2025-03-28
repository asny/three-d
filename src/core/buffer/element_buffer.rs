use std::marker::PhantomData;

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
pub struct ElementBuffer<T: ElementBufferDataType> {
    context: Context,
    id: crate::context::Buffer,
    count: u32,
    _d: PhantomData<T>,
}

impl<T: ElementBufferDataType> ElementBuffer<T> {
    ///
    /// Creates a new empty element buffer.
    ///
    pub fn new(context: &Context) -> Self {
        let id = unsafe { context.create_buffer().expect("Failed creating buffer") };
        Self {
            context: context.clone(),
            id,
            count: 0,
            _d: PhantomData,
        }
    }

    ///
    /// Creates a new element buffer and fills it with the given indices which must be divisable by 3.
    ///
    pub fn new_with_data(context: &Context, data: &[T]) -> Self {
        let mut buffer = Self::new(context);
        if !data.is_empty() {
            buffer.fill(data);
        }
        buffer
    }

    ///
    /// Fills the buffer with the given indices which must be divisable by 3.
    /// This function will resize the buffer to have the same size as the indices array, if that is not desired, use [fill_subset](Self::fill_subset) instead.
    ///
    pub fn fill(&mut self, indices: &[T]) {
        self.bind();
        unsafe {
            self.context.buffer_data_u8_slice(
                crate::context::ELEMENT_ARRAY_BUFFER,
                to_byte_slice(indices),
                crate::context::STATIC_DRAW,
            );
            self.context
                .bind_buffer(crate::context::ELEMENT_ARRAY_BUFFER, None);
        }
        self.count = indices.len() as u32;
    }

    ///
    /// Fills the buffer with the given indices starting at the given offset.
    /// This will increase the size of the buffer if there's not enough room. Otherwise, the size will remain unchanged.
    ///
    pub fn fill_subset(&mut self, offset: u32, indices: &[T]) {
        self.bind();
        unsafe {
            self.context.buffer_sub_data_u8_slice(
                crate::context::ELEMENT_ARRAY_BUFFER,
                offset as i32,
                to_byte_slice(indices),
            );
            self.context
                .bind_buffer(crate::context::ELEMENT_ARRAY_BUFFER, None);
        }
        self.count = (offset + indices.len() as u32).max(self.count);
    }

    ///
    /// The number of values in the buffer.
    ///
    pub fn count(&self) -> u32 {
        self.count
    }

    ///
    /// The number of triangles in the buffer.
    ///
    pub fn triangle_count(&self) -> u32 {
        self.count / 3
    }

    pub(crate) fn bind(&self) {
        unsafe {
            self.context
                .bind_buffer(crate::context::ELEMENT_ARRAY_BUFFER, Some(self.id));
        }
    }
}

impl<T: ElementBufferDataType> Drop for ElementBuffer<T> {
    fn drop(&mut self) {
        unsafe {
            self.context.delete_buffer(self.id);
        }
    }
}
