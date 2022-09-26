use super::*;
use crate::renderer::*;

///
/// A control that makes the camera move like it is a person on the ground.
///
pub struct FirstPersonControl {
    control: CameraControl,
}

impl FirstPersonControl {
    /// Creates a new first person control with the given speed of movements.
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
                ..Default::default()
            },
        }
    }

    /// Handles the events. Must be called each frame.
    pub fn handle_events(&mut self, camera: &mut Camera, events: &mut [Event]) -> bool {
        self.control.handle_events(camera, events)
    }
}
