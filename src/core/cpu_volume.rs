use crate::core::*;

///
/// A CPU-side version of volume data consisting of voxels.
///
#[derive(Debug)]
pub struct CpuVolume<T: TextureDataType> {
    /// Name.
    pub name: String,

    pub voxels: CpuTexture3D<T>,

    pub size: Vec3,
}

impl<T: TextureDataType> std::default::Default for CpuVolume<T> {
    fn default() -> Self {
        Self {
            name: String::default(),
            voxels: CpuTexture3D::default(),
            size: vec3(2.0, 2.0, 2.0),
        }
    }
}
