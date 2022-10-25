use crate::context::UniformLocation;
use crate::core::*;

pub enum UniformType {
    Value,
    Vec2,
    Vec3,
    Vec4,
    Mat2,
    Mat3,
    Mat4,
}

pub trait PrimitiveDataType: DataType + Copy + Default {
    fn send_uniform_with_type(
        context: &Context,
        location: &UniformLocation,
        data: &[Self],
        type_: UniformType,
    );
    fn internal_format_with_size(size: u32) -> u32;
}

impl PrimitiveDataType for u8 {
    fn internal_format_with_size(size: u32) -> u32 {
        match size {
            1 => crate::context::R8,
            2 => crate::context::RG8,
            3 => crate::context::RGB8,
            4 => crate::context::RGBA8,
            _ => unreachable!(),
        }
    }

    fn send_uniform_with_type(
        context: &Context,
        location: &UniformLocation,
        data: &[Self],
        type_: UniformType,
    ) {
        let data = data.iter().map(|v| *v as u32).collect::<Vec<_>>();
        u32::send_uniform_with_type(context, location, &data, type_)
    }
}
impl PrimitiveDataType for u16 {
    fn internal_format_with_size(size: u32) -> u32 {
        match size {
            1 => crate::context::R16UI,
            2 => crate::context::RG16UI,
            3 => crate::context::RGB16UI,
            4 => crate::context::RGBA16UI,
            _ => unreachable!(),
        }
    }

    fn send_uniform_with_type(
        context: &Context,
        location: &UniformLocation,
        data: &[Self],
        type_: UniformType,
    ) {
        let data = data.iter().map(|v| *v as u32).collect::<Vec<_>>();
        u32::send_uniform_with_type(context, location, &data, type_)
    }
}
impl PrimitiveDataType for u32 {
    fn internal_format_with_size(size: u32) -> u32 {
        match size {
            1 => crate::context::R32UI,
            2 => crate::context::RG32UI,
            3 => crate::context::RGB32UI,
            4 => crate::context::RGBA32UI,
            _ => unreachable!(),
        }
    }

    fn send_uniform_with_type(
        context: &Context,
        location: &UniformLocation,
        data: &[Self],
        type_: UniformType,
    ) {
        unsafe {
            match type_ {
                UniformType::Value => context.uniform_1_u32_slice(Some(location), data),
                UniformType::Vec2 => context.uniform_2_u32_slice(Some(location), data),
                UniformType::Vec3 => context.uniform_3_u32_slice(Some(location), data),
                UniformType::Vec4 => context.uniform_4_u32_slice(Some(location), data),
                _ => unimplemented!(),
            }
        }
    }
}
impl PrimitiveDataType for i8 {
    fn internal_format_with_size(size: u32) -> u32 {
        match size {
            1 => crate::context::R8I,
            2 => crate::context::RG8I,
            3 => crate::context::RGB8I,
            4 => crate::context::RGBA8I,
            _ => unreachable!(),
        }
    }

    fn send_uniform_with_type(
        context: &Context,
        location: &UniformLocation,
        data: &[Self],
        type_: UniformType,
    ) {
        let data = data.iter().map(|v| *v as i32).collect::<Vec<_>>();
        i32::send_uniform_with_type(context, location, &data, type_)
    }
}
impl PrimitiveDataType for i16 {
    fn internal_format_with_size(size: u32) -> u32 {
        match size {
            1 => crate::context::R16I,
            2 => crate::context::RG16I,
            3 => crate::context::RGB16I,
            4 => crate::context::RGBA16I,
            _ => unreachable!(),
        }
    }

    fn send_uniform_with_type(
        context: &Context,
        location: &UniformLocation,
        data: &[Self],
        type_: UniformType,
    ) {
        let data = data.iter().map(|v| *v as i32).collect::<Vec<_>>();
        i32::send_uniform_with_type(context, location, &data, type_)
    }
}
impl PrimitiveDataType for i32 {
    fn internal_format_with_size(size: u32) -> u32 {
        match size {
            1 => crate::context::R32I,
            2 => crate::context::RG32I,
            3 => crate::context::RGB32I,
            4 => crate::context::RGBA32I,
            _ => unreachable!(),
        }
    }

