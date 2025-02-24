use super::*;

///
/// A control for 2D camera movements.
///
#[derive(Clone, Copy, Debug)]
pub struct Control2D {
    /// The minimum zoom factor.
    pub min_zoom_factor: f32,
    /// The maximum zoom factor.
    pub max_zoom_factor: f32,
}

impl Control2D {
    /// Creates a new 2D camera control with the given minimum and maximum zoom factor.
    pub fn new(min_zoom_factor: f32, max_zoom_factor: f32) -> Self {
        Self {
            min_zoom_factor,
            max_zoom_factor,
        }
    }

    /// Handles the events. Must be called each frame.
    pub fn handle_events(
        &mut self,
        camera: &mut three_d_asset::Camera,
        events: &mut [Event],
        device_pixel_ratio: f32,
    ) -> bool {
        let mut change = false;
        for event in events.iter_mut() {
            match event {
                Event::PinchGesture {
                    delta,
                    position,
                    handled,
                    ..
                } => {
                    if !*handled {
                        self.zoom(camera, *delta, *position, 0.5);
                        *handled = true;
                        change = true;
                    }
                }
                Event::MouseWheel {
                    delta,
                    position,
                    handled,
                    ..
                } => {
                    if !*handled && delta.0.abs() + delta.1.abs() > std::f32::EPSILON {
                        if delta.0.abs() < std::f32::EPSILON
                            && delta.1.fract().abs() > std::f32::EPSILON
                        {
                            self.zoom(camera, delta.1, *position, 0.005);
                        } else {
                            self.pan(camera, *delta, device_pixel_ratio);
                        }
                        *handled = true;
                        change = true;
                    }
                }
                Event::MouseMotion {
                    delta,
                    button,
                    handled,
                    ..
                } => {
                    if !*handled && Some(MouseButton::Right) == *button {
                        self.pan(camera, *delta, device_pixel_ratio);
                        *handled = true;
                        change = true;
                    }
                }
                _ => {}
            }
        }
        change
    }

    fn zoom(
        &self,
        camera: &mut three_d_asset::Camera,
        delta: f32,
        position: PhysicalPoint,
        speed: f32,
    ) {
        let speed = speed / camera.zoom_factor();
        let mut target = camera.position_at_pixel(position);
        target.z = 0.0;
        camera.zoom_towards(
            target,
            speed * delta,
            1.0 / self.max_zoom_factor,
            1.0 / self.min_zoom_factor,
        );
    }

    fn pan(&self, camera: &mut three_d_asset::Camera, delta: (f32, f32), device_pixel_ratio: f32) {
        let origo = camera.position_at_pixel(vec2(0.0, 0.0));
        let point = camera.position_at_pixel(vec2(device_pixel_ratio * delta.0, 0.0));
        let x = delta.0.signum() * (point - origo).magnitude();
        let point = camera.position_at_pixel(vec2(device_pixel_ratio * delta.1, 0.0));
        let y = delta.1.signum() * (point - origo).magnitude();
        let right = camera.right_direction();
        let up = right.cross(camera.view_direction());
        camera.translate(-right * x + up * y);
    }
}
