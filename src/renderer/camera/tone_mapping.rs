use crate::{core::*, Camera, Effect, FragmentAttributes};

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

impl Effect for ToneMapping {
    fn fragment_shader_source(
        &self,
        _lights: &[&dyn crate::Light],
        color_texture: Option<ColorTexture>,
        _depth_texture: Option<DepthTexture>,
    ) -> String {
        let color_texture =
            color_texture.expect("Must supply a color texture to apply a tone mapping effect");
        format!(
            "
            {}
            {}

            in vec2 uvs;

            layout (location = 0) out vec4 outColor;

            void main()
            {{
                outColor = sample_color(uvs);
                outColor.rgb = tone_mapping(outColor.rgb);
            }}

        ",
            Self::fragment_shader_source(),
            color_texture.fragment_shader_source()
        )
    }

    fn id(&self) -> u16 {
        0b11u16 << 14 | 0b110u16
    }

    fn fragment_attributes(&self) -> FragmentAttributes {
        FragmentAttributes {
            uv: true,
            ..FragmentAttributes::NONE
        }
    }

    fn use_uniforms(
        &self,
        program: &Program,
        _camera: &Camera,
        _lights: &[&dyn crate::Light],
        color_texture: Option<ColorTexture>,
        _depth_texture: Option<DepthTexture>,
    ) {
        self.use_uniforms(program);
        let color_texture =
            color_texture.expect("Must supply a color texture to apply a tone mapping effect");
        color_texture.use_uniforms(program);
    }

    fn render_states(&self) -> RenderStates {
        RenderStates::default()
    }
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
        program.use_uniform("toneMappingType", *self as u32);
    }
}
