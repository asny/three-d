use super::math::*;

///
/// Possible types that can be send as a uniform to a shader (a variable that is uniformly available when processing all vertices and fragments).
///
pub trait UniformDataType: std::fmt::Debug + internal::UniformDataTypeExtension {}

impl UniformDataType for i32 {}

impl UniformDataType for f32 {}
impl UniformDataType for Vec2 {}
impl UniformDataType for Vec3 {}
impl UniformDataType for Vec4 {}

impl UniformDataType for [f32; 2] {}
impl UniformDataType for [f32; 3] {}
impl UniformDataType for [f32; 4] {}

impl UniformDataType for Quat {}

impl UniformDataType for Mat2 {}
impl UniformDataType for Mat3 {}
impl UniformDataType for Mat4 {}

impl<T: UniformDataType + ?Sized> UniformDataType for &T {}

mod internal {
    use crate::context::UniformLocation;
    use crate::core::*;

    pub trait UniformDataTypeExtension: Copy {
        fn send(&self, context: &Context, location: &UniformLocation);
        fn send_array(data: &[Self], context: &Context, location: &UniformLocation);
    }
    impl<T: UniformDataTypeExtension + ?Sized> UniformDataTypeExtension for &T {
        fn send(&self, context: &Context, location: &UniformLocation) {
            (*self).send(context, location)
        }
        fn send_array(data: &[Self], context: &Context, location: &UniformLocation) {
            Self::send_array(
                &data.iter().map(|v| *v).collect::<Vec<_>>(),
                context,
                location,
            )
        }
    }

    impl UniformDataTypeExtension for i32 {
        fn send(&self, context: &Context, location: &UniformLocation) {
            unsafe {
                context.uniform_1_i32(Some(location), *self);
            }
        }
        fn send_array(data: &[Self], context: &Context, location: &UniformLocation) {
            unsafe {
                context.uniform_1_i32_slice(Some(location), &data);
            }
        }
    }

    impl UniformDataTypeExtension for f32 {
        fn send(&self, context: &Context, location: &UniformLocation) {
            unsafe {
                context.uniform_1_f32(Some(location), *self);
            }
        }
        fn send_array(data: &[Self], context: &Context, location: &UniformLocation) {
            unsafe {
                context.uniform_1_f32_slice(Some(location), &data);
            }
        }
    }

    impl UniformDataTypeExtension for Vec2 {
        fn send(&self, context: &Context, location: &UniformLocation) {
            unsafe {
                context.uniform_2_f32_slice(Some(location), &self.as_array());
            }
        }
        fn send_array(data: &[Self], context: &Context, location: &UniformLocation) {
            unsafe {
                context.uniform_2_f32_slice(
                    Some(location),
                    &data.iter().flat_map(|v| v.as_array()).collect::<Vec<_>>(),
                );
            }
        }
    }

    impl UniformDataTypeExtension for Vec3 {
        fn send(&self, context: &Context, location: &UniformLocation) {
            unsafe {
                context.uniform_3_f32_slice(Some(location), &self.as_array());
            }
        }
        fn send_array(data: &[Self], context: &Context, location: &UniformLocation) {
            unsafe {
                context.uniform_3_f32_slice(
                    Some(location),
                    &data.iter().flat_map(|v| v.as_array()).collect::<Vec<_>>(),
                );
            }
        }
    }

    impl UniformDataTypeExtension for Vec4 {
        fn send(&self, context: &Context, location: &UniformLocation) {
            unsafe {
                context.uniform_4_f32_slice(Some(location), &self.as_array());
            }
        }
        fn send_array(data: &[Self], context: &Context, location: &UniformLocation) {
            unsafe {
                context.uniform_4_f32_slice(
                    Some(location),
                    &data.iter().flat_map(|v| v.as_array()).collect::<Vec<_>>(),
                );
            }
        }
    }

    impl UniformDataTypeExtension for [f32; 2] {
        fn send(&self, context: &Context, location: &UniformLocation) {
            unsafe {
                context.uniform_2_f32_slice(Some(location), self);
            }
        }
        fn send_array(data: &[Self], context: &Context, location: &UniformLocation) {
            unsafe {
                context.uniform_2_f32_slice(
                    Some(location),
                    &data.iter().flat_map(|v| *v).collect::<Vec<_>>(),
                );
            }
        }
    }

    impl UniformDataTypeExtension for [f32; 3] {
        fn send(&self, context: &Context, location: &UniformLocation) {
            unsafe {
                context.uniform_3_f32_slice(Some(location), self);
            }
        }
        fn send_array(data: &[Self], context: &Context, location: &UniformLocation) {
            unsafe {
                context.uniform_3_f32_slice(
                    Some(location),
                    &data.iter().flat_map(|v| *v).collect::<Vec<_>>(),
                );
            }
        }
    }

    impl UniformDataTypeExtension for [f32; 4] {
        fn send(&self, context: &Context, location: &UniformLocation) {
            unsafe {
                context.uniform_4_f32_slice(Some(location), self);
            }
        }
        fn send_array(data: &[Self], context: &Context, location: &UniformLocation) {
            unsafe {
                context.uniform_4_f32_slice(
                    Some(location),
                    &data.iter().flat_map(|v| *v).collect::<Vec<_>>(),
                );
            }
        }
    }

    impl UniformDataTypeExtension for Quat {
        fn send(&self, context: &Context, location: &UniformLocation) {
            unsafe {
                context.uniform_4_f32_slice(Some(location), &self.as_array());
            }
        }
        fn send_array(data: &[Self], context: &Context, location: &UniformLocation) {
            unsafe {
                context.uniform_4_f32_slice(
                    Some(location),
                    &data.iter().flat_map(|v| v.as_array()).collect::<Vec<_>>(),
                );
            }
        }
    }

    impl UniformDataTypeExtension for Mat2 {
        fn send(&self, context: &Context, location: &UniformLocation) {
            unsafe {
                context.uniform_matrix_2_f32_slice(Some(location), false, &self.as_array());
            }
        }
        fn send_array(data: &[Self], context: &Context, location: &UniformLocation) {
            unsafe {
                context.uniform_matrix_2_f32_slice(
                    Some(location),
                    false,
                    &data.iter().flat_map(|v| v.as_array()).collect::<Vec<_>>(),
                );
            }
        }
    }

    impl UniformDataTypeExtension for Mat3 {
        fn send(&self, context: &Context, location: &UniformLocation) {
            unsafe {
                context.uniform_matrix_3_f32_slice(Some(location), false, &self.as_array());
            }
        }
        fn send_array(data: &[Self], context: &Context, location: &UniformLocation) {
            unsafe {
                context.uniform_matrix_3_f32_slice(
                    Some(location),
                    false,
                    &data.iter().flat_map(|v| v.as_array()).collect::<Vec<_>>(),
                );
            }
        }
    }

    impl UniformDataTypeExtension for Mat4 {
        fn send(&self, context: &Context, location: &UniformLocation) {
            unsafe {
                context.uniform_matrix_4_f32_slice(Some(location), false, &self.as_array());
            }
        }
        fn send_array(data: &[Self], context: &Context, location: &UniformLocation) {
            unsafe {
                context.uniform_matrix_4_f32_slice(
                    Some(location),
                    false,
                    &data.iter().flat_map(|v| v.as_array()).collect::<Vec<_>>(),
                );
            }
        }
    }
}
