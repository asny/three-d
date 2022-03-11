use crate::core::*;
use crate::io::*;
use std::path::Path;

impl Loaded {
    ///
    /// Deserialize a loaded .vol file into a [CpuVolume].
    ///
    /// **Note:** Border is not supported.
    ///
    pub fn vol(&mut self, path: impl AsRef<Path>) -> ThreeDResult<CpuVolume<u8>> {
        let bytes = self.remove_bytes(path.as_ref())?;
        let width = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        let height = u32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        let depth = u32::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);
        let format = match (bytes.len() as u32 - 28) / (width * height * depth) {
            1 => Ok(Format::R),
            2 => Ok(Format::RG),
            3 => Ok(Format::RGB),
            4 => Ok(Format::RGBA),
            _ => Err(IOError::VolCorruptData),
        }?;
        Ok(CpuVolume {
            voxels: CpuTexture3D {
                data: bytes[28..].to_vec(),
                width,
                height,
                depth,
                format,
                ..Default::default()
            },
            size: vec3(
                f32::from_be_bytes([bytes[16], bytes[17], bytes[18], bytes[19]]),
                f32::from_be_bytes([bytes[20], bytes[21], bytes[22], bytes[23]]),
                f32::from_be_bytes([bytes[24], bytes[25], bytes[26], bytes[27]]),
            ),
            ..Default::default()
        })
    }
}
