use crate::camera::*;
use crate::core::Error;
use crate::math::*;
use crate::frame::*;

pub struct EventHandler {
    pub left_drag: Option<ControlType>,
    pub middle_drag: Option<ControlType>,
    pub right_drag: Option<ControlType>,
    pub scroll: Option<ControlType>
}

impl Default for EventHandler {
    fn default() -> Self {
        Self {
            left_drag: None,
            middle_drag: None,
            right_drag: None,
            scroll: None
        }
    }
}

pub enum ControlType {
    Rotate {speed: f32},
    RotateAroundUp {speed: f32},
    Pan {speed: f32},
    ZoomHorizontal {speed: f32, min: f32, max: f32},
    ZoomVertical {speed: f32, min: f32, max: f32}
}

///
/// 3D controls for a camera. Use this to add additional control functionality to a [camera](crate::Camera).
///
pub struct CameraControl {
    camera: Camera,
    left: bool,
    middle: bool,
    right: bool
}

impl CameraControl {
    pub fn new(camera: Camera) -> Self {
        Self { camera, left: false, middle: false, right: false }
    }

    pub fn handle_events(&mut self, frame_input: &FrameInput, event_handler: EventHandler) -> Result<bool, Error> {
        let mut change = false;
        for event in frame_input.events.iter() {
            match event {
                Event::MouseClick { state, button, handled, .. } => {
                    if !*handled {
                        self.left = *button == MouseButton::Left && *state == State::Pressed;
                        self.middle = *button == MouseButton::Middle && *state == State::Pressed;
                        self.right = *button == MouseButton::Right && *state == State::Pressed;
                    }
                }
                Event::MouseMotion { delta: (x, y), handled, .. } => {
                    if !*handled {
                        if self.left {
                            change |= self.handle_drag(&event_handler.left_drag, *x, *y)?;
                        }
                        if self.middle {
                            change |= self.handle_drag(&event_handler.middle_drag, *x, *y)?;
                        }
                        if self.right {
                            change |= self.handle_drag(&event_handler.right_drag, *x, *y)?;
                        }
                    }
                }
                Event::MouseWheel { delta: (x, y), handled, .. } => {
                    if !*handled {
                        change |= self.handle_drag(&event_handler.scroll, *x, *y)?;
                    }
                }
                _ => {}
            }
        }
        Ok(change)
    }

    fn handle_drag(&mut self, control: &Option<ControlType>, x: f64, y: f64) -> Result<bool, Error> {
        if let Some(ref control_type) = control {
            match control_type {
                ControlType::Rotate {speed} => {
                    self.rotate(speed * x as f32, speed * y as f32)?;
                }
                ControlType::RotateAroundUp {speed} => {
                    self.rotate_around_up(speed * x as f32, speed * y as f32)?;
                }
                ControlType::Pan {speed} => {
                    self.pan(speed * x as f32, speed * y as f32)?;
                },
                ControlType::ZoomHorizontal {speed, min, max} => {
                    self.zoom(speed * x as f32, *min, *max)?;
                }
                ControlType::ZoomVertical {speed, min, max} => {
                    self.zoom(speed * y as f32, *min, *max)?;
                }
            }
        }
        Ok(control.is_some())
    }

    pub fn translate(&mut self, change: &Vec3) -> Result<(), Error> {
        let position = *self.position();
        let target = *self.target();
        let up = *self.up();
        self.set_view(position + change, target + change, up)?;
        Ok(())
    }

    pub fn rotate(&mut self, x: f32, y: f32) -> Result<(), Error> {
        let target = *self.target();
        let right = self.right_direction();
        let up = right.cross(self.view_direction());
        let mut new_pos = self.position() - right * x + up * y;
        new_pos = target + self.distance_to_target() * (new_pos - self.target()).normalize();
        self.set_view(new_pos, target, up)?;
        Ok(())
    }

    pub fn rotate_around_up(&mut self, x: f32, y: f32) -> Result<(), Error> {
        let target = *self.target();
        let up = *self.up();
        let right = self.right_direction();
        let new_pos = self.position() - right * x + right.cross(self.view_direction()) * y;
        let new_dir = (self.target() - new_pos).normalize();
        if new_dir.dot(up).abs() < 0.999 {
            let zoom = self.distance_to_target();
            self.set_view(target - new_dir * zoom, target, up)?;
        }
        Ok(())
    }

    pub fn pan(&mut self, x: f32, y: f32) -> Result<(), Error> {
        let direction = self.view_direction();
        let right = self.right_direction();
        let delta = -right * x + right.cross(direction) * y;
        self.translate(&delta)?;
        Ok(())
    }

    pub fn zoom(&mut self, wheel: f32, min: f32, max: f32) -> Result<(), Error> {
        match self.projection_type() {
            ProjectionType::Orthographic {
                width,
                height,
                depth,
            } => {
                let h = (height - wheel * self.distance_to_target()).max(min).min(max);
                let w = h * width / height;
                let d = *depth;
                self.set_orthographic_projection(w, h, d)?;
            }
            ProjectionType::Perspective { .. } => {
                let target = *self.target();
                let up = *self.up();
                let direction = self.view_direction();
                let mut zoom = (wheel + 1.0) * self.distance_to_target();
                zoom = zoom.max(min).min(max);
                self.set_view(target - direction * zoom, target, up)?;
            }
        }
        Ok(())
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
