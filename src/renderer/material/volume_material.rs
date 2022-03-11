use crate::core::*;
use crate::renderer::*;

pub struct VolumeMaterial {
    pub texture: Texture3D<u8>,
    /// The lighting model used when rendering this material
    pub lighting_model: LightingModel,
    pub max_distance: f32,
}

impl Material for VolumeMaterial {
    fn fragment_shader_source(&self, use_vertex_colors: bool, lights: &[&dyn Light]) -> String {
        let mut output = lights_fragment_shader_source(lights, self.lighting_model);
        output.push_str(include_str!("shaders/volume_material.frag"));
        output
    }
    fn use_uniforms(
        &self,
        program: &Program,
        camera: &Camera,
        lights: &[&dyn Light],
    ) -> ThreeDResult<()> {
        for (i, light) in lights.iter().enumerate() {
            light.use_uniforms(program, i as u32)?;
        }
        program.use_uniform_block("Camera", camera.uniform_buffer());
        program.use_uniform("max_distance", self.max_distance)?;
        program.use_texture("tex", &self.texture)
    }
    fn render_states(&self) -> RenderStates {
        RenderStates {
            blend: Blend::TRANSPARENCY,
            ..Default::default()
        }
    }
    fn is_transparent(&self) -> bool {
        false
    }
}
