//! An ultra-light private math library to make our short lives easier as we
//! implement super-complex noise stuff.

use std::ops::{Add, Mul, Sub};

/// Cast a numeric type without having to unwrap - we don't expect any overflow
/// errors...
#[inline]
pub fn cast<T, U: From<T>>(x: T) -> U {
    From::from(x)
}

/// A 2-dimensional point. This is a fixed sized array, so should be compatible
/// with most linear algebra libraries.
pub type Point2<T> = [T; 2];

/// A 3-dimensional point. This is a fixed sized array, so should be compatible
/// with most linear algebra libraries.
pub type Point3<T> = [T; 3];

/// A 4-dimensional point. This is a fixed sized array, so should be compatible
/// with most linear algebra libraries.
pub type Point4<T> = [T; 4];

/// A 2-dimensional vector, for internal use.
pub type Vector2<T> = [T; 2];
/// A 3-dimensional vector, for internal use.
pub type Vector3<T> = [T; 3];
/// A 4-dimensional vector, for internal use.
pub type Vector4<T> = [T; 4];

#[inline]
pub fn clamp<T: PartialOrd>(val: T, min: T, max: T) -> T {
    assert!(max >= min);
    match () {
        _ if val < min => min,
        _ if val > max => max,
        _ => val,
    }
}

#[inline]
pub fn map2<T, U, F>(a: Vector2<T>, f: F) -> Vector2<U>
where
    T: Copy,
    F: Fn(T) -> U,
{
    let (ax, ay) = (a[0], a[1]);
    [f(ax), f(ay)]
}

#[inline]
pub fn map3<T, U, F>(a: Vector3<T>, f: F) -> Vector3<U>
where
    T: Copy,
    F: Fn(T) -> U,
{
    let (ax, ay, az) = (a[0], a[1], a[2]);
    [f(ax), f(ay), f(az)]
}

#[inline]
pub fn map4<T, U, F>(a: Vector4<T>, f: F) -> Vector4<U>
where
    T: Copy,
    F: Fn(T) -> U,
{
    let (ax, ay, az, aw) = (a[0], a[1], a[2], a[3]);
    [f(ax), f(ay), f(az), f(aw)]
}

#[inline]
pub fn zip_with2<T, U, V, F>(a: Vector2<T>, b: Vector2<U>, f: F) -> Vector2<V>
where
    T: Copy,
    U: Copy,
    F: Fn(T, U) -> V,
{
    let (ax, ay) = (a[0], a[1]);
    let (bx, by) = (b[0], b[1]);
    [f(ax, bx), f(ay, by)]
}

#[inline]
pub fn zip_with3<T, U, V, F>(a: Vector3<T>, b: Vector3<U>, f: F) -> Vector3<V>
where
    T: Copy,
    U: Copy,
    F: Fn(T, U) -> V,
{
    let (ax, ay, az) = (a[0], a[1], a[2]);
    let (bx, by, bz) = (b[0], b[1], b[2]);
    [f(ax, bx), f(ay, by), f(az, bz)]
}

#[inline]
pub fn zip_with4<T, U, V, F>(a: Vector4<T>, b: Vector4<U>, f: F) -> Vector4<V>
where
    T: Copy,
    U: Copy,
    F: Fn(T, U) -> V,
{
    let (ax, ay, az, aw) = (a[0], a[1], a[2], a[3]);
    let (bx, by, bz, bw) = (b[0], b[1], b[2], b[3]);
    [f(ax, bx), f(ay, by), f(az, bz), f(aw, bw)]
}

#[inline]
pub fn fold2<T, F>(a: Vector2<T>, f: F) -> T
where
    T: Copy,
    F: Fn(T, T) -> T,
{
    let (ax, ay) = (a[0], a[1]);
    f(ax, ay)
}

#[inline]
pub fn fold3<T, F>(a: Vector3<T>, f: F) -> T
where
    T: Copy,
    F: Fn(T, T) -> T,
{
    let (ax, ay, az) = (a[0], a[1], a[2]);
    f(f(ax, ay), az)
}

#[inline]
pub fn fold4<T, F>(a: Vector4<T>, f: F) -> T
where
    T: Copy,
    F: Fn(T, T) -> T,
{
    let (ax, ay, az, aw) = (a[0], a[1], a[2], a[3]);
    f(f(f(ax, ay), az), aw)
}

#[inline]
pub fn add2<T>(a: Point2<T>, b: Vector2<T>) -> Point2<T>
where
    T: Copy + Add<T, Output = T>,
{
    zip_with2(a, b, Add::add)
}

#[inline]
pub fn add3<T>(a: Point3<T>, b: Vector3<T>) -> Point3<T>
where
    T: Copy + Add<T, Output = T>,
{
    zip_with3(a, b, Add::add)
}

#[inline]
pub fn add4<T>(a: Point4<T>, b: Vector4<T>) -> Point4<T>
where
    T: Copy + Add<T, Output = T>,
{
    zip_with4(a, b, Add::add)
}

#[inline]
pub fn sub2<T>(a: Point2<T>, b: Point2<T>) -> Vector2<T>
where
    T: Copy + Sub<T, Output = T>,
{
    zip_with2(a, b, Sub::sub)
}

#[inline]
pub fn sub3<T>(a: Point3<T>, b: Point3<T>) -> Vector3<T>
where
    T: Copy + Sub<T, Output = T>,
{
    zip_with3(a, b, Sub::sub)
}

#[inline]
pub fn sub4<T>(a: Point4<T>, b: Point4<T>) -> Vector4<T>
where
    T: Copy + Sub<T, Output = T>,
{
    zip_with4(a, b, Sub::sub)
}

