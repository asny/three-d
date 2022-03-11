use crate::core::*;
use crate::renderer::*;

///
/// A material that renders the surface defined by the voxel data in the [VolumeMaterial::texture].
/// The surface is defined by all the positions in the volume where the red channel of the voxel data is larger than [VolumeMaterial::threshold].
///
///
#[derive(Clone)]
pub struct VolumeSurfaceMaterial<T: TextureDataType> {
    pub texture: std::rc::Rc<Texture3D<T>>,
    /// Base surface color. Assumed to be in linear color space.
    pub color: Color,
    /// A value in the range `[0..1]` specifying how metallic the surface is.
    pub metallic: f32,
    /// A value in the range `[0..1]` specifying how rough the surface is.
    pub roughness: f32,
    /// The size of the cube that is used to render the voxel data. The texture is scaled to fill the entire cube.
    pub size: Vec3,
    /// Threshold (in the range [0..1]) that defines the surface in the voxel data.
    pub threshold: f32,
    /// The lighting model used when rendering this material
    pub lighting_model: LightingModel,
}

impl<T: TextureDataType> Material for VolumeSurfaceMaterial<T> {
    fn fragment_shader_source(&self, _use_vertex_colors: bool, lights: &[&dyn Light]) -> String {
        let mut output = lights_fragment_shader_source(lights, self.lighting_model);
        output.push_str(include_str!("shaders/volume_surface_material.frag"));
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
        program.use_uniform("camera_position", camera.position())?;
        program.use_uniform("surface_color", self.color.to_vec4())?;
        program.use_uniform("metallic", self.metallic)?;
        program.use_uniform("roughness", self.roughness)?;
        program.use_uniform("size", self.size)?;
        program.use_uniform("threshold", self.threshold)?;
        program.use_texture("tex", &self.texture)
    }
    fn render_states(&self) -> RenderStates {
        RenderStates {
            blend: Blend::TRANSPARENCY,
            ..Default::default()
        }
    }
    fn is_transparent(&self) -> bool {
        true
    }
}
