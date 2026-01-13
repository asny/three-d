//!
//! Utilities for cross-generating PBR textures.
//!

use crate::core::*;

/// Extracts the red channel from a CpuTexture as normalized f32 values (0.0 to 1.0).
/// Works with all supported texture formats.
fn extract_red_channel(cpu_texture: &CpuTexture) -> Vec<f32> {
    match &cpu_texture.data {
        TextureData::RU8(data) => data.iter().map(|&v| v as f32 / 255.0).collect(),
        TextureData::RgU8(data) => data.iter().map(|v| v[0] as f32 / 255.0).collect(),
        TextureData::RgbU8(data) => data.iter().map(|v| v[0] as f32 / 255.0).collect(),
        TextureData::RgbaU8(data) => data.iter().map(|v| v[0] as f32 / 255.0).collect(),
        TextureData::RF16(data) => data.iter().map(|v| v.to_f32()).collect(),
        TextureData::RgF16(data) => data.iter().map(|v| v[0].to_f32()).collect(),
        TextureData::RgbF16(data) => data.iter().map(|v| v[0].to_f32()).collect(),
        TextureData::RgbaF16(data) => data.iter().map(|v| v[0].to_f32()).collect(),
        TextureData::RF32(data) => data.clone(),
        TextureData::RgF32(data) => data.iter().map(|v| v[0]).collect(),
        TextureData::RgbF32(data) => data.iter().map(|v| v[0]).collect(),
        TextureData::RgbaF32(data) => data.iter().map(|v| v[0]).collect(),
    }
}

/// Samples a texel from a flat data array with clamped boundary handling.
#[inline]
fn get_texel_clamped(data: &[f32], width: u32, height: u32, x: i32, y: i32) -> f32 {
    let x = x.clamp(0, width as i32 - 1) as usize;
    let y = y.clamp(0, height as i32 - 1) as usize;
    data[y * width as usize + x]
}

/// Generates a normal map from a heightmap using Sobel filtering.
///
/// # Arguments
/// * `heightmap` - The source heightmap texture (height values read from red channel)
/// * `strength` - Normal strength multiplier (typical range: 1.0 to 10.0)
///
/// # Returns
/// A new CpuTexture containing the generated normal map in tangent space (RGB format).
/// The normals are stored as `(normal * 0.5 + 0.5)` to fit in [0, 1] range.
pub fn heightmap_to_normal(heightmap: &CpuTexture, strength: f32) -> CpuTexture {
    let width = heightmap.width;
    let height = heightmap.height;

    // Handle empty texture
    if width == 0 || height == 0 {
        return CpuTexture {
            name: format!("{}_normal", heightmap.name),
            data: TextureData::RgbU8(vec![]),
            width,
            height,
            min_filter: heightmap.min_filter,
            mag_filter: heightmap.mag_filter,
            mipmap: heightmap.mipmap,
            wrap_s: heightmap.wrap_s,
            wrap_t: heightmap.wrap_t,
        };
    }

    let heights = extract_red_channel(heightmap);

    let mut normals: Vec<[u8; 3]> = Vec::with_capacity((width * height) as usize);

    for y in 0..height as i32 {
        for x in 0..width as i32 {
            // Sobel filter for better quality gradients
            let tl = get_texel_clamped(&heights, width, height, x - 1, y - 1);
            let t = get_texel_clamped(&heights, width, height, x, y - 1);
            let tr = get_texel_clamped(&heights, width, height, x + 1, y - 1);
            let l = get_texel_clamped(&heights, width, height, x - 1, y);
            let r = get_texel_clamped(&heights, width, height, x + 1, y);
            let bl = get_texel_clamped(&heights, width, height, x - 1, y + 1);
            let b = get_texel_clamped(&heights, width, height, x, y + 1);
            let br = get_texel_clamped(&heights, width, height, x + 1, y + 1);

            // Sobel operators
            let dx = (tr + 2.0 * r + br) - (tl + 2.0 * l + bl);
            let dy = (bl + 2.0 * b + br) - (tl + 2.0 * t + tr);

            // Construct normal vector
            let nx = -dx * strength;
            let ny = -dy * strength;
            let nz = 1.0;

            // Normalize
            let len = (nx * nx + ny * ny + nz * nz).sqrt();
            let nx = nx / len;
            let ny = ny / len;
            let nz = nz / len;

            // Pack to [0, 255] range: normal * 0.5 + 0.5
            normals.push([
                ((nx * 0.5 + 0.5) * 255.0) as u8,
                ((ny * 0.5 + 0.5) * 255.0) as u8,
                ((nz * 0.5 + 0.5) * 255.0) as u8,
            ]);
        }
    }

    CpuTexture {
        name: format!("{}_normal", heightmap.name),
        data: TextureData::RgbU8(normals),
        width,
        height,
        min_filter: heightmap.min_filter,
        mag_filter: heightmap.mag_filter,
        mipmap: heightmap.mipmap,
        wrap_s: heightmap.wrap_s,
        wrap_t: heightmap.wrap_t,
    }
}

