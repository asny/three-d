use crate::core::*;
use crate::renderer::*;

pub struct WireframeMaterial {
    pub line_width: f32,
}

impl Material for WireframeMaterial {
    fn fragment_shader_source(&self, _lights: &[&dyn Light]) -> String {
        r#"
layout (location = 0) out vec4 outColor;

uniform float u_line_width = 0.5;

in vec3 bary;
in vec3 pos;

void main() {
    vec3 d = fwidth(bary);
    vec3 f = step(d * u_line_width, bary);
    float b = min(min(f.x, f.y), f.z);
    outColor = vec4(1.-b);
}
            "#
        .into()
    }

    fn id(&self) -> EffectMaterialId {
        //TODO!
        EffectMaterialId(0)
    }

    fn use_uniforms(&self, program: &Program, _viewer: &dyn Viewer, _lights: &[&dyn Light]) {
        program.use_uniform("u_line_width", self.line_width);
    }

    fn render_states(&self) -> RenderStates {
        RenderStates {
            depth_test: DepthTest::Always,
            blend: Blend::TRANSPARENCY,
            cull: Cull::None,
            ..Default::default()
        }
    }

    fn material_type(&self) -> MaterialType {
        MaterialType::Transparent
    }
}
