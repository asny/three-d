use crate::core::*;
use crate::renderer::*;

///
/// Used for rendering the distance from the camera to the object with this material in each pixel, together with object index.
/// Can be used for debug purposes but is also used to create shadow maps from light sources. Render depth as red component, geometry index as green component (both f32). Note: must render to a f32 texture.
///
#[derive(Default, Clone)]
pub struct IdMaterial {
    /// Index of the rendered geometry.
    pub geometry_id: u32,
    /// Render states.
    pub render_states: RenderStates,
}

impl FromCpuMaterial for IdMaterial {
    fn from_cpu_material(_context: &Context, _cpu_material: &CpuMaterial) -> Self {
        Self::default()
    }
}

impl Material for IdMaterial {
    fn id(&self) -> u16 {
        0b1u16 << 15 | 0b10u16
    }

    fn fragment_shader_source(&self, _lights: &[&dyn Light]) -> String {
        include_str!("shaders/id_material.frag").to_string()
    }

    fn fragment_attributes(&self) -> FragmentAttributes {
        FragmentAttributes {
            position: true,
            ..FragmentAttributes::NONE
        }
    }

    fn use_uniforms(&self, program: &Program, camera: &Camera, _lights: &[&dyn Light]) {
        program.use_uniform("z_near", camera.z_near());

        program.use_uniform("z_far", camera.z_far());

        program.use_uniform("geometry_id", self.geometry_id as f32);

        program.use_uniform("eye", camera.position());
    }

    fn render_states(&self) -> RenderStates {
        self.render_states
    }

    fn material_type(&self) -> MaterialType {
        MaterialType::Opaque
    }
}