/// Generates an ambient occlusion map from a heightmap using horizon-based ray tracing.
///
/// For each texel, traces rays in multiple directions along the heightmap surface
/// to determine how much of the hemisphere is occluded by nearby elevated regions.
///
/// # Arguments
/// * `heightmap` - The source heightmap texture (height values read from red channel)
/// * `ray_count` - Number of rays to trace per texel (typical: 4-16, higher = better quality but slower)
/// * `max_distance` - Maximum ray distance in texels (typical: 8-32)
/// * `intensity` - AO intensity multiplier (typical: 1.0 to 2.0)
/// * `angle_offset` - Rotation offset in radians to avoid axis-aligned artifacts.
///   Use ~0.1 radians (≈6 deg) for good results. Use 0.0 for no offset.
///
/// # Returns
/// A new CpuTexture containing the generated ambient occlusion map (grayscale, R channel).
/// White (1.0) = fully lit, Black (0.0) = fully occluded.
pub fn heightmap_to_ao(
    heightmap: &CpuTexture,
    ray_count: u32,
    max_distance: u32,
    intensity: f32,
    angle_offset: f32,
) -> CpuTexture {
    let width = heightmap.width;
    let height = heightmap.height;

    // Handle empty texture or zero ray count - return white (no occlusion) texture
    if width == 0 || height == 0 || ray_count == 0 {
        return CpuTexture {
            name: format!("{}_ao", heightmap.name),
            data: TextureData::RU8(vec![255; (width * height) as usize]),
            width,
            height,
            min_filter: heightmap.min_filter,
            mag_filter: heightmap.mag_filter,
            mipmap: heightmap.mipmap,
            wrap_s: heightmap.wrap_s,
            wrap_t: heightmap.wrap_t,
        };
    }

    let heights = extract_red_channel(heightmap);

    let mut ao_values: Vec<u8> = Vec::with_capacity((width * height) as usize);

    // Precompute ray directions (evenly distributed around circle with offset to avoid axis alignment)
    let ray_dirs: Vec<(f32, f32)> = (0..ray_count)
        .map(|i| {
            let angle = (i as f32 / ray_count as f32) * std::f32::consts::TAU + angle_offset;
            (angle.cos(), angle.sin())
        })
        .collect();

    for y in 0..height as i32 {
        for x in 0..width as i32 {
            let center_height = get_texel_clamped(&heights, width, height, x, y);
            let mut total_occlusion = 0.0;

            // Trace rays in each direction
            for &(dx, dy) in &ray_dirs {
                let mut max_horizon_angle = 0.0_f32;

                // March along the ray
                for step in 1..=max_distance {
                    let sample_x = x as f32 + dx * step as f32;
                    let sample_y = y as f32 + dy * step as f32;

                    // Bilinear sample would be better, but for simplicity use nearest
                    let sx = sample_x.round() as i32;
                    let sy = sample_y.round() as i32;

                    // Skip if out of bounds
                    if sx < 0 || sx >= width as i32 || sy < 0 || sy >= height as i32 {
                        break;
                    }

                    let sample_height = get_texel_clamped(&heights, width, height, sx, sy);
                    let height_diff = sample_height - center_height;
                    let distance = step as f32;

                    // Calculate horizon angle (atan2 of height difference over distance)
                    // Using a simpler approximation for speed
                    let horizon_angle = (height_diff / distance).atan();
                    max_horizon_angle = max_horizon_angle.max(horizon_angle);
                }

                // Convert horizon angle to occlusion (0 = no occlusion, 1 = full occlusion)
                // Normalize from [-PI/2, PI/2] range, but we only care about positive angles
                let occlusion = (max_horizon_angle * 2.0 / std::f32::consts::PI).max(0.0);
                total_occlusion += occlusion;
            }

            // Average occlusion across all rays and apply intensity
            let avg_occlusion = (total_occlusion / ray_count as f32) * intensity;
            // Invert: high occlusion = dark, convert to [0, 255]
            let ao = ((1.0 - avg_occlusion).clamp(0.0, 1.0) * 255.0) as u8;
            ao_values.push(ao);
        }
    }

    CpuTexture {
        name: format!("{}_ao", heightmap.name),
        data: TextureData::RU8(ao_values),
        width,
        height,
        min_filter: heightmap.min_filter,
        mag_filter: heightmap.mag_filter,
        mipmap: heightmap.mipmap,
        wrap_s: heightmap.wrap_s,
        wrap_t: heightmap.wrap_t,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heightmap_to_normal_flat() {
        // A flat heightmap should produce normals pointing straight up (0, 0, 1)
        let heightmap = CpuTexture {
            name: "test".to_string(),
            data: TextureData::RU8(vec![128; 16]), // 4x4 flat
            width: 4,
            height: 4,
            min_filter: Interpolation::Linear,
            mag_filter: Interpolation::Linear,
            mipmap: None,
            wrap_s: Wrapping::Repeat,
            wrap_t: Wrapping::Repeat,
        };

        let normal_map = heightmap_to_normal(&heightmap, 1.0);

        if let TextureData::RgbU8(data) = &normal_map.data {
            // Center should be (0.5, 0.5, 1.0) in packed form = (127, 127, 255)
            for pixel in data {
                // Allow some tolerance due to edge effects
                assert!(pixel[2] > 250, "Z component should be near 1.0");
            }
        } else {
            panic!("Expected RgbU8 output");
        }
    }

    #[test]
    fn test_heightmap_to_ao_flat() {
        // A flat heightmap should produce white AO (no occlusion)
        let heightmap = CpuTexture {
            name: "test".to_string(),
            data: TextureData::RU8(vec![128; 16]), // 4x4 flat
            width: 4,
            height: 4,
            min_filter: Interpolation::Linear,
            mag_filter: Interpolation::Linear,
            mipmap: None,
            wrap_s: Wrapping::Repeat,
            wrap_t: Wrapping::Repeat,
        };

        let ao_map = heightmap_to_ao(&heightmap, 8, 4, 1.0, 0.0);

        if let TextureData::RU8(data) = &ao_map.data {
            for &value in data {
                // Flat surface should have minimal occlusion (high AO value)
                assert!(value > 200, "Flat surface should have minimal occlusion");
            }
        } else {
            panic!("Expected RU8 output");
        }
    }

    #[test]
    fn test_heightmap_to_ao_zero_rays() {
        // Zero rays should return white (no occlusion)
        let heightmap = CpuTexture {
            name: "test".to_string(),
            data: TextureData::RU8(vec![128; 4]),
            width: 2,
            height: 2,
            min_filter: Interpolation::Linear,
            mag_filter: Interpolation::Linear,
            mipmap: None,
            wrap_s: Wrapping::Repeat,
            wrap_t: Wrapping::Repeat,
        };

        let ao_map = heightmap_to_ao(&heightmap, 0, 4, 1.0, 0.0);

        if let TextureData::RU8(data) = &ao_map.data {
            for &value in data {
                assert_eq!(value, 255, "Zero rays should produce white (no occlusion)");
            }
        } else {
            panic!("Expected RU8 output");
        }
    }
}
