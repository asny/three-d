use crate::core::*;
use crate::renderer::*;

#[derive(Clone, Default)]
pub struct DepthMaterial {}

impl Paint for DepthMaterial {
    fn fragment_shader_source(
        &self,
        _ambient_light: Option<&AmbientLight>,
        _directional_lights: &[&DirectionalLight],
        _spot_lights: &[&SpotLight],
        _point_lights: &[&PointLight],
    ) -> String {
        "void main() {}".to_string()
    }
    fn bind(
        &self,
        _program: &Program,
        _camera: &Camera,
        _ambient_light: Option<&AmbientLight>,
        _directional_lights: &[&DirectionalLight],
        _spot_lights: &[&SpotLight],
        _point_lights: &[&PointLight],
    ) -> Result<()> {
        Ok(())
    }
    fn render_states(&self) -> RenderStates {
        RenderStates {
            write_mask: WriteMask::DEPTH,
            ..Default::default()
        }
    }
}
