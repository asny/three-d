use crate::camera::*;

///
/// 3D controls for a camera. Use this to add additional control functionality to a [camera](crate::Camera).
///
pub struct CameraControl {
    camera: Camera,
}

impl CameraControl {
    ///
    /// Extends the given camera with additional functionality for camera control.
    ///
    pub fn new(camera: Camera) -> Self {
        Self { camera }
    }
}

impl std::ops::Deref for CameraControl {
    type Target = Camera;

    fn deref(&self) -> &Self::Target {
        &self.camera
    }
}

impl std::ops::DerefMut for CameraControl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.camera
    }
}
