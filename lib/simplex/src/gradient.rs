use crate::math;
use std;

#[inline(always)]
#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn get2(index: usize) -> math::Vector2<f64> {
    // Vectors are combinations of -1, 0, and 1, precompute the normalized element
    let diag = std::f64::consts::FRAC_1_SQRT_2;

    match index % 8 {
        0 => [  1.0,   0.0],
        1 => [ -1.0,   0.0],
        2 => [  0.0,   1.0],
        3 => [  0.0,  -1.0],
        4 => [ diag,  diag],
        5 => [-diag,  diag],
        6 => [ diag, -diag],
        7 => [-diag, -diag],
        _ => panic!("Attempt to access gradient {} of 8", index % 8),
    }
}

#[inline(always)]
#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn get3(index: usize) -> math::Vector3<f64> {
    // Vectors are combinations of -1, 0, and 1, precompute the normalized elements
    let diag = std::f64::consts::FRAC_1_SQRT_2;
    let diag2 = 0.5773502691896258;

    match index % 32 {
        // 12 edges repeated twice then 8 corners
        0  | 12 => [  diag,   diag,    0.0],
        1  | 13 => [ -diag,   diag,    0.0],
        2  | 14 => [  diag,  -diag,    0.0],
        3  | 15 => [ -diag,  -diag,    0.0],
        4  | 16 => [  diag,    0.0,   diag],
        5  | 17 => [ -diag,    0.0,   diag],
        6  | 18 => [  diag,    0.0,  -diag],
        7  | 19 => [ -diag,    0.0,  -diag],
        8  | 20 => [   0.0,   diag,   diag],
        9  | 21 => [   0.0,  -diag,   diag],
        10 | 22 => [   0.0,   diag,  -diag],
        11 | 23 => [   0.0,  -diag,  -diag],
        24      => [ diag2,  diag2,  diag2],
        25      => [-diag2,  diag2,  diag2],
        26      => [ diag2, -diag2,  diag2],
        27      => [-diag2, -diag2,  diag2],
        28      => [ diag2,  diag2, -diag2],
        29      => [-diag2,  diag2, -diag2],
        30      => [ diag2, -diag2, -diag2],
        31      => [-diag2, -diag2, -diag2],
        _       => panic!("Attempt to access gradient {} of 32", index % 32),
    }
}

#[inline(always)]
#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn get4(index: usize) -> math::Vector4<f64> {
    // Vectors are combinations of -1, 0, and 1, precompute the normalized elements
    let diag = 0.5773502691896258;
    let diag2 = 0.5;

    match index % 64 {
        // 32 edges then 16 corners repeated twice
        0       => [   0.0,   diag,   diag,   diag],
        1       => [   0.0,   diag,   diag,  -diag],
        2       => [   0.0,   diag,  -diag,   diag],
        3       => [   0.0,   diag,  -diag,  -diag],
        4       => [   0.0,  -diag,   diag,   diag],
        5       => [   0.0,  -diag,   diag,  -diag],
        6       => [   0.0,  -diag,  -diag,   diag],
        7       => [   0.0,  -diag,  -diag,  -diag],
        8       => [  diag,    0.0,   diag,   diag],
        9       => [  diag,    0.0,   diag,  -diag],
        10      => [  diag,    0.0,  -diag,   diag],
        11      => [  diag,    0.0,  -diag,  -diag],
        12      => [ -diag,    0.0,   diag,   diag],
        13      => [ -diag,    0.0,   diag,  -diag],
        14      => [ -diag,    0.0,  -diag,   diag],
        15      => [ -diag,    0.0,  -diag,  -diag],
        16      => [  diag,   diag,    0.0,   diag],
        17      => [  diag,   diag,    0.0,  -diag],
        18      => [  diag,  -diag,    0.0,   diag],
        19      => [  diag,  -diag,    0.0,  -diag],
        20      => [ -diag,   diag,    0.0,   diag],
        21      => [ -diag,   diag,    0.0,  -diag],
        22      => [ -diag,  -diag,    0.0,   diag],
        23      => [ -diag,  -diag,    0.0,  -diag],
        24      => [  diag,   diag,   diag,    0.0],
        25      => [  diag,   diag,  -diag,    0.0],
        26      => [  diag,  -diag,   diag,    0.0],
        27      => [  diag,  -diag,  -diag,    0.0],
        28      => [ -diag,   diag,   diag,    0.0],
        29      => [ -diag,   diag,  -diag,    0.0],
        30      => [ -diag,  -diag,   diag,    0.0],
        31      => [ -diag,  -diag,  -diag,    0.0],
        32 | 48 => [ diag2,  diag2,  diag2,  diag2],
        33 | 49 => [-diag2,  diag2,  diag2,  diag2],
        34 | 50 => [ diag2, -diag2,  diag2,  diag2],
        35 | 51 => [-diag2, -diag2,  diag2,  diag2],
        36 | 52 => [ diag2,  diag2, -diag2,  diag2],
        37 | 53 => [-diag2,  diag2, -diag2,  diag2],
        38 | 54 => [ diag2,  diag2,  diag2, -diag2],
        39 | 55 => [-diag2,  diag2,  diag2, -diag2],
        40 | 56 => [ diag2, -diag2, -diag2,  diag2],
        41 | 57 => [-diag2, -diag2, -diag2,  diag2],
        42 | 58 => [ diag2, -diag2,  diag2, -diag2],
        43 | 59 => [-diag2, -diag2,  diag2, -diag2],
        44 | 60 => [ diag2,  diag2, -diag2, -diag2],
        45 | 61 => [-diag2,  diag2, -diag2, -diag2],
        46 | 62 => [ diag2, -diag2, -diag2, -diag2],
        47 | 63 => [-diag2, -diag2, -diag2, -diag2],
        _       => panic!("Attempt to access gradient {} of 64", index % 64),
    }
}