
use crate::math::*;
use crate::camera::*;

pub struct CameraControl {
    camera: Camera
}

impl CameraControl {
    pub fn new(camera: Camera) -> Self {
        Self {camera}
    }

    pub fn translate(&mut self, change: &Vec3)
    {
        let position = *self.position();
        let target = *self.target();
        let up = *self.up();
        self.set_view(position + change, target + change, up);
    }

    pub fn rotate(&mut self, x: f32, y: f32)
    {
        let target = *self.target();
        let mut direction = self.target() - self.position();
        let zoom = direction.magnitude();
        direction /= zoom;
        let right = direction.cross(*self.up());
        let up = right.cross(direction);
        let new_pos = self.position() + (-right * x + up * y) * 0.1;
        let new_dir = (self.target() - new_pos).normalize();
        self.set_view(target - new_dir * zoom, target, up);
    }

    pub fn rotate_around_up(&mut self, x: f32, y: f32)
    {
        let target = *self.target();
        let up = *self.up();
        let mut direction = target - self.position();
        let zoom = direction.magnitude();
        direction /= zoom;
        let right = direction.cross(up);
        let new_pos = self.position() + (-right * x + right.cross(direction) * y) * 0.1;
        let new_dir = (self.target() - new_pos).normalize();
        if new_dir.dot(up).abs() < 0.999 {
            self.set_view(target - new_dir * zoom, target, up);
        }
    }

    pub fn pan(&mut self, x: f32, y: f32)
    {
        let position = *self.position();
        let target = *self.target();
        let up = *self.up();
        let mut direction = target - position;
        let zoom = direction.magnitude();
        direction /= zoom;
        let right = direction.cross(up);
        let delta = (-right * x + right.cross(direction) * y) * zoom * 0.005;
        self.set_view(position + delta, target + delta, up);
    }

    pub fn zoom(&mut self, wheel: f32)
    {
        match self.projection_type() {
            ProjectionType::Orthographic {width, height, depth} => {
                let h = (height - wheel).max(0.001);
                let w = h * width / height;
                let d = *depth;
                self.set_orthographic_projection(w, h, d);
            },
            ProjectionType::Perspective {..} => {
                let position = *self.position();
                let target = *self.target();
                let up = *self.up();
                let mut direction = target - position;
                let mut zoom = direction.magnitude();
                direction /= zoom;
                zoom += wheel;
                zoom = zoom.max(1.0);
                self.set_view(target - direction * zoom, target, up);
            }
        }
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