mod tone_mapping;
pub use tone_mapping::*;

use crate::core::*;

#[derive(Clone, Debug)]
pub struct Camera {
    camera: three_d_asset::Camera,
    /// Tone mapping applied to the final color
    pub tone_mapping: ToneMapping,
}

impl Camera {
    pub fn new_orthographic(
        viewport: Viewport,
        position: Vec3,
        target: Vec3,
        up: Vec3,
        height: f32,
        z_near: f32,
        z_far: f32,
    ) -> Self {
        Self {
            camera: three_d_asset::Camera::new_orthographic(
                viewport, position, target, up, height, z_near, z_far,
            ),
            tone_mapping: ToneMapping::default(),
        }
    }
    pub fn new_perspective(
        viewport: Viewport,
        position: Vec3,
        target: Vec3,
        up: Vec3,
        field_of_view_y: impl Into<Radians>,
        z_near: f32,
        z_far: f32,
    ) -> Self {
        Self {
            camera: three_d_asset::Camera::new_perspective(
                viewport,
                position,
                target,
                up,
                field_of_view_y,
                z_near,
                z_far,
            ),
            tone_mapping: ToneMapping::default(),
        }
    }
}

use std::ops::Deref;
impl Deref for Camera {
    type Target = three_d_asset::Camera;
    fn deref(&self) -> &Self::Target {
        &self.camera
    }
}

impl std::ops::DerefMut for Camera {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.camera
    }
}
