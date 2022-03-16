//!
//! Different types of buffers used for sending data (primarily geometry data) to the GPU.
//!
mod element_buffer;
#[doc(inline)]
pub use element_buffer::*;

mod vertex_buffer;
#[doc(inline)]
pub use vertex_buffer::*;

mod instance_buffer;
#[doc(inline)]
pub use instance_buffer::*;

mod uniform_buffer;
#[doc(inline)]
pub use uniform_buffer::*;

use crate::context::HasContext;
use crate::core::*;

/// The basic data type used for each element in a [VertexBuffer] or [InstanceBuffer].
pub trait BufferDataType: std::fmt::Debug + Clone + Copy + internal::DataType {}
impl BufferDataType for u8 {}
impl BufferDataType for u16 {}
impl BufferDataType for f16 {}
impl BufferDataType for f32 {}
impl<T: BufferDataType> BufferDataType for Vector2<T> {}
impl<T: BufferDataType> BufferDataType for Vector3<T> {}
impl<T: BufferDataType> BufferDataType for Vector4<T> {}
impl BufferDataType for Color {}

struct Buffer<T: BufferDataType> {
    context: Context,
    id: glow::Buffer,
    attribute_count: u32,
    _dummy: T,
}

impl<T: BufferDataType> Buffer<T> {
    pub fn new(context: &Context) -> ThreeDResult<Self> {
        Ok(Self {
            context: context.clone(),
            id: unsafe {
                context
                    .create_buffer()
                    .map_err(|e| CoreError::BufferCreation(e))?
            },
            attribute_count: 0,
            _dummy: T::default(),
        })
    }

    pub fn new_with_data(context: &Context, data: &[T]) -> ThreeDResult<Self> {
        let mut buffer = Self::new(context)?;
        if data.len() > 0 {
            buffer.fill(data)?;
        }
        Ok(buffer)
    }

    pub fn fill(&mut self, data: &[T]) -> ThreeDResult<()> {
        self.bind();
        unsafe {
            self.context.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                super::internal::to_byte_slice(data),
                if self.attribute_count > 0 {
                    glow::DYNAMIC_DRAW
                } else {
                    glow::STATIC_DRAW
                },
            );
            self.context.bind_buffer(glow::ARRAY_BUFFER, None);
        }
        self.attribute_count = data.len() as u32;
        self.context.error_check()
    }

    pub fn attribute_count(&self) -> u32 {
        self.attribute_count
    }

    pub fn bind(&self) {
        unsafe {
            self.context.bind_buffer(glow::ARRAY_BUFFER, Some(self.id));
        }
    }
}

impl<T: BufferDataType> Drop for Buffer<T> {
    fn drop(&mut self) {
        unsafe {
            self.context.delete_buffer(self.id);
        }
    }
}
