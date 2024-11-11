use crate::renderer::*;

///
/// A set of possible actions to apply to a camera when recieving input.
///
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CameraAction {
    /// No action.
    None,
    /// Rotate the camera around the horizontal axis as seen from the camera.
    Pitch {
        /// The speed of the rotation.
        speed: f32,
    },
    /// Orbits around the given target in the up direction as seen from the camera.
    OrbitUp {
        /// The target of the rotation.
        target: Vec3,
        /// The speed of the rotation.
        speed: f32,
    },
    /// Freely orbits around the given target in the up direction as seen from the camera.
    FreeOrbitUp {
        /// The target of the rotation.
        target: Vec3,
        /// The speed of the rotation.
        speed: f32,
    },
    /// Rotate the camera around the vertical axis as seen from the camera.
    Yaw {
        /// The speed of the rotation.
        speed: f32,
    },
    /// Orbits around the given target in the left direction as seen from the camera.
    OrbitLeft {
        /// The target of the rotation.
        target: Vec3,
        /// The speed of the rotation.
        speed: f32,
    },
    /// Freely orbits around the given target in the left direction as seen from the camera.
    FreeOrbitLeft {
        /// The target of the rotation.
        target: Vec3,
        /// The speed of the rotation.
        speed: f32,
    },
    /// Rotate the camera around the forward axis as seen from the camera.
    Roll {
        /// The speed of the rotation.
        speed: f32,
    },
    /// Moves the camera to the left.
    Left {
        /// The speed of the translation.
        speed: f32,
    },
    /// Moves the camera up.
    Up {
        /// The speed of the translation.
        speed: f32,
    },
    /// Moves the camera forward.
    Forward {
        /// The speed of the translation.
        speed: f32,
    },
    /// Zooms towards the given target.
    Zoom {
        /// The target of the zoom.
        target: Vec3,
        /// The speed of the zoom.
        speed: f32,
        /// The minimum distance to the target.
        min: f32,
        /// The maximum distance to the target.
        max: f32,
    },
}

impl std::default::Default for CameraAction {
    fn default() -> Self {
        Self::None
    }
}

impl CameraAction {
    /// Returns true if the action is `CameraAction::None`.
    pub fn is_none(self) -> bool {
        self == CameraAction::None
    }

    /// Returns true if the action is not `CameraAction::None`.
    pub fn is_some(self) -> bool {
        self != CameraAction::None
    }

    /// Applies the effects of this action to the camera. Can be used to implement additional camera control events.
    pub fn apply(self, camera: &mut Camera, x: f32) {
        match self {
            CameraAction::Pitch { speed } => {
                camera.pitch(radians(speed * x));
            }
            CameraAction::OrbitUp { speed, target } => {
                camera.rotate_around_with_fixed_up(&target, 0.0, speed * x);
            }
            CameraAction::FreeOrbitUp { speed, target } => {
                camera.rotate_around(&target, 0.0, speed * x);
            }
            CameraAction::Yaw { speed } => {
                camera.yaw(radians(speed * x));
            }
            CameraAction::OrbitLeft { speed, target } => {
                camera.rotate_around_with_fixed_up(&target, speed * x, 0.0);
            }
            CameraAction::FreeOrbitLeft { speed, target } => {
                camera.rotate_around(&target, speed * x, 0.0);
            }
            CameraAction::Roll { speed } => {
                camera.roll(radians(speed * x));
            }
            CameraAction::Left { speed } => {
                let change = -camera.right_direction() * x * speed;
                camera.translate(&change);
            }
            CameraAction::Up { speed } => {
                let right = camera.right_direction();
                let up = right.cross(camera.view_direction());
                let change = up * x * speed;
                camera.translate(&change);
            }
            CameraAction::Forward { speed } => {
                let change = camera.view_direction() * speed * x;
                camera.translate(&change);
            }
            CameraAction::Zoom {
                target,
                speed,
                min,
                max,
            } => {
                camera.zoom_towards(&target, speed * x, min, max);
            }
            CameraAction::None => {}
        }
    }
}

///
/// A customizable controller for the camera.
/// It is possible to specify a [CameraAction] for each of the input events.
///
#[derive(Clone, Copy, Debug, Default)]
pub struct CameraControl {
    /// Specifies what happens when dragging horizontally with the left mouse button.
    pub left_drag_horizontal: CameraAction,
    /// Specifies what happens when dragging vertically with the left mouse button.
    pub left_drag_vertical: CameraAction,
    /// Specifies what happens when dragging horizontally with the middle mouse button.
    pub middle_drag_horizontal: CameraAction,
    /// Specifies what happens when dragging vertically with the middle mouse button.
    pub middle_drag_vertical: CameraAction,
    /// Specifies what happens when dragging horizontally with the right mouse button.
    pub right_drag_horizontal: CameraAction,
    /// Specifies what happens when dragging vertically with the right mouse button.
    pub right_drag_vertical: CameraAction,
    /// Specifies what happens when scrolling horizontally.
    pub scroll_horizontal: CameraAction,
    /// Specifies what happens when scrolling vertically.
    pub scroll_vertical: CameraAction,
}

impl CameraControl {
    /// Creates a new default CameraControl.
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the events. Must be called each frame.
    pub fn handle_events(&self, camera: &mut Camera, events: &mut [Event]) -> bool {
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
                            control_horizontal.apply(camera, delta.0);
                            control_vertical.apply(camera, delta.1);
                            *handled = control_horizontal.is_some() || control_vertical.is_some();
                            change |= *handled;
                        }
                    }
                }
                Event::MouseWheel { delta, handled, .. } => {
                    if !*handled {
                        self.scroll_horizontal.apply(camera, delta.0);
                        self.scroll_vertical.apply(camera, delta.1);
                        *handled =
                            self.scroll_horizontal.is_some() || self.scroll_vertical.is_some();
                        change |= *handled;
                    }
                }
                _ => {}
            }
        }
        change
    }
}
