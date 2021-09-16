use crate::core::*;
use crate::renderer::*;

#[derive(Clone)]
pub struct ColorMaterial {
    pub color: Color,
}

impl Paint for ColorMaterial {
    fn fragment_shader_source(
        &self,
        _ambient_light: Option<&AmbientLight>,
        _directional_lights: &[&DirectionalLight],
        _spot_lights: &[&SpotLight],
        _point_lights: &[&PointLight],
    ) -> String {
        include_str!("../object/shaders/mesh_color.frag").to_owned()
    }
    fn bind(
        &self,
        program: &Program,
        _camera: &Camera,
        _ambient_light: Option<&AmbientLight>,
        _directional_lights: &[&DirectionalLight],
        _spot_lights: &[&SpotLight],
        _point_lights: &[&PointLight],
    ) -> Result<()> {
        program.use_uniform_vec4("color", &self.color.to_vec4())
    }
    fn render_states(&self) -> RenderStates {
        if self.color.a != 255u8 {
            RenderStates {
                write_mask: WriteMask::COLOR,
                blend: Blend::TRANSPARENCY,
                ..Default::default()
            }
        } else {
            RenderStates::default()
        }
    }
}
