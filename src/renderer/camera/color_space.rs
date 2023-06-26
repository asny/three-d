use crate::core::*;
use crate::Camera;
use crate::Effect;
use crate::FragmentAttributes;

#[derive(Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum ColorSpace {
    Compute = 0,
    #[default]
    Srgb = 1,
}

impl ColorSpace {
    pub fn fragment_shader_source() -> &'static str {
        "
        uniform uint colorSpaceType;

        vec3 color_mapping(vec3 color) {
            if (colorSpaceType == 1u) {
                vec3 a = vec3(0.055, 0.055, 0.055);
                vec3 ap1 = vec3(1.0, 1.0, 1.0) + a;
                vec3 g = vec3(2.4, 2.4, 2.4);
                vec3 ginv = 1.0 / g;
                vec3 select = step(vec3(0.0031308, 0.0031308, 0.0031308), color);
                vec3 lo = color * 12.92;
                vec3 hi = ap1 * pow(color, ginv) - a;
                color = mix(lo, hi, select);
            } 

            return color;
        }
        "
    }

    pub fn use_uniforms(&self, program: &Program) {
        program.use_uniform("colorSpaceType", *self as u32);
    }
}