    fn send_uniform_with_type(
        context: &Context,
        location: &UniformLocation,
        data: &[Self],
        type_: UniformType,
    ) {
        unsafe {
            match type_ {
                UniformType::Value => context.uniform_1_i32_slice(Some(location), data),
                UniformType::Vec2 => context.uniform_2_i32_slice(Some(location), data),
                UniformType::Vec3 => context.uniform_3_i32_slice(Some(location), data),
                UniformType::Vec4 => context.uniform_4_i32_slice(Some(location), data),
                _ => unimplemented!(),
            }
        }
    }
}
impl PrimitiveDataType for f16 {
    fn internal_format_with_size(size: u32) -> u32 {
        match size {
            1 => crate::context::R16F,
            2 => crate::context::RG16F,
            3 => crate::context::RGB16F,
            4 => crate::context::RGBA16F,
            _ => unreachable!(),
        }
    }

    fn send_uniform_with_type(
        context: &Context,
        location: &UniformLocation,
        data: &[Self],
        type_: UniformType,
    ) {
        let data = data.iter().map(|v| v.to_f32()).collect::<Vec<_>>();
        f32::send_uniform_with_type(context, location, &data, type_)
    }
}
impl PrimitiveDataType for f32 {
    fn internal_format_with_size(size: u32) -> u32 {
        match size {
            1 => crate::context::R32F,
            2 => crate::context::RG32F,
            3 => crate::context::RGB32F,
            4 => crate::context::RGBA32F,
            _ => unreachable!(),
        }
    }

    fn send_uniform_with_type(
        context: &Context,
        location: &UniformLocation,
        data: &[Self],
        type_: UniformType,
    ) {
        unsafe {
            match type_ {
                UniformType::Value => context.uniform_1_f32_slice(Some(location), data),
                UniformType::Vec2 => context.uniform_2_f32_slice(Some(location), data),
                UniformType::Vec3 => context.uniform_3_f32_slice(Some(location), data),
                UniformType::Vec4 => context.uniform_4_f32_slice(Some(location), data),
                UniformType::Mat2 => {
                    context.uniform_matrix_2_f32_slice(Some(location), false, data)
                }
                UniformType::Mat3 => {
                    context.uniform_matrix_3_f32_slice(Some(location), false, data)
                }
                UniformType::Mat4 => {
                    context.uniform_matrix_4_f32_slice(Some(location), false, data)
                }
            }
        }
    }
}

pub trait DataType: std::fmt::Debug + Clone {
    fn internal_format() -> u32;
    fn data_type() -> u32;
    fn size() -> u32;
    fn send_uniform(context: &Context, location: &UniformLocation, data: &[Self]);
}

impl<T: DataType + ?Sized> DataType for &T {
    fn internal_format() -> u32 {
        T::internal_format()
    }
    fn data_type() -> u32 {
        T::data_type()
    }
    fn size() -> u32 {
        T::size()
    }

    fn send_uniform(context: &Context, location: &UniformLocation, data: &[Self]) {
        T::send_uniform(
            context,
            location,
            &data.iter().map(|v| (*v).clone()).collect::<Vec<_>>(),
        )
    }
}

impl DataType for u8 {
    fn internal_format() -> u32 {
        Self::internal_format_with_size(1)
    }

    fn data_type() -> u32 {
        crate::context::UNSIGNED_BYTE
    }

    fn size() -> u32 {
        1
    }

    fn send_uniform(context: &Context, location: &UniformLocation, data: &[Self]) {
        Self::send_uniform_with_type(context, location, data, UniformType::Value)
    }
}

impl DataType for u16 {
    fn internal_format() -> u32 {
        Self::internal_format_with_size(1)
    }
    fn data_type() -> u32 {
        crate::context::UNSIGNED_SHORT
    }

    fn size() -> u32 {
        1
    }

    fn send_uniform(context: &Context, location: &UniformLocation, data: &[Self]) {
        Self::send_uniform_with_type(context, location, data, UniformType::Value)
    }
}

impl DataType for u32 {
    fn internal_format() -> u32 {
        Self::internal_format_with_size(1)
    }

    fn data_type() -> u32 {
        crate::context::UNSIGNED_INT
    }

    fn size() -> u32 {
        1
    }

    fn send_uniform(context: &Context, location: &UniformLocation, data: &[Self]) {
        Self::send_uniform_with_type(context, location, data, UniformType::Value)
    }
}

impl DataType for i8 {
    fn internal_format() -> u32 {
        Self::internal_format_with_size(1)
    }

    fn data_type() -> u32 {
        crate::context::BYTE
    }

    fn size() -> u32 {
        1
    }

    fn send_uniform(context: &Context, location: &UniformLocation, data: &[Self]) {
        Self::send_uniform_with_type(context, location, data, UniformType::Value)
    }
}

