use crate::context::{consts, DataType};
use crate::core::*;

/// The basic data type used for each index in an element buffer.
pub trait ElementBufferDataType:
    Default + std::fmt::Debug + Clone + internal::BufferDataTypeExtension
{
    ///
    /// Converts the index to `u32`.
    ///
    fn into_u32(&self) -> u32;
}
impl ElementBufferDataType for u8 {
    fn into_u32(&self) -> u32 {
        *self as u32
    }
}
impl ElementBufferDataType for u16 {
    fn into_u32(&self) -> u32 {
        *self as u32
    }
}
impl ElementBufferDataType for u32 {
    fn into_u32(&self) -> u32 {
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
    data_type: DataType,
}

impl ElementBuffer {
    ///
    /// Creates a new empty element buffer.
    ///
    pub fn new<T: ElementBufferDataType>(context: &Context) -> ThreeDResult<ElementBuffer> {
        let id = context.create_buffer().unwrap();
        Ok(ElementBuffer {
            context: context.clone(),
            id,
            count: 0,
            data_type: T::data_type(),
        })
    }

    ///
    /// Creates a new element buffer and fills it with the given indices which must be divisable by 3.
    ///
    pub fn new_with<T: ElementBufferDataType>(
        context: &Context,
        data: &[T],
    ) -> ThreeDResult<ElementBuffer> {
        let mut buffer = Self::new::<T>(context)?;
        if data.len() > 0 {
            buffer.fill_with(data)?;
        }
        Ok(buffer)
    }
    ///
    /// Fills the buffer with the given indices which must be divisable by 3.
    ///
    pub fn fill_with<T: ElementBufferDataType>(&mut self, data: &[T]) -> ThreeDResult<()> {
        if data.len() % 3 != 0 {
            Err(CoreError::InvalidNumberOfVertices(data.len()))?;
        }

        self.bind();
        T::buffer_data(
            &self.context,
            consts::ELEMENT_ARRAY_BUFFER,
            data,
            consts::STATIC_DRAW,
        );
        self.data_type = T::data_type();
        self.context.unbind_buffer(consts::ELEMENT_ARRAY_BUFFER);
        self.count = data.len();
        Ok(())
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
        self.context
            .bind_buffer(consts::ELEMENT_ARRAY_BUFFER, &self.id);
    }
}

impl Drop for ElementBuffer {
    fn drop(&mut self) {
        self.context.delete_buffer(&self.id);
    }
}
