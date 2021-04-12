use crate::camera::*;
use crate::core::Error;
use crate::math::*;

///
/// 3D controls for a camera. Use this to add additional control functionality to a [camera](crate::Camera).
///
pub struct CameraControl {
    camera: Camera,
}

impl CameraControl {
    pub fn new(camera: Camera) -> Self {
        Self { camera }
    }

    pub fn translate(&mut self, change: &Vec3) -> Result<(), Error> {
        let position = *self.position();
        let target = *self.target();
        let up = *self.up();
        self.set_view(position + change, target + change, up)?;
        Ok(())
    }

    pub fn rotate_around(&mut self, point: &Vec3, x: f32, y: f32) -> Result<(), Error> {
        let dir = (point - self.position()).normalize();
        let right = dir.cross(*self.up());
        let up = right.cross(dir);
        let new_pos = self.position() - right * x + up * y;
        let new_dir = (point - new_pos).normalize();
        let dist = point.distance(*self.position());
        let target = *self.target();
        self.set_view(point - dist * new_dir, target, up)?;
        Ok(())
    }

    pub fn rotate_around_up(&mut self, point: &Vec3, x: f32, y: f32) -> Result<(), Error> {
        let dir = (point - self.position()).normalize();
        let right = dir.cross(*self.up());
        let mut up = right.cross(dir);
        let new_pos = self.position() - right * x + up * y;
        let new_dir = (point - new_pos).normalize();
        up = *self.up();
        if new_dir.dot(up).abs() < 0.999 {
            let dist = point.distance(*self.position());
            let target = *self.target();
            self.set_view(point - dist * new_dir, target, up)?;
        }
        Ok(())
    }

    pub fn pan(&mut self, x: f32, y: f32) -> Result<(), Error> {
        let right = self.right_direction();
        let up = right.cross(self.view_direction());
        let delta = -right * x + up * y;
        self.translate(&(delta * self.distance_to_target()))?;
        Ok(())
    }

    pub fn zoom(&mut self, wheel: f32, min: f32, max: f32) -> Result<(), Error> {
        match self.projection_type() {
            ProjectionType::Orthographic {
                width,
                height,
                depth,
            } => {
                let h = (height - wheel * self.distance_to_target())
                    .max(min)
                    .min(max);
                let w = h * width / height;
                let d = *depth;
                self.set_orthographic_projection(w, h, d)?;
            }
            ProjectionType::Perspective { .. } => {
                let target = *self.target();
                let up = *self.up();
                let direction = self.view_direction();
                let mut zoom = (wheel + 1.0) * self.distance_to_target();
                zoom = zoom.max(min).min(max);
                self.set_view(target - direction * zoom, target, up)?;
            }
        }
        Ok(())
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
