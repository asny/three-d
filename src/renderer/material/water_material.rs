use crate::core::*;
use crate::renderer::*;

pub struct WaterMaterial<'a> {
    pub environment_texture: &'a TextureCubeMap,
    pub color_texture: &'a Texture2D,
    pub depth_texture: &'a DepthTargetTexture2D,
}

impl Material for WaterMaterial<'_> {
    fn fragment_shader_source(&self, _use_vertex_colors: bool, lights: &[&dyn Light]) -> String {
        format!(
            "{}\n{}",
            lights_shader_source(
                lights,
                LightingModel::Cook(
                    NormalDistributionFunction::TrowbridgeReitzGGX,
                    GeometryFunction::SmithSchlickGGX,
                ),
            ),
            include_str!("shaders/water.frag")
        )
    }

    fn render_states(&self) -> RenderStates {
        RenderStates {
            blend: Blend::TRANSPARENCY,
            ..Default::default()
        }
    }

    fn material_type(&self) -> MaterialType {
        MaterialType::Opaque
    }

    fn use_uniforms(&self, program: &Program, camera: &Camera, lights: &[&dyn Light]) {
        for (i, light) in lights.iter().enumerate() {
            light.use_uniforms(program, i as u32);
        }
        program.use_uniform("viewProjection", camera.projection() * camera.view());
        program.use_uniform(
            "viewProjectionInverse",
            &(camera.projection() * camera.view()).invert().unwrap(),
        );
        program.use_uniform("cameraPosition", camera.position());
        program.use_uniform(
            "screenSize",
            &vec2(
                camera.viewport().width as f32,
                camera.viewport().height as f32,
            ),
        );
        program.use_texture("colorMap", self.color_texture);
        program.use_depth_texture("depthMap", self.depth_texture);
        program.use_texture_cube("environmentMap", self.environment_texture);
    }
}
