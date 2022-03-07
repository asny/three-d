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

/// The basic data type used for each element in a [Buffer] or [InstancedBuffer].
pub trait BufferDataType:
    std::fmt::Debug + Clone + Copy + internal::BufferDataTypeExtension
{
}
impl BufferDataType for u8 {}
impl BufferDataType for u16 {}
impl BufferDataType for f16 {}
impl BufferDataType for f32 {}
impl<T: BufferDataType> BufferDataType for Vector2<T> {}
impl<T: BufferDataType> BufferDataType for Vector3<T> {}
impl<T: BufferDataType> BufferDataType for Vector4<T> {}
impl BufferDataType for Color {}

pub(crate) mod internal {
    use crate::context::DataType;
    use crate::core::*;

    pub trait BufferDataTypeExtension: Clone {
        fn buffer_data(context: &Context, target: u32, data: &[Self], usage: u32);
        fn data_type() -> DataType;
        fn size() -> u32;
        fn default() -> Self;
    }

    impl BufferDataTypeExtension for u8 {
        fn buffer_data(context: &Context, target: u32, data: &[Self], usage: u32) {
            context.buffer_data_u8(target, data, usage);
        }
        fn data_type() -> DataType {
            DataType::UnsignedByte
        }
        fn size() -> u32 {
            1
        }
        fn default() -> Self {
            0
        }
    }

    impl BufferDataTypeExtension for u16 {
        fn buffer_data(context: &Context, target: u32, data: &[Self], usage: u32) {
            context.buffer_data_u16(target, data, usage);
        }
        fn data_type() -> DataType {
            DataType::UnsignedShort
        }
        fn size() -> u32 {
            1
        }
        fn default() -> Self {
            0
        }
    }

    impl BufferDataTypeExtension for f16 {
        fn buffer_data(context: &Context, target: u32, data: &[Self], usage: u32) {
            context.buffer_data_u16(
                target,
                &data.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
                usage,
            );
        }
        fn data_type() -> DataType {
            DataType::HalfFloat
        }
        fn size() -> u32 {
            1
        }
        fn default() -> Self {
            f16::from_f32(0.0)
        }
    }

    impl BufferDataTypeExtension for f32 {
        fn buffer_data(context: &Context, target: u32, data: &[Self], usage: u32) {
            context.buffer_data_f32(target, data, usage);
        }
        fn data_type() -> DataType {
            DataType::Float
        }
        fn size() -> u32 {
            1
        }
        fn default() -> Self {
            0.0
        }
    }

    impl BufferDataTypeExtension for u32 {
        fn buffer_data(context: &Context, target: u32, data: &[Self], usage: u32) {
            context.buffer_data_u32(target, data, usage);
        }
        fn data_type() -> DataType {
            DataType::UnsignedInt
        }
        fn size() -> u32 {
            1
        }
        fn default() -> Self {
            0
        }
    }

    impl<T: BufferDataType> BufferDataTypeExtension for Vector2<T> {
        fn buffer_data(context: &Context, target: u32, data: &[Self], usage: u32) {
            let mut flattened_data = Vec::with_capacity(data.len() * Self::size() as usize);
            for d in data {
                flattened_data.push(d.x);
                flattened_data.push(d.y);
            }
            T::buffer_data(context, target, &flattened_data, usage)
        }
        fn data_type() -> DataType {
            T::data_type()
        }
        fn size() -> u32 {
            2
        }
        fn default() -> Self {
            Self::new(T::default(), T::default())
        }
    }

    impl<T: BufferDataType> BufferDataTypeExtension for Vector3<T> {
        fn buffer_data(context: &Context, target: u32, data: &[Self], usage: u32) {
            let mut flattened_data = Vec::with_capacity(data.len() * Self::size() as usize);
            for d in data {
                flattened_data.push(d.x);
                flattened_data.push(d.y);
                flattened_data.push(d.z);
            }
            T::buffer_data(context, target, &flattened_data, usage)
        }
        fn data_type() -> DataType {
            T::data_type()
        }
        fn size() -> u32 {
            3
        }
        fn default() -> Self {
            Self::new(T::default(), T::default(), T::default())
        }
    }

    impl<T: BufferDataType> BufferDataTypeExtension for Vector4<T> {
        fn buffer_data(context: &Context, target: u32, data: &[Self], usage: u32) {
            let mut flattened_data = Vec::with_capacity(data.len() * Self::size() as usize);
            for d in data {
                flattened_data.push(d.x);
                flattened_data.push(d.y);
                flattened_data.push(d.z);
                flattened_data.push(d.w);
            }
            T::buffer_data(context, target, &flattened_data, usage)
        }
        fn data_type() -> DataType {
            T::data_type()
        }
        fn size() -> u32 {
            4
        }
        fn default() -> Self {
            Self::new(T::default(), T::default(), T::default(), T::default())
        }
    }

    impl BufferDataTypeExtension for Color {
        fn buffer_data(context: &Context, target: u32, data: &[Self], usage: u32) {
            let mut flattened_data = Vec::with_capacity(data.len() * Self::size() as usize);
            for d in data {
                flattened_data.push(d.r);
                flattened_data.push(d.g);
                flattened_data.push(d.b);
                flattened_data.push(d.a);
            }
            u8::buffer_data(context, target, &flattened_data, usage)
        }
        fn data_type() -> DataType {
            u8::data_type()
        }
        fn size() -> u32 {
            4
        }
        fn default() -> Self {
            Color::WHITE
        }
    }
}