#[inline]
pub fn mul2<T>(a: Vector2<T>, b: T) -> Vector2<T>
where
    T: Copy + Mul<T, Output = T>,
{
    zip_with2(a, const2(b), Mul::mul)
}

#[inline]
pub fn mul3<T>(a: Vector3<T>, b: T) -> Vector3<T>
where
    T: Copy + Mul<T, Output = T>,
{
    zip_with3(a, const3(b), Mul::mul)
}

#[inline]
pub fn mul4<T>(a: Vector4<T>, b: T) -> Vector4<T>
where
    T: Copy + Mul<T, Output = T>,
{
    zip_with4(a, const4(b), Mul::mul)
}

#[inline]
pub fn dot2(a: Vector2<f64>, b: Vector2<f64>) -> f64 {
    fold2(zip_with2(a, b, Mul::mul), Add::add)
}

#[inline]
pub fn dot3(a: Vector3<f64>, b: Vector3<f64>) -> f64 {
    fold3(zip_with3(a, b, Mul::mul), Add::add)
}

#[inline]
pub fn dot4(a: Vector4<f64>, b: Vector4<f64>) -> f64 {
    fold4(zip_with4(a, b, Mul::mul), Add::add)
}

#[inline]
pub fn const2<T: Copy>(x: T) -> Vector2<T> {
    [x, x]
}

#[inline]
pub fn const3<T: Copy>(x: T) -> Vector3<T> {
    [x, x, x]
}

#[inline]
pub fn const4<T: Copy>(x: T) -> Vector4<T> {
    [x, x, x, x]
}

#[inline]
pub fn one2<T: Copy + From<i8>>() -> Vector2<T> {
    cast2(const2(1))
}

#[inline]
pub fn one3<T: Copy + From<i8>>() -> Vector3<T> {
    cast3(const3(1))
}

#[inline]
pub fn one4<T: Copy + From<i8>>() -> Vector4<T> {
    cast4(const4(1))
}

#[inline]
pub fn cast2<T, U>(x: Point2<T>) -> Point2<U>
where
    T: Copy,
    U: Copy + From<T>,
{
    map2(x, cast)
}

#[inline]
pub fn cast3<T, U>(x: Point3<T>) -> Point3<U>
where
    T: Copy,
    U: Copy + From<T>,
{
    map3(x, cast)
}

#[inline]
pub fn cast4<T, U>(x: Point4<T>) -> Point4<U>
where
    T: Copy,
    U: Copy + From<T>,
{
    map4(x, cast)
}

// f64 doesn't implement From<isize>
#[inline]
pub fn to_f642(x: Point2<isize>) -> Point2<f64> {
    [x[0] as f64, x[1] as f64]
}

#[inline]
pub fn to_f643(x: Point3<isize>) -> Point3<f64> {
    [x[0] as f64, x[1] as f64, x[2] as f64]
}

#[inline]
pub fn to_f644(x: Point4<isize>) -> Point4<f64> {
    [x[0] as f64, x[1] as f64, x[2] as f64, x[3] as f64]
}

// isize doesn't implement From<f64>
#[inline]
pub fn to_isize2(x: Point2<f64>) -> Point2<isize> {
    [x[0] as isize, x[1] as isize]
}

#[cfg(not(target_os = "emscripten"))]
#[inline]
pub fn scale_shift(value: f64, n: f64) -> f64 {
    value.abs().mul_add(n, -1.0_f64)
}

#[cfg(target_os = "emscripten")]
#[inline]
pub fn scale_shift(value: f64, n: f64) -> f64 {
    (value.abs() * n) + -1.0_f64
}

#[inline]
pub fn to_isize3(x: Point3<f64>) -> Point3<isize> {
    [x[0] as isize, x[1] as isize, x[2] as isize]
}

#[inline]
pub fn to_isize4(x: Point4<f64>) -> Point4<isize> {
    [x[0] as isize, x[1] as isize, x[2] as isize, x[3] as isize]
}

pub mod interp {
    /// Performs linear interploation between two values.
    #[cfg(not(target_os = "emscripten"))]
    #[inline]
    pub fn linear(a: f64, b: f64, x: f64) -> f64 {
        x.mul_add(b - a, a)
    }

    /// Performs linear interploation between two values.
    #[cfg(target_os = "emscripten")]
    #[inline]
    pub fn linear(a: f64, b: f64, x: f64) -> f64 {
        (x * (b - a)) + a
    }

    /// Performs cubic interpolation between two values bound between two other
    /// values.
    ///
    /// - n0 - The value before the first value.
    /// - n1 - The first value.
    /// - n2 - The second value.
    /// - n3 - The value after the second value.
    /// - x - The alpha value.
    ///
    /// The alpha value should range from 0.0 to 1.0. If the alpha value is
    /// 0.0, this function returns _n1_. If the alpha value is 1.0, this
    /// function returns _n2_.
    #[inline]
    pub fn cubic(n0: f64, n1: f64, n2: f64, n3: f64, x: f64) -> f64 {
        let p = (n3 - n2) - (n0 - n1);
        let q = (n0 - n1) - p;
        let r = n2 - n0;
        let s = n1;
        p * x * x * x + q * x * x + r * x + s
    }

    /// Maps a value onto a cubic S-curve.
    #[inline]
    pub fn s_curve3(x: f64) -> f64 {
        x * x * (3.0 - (x * 2.0))
    }

    /// Maps a value onto a quintic S-curve.
    #[inline]
    pub fn s_curve5(x: f64) -> f64 {
        x * x * x * (x * (x * 6.0 - 15.0) + 10.0)
    }
}