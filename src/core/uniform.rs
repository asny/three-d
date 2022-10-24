use crate::core::*;
use data_type::*;

///
/// Possible types that can be send as a uniform to a shader (a variable that is uniformly available when processing all vertices and fragments).
///
pub trait UniformDataType: DataType {}

impl UniformDataType for u8 {}
impl UniformDataType for u16 {}
impl UniformDataType for u32 {}
impl UniformDataType for i8 {}
impl UniformDataType for i16 {}
impl UniformDataType for i32 {}
impl UniformDataType for f16 {}
impl UniformDataType for f32 {}

impl<T: UniformDataType + PrimitiveDataType> UniformDataType for Vector2<T> {}
impl<T: UniformDataType + PrimitiveDataType> UniformDataType for Vector3<T> {}
impl<T: UniformDataType + PrimitiveDataType> UniformDataType for Vector4<T> {}

impl UniformDataType for Color {}
impl UniformDataType for Quat {}

impl<T: UniformDataType + PrimitiveDataType> UniformDataType for Matrix2<T> {}
impl<T: UniformDataType + PrimitiveDataType> UniformDataType for Matrix3<T> {}
impl<T: UniformDataType + PrimitiveDataType> UniformDataType for Matrix4<T> {}

impl<T: UniformDataType + ?Sized> UniformDataType for &T {}
