use crate::renderer::*;

///
/// A control that makes the camera orbit around a target, with no fixed up direction.
///
#[derive(Clone, Copy, Debug)]
pub struct FreeOrbitControl {
    /// The target point to orbit around.
    pub target: Vec3,
    /// The minimum distance to the target point.
    pub min_distance: f32,
    /// The maximum distance to the target point.
    pub max_distance: f32,
}

impl FreeOrbitControl {
    /// Creates a new free orbit control with the given target and minimum and maximum distance to the target.
    pub fn new(target: Vec3, min_distance: f32, max_distance: f32) -> Self {
        Self {
            target,
            min_distance,
            max_distance,
        }
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
                            let speed = 0.01 * self.target.distance(camera.position()) + 0.001;
                            camera.rotate_around(self.target, speed * delta.0, speed * delta.1);
                            *handled = true;
                            change = true;
                        }
                    }
                }
                Event::MouseWheel { delta, handled, .. } => {
                    if !*handled {
                        let speed = 0.01 * self.target.distance(camera.position()) + 0.001;
                        camera.zoom_towards(
                            self.target,
                            speed * delta.1,
                            self.min_distance,
                            self.max_distance,
                        );
                        *handled = true;
                        change = true;
                    }
                }
                Event::PinchGesture { delta, handled, .. } => {
                    if !*handled {
                        let speed = self.target.distance(camera.position()) + 0.1;
                        camera.zoom_towards(
                            self.target,
                            speed * *delta,
                            self.min_distance,
                            self.max_distance,
                        );
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
