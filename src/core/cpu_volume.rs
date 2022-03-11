use crate::core::*;

///
/// A CPU-side version of volume data consisting of voxels.
///
#[derive(Debug)]
pub struct CpuVolume {
    /// Name.
    pub name: String,

    pub voxels: CpuTexture3D<u8>,

    pub size: Vec3,
}

impl std::default::Default for CpuVolume {
    fn default() -> Self {
        Self {
            name: String::default(),
            voxels: CpuTexture3D::<u8>::default(),
            size: vec3(2.0, 2.0, 2.0),
        }
    }
}
