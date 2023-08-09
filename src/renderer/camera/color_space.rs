use crate::core::*;

///
/// Color space mapping used for mapping to/from color spaces when rendering.
///
#[derive(Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum ColorMapping {
    /// No color mapping. Use this if you are rendering into an intermediate render target, ie. this is not the final render pass that renders into the screen.
    None = 0,
    /// Maps from compute color space (HDR or linear sRGB) to sRGB color space. Use this if this is the final render pass, ie. you write to the screen or want to save it as an image.
    #[default]
    ComputeToSrgb = 1,
}

impl ColorMapping {
    ///
    /// Returns the fragment shader source for mapping to the specified color space in a shader.
    ///
    pub fn fragment_shader_source() -> &'static str {
        "
        uniform uint ColorMappingType;

        vec3 color_mapping(vec3 color) {
            if (ColorMappingType == 1u) {
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

    ///
    /// Sends the uniform data needed to apply this color space mapping to the fragment shader.
    ///
    pub fn use_uniforms(&self, program: &Program) {
        program.use_uniform("ColorMappingType", *self as u32);
    }
}
