use crate::core::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum ToneMapping {
    #[default]
    None = 0,
    Reinhard = 1,
    Aces = 2,
}

impl ToneMapping {
    pub fn fragment_shader_source() -> &'static str {
        "
        uniform uint toneMappingType;

        vec3 tone_mapping(vec3 color) {
            if (toneMappingType == 1u) {
                return color / (color + vec3(1.0));
            } else if(toneMappingType == 2u) {
                const float a = 2.51;
                const float b = 0.03;
                const float c = 2.43;
                const float d = 0.59;
                const float e = 0.14;
                return clamp((color * (a*color+b)) / (color * (c*color+d) + e), 0.0, 1.0);
            }
            return color;
        }

        vec3 inverse_tone_mapping(vec3 color) {
            if (toneMappingType == 1u) {
                return color / max(vec3(1.0) - color, vec3(0.001, 0.001, 0.001));
            }
            return color;
        }
        "
    }

    pub fn use_uniforms(&self, program: &Program) {
        program.use_uniform("toneMappingType", *self as u32)
    }
}

#[derive(Clone, Debug)]
pub struct Camera {
    camera: three_d_asset::Camera,
    /// Tone mapping applied to the final color
    pub tone_mapping: ToneMapping,
}

impl Camera {
    pub fn new_orthographic(
        viewport: Viewport,
        position: Vec3,
        target: Vec3,
        up: Vec3,
        height: f32,
        z_near: f32,
        z_far: f32,
    ) -> Self {
        Self {
            camera: three_d_asset::Camera::new_orthographic(
                viewport, position, target, up, height, z_near, z_far,
            ),
            tone_mapping: ToneMapping::default(),
        }
    }
    pub fn new_perspective(
        viewport: Viewport,
        position: Vec3,
        target: Vec3,
        up: Vec3,
        field_of_view_y: impl Into<Radians>,
        z_near: f32,
        z_far: f32,
    ) -> Self {
        Self {
            camera: three_d_asset::Camera::new_perspective(
                viewport,
                position,
                target,
                up,
                field_of_view_y,
                z_near,
                z_far,
            ),
            tone_mapping: ToneMapping::default(),
        }
    }
}

use std::ops::Deref;
impl Deref for Camera {
    type Target = three_d_asset::Camera;
    fn deref(&self) -> &Self::Target {
        &self.camera
    }
}

impl std::ops::DerefMut for Camera {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.camera
    }
}
