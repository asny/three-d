use crate::renderer::*;

///
/// A control that makes the camera move like it is a person on the ground.
///
#[derive(Clone, Copy, Debug)]
pub struct FirstPersonControl {
    /// The speed of movements.
    pub speed: f32,
}

impl FirstPersonControl {
    /// Creates a new first person control with the given speed of movements.
    pub fn new(speed: f32) -> Self {
        Self { speed }
    }

    /// Handles the events. Must be called each frame.
    pub fn handle_events(&mut self, camera: &mut Camera, events: &mut [Event]) -> bool {
        let mut change = false;
        for event in events.iter_mut() {
            match event {
                Event::MouseMotion {
                    delta,
                    button,
                    handled,
                    ..
                } => {
                    if !*handled {
                        if Some(MouseButton::Left) == *button {
                            camera.yaw(radians(delta.0 * std::f32::consts::PI / 1800.0));
                            camera.pitch(radians(delta.1 * std::f32::consts::PI / 1800.0));
                            *handled = true;
                            change = true;
                        }
                    }
                }
                Event::MouseWheel { delta, handled, .. } => {
                    if !*handled {
                        let v = camera.view_direction() * self.speed * delta.1;
                        camera.translate(v);
                        *handled = true;
                        change = true;
                    }
                }
                _ => {}
            }
        }
        change
    }
}