impl DataType for i16 {
    fn internal_format() -> u32 {
        Self::internal_format_with_size(1)
    }

    fn data_type() -> u32 {
        crate::context::SHORT
    }

    fn size() -> u32 {
        1
    }

    fn send_uniform(context: &Context, location: &UniformLocation, data: &[Self]) {
        Self::send_uniform_with_type(context, location, data, UniformType::Value)
    }
}

impl DataType for i32 {
    fn internal_format() -> u32 {
        Self::internal_format_with_size(1)
    }

    fn data_type() -> u32 {
        crate::context::INT
    }

    fn size() -> u32 {
        1
    }

    fn send_uniform(context: &Context, location: &UniformLocation, data: &[Self]) {
        Self::send_uniform_with_type(context, location, data, UniformType::Value)
    }
}

impl DataType for f16 {
    fn internal_format() -> u32 {
        Self::internal_format_with_size(1)
    }
    fn data_type() -> u32 {
        crate::context::HALF_FLOAT
    }

    fn size() -> u32 {
        1
    }

    fn send_uniform(context: &Context, location: &UniformLocation, data: &[Self]) {
        Self::send_uniform_with_type(context, location, data, UniformType::Value)
    }
}

impl DataType for f32 {
    fn internal_format() -> u32 {
        Self::internal_format_with_size(1)
    }

    fn data_type() -> u32 {
        crate::context::FLOAT
    }

    fn size() -> u32 {
        1
    }

    fn send_uniform(context: &Context, location: &UniformLocation, data: &[Self]) {
        Self::send_uniform_with_type(context, location, data, UniformType::Value)
    }
}

impl<T: PrimitiveDataType> DataType for Vector2<T> {
    fn internal_format() -> u32 {
        T::internal_format_with_size(Self::size())
    }

    fn data_type() -> u32 {
        T::data_type()
    }

    fn size() -> u32 {
        2
    }

    fn send_uniform(context: &Context, location: &UniformLocation, data: &[Self]) {
        let data = data.iter().flat_map(|v| [v.x, v.y]).collect::<Vec<_>>();
        T::send_uniform_with_type(context, location, &data, UniformType::Vec2)
    }
}

impl<T: PrimitiveDataType> DataType for [T; 2] {
    fn internal_format() -> u32 {
        T::internal_format_with_size(Self::size())
    }

    fn data_type() -> u32 {
        T::data_type()
    }

    fn size() -> u32 {
        2
    }

    fn send_uniform(context: &Context, location: &UniformLocation, data: &[Self]) {
        let data = data.iter().flatten().map(|v| *v).collect::<Vec<_>>();
        T::send_uniform_with_type(context, location, &data, UniformType::Vec2)
    }
}

impl<T: PrimitiveDataType> DataType for Vector3<T> {
    fn internal_format() -> u32 {
        T::internal_format_with_size(Self::size())
    }
    fn data_type() -> u32 {
        T::data_type()
    }

    fn size() -> u32 {
        3
    }

    fn send_uniform(context: &Context, location: &UniformLocation, data: &[Self]) {
        let data = data
            .iter()
            .flat_map(|v| [v.x, v.y, v.z])
            .collect::<Vec<_>>();
        T::send_uniform_with_type(context, location, &data, UniformType::Vec3)
    }
}

impl<T: PrimitiveDataType> DataType for [T; 3] {
    fn internal_format() -> u32 {
        T::internal_format_with_size(Self::size())
    }
    fn data_type() -> u32 {
        T::data_type()
    }

    fn size() -> u32 {
        3
    }

    fn send_uniform(context: &Context, location: &UniformLocation, data: &[Self]) {
        let data = data.iter().flatten().map(|v| *v).collect::<Vec<_>>();
        T::send_uniform_with_type(context, location, &data, UniformType::Vec3)
    }
}

impl<T: PrimitiveDataType> DataType for Vector4<T> {
    fn internal_format() -> u32 {
        T::internal_format_with_size(Self::size())
    }

    fn data_type() -> u32 {
        T::data_type()
    }

    fn size() -> u32 {
        4
    }

    fn send_uniform(context: &Context, location: &UniformLocation, data: &[Self]) {
        let data = data
            .iter()
            .flat_map(|v| [v.x, v.y, v.z, v.w])
            .collect::<Vec<_>>();
        T::send_uniform_with_type(context, location, &data, UniformType::Vec4)
    }
}

