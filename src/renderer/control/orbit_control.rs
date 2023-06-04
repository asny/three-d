use crate::renderer::*;

///
/// A control that makes the camera orbit around a target.
///
pub struct OrbitControl {
    control: CameraControl,
}

impl OrbitControl {
    /// Creates a new orbit control with the given target and minimum and maximum distance to the target.
    pub fn new(target: Vec3, min_distance: f32, max_distance: f32) -> Self {
        Self {
            control: CameraControl {
                left_drag_horizontal: CameraAction::OrbitLeft { target, speed: 0.1 },
                left_drag_vertical: CameraAction::OrbitUp { target, speed: 0.1 },
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

    /// Handles the events. Must be called each frame.
    pub fn handle_events(&mut self, camera: &mut Camera, events: &mut [Event]) -> bool {
        if let CameraAction::Zoom { speed, target, .. } = &mut self.control.scroll_vertical {
            let x = target.distance(*camera.position());
            *speed = 0.01 * x + 0.001;
        }
        if let CameraAction::OrbitLeft { speed, target } = &mut self.control.left_drag_horizontal {
            let x = target.distance(*camera.position());
            *speed = 0.01 * x + 0.001;
        }
        if let CameraAction::OrbitUp { speed, target } = &mut self.control.left_drag_vertical {
            let x = target.distance(*camera.position());
            *speed = 0.01 * x + 0.001;
        }
        self.control.handle_events(camera, events)
    }
}
