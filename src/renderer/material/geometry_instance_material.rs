use crate::core::*;
use crate::renderer::*;

///
/// Used for rendering a unique color for each instance of an object with this material,
/// as to allow uniquely determining which pixels belong to which object instance.
///
#[derive(Default, Clone)]
pub struct GeometryInstanceMaterial {
    /// Geometry ID
    pub id: usize,
    /// Render states.
    pub render_states: RenderStates,
}

impl FromCpuMaterial for GeometryInstanceMaterial {
    fn from_cpu_material(_context: &Context, _cpu_material: &CpuMaterial) -> Self {
        Self::default()
    }
}

impl Material for GeometryInstanceMaterial {
    fn id(&self) -> EffectMaterialId {
        EffectMaterialId::GeometryInstanceMaterial
    }

    fn fragment_shader_source(&self, _lights: &[&dyn Light]) -> String {
        include_str!("shaders/geometry_instance_material.frag").to_string()
    }

    fn fragment_attributes(&self) -> FragmentAttributes {
        FragmentAttributes {
            instance_id: true,
            ..FragmentAttributes::NONE
        }
    }

    fn use_uniforms(&self, program: &Program, camera: &Camera, _lights: &[&dyn Light]) {
        program.use_uniform("id", self.id as f32);
        program.use_uniform("eye", camera.position());
    }

    fn render_states(&self) -> RenderStates {
        self.render_states
    }

    fn material_type(&self) -> MaterialType {
        MaterialType::Opaque
    }
}