impl<T: PrimitiveDataType> DataType for [T; 4] {
    fn internal_format() -> u32 {
        T::internal_format_with_size(Self::size())
    }

    fn data_type() -> u32 {
        T::data_type()
    }

    fn size() -> u32 {
        4
    }

    fn send_uniform(context: &Context, location: &UniformLocation, data: &[Self]) {
        let data = data.iter().flatten().map(|v| *v).collect::<Vec<_>>();
        T::send_uniform_with_type(context, location, &data, UniformType::Vec4)
    }
}

impl<T: PrimitiveDataType> DataType for Quaternion<T> {
    fn internal_format() -> u32 {
        T::internal_format_with_size(Self::size())
    }

    fn data_type() -> u32 {
        T::data_type()
    }

    fn size() -> u32 {
        4
    }

    fn send_uniform(context: &Context, location: &UniformLocation, data: &[Self]) {
        let data = data
            .iter()
            .flat_map(|v| [v.v.x, v.v.y, v.v.z, v.s])
            .collect::<Vec<_>>();
        T::send_uniform_with_type(context, location, &data, UniformType::Vec4)
    }
}

impl DataType for Color {
    fn internal_format() -> u32 {
        u8::internal_format_with_size(Self::size())
    }

    fn data_type() -> u32 {
        u8::data_type()
    }

    fn size() -> u32 {
        4
    }

    fn send_uniform(context: &Context, location: &UniformLocation, data: &[Self]) {
        let data = data
            .iter()
            .flat_map(|v| {
                [
                    v.r as f32 / 255.0,
                    v.g as f32 / 255.0,
                    v.b as f32 / 255.0,
                    v.a as f32 / 255.0,
                ]
            })
            .collect::<Vec<_>>();
        f32::send_uniform_with_type(context, location, &data, UniformType::Vec4)
    }
}

impl<T: PrimitiveDataType> DataType for Matrix2<T> {
    fn internal_format() -> u32 {
        T::internal_format_with_size(Self::size())
    }

    fn data_type() -> u32 {
        T::data_type()
    }

    fn size() -> u32 {
        4
    }

    fn send_uniform(context: &Context, location: &UniformLocation, data: &[Self]) {
        let data = data
            .iter()
            .flat_map(|v| [v.x.x, v.x.y, v.y.x, v.y.y])
            .collect::<Vec<_>>();
        T::send_uniform_with_type(context, location, &data, UniformType::Mat2)
    }
}

impl<T: PrimitiveDataType> DataType for Matrix3<T> {
    fn internal_format() -> u32 {
        T::internal_format_with_size(Self::size())
    }

    fn data_type() -> u32 {
        T::data_type()
    }

    fn size() -> u32 {
        9
    }

    fn send_uniform(context: &Context, location: &UniformLocation, data: &[Self]) {
        let data = data
            .iter()
            .flat_map(|v| {
                [
                    v.x.x, v.x.y, v.x.z, v.y.x, v.y.y, v.y.z, v.z.x, v.z.y, v.z.z,
                ]
            })
            .collect::<Vec<_>>();
        T::send_uniform_with_type(context, location, &data, UniformType::Mat3)
    }
}

impl<T: PrimitiveDataType> DataType for Matrix4<T> {
    fn internal_format() -> u32 {
        T::internal_format_with_size(Self::size())
    }

    fn data_type() -> u32 {
        T::data_type()
    }

    fn size() -> u32 {
        16
    }

    fn send_uniform(context: &Context, location: &UniformLocation, data: &[Self]) {
        let data = data
            .iter()
            .flat_map(|v| {
                [
                    v.x.x, v.x.y, v.x.z, v.x.w, v.y.x, v.y.y, v.y.z, v.y.w, v.z.x, v.z.y, v.z.z,
                    v.z.w, v.w.x, v.w.y, v.w.z, v.w.w,
                ]
            })
            .collect::<Vec<_>>();
        T::send_uniform_with_type(context, location, &data, UniformType::Mat4)
    }
}

pub trait DepthDataType {
    fn internal_format() -> u32;
}

impl DepthDataType for f16 {
    fn internal_format() -> u32 {
        crate::context::DEPTH_COMPONENT16
    }
}
impl DepthDataType for f24 {
    fn internal_format() -> u32 {
        crate::context::DEPTH_COMPONENT24
    }
}
impl DepthDataType for f32 {
    fn internal_format() -> u32 {
        crate::context::DEPTH_COMPONENT32F
    }
}
