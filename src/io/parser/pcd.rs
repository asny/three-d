use crate::core::*;
use crate::io::*;
use pcd_rs::anyhow::Result;
use pcd_rs::{PcdDeserialize, Reader};
use std::mem;
use std::path::Path;

#[derive(PcdDeserialize)]
struct PcdPoint {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(PcdDeserialize)]
struct PcdPointWithColor {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub rgb: f32,
}

fn decode_color(PcdPointWithColor { rgb, .. }: &PcdPointWithColor) -> Color {
    unsafe {
        let rgb: u32 = mem::transmute_copy(rgb);
        let r = ((rgb >> 16) & 255).try_into().unwrap();
        let g = ((rgb >> 8) & 255).try_into().unwrap();
        let b = (rgb & 255).try_into().unwrap();
        Color { r, g, b, a: 1 }
    }
}

fn min(point: &Vector3<f32>) -> f32 {
    f32::min(point.x, f32::min(point.y, point.z))
}

fn max(point: &Vector3<f32>) -> f32 {
    f32::max(point.x, f32::max(point.y, point.z))
}

fn normalize_value(value: f32, min: &f32, max: &f32) -> f32 {
    (2.0 * ((value - min) / (max - min))) - 1.0
}

fn normalize_point(point: &Vector3<f32>, min: f32, max: f32) -> Vector3<f32> {
    return Vector3 {
        x: normalize_value(point.x, &min, &max),
        y: normalize_value(point.y, &min, &max),
        z: normalize_value(point.z, &min, &max),
    };
}

impl Loaded {
    ///
    /// Deserialize a loaded .pcd file into a [CpuPointCloud].
    ///
    pub fn pcd(
        &mut self,
        path: impl AsRef<Path>,
        color: bool,
        normalize: bool,
    ) -> ThreeDResult<CpuPointCloud> {
        let (mut positions, colors) = if color {
            let reader = Reader::open(path)?;
            let colored_points: Result<Vec<PcdPointWithColor>> = reader.collect();
            let colored_points = colored_points?;
            let positions: Vec<_> = colored_points
                .iter()
                .map(|p| Vec3 {
                    x: p.x,
                    y: p.y,
                    z: p.z,
                })
                .collect();
            let colors: Option<Vec<_>> = Some(colored_points.iter().map(decode_color).collect());

            (positions, colors)
        } else {
            let reader = Reader::open(path)?;
            let points: Result<Vec<PcdPoint>> = reader.collect();
            let points = points?;
            let positions: Vec<_> = points
                .iter()
                .map(|p| Vec3 {
                    x: p.x,
                    y: p.y,
                    z: p.z,
                })
                .collect();
            let colors = None;

            (positions, colors)
        };

        if normalize {
            let max = positions.iter().map(max).fold(0.0, f32::max);
            let min = positions.iter().map(min).fold(0.0, f32::min);
            for i in 0..positions.len() {
                positions[i] = normalize_point(&positions[i], min, max)
            }
        }

        Ok(CpuPointCloud {
            positions: Positions::F32(positions),
            colors,
            ..Default::default()
        })
    }
}
