use crate::core::*;
use crate::io::*;
use std::path::Path;

impl Loaded {
    ///
    /// Deserialize a loaded .vol file into a [CpuVolume].
    ///
    /// **Note:** Border is not supported.
    ///
    pub fn vol(&mut self, path: impl AsRef<Path>) -> ThreeDResult<CpuVolume> {
        let bytes = self.remove_bytes(path.as_ref())?;
        let width = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        let height = u32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        let depth = u32::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);
        let size = vec3(
            f32::from_be_bytes([bytes[16], bytes[17], bytes[18], bytes[19]]),
            f32::from_be_bytes([bytes[20], bytes[21], bytes[22], bytes[23]]),
            f32::from_be_bytes([bytes[24], bytes[25], bytes[26], bytes[27]]),
        );
        let bytes = &bytes[28..];
        let data = match (bytes.len() as u32 - 28) / (width * height * depth) {
            1 => TextureData::RU8(bytes.to_vec()),
            2 => {
                let mut data = Vec::new();
                for i in 0..bytes.len() / 2 {
                    data.push(vec2(bytes[i * 2], bytes[i * 2 + 1]));
                }
                TextureData::RgU8(data)
            }
            3 => {
                let mut data = Vec::new();
                for i in 0..bytes.len() / 3 {
                    data.push(vec3(bytes[i * 3], bytes[i * 3 + 1], bytes[i * 3 + 2]));
                }
                TextureData::RgbU8(data)
            }
            4 => {
                let mut data = Vec::new();
                for i in 0..bytes.len() / 4 {
                    data.push(vec4(
                        bytes[i * 4],
                        bytes[i * 4 + 1],
                        bytes[i * 4 + 2],
                        bytes[i * 4 + 3],
                    ));
                }
                TextureData::RgbaU8(data)
            }
            _ => Err(IOError::VolCorruptData)?,
        };
        Ok(CpuVolume {
            voxels: CpuTexture3D {
                data,
                width,
                height,
                depth,
                ..Default::default()
            },
            size,
            ..Default::default()
        })
    }
}
