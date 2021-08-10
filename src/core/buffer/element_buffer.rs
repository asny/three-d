use crate::context::{consts, Context, DataType};
use crate::core::{ElementBufferDataType, Error};

///
/// A buffer containing indices for rendering, see for example [draw_elements](crate::core::Program::draw_elements).
/// Also known as an index buffer.
///
pub struct ElementBuffer {
    context: Context,
    id: crate::context::Buffer,
    count: usize,
    data_type: DataType,
}

impl ElementBuffer {
    ///
    /// Creates a new element buffer and fills it with the given indices.
    ///
    pub fn new<T: ElementBufferDataType>(
        context: &Context,
        data: &[T],
    ) -> Result<ElementBuffer, Error> {
        let id = context.create_buffer().unwrap();
        let mut buffer = ElementBuffer {
            context: context.clone(),
            id,
            count: 0,
            data_type: T::data_type(),
        };
        if data.len() > 0 {
            buffer.fill_with(data);
        }
        Ok(buffer)
    }

    ///
    /// Fills the buffer with the given indices.
    ///
    pub fn fill_with<T: ElementBufferDataType>(&mut self, data: &[T]) {
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
