use crate::core::*;

///
/// Volume data consisting of voxel data inside a cube.
///
#[derive(Debug)]
pub struct CpuVolume<T: TextureDataType> {
    /// Name.
    pub name: String,

    /// Voxel data, ie. small cubes in 3D (analogue to pixels in 2D) that contain 1-4 values.
    pub voxels: CpuTexture3D<T>,

    /// The size of the cube that is spanned by the voxel data.
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
