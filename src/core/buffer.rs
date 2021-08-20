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

pub(crate) mod internal {
    use crate::context::{Context, DataType};

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
}
