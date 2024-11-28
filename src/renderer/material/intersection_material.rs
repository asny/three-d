use crate::core::*;
use crate::renderer::*;

///
/// Used for intersection tests, see [pick] and [ray_intersect].
/// When rendering with this material, the output in each pixel is:
/// - Red channel: The depth (same as [DepthMaterial]).
/// - Green channel: The [IntersectionMaterial::geometry_id].
/// - Blue channel: The [gl_InstanceID](https://registry.khronos.org/OpenGL-Refpages/gl4/html/gl_InstanceID.xhtml).
///
/// Note: The geometry needs to pass the instance ID to the fragment shader, see [Geometry] for more information.
///
#[derive(Default, Clone)]
pub struct IntersectionMaterial {
    /// The minimum distance from the camera to any object. If None, then the near plane of the camera is used.
    pub min_distance: Option<f32>,
    /// The maximum distance from the camera to any object. If None, then the far plane of the camera is used.
    pub max_distance: Option<f32>,
    /// Render states.
    pub render_states: RenderStates,
    /// A geometry ID for the currently rendered geometry. The result is outputted in the green color channel.
    pub geometry_id: u32,
}

impl FromCpuMaterial for IntersectionMaterial {
    fn from_cpu_material(_context: &Context, _cpu_material: &CpuMaterial) -> Self {
        Self::default()
    }
}

impl Material for IntersectionMaterial {
    fn id(&self) -> EffectMaterialId {
        EffectMaterialId::IntersectionMaterial
    }

    fn fragment_shader_source(&self, _lights: &[&dyn Light]) -> String {
        include_str!("shaders/intersection_material.frag").to_string()
    }

    fn use_uniforms(&self, program: &Program, viewer: &dyn Viewer, _lights: &[&dyn Light]) {
        program.use_uniform(
            "minDistance",
            self.min_distance.unwrap_or_else(|| viewer.z_near()),
        );
        program.use_uniform(
            "maxDistance",
            self.max_distance.unwrap_or_else(|| viewer.z_far()),
        );
        program.use_uniform("eye", viewer.position());
        program.use_uniform("geometryId", self.geometry_id);
    }

    fn render_states(&self) -> RenderStates {
        self.render_states
    }

    fn material_type(&self) -> MaterialType {
        MaterialType::Opaque
    }
}
