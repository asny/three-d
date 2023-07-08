use crate::core::*;

/// Color space used for specifying the targeted color space when rendering.
#[derive(Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum ColorSpace {
    /// Use this if you want to use the rendered result as input to a following render pass.
    Compute = 0,
    /// Use this if this is the final render pass, ie. you write to the screen or want to save it as an image.
    #[default]
    Srgb = 1,
}

impl ColorSpace {
    ///
    /// Returns the fragment shader source for converting to the specified color space in a shader.
    ///
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
