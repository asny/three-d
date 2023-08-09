use crate::renderer::*;

pub struct LightingPassEffect {}

impl Effect for LightingPassEffect {
    fn fragment_shader_source(
        &self,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) -> String {
        let mut fragment_shader = lights_shader_source(
            lights,
            LightingModel::Cook(
                NormalDistributionFunction::TrowbridgeReitzGGX,
                GeometryFunction::SmithSchlickGGX,
            ),
        );
        fragment_shader.push_str(&color_texture.unwrap().fragment_shader_source());
        fragment_shader.push_str(&depth_texture.unwrap().fragment_shader_source());
        fragment_shader.push_str(ToneMapping::fragment_shader_source());
        fragment_shader.push_str(ColorMapping::fragment_shader_source());
        fragment_shader.push_str(include_str!("shaders/deferred_lighting.frag"));
        fragment_shader
    }

    fn id(&self, color_texture: Option<ColorTexture>, depth_texture: Option<DepthTexture>) -> u16 {
        0b1u16 << 14 | 0b1u16 << 12 | color_texture.unwrap().id() | depth_texture.unwrap().id()
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
        camera: &Camera,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) {
        camera.tone_mapping.use_uniforms(program);
        camera.color_mapping.use_uniforms(program);
        color_texture.unwrap().use_uniforms(program);
        depth_texture.unwrap().use_uniforms(program);
        program.use_uniform_if_required("cameraPosition", camera.position());
        for (i, light) in lights.iter().enumerate() {
            light.use_uniforms(program, i as u32);
        }
        program.use_uniform_if_required(
            "viewProjectionInverse",
            (camera.projection() * camera.view()).invert().unwrap(),
        );
        program.use_uniform("debug_type", DebugType::None as i32);
    }

    fn render_states(&self) -> RenderStates {
        RenderStates {
            depth_test: DepthTest::Always,
            cull: Cull::Back,
            ..Default::default()
        }
    }
}

///
/// Used for debug purposes - only internal.
///
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(missing_docs)]
#[allow(dead_code)]
enum DebugType {
    Position,
    Normal,
    Color,
    Depth,
    Orm,
    Uv,
    None,
}
