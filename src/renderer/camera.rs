use crate::core::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum ToneMapping {
    None = 0,
    /// Photographic Tone Reproduction for Digital Images. http://www.cmap.polytechnique.fr/~peyre/cours/x2005signal/hdr_photographic.pdf
    Reinhard = 1,
    /// ACES Filmic Tone Mapping Curve. https://knarkowicz.wordpress.com/2016/01/06/aces-filmic-tone-mapping-curve/
    #[default]
    Aces = 2,
    /// John Hables presentation "Uncharted 2 HDR Lighting", Page 142 to 143. http://www.gdcvault.com/play/1012459/Uncharted_2__HDR_Lighting
    Filmic = 3,
}

impl ToneMapping {
    pub fn fragment_shader_source() -> &'static str {
        "
        uniform uint toneMappingType;

        vec3 tone_mapping(vec3 color) {
            if (toneMappingType == 1u) {
                color = color / (color + vec3(1.0));
            } else if(toneMappingType == 2u) {
                color = color*(2.51*color + .03) / (color*(2.43*color + .59) + .14);
            } else if(toneMappingType == 3u) {
                const float A = 0.15;
                const float B = 0.50;
                const float C = 0.10;
                const float D = 0.20;
                const float E = 0.02;
                const float F = 0.30;
                const float W = 11.2;
                
                vec4 x = vec4(color, W);
                x = ((x*(A*x+C*B)+D*E)/(x*(A*x+B)+D*F))-E/F;
                color = x.xyz / x.w;
            }
            return clamp(color, 0.0, 1.0);
        }

        vec3 inverse_tone_mapping(vec3 color) {
            if (toneMappingType == 1u) {
                return color / max(vec3(1.0) - color, vec3(0.001, 0.001, 0.001));
            } else if(toneMappingType == 2u) {
                color = (sqrt(-10127.*color*color + 13702.*color + 9.) + 59.*color - 3.) / (502. - 486.*color);
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
