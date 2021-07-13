use crate::camera::*;
use crate::frame::*;
use crate::math::*;
use crate::Error;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CameraAction {
    None,
    Pitch {
        speed: f32,
    },
    OrbitUp {
        target: Vec3,
        speed: f32,
    },
    Yaw {
        speed: f32,
    },
    OrbitLeft {
        target: Vec3,
        speed: f32,
    },
    Roll {
        speed: f32,
    },
    Left {
        speed: f32,
    },
    Up {
        speed: f32,
    },
    Forward {
        speed: f32,
    },
    Zoom {
        target: Vec3,
        speed: f32,
        min: f32,
        max: f32,
    },
}

impl std::default::Default for CameraAction {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CameraControl {
    pub left_drag_horizontal: CameraAction,
    pub left_drag_vertical: CameraAction,
    pub middle_drag_horizontal: CameraAction,
    pub middle_drag_vertical: CameraAction,
    pub right_drag_horizontal: CameraAction,
    pub right_drag_vertical: CameraAction,
    pub scroll_horizontal: CameraAction,
    pub scroll_vertical: CameraAction,
}

impl CameraControl {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn handle_events(
        &mut self,
        camera: &mut Camera,
        events: &mut [Event],
    ) -> Result<bool, Error> {
        let mut change = false;
        for event in events.iter_mut() {
            match event {
                Event::MouseMotion {
                    delta,
                    button,
                    handled,
                    ..
                } => {
                    if !*handled && button.is_some() {
                        if let Some(b) = button {
                            let (control_horizontal, control_vertical) = match b {
                                MouseButton::Left => {
                                    (self.left_drag_horizontal, self.left_drag_vertical)
                                }
                                MouseButton::Middle => {
                                    (self.middle_drag_horizontal, self.middle_drag_vertical)
                                }
                                MouseButton::Right => {
                                    (self.right_drag_horizontal, self.right_drag_vertical)
                                }
                            };
                            *handled = self.handle_action(camera, control_horizontal, delta.0)?;
                            *handled |= self.handle_action(camera, control_vertical, delta.1)?;
                            change |= *handled;
                        }
                    }
                }
                Event::MouseWheel { delta, handled, .. } => {
                    if !*handled {
                        *handled = self.handle_action(camera, self.scroll_horizontal, delta.0)?;
                        *handled |= self.handle_action(camera, self.scroll_vertical, delta.1)?;
                        change |= *handled;
                    }
                }
                _ => {}
            }
        }
        Ok(change)
    }

    fn handle_action(
        &mut self,
        camera: &mut Camera,
        control_type: CameraAction,
        x: f64,
    ) -> Result<bool, Error> {
        match control_type {
            CameraAction::Pitch { speed } => {
                camera.pitch(radians(speed * x as f32))?;
            }
            CameraAction::OrbitUp { speed, target } => {
                camera.rotate_around_with_fixed_up(&target, 0.0, speed * x as f32)?;
            }
            CameraAction::Yaw { speed } => {
                camera.yaw(radians(speed * x as f32))?;
            }
            CameraAction::OrbitLeft { speed, target } => {
                camera.rotate_around_with_fixed_up(&target, speed * x as f32, 0.0)?;
            }
            CameraAction::Roll { speed } => {
                camera.roll(radians(speed * x as f32))?;
            }
            CameraAction::Left { speed } => {
                let change = -camera.right_direction() * x as f32 * speed;
                camera.translate(&change)?;
            }
            CameraAction::Up { speed } => {
                let right = camera.right_direction();
                let up = right.cross(camera.view_direction());
                let change = up * x as f32 * speed;
                camera.translate(&change)?;
            }
            CameraAction::Forward { speed } => {
                let change = camera.view_direction() * speed * x as f32;
                camera.translate(&change)?;
            }
            CameraAction::Zoom {
                target,
                speed,
                min,
                max,
            } => {
                camera.zoom_towards(&target, speed * x as f32, min, max)?;
            }
            CameraAction::None => {}
        }
        Ok(control_type != CameraAction::None)
    }
}
