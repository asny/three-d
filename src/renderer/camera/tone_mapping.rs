use crate::core::*;

///
/// Tone mapping is the process of mapping HDR color values computed with physical based rendering in the range `[0,∞)`
/// into LDR values that can be displayed on the screen in the range `[0,1]`.
///
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum ToneMapping {
    /// No tone mapping. Use this if you are rendering into an intermediate render target, ie. this is not the final render pass that renders into the screen.
    None = 0,
    /// Photographic Tone Reproduction for Digital Images. `<http://www.cmap.polytechnique.fr/~peyre/cours/x2005signal/hdr_photographic.pdf>`
    Reinhard = 1,
    /// ACES Filmic Tone Mapping Curve. `<https://knarkowicz.wordpress.com/2016/01/06/aces-filmic-tone-mapping-curve/>`
    #[default]
    Aces = 2,
    /// John Hables presentation "Uncharted 2 HDR Lighting", Page 142 to 143. `<http://www.gdcvault.com/play/1012459/Uncharted_2__HDR_Lighting>`
    Filmic = 3,
}

impl ToneMapping {
    ///
    /// Returns the fragment shader source for applying the specified tone mapping in a shader.
    ///
    pub fn fragment_shader_source() -> &'static str {
        "
        uniform uint toneMappingType;

        vec3 tone_mapping(vec3 color) {
            if (toneMappingType == 1u) {
                color = color / (color + vec3(1.0));
                color = clamp(color, 0.0, 1.0);
            } else if(toneMappingType == 2u) {
                color = color*(2.51*color + .03) / (color*(2.43*color + .59) + .14);
                color = clamp(color, 0.0, 1.0);
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
                color = clamp(color, 0.0, 1.0);
            }
            return color;
        }
        "
    }

    ///
    /// Sends the uniform data needed to apply this tone mapping to the fragment shader.
    ///
    pub fn use_uniforms(&self, program: &Program) {
        program.use_uniform("toneMappingType", *self as u32);
    }
}
