use camera;
use geo_proc::*;

pub enum CameraState
{
    FIRST, SPHERICAL
}

pub struct CameraHandler {
    pub state: CameraState
}


impl CameraHandler
{
    pub fn create() -> CameraHandler
    {
        CameraHandler {state: CameraState::FIRST}
    }

    pub fn set_state(&mut self, state: CameraState)
    {
        self.state = state;
    }

    pub fn next_state(&mut self)
    {
        match self.state {
            CameraState::FIRST => {self.set_state(CameraState::SPHERICAL);},
            CameraState::SPHERICAL => {self.set_state(CameraState::FIRST);}
        }

    }

    pub fn translate(&mut self, camera: &mut camera::Camera, position: &Vec3, view_direction: &Vec3, up: &Vec3)
    {
        match self.state {
            CameraState::FIRST => {
                camera.set_view(*position, *position + *view_direction, *up);
            },
            CameraState::SPHERICAL => {
                let camera_position = *camera.position();
                let change = *position - *camera.target();
                camera.set_view(camera_position + change, *position, *up);
            }
        }
    }

    pub fn rotate(&mut self, camera: &mut camera::Camera, xrel: i32, yrel: i32)
    {
        match self.state {
            CameraState::SPHERICAL => {
                let x = -xrel as f32;
                let y = yrel as f32;
                let direction = (*camera.target() - *camera.position()).normalize();
                let mut up_direction = vec3(0., 1., 0.);
                let right_direction = direction.cross(&up_direction);
                up_direction = right_direction.cross(&direction);
                let mut camera_position = *camera.position();
                let target = *camera.target();
                let zoom = (camera_position - target).norm();
                camera_position = camera_position + (right_direction * x + up_direction * y) * 0.1;
                camera_position = target + (camera_position - target).normalize() * zoom;
                camera.set_view(camera_position, target, up_direction);
            },
            _ => {}
        }
    }

    pub fn zoom(&mut self, camera: &mut camera::Camera, wheel: i32)
    {
        match self.state {
            CameraState::SPHERICAL => {
                let mut position = *camera.position();
                let target = *camera.target();
                let up = *camera.up();
                let mut zoom = (position - target).norm();
                zoom += wheel as f32;
                zoom = zoom.max(1.0);
                position = target + (*camera.position() - *camera.target()).normalize() * zoom;
                camera.set_view(position, target, up);
            },
            _ => {}
        }
    }
}
