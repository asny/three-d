use crate::renderer::*;
use crate::window::*;

pub struct FlyControl {
    control: CameraControl,
}

impl FlyControl {
    pub fn new(speed: f32) -> Self {
        Self {
            control: CameraControl {
                left_drag_horizontal: CameraAction::Yaw {
                    speed: std::f32::consts::PI / 1800.0,
                },
                left_drag_vertical: CameraAction::Pitch {
                    speed: std::f32::consts::PI / 1800.0,
                },
                scroll_vertical: CameraAction::Forward { speed },
                right_drag_horizontal: CameraAction::Left { speed },
                right_drag_vertical: CameraAction::Up { speed },
                ..Default::default()
            },
        }
    }

    pub fn handle_events(
        &mut self,
        camera: &mut Camera,
        events: &mut [Event],
    ) -> Result<bool, Error> {
        self.control.handle_events(camera, events)
    }
}
