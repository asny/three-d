use crate::camera::*;
use crate::core::*;
use crate::frame::*;
use crate::math::*;

pub struct EventHandler {
    pub left_drag: Option<ControlType>,
    pub middle_drag: Option<ControlType>,
    pub right_drag: Option<ControlType>,
    pub scroll: Option<ControlType>,
}

impl Default for EventHandler {
    fn default() -> Self {
        Self {
            left_drag: None,
            middle_drag: None,
            right_drag: None,
            scroll: None,
        }
    }
}

pub enum FocusPoint {
    Fixed(Vec3),
    Pick,
}

pub enum SpeedFunction {
    Constant(f32),
}

pub enum ControlType {
    Rotate { speed: f32, point: FocusPoint },
    RotateAroundUp { speed: f32, point: FocusPoint },
    Pan { speed: f32 },
    ZoomHorizontal { speed: f32, min: f32, max: f32 },
    ZoomVertical { speed: f32, min: f32, max: f32 },
}

enum Action {
    Drag(Option<Vec3>),
    None,
}

///
/// 3D controls for a camera. Use this to add additional control functionality to a [camera](crate::Camera).
///
pub struct CameraControl {
    camera: Camera,
    left: Action,
    middle: Action,
    right: Action,
}

impl CameraControl {
    pub fn new(camera: Camera) -> Self {
        Self {
            camera,
            left: Action::None,
            middle: Action::None,
            right: Action::None,
        }
    }

    pub fn handle_events(
        &mut self,
        frame_input: &FrameInput,
        viewport: Viewport,
        event_handler: EventHandler,
        objects: &[&dyn Pickable],
    ) -> Result<bool, Error> {
        let mut change = false;
        for event in frame_input.events.iter() {
            match event {
                Event::MouseClick {
                    state,
                    button,
                    handled,
                    position,
                    ..
                } => {
                    if !*handled {
                        let pick = if *state == State::Pressed {
                            let pos = *self.position();
                            let dir = self.view_direction_at((
                                (position.0 * frame_input.device_pixel_ratio - viewport.x as f64)
                                    / viewport.width as f64,
                                (position.1 * frame_input.device_pixel_ratio - viewport.y as f64)
                                    / viewport.height as f64,
                            ));
                            Action::Drag(self.pick(pos, dir, 100.0, objects)?)
                        } else {
                            Action::None
                        };
                        match *button {
                            MouseButton::Left => {
                                self.left = pick;
                            }
                            MouseButton::Middle => {
                                self.middle = pick;
                            }
                            MouseButton::Right => {
                                self.right = pick;
                            }
                        }
                    }
                }
                Event::MouseMotion {
                    delta: (x, y),
                    handled,
                    ..
                } => {
                    if !*handled {
                        if let Action::Drag(pick) = self.left {
                            change |= self.handle_drag(pick, &event_handler.left_drag, *x, *y)?;
                        }
                        if let Action::Drag(pick) = self.middle {
                            change |= self.handle_drag(pick, &event_handler.middle_drag, *x, *y)?;
                        }
                        if let Action::Drag(pick) = self.right {
                            change |= self.handle_drag(pick, &event_handler.right_drag, *x, *y)?;
                        }
                    }
                }
                Event::MouseWheel {
                    delta: (x, y),
                    handled,
                    ..
                } => {
                    if !*handled {
                        change |= self.handle_drag(None, &event_handler.scroll, *x, *y)?;
                    }
                }
                _ => {}
            }
        }
        Ok(change)
    }

    fn handle_drag(
        &mut self,
        pick: Option<Vec3>,
        control: &Option<ControlType>,
        x: f64,
        y: f64,
    ) -> Result<bool, Error> {
        if let Some(ref control_type) = control {
            match control_type {
                ControlType::Rotate { speed, point } => {
                    match point {
                        FocusPoint::Fixed(p) => {
                            self.rotate_around(p, speed * x as f32, speed * y as f32)?;
                        }
                        FocusPoint::Pick => {
                            if let Some(ref p) = pick {
                                self.rotate_around(p, speed * x as f32, speed * y as f32)?;
                            }
                        }
                    };
                }
                ControlType::RotateAroundUp { speed, point } => {
                    match point {
                        FocusPoint::Fixed(p) => {
                            self.rotate_around_up(p, speed * x as f32, speed * y as f32)?;
                        }
                        FocusPoint::Pick => {
                            if let Some(ref p) = pick {
                                self.rotate_around_up(p, speed * x as f32, speed * y as f32)?;
                            }
                        }
                    };
                }
                ControlType::Pan { speed } => {
                    self.pan(speed * x as f32, speed * y as f32)?;
                }
                ControlType::ZoomHorizontal { speed, min, max } => {
                    self.zoom(speed * x as f32, *min, *max)?;
                }
                ControlType::ZoomVertical { speed, min, max } => {
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

    pub fn rotate_around(&mut self, point: &Vec3, x: f32, y: f32) -> Result<(), Error> {
        let dir = (point - self.position()).normalize();
        let right = dir.cross(*self.up());
        let up = right.cross(dir);
        let new_pos = self.position() - right * x + up * y;
        let new_dir = (point - new_pos).normalize();
        let dist = point.distance(*self.position());
        let target = *self.target();
        self.set_view(point - dist * new_dir, target, up)?;
        Ok(())
    }

    pub fn rotate_around_up(&mut self, point: &Vec3, x: f32, y: f32) -> Result<(), Error> {
        let dir = (point - self.position()).normalize();
        let right = dir.cross(*self.up());
        let mut up = right.cross(dir);
        let new_pos = self.position() - right * x + up * y;
        let new_dir = (point - new_pos).normalize();
        up = *self.up();
        if new_dir.dot(up).abs() < 0.999 {
            let dist = point.distance(*self.position());
            let target = *self.target();
            self.set_view(point - dist * new_dir, target, up)?;
        }
        Ok(())
    }

    pub fn pan(&mut self, x: f32, y: f32) -> Result<(), Error> {
        let right = self.right_direction();
        let up = right.cross(self.view_direction());
        let delta = -right * x + up * y;
        self.translate(&(delta * self.distance_to_target()))?;
        Ok(())
    }

    pub fn zoom(&mut self, wheel: f32, min: f32, max: f32) -> Result<(), Error> {
        match self.projection_type() {
            ProjectionType::Orthographic {
                width,
                height,
                depth,
            } => {
                let h = (height - wheel * self.distance_to_target())
                    .max(min)
                    .min(max);
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
