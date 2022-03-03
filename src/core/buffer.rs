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

/// The basic data type used for each element in a [VertexBuffer] or [InstancedBuffer].
pub trait BufferDataType:
    Default + std::fmt::Debug + Clone + Copy + internal::BufferDataTypeExtension
{
}
impl BufferDataType for u8 {}
impl BufferDataType for u16 {}
impl BufferDataType for f16 {}
impl BufferDataType for f32 {}

pub trait Attribute<T: BufferDataType>:
    std::fmt::Debug + Clone + internal::AttributeExtension<T>
{
}

impl<T: BufferDataType> Attribute<T> for T {}
impl<T: BufferDataType> Attribute<T> for Vector2<T> {}
impl<T: BufferDataType> Attribute<T> for Vector3<T> {}
impl<T: BufferDataType> Attribute<T> for Vector4<T> {}
impl Attribute<u8> for Color {}

pub(crate) mod internal {
    use crate::context::DataType;
    use crate::core::*;

    pub trait BufferDataTypeExtension: Clone {
        fn buffer_data(context: &Context, target: u32, data: &[Self], usage: u32);
        fn data_type() -> DataType;
    }

    impl BufferDataTypeExtension for u8 {
        fn buffer_data(context: &Context, target: u32, data: &[Self], usage: u32) {
            context.buffer_data_u8(target, data, usage);
        }
        fn data_type() -> DataType {
            DataType::UnsignedByte
        }
    }

    impl BufferDataTypeExtension for u16 {
        fn buffer_data(context: &Context, target: u32, data: &[Self], usage: u32) {
            context.buffer_data_u16(target, data, usage);
        }
        fn data_type() -> DataType {
            DataType::UnsignedShort
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
    }

    impl BufferDataTypeExtension for f32 {
        fn buffer_data(context: &Context, target: u32, data: &[Self], usage: u32) {
            context.buffer_data_f32(target, data, usage);
        }
        fn data_type() -> DataType {
            DataType::Float
        }
    }

    impl BufferDataTypeExtension for u32 {
        fn buffer_data(context: &Context, target: u32, data: &[Self], usage: u32) {
            context.buffer_data_u32(target, data, usage);
        }
        fn data_type() -> DataType {
            DataType::UnsignedInt
        }
    }

    pub trait AttributeExtension<T: BufferDataType>: Clone {
        fn length() -> u32;
        fn flatten(data: &[Self]) -> Vec<T>;
    }

    impl<T: BufferDataType> AttributeExtension<T> for T {
        fn length() -> u32 {
            1
        }

        fn flatten(data: &[Self]) -> Vec<T> {
            data.to_vec()
        }
    }

    impl<T: BufferDataType> AttributeExtension<T> for Vector2<T> {
        fn length() -> u32 {
            2
        }

        fn flatten(data: &[Self]) -> Vec<T> {
            let mut res = Vec::with_capacity(data.len() * Self::length() as usize);
            for d in data {
                res.push(d.x);
                res.push(d.y);
            }
            res
        }
    }

    impl<T: BufferDataType> AttributeExtension<T> for Vector3<T> {
        fn length() -> u32 {
            3
        }

        fn flatten(data: &[Self]) -> Vec<T> {
            let mut res = Vec::with_capacity(data.len() * Self::length() as usize);
            for d in data {
                res.push(d.x);
                res.push(d.y);
                res.push(d.z);
            }
            res
        }
    }

    impl<T: BufferDataType> AttributeExtension<T> for Vector4<T> {
        fn length() -> u32 {
            4
        }

        fn flatten(data: &[Self]) -> Vec<T> {
            let mut res = Vec::with_capacity(data.len() * Self::length() as usize);
            for d in data {
                res.push(d.x);
                res.push(d.y);
                res.push(d.z);
                res.push(d.w);
            }
            res
        }
    }

    impl AttributeExtension<u8> for Color {
        fn length() -> u32 {
            4
        }

        fn flatten(data: &[Self]) -> Vec<u8> {
            let mut res = Vec::with_capacity(data.len() * Self::length() as usize);
            for d in data {
                res.push(d.r);
                res.push(d.g);
                res.push(d.b);
                res.push(d.a);
            }
            res
        }
    }
}
