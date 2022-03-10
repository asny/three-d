use crate::core::*;
use crate::renderer::*;

pub struct VolumeMaterial<T: TextureDataType> {
    pub texture: Texture3D<T>,
    /// The lighting model used when rendering this material
    pub lighting_model: LightingModel,
}

impl<T: TextureDataType> Material for VolumeMaterial<T> {
    fn fragment_shader_source(&self, use_vertex_colors: bool, lights: &[&dyn Light]) -> String {
        let mut output = lights_fragment_shader_source(lights, self.lighting_model);
        output.push_str(include_str!("shaders/volume_material.frag"));
        output
    }
    fn use_uniforms(
        &self,
        program: &Program,
        camera: &Camera,
        _lights: &[&dyn Light],
    ) -> ThreeDResult<()> {
        program.use_uniform_block("Camera", camera.uniform_buffer());
        program.use_texture("tex", &self.texture)
    }
    fn render_states(&self) -> RenderStates {
        RenderStates::default()
    }
    fn is_transparent(&self) -> bool {
        false
    }
}
