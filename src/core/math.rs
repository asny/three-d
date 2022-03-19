#![allow(missing_docs)]
//!
//! Basic math functionality. Mostly just an re-export of [cgmath](https://crates.io/crates/cgmath).
//!

pub use half::f16;
pub use half::slice::HalfFloatSliceExt;
pub use half::vec::HalfFloatVecExt;

pub(crate) use cgmath::ortho;
pub(crate) use cgmath::perspective;
pub use cgmath::prelude::*;
use cgmath::{Basis3, Deg, Point3, Rad};
pub use cgmath::{Matrix2, Matrix3, Matrix4, Quaternion, Vector2, Vector3, Vector4};

pub type Vec2 = Vector2<f32>;
pub type Vec3 = Vector3<f32>;
pub type Vec4 = Vector4<f32>;
pub type Mat2 = Matrix2<f32>;
pub type Mat3 = Matrix3<f32>;
pub type Mat4 = Matrix4<f32>;
pub type Point = Point3<f32>;
pub type Degrees = Deg<f32>;
pub type Radians = Rad<f32>;
pub type Quat = Quaternion<f32>;

pub const fn vec2<T>(x: T, y: T) -> Vector2<T> {
    Vector2::new(x, y)
}

pub const fn vec3<T>(x: T, y: T, z: T) -> Vector3<T> {
    Vector3::new(x, y, z)
}

pub const fn vec4<T>(x: T, y: T, z: T, w: T) -> Vector4<T> {
    Vector4::new(x, y, z, w)
}

pub trait Vec2Ext {
    fn as_array(&self) -> [f32; 2];
}
impl Vec2Ext for Vec2 {
    fn as_array(&self) -> [f32; 2] {
        (*self).into()
    }
}

pub trait Vec3Ext {
    fn as_array(&self) -> [f32; 3];
}

impl Vec3Ext for Vec3 {
    fn as_array(&self) -> [f32; 3] {
        (*self).into()
    }
}

pub trait Vec4Ext {
    fn as_array(&self) -> [f32; 4];
}

impl Vec4Ext for Vec4 {
    fn as_array(&self) -> [f32; 4] {
        (*self).into()
    }
}

impl Vec4Ext for Quat {
    fn as_array(&self) -> [f32; 4] {
        [self.v.x, self.v.y, self.v.z, self.s]
    }
}

pub trait Mat2Ext {
    fn as_array(&self) -> [f32; 4];
}

impl Mat2Ext for Mat2 {
    fn as_array(&self) -> [f32; 4] {
        [self.x.x, self.x.y, self.y.x, self.y.y]
    }
}

pub trait Mat3Ext {
    fn as_array(&self) -> [f32; 9];
}

impl Mat3Ext for Mat3 {
    fn as_array(&self) -> [f32; 9] {
        [
            self.x.x, self.x.y, self.x.z, self.y.x, self.y.y, self.y.z, self.z.x, self.z.y,
            self.z.z,
        ]
    }
}

pub trait Mat4Ext {
    fn as_array(&self) -> [f32; 16];
}

impl Mat4Ext for Mat4 {
    fn as_array(&self) -> [f32; 16] {
        [
            self.x.x, self.x.y, self.x.z, self.x.w, self.y.x, self.y.y, self.y.z, self.y.w,
            self.z.x, self.z.y, self.z.z, self.z.w, self.w.x, self.w.y, self.w.z, self.w.w,
        ]
    }
}

pub const fn degrees(v: f32) -> Degrees {
    Deg(v)
}
pub const fn radians(v: f32) -> Radians {
    Rad(v)
}

pub fn rotation_matrix_from_dir_to_dir(source_dir: Vec3, target_dir: Vec3) -> Mat4 {
    Mat4::from(Mat3::from(Basis3::between_vectors(source_dir, target_dir)))
}
