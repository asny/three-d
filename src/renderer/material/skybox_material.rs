use crate::core::*;
use crate::renderer::*;

pub struct SkyboxMaterial<T: TextureDataType> {
    pub texture: TextureCubeMap<T>,
}

impl<T: TextureDataType> Material for SkyboxMaterial<T> {
    fn fragment_shader_source(&self, _use_vertex_colors: bool, _lights: &[&dyn Light]) -> String {
        format!(
            "{}{}",
            include_str!("../../core/shared.frag"),
            include_str!("shaders/skybox_material.frag")
        )
    }

    fn use_uniforms(
        &self,
        program: &Program,
        camera: &Camera,
        _lights: &[&dyn Light],
    ) -> ThreeDResult<()> {
        program.use_uniform("isHDR", if T::bits_per_channel() > 8 { &1 } else { &0 })?;
        program.use_texture_cube("texture0", &self.texture)?;
        program.use_uniform_block("Camera", camera.uniform_buffer());
        Ok(())
    }

    fn render_states(&self) -> RenderStates {
        RenderStates {
            depth_test: DepthTest::LessOrEqual,
            cull: Cull::Front,
            ..Default::default()
        }
    }

    fn is_transparent(&self) -> bool {
        false
    }
}
