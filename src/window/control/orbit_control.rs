use crate::core::*;
use crate::window::*;

pub struct OrbitControl {
    control: CameraControl,
}

impl OrbitControl {
    pub fn new(target: Vec3, min_distance: f32, max_distance: f32) -> Self {
        Self {
            control: CameraControl {
                left_drag_horizontal: CameraAction::OrbitLeft { target, speed: 0.5 },
                left_drag_vertical: CameraAction::OrbitUp { target, speed: 0.5 },
                scroll_vertical: CameraAction::Zoom {
                    min: min_distance,
                    max: max_distance,
                    speed: 0.1,
                    target,
                },
                ..Default::default()
            },
        }
    }

    pub fn handle_events(&mut self, camera: &mut Camera, events: &mut [Event]) -> Result<bool> {
        if let CameraAction::Zoom { speed, target, .. } = &mut self.control.scroll_horizontal {
            *speed = 0.1 / target.distance(*camera.position());
        }
        self.control.handle_events(camera, events)
    }
}
