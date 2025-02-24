use super::*;

///
/// A control that makes the camera fly through the 3D scene.
///
#[derive(Clone, Copy, Debug)]
pub struct FlyControl {
    /// The speed of movements.
    pub speed: f32,
}

impl FlyControl {
    /// Creates a new fly control with the given speed of movements.
    pub fn new(speed: f32) -> Self {
        Self { speed }
    }

    /// Handles the events. Must be called each frame.
    pub fn handle_events(
        &mut self,
        camera: &mut three_d_asset::Camera,
        events: &mut [Event],
    ) -> bool {
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
                        if Some(MouseButton::Right) == *button {
                            let right = camera.right_direction();
                            let up = right.cross(camera.view_direction());
                            camera.translate(
                                -right * delta.0 * self.speed + up * delta.1 * self.speed,
                            );
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
