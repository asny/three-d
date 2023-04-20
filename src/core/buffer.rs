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

use crate::core::*;
use data_type::*;

/// The basic data type used for each element in a [VertexBuffer] or [InstanceBuffer].
pub trait BufferDataType: DataType {
    /// How many `vertex_attrib_pointer` calls are needed to configure buffers
    /// of this type.
    const ATTRIBUTE_SLOTS: u32 = 1;
}

impl BufferDataType for u8 {}
impl BufferDataType for u16 {}
impl BufferDataType for u32 {}
impl BufferDataType for i8 {}
impl BufferDataType for i16 {}
impl BufferDataType for i32 {}
impl BufferDataType for f16 {}
impl BufferDataType for f32 {}

impl<T: BufferDataType + PrimitiveDataType> BufferDataType for Vector2<T> {}
impl<T: BufferDataType + PrimitiveDataType> BufferDataType for Vector3<T> {}
impl<T: BufferDataType + PrimitiveDataType> BufferDataType for Vector4<T> {}
impl<T: BufferDataType + PrimitiveDataType> BufferDataType for [T; 2] {}
impl<T: BufferDataType + PrimitiveDataType> BufferDataType for [T; 3] {}
impl<T: BufferDataType + PrimitiveDataType> BufferDataType for [T; 4] {}

impl<T: BufferDataType + PrimitiveDataType> BufferDataType for Matrix2<T> {
    const ATTRIBUTE_SLOTS: u32 = 2;
}
impl<T: BufferDataType + PrimitiveDataType> BufferDataType for Matrix3<T> {
    const ATTRIBUTE_SLOTS: u32 = 3;
}
impl<T: BufferDataType + PrimitiveDataType> BufferDataType for Matrix4<T> {
    const ATTRIBUTE_SLOTS: u32 = 4;
}

impl BufferDataType for Color {}
impl BufferDataType for Quat {}

impl<T: BufferDataType + ?Sized> BufferDataType for &T {}

struct Buffer {
    context: Context,
    id: crate::context::Buffer,
    attribute_count: u32,
    attribute_slot_count: u32,
    data_type: u32,
    data_size: u32,
    stride: i32,
    normalized: bool,
}

impl Buffer {
    pub fn new(context: &Context) -> Self {
        Self {
            context: context.clone(),
            id: unsafe { context.create_buffer().expect("Failed creating buffer") },
            attribute_count: 0,
            attribute_slot_count: 0,
            data_type: 0,
            data_size: 0,
            stride: 0,
            normalized: false,
        }
    }

    pub fn new_with_data<T: BufferDataType>(context: &Context, data: &[T]) -> Self {
        let mut buffer = Self::new(context);
        if !data.is_empty() {
            buffer.fill(data);
        }
        buffer
    }

    pub fn fill<T: BufferDataType>(&mut self, data: &[T]) {
        self.bind();
        unsafe {
            self.context.buffer_data_u8_slice(
                crate::context::ARRAY_BUFFER,
                to_byte_slice(data),
                if self.attribute_count > 0 {
                    crate::context::DYNAMIC_DRAW
                } else {
                    crate::context::STATIC_DRAW
                },
            );
            self.context.bind_buffer(crate::context::ARRAY_BUFFER, None);
        }
        self.attribute_count = data.len() as u32;
        self.attribute_slot_count = T::ATTRIBUTE_SLOTS;
        self.data_type = T::data_type();
        self.data_size = T::size();
        self.stride = T::bytes() as i32;
        self.normalized = T::normalized();
    }

    pub fn attribute_count(&self) -> u32 {
        self.attribute_count
    }

    pub fn attribute_slots(&self) -> impl Iterator<Item = (u32, i32)> + '_ {
        let local_stride = self.stride / self.attribute_slot_count as i32;
        (0..self.attribute_slot_count).map(move |i| (i, i as i32 * local_stride))
    }

    pub fn bind(&self) {
        unsafe {
            self.context
                .bind_buffer(crate::context::ARRAY_BUFFER, Some(self.id));
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            self.context.delete_buffer(self.id);
        }
    }
}
