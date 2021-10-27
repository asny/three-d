#![allow(missing_docs)]
//!
//! Basic math functionality. Mostly just an re-export of [cgmath](https://crates.io/crates/cgmath).
//!

pub(crate) use cgmath::ortho;
pub(crate) use cgmath::perspective;
pub use cgmath::prelude::*;
use cgmath::{
    Basis3, Deg, Matrix2, Matrix3, Matrix4, Point3, Quaternion, Rad, Vector2, Vector3, Vector4,
};

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

pub const fn vec2(x: f32, y: f32) -> Vec2 {
    Vector2::new(x, y)
}

pub const fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vector3::new(x, y, z)
}

pub const fn vec4(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
    Vector4::new(x, y, z, w)
}

pub(crate) trait Vec2Ext {
    fn to_slice(&self) -> [f32; 2];
}

impl Vec2Ext for Vec2 {
    fn to_slice(&self) -> [f32; 2] {
        [self.x, self.y]
    }
}

pub(crate) trait Vec3Ext {
    fn to_slice(&self) -> [f32; 3];
}

impl Vec3Ext for Vec3 {
    fn to_slice(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

pub(crate) trait Vec4Ext {
    fn to_slice(&self) -> [f32; 4];
}

impl Vec4Ext for Vec4 {
    fn to_slice(&self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }
}

impl Vec4Ext for Quat {
    fn to_slice(&self) -> [f32; 4] {
        [self.v.x, self.v.y, self.v.z, self.s]
    }
}

pub(crate) trait Mat2Ext {
    fn to_slice(&self) -> [f32; 4];
}

impl Mat2Ext for Mat2 {
    fn to_slice(&self) -> [f32; 4] {
        [self.x.x, self.x.y, self.y.x, self.y.y]
    }
}

pub(crate) trait Mat3Ext {
    fn to_slice(&self) -> [f32; 9];
}

impl Mat3Ext for Mat3 {
    fn to_slice(&self) -> [f32; 9] {
        [
            self.x.x, self.x.y, self.x.z, self.y.x, self.y.y, self.y.z, self.z.x, self.z.y,
            self.z.z,
        ]
    }
}

pub(crate) trait Mat4Ext {
    fn to_slice(&self) -> [f32; 16];
}

impl Mat4Ext for Mat4 {
    fn to_slice(&self) -> [f32; 16] {
        [
            self.x.x, self.x.y, self.x.z, self.x.w, self.y.x, self.y.y, self.y.z, self.y.w,
            self.z.x, self.z.y, self.z.z, self.z.w, self.w.x, self.w.y, self.w.z, self.w.w,
        ]
    }
}

pub fn degrees(v: f32) -> Degrees {
    Deg(v)
}
pub fn radians(v: f32) -> Radians {
    Rad(v)
}

pub fn rotation_matrix_from_dir_to_dir(source_dir: Vec3, target_dir: Vec3) -> Mat4 {
    Mat4::from(Mat3::from(Basis3::between_vectors(source_dir, target_dir)))
}
