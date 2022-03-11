use crate::core::*;
use crate::renderer::*;

#[derive(Clone)]
pub struct VolumeMaterial<T: TextureDataType> {
    pub texture: std::rc::Rc<Texture3D<T>>,
    /// Base surface color. Assumed to be in linear color space.
    pub color: Color,
    /// A value in the range `[0..1]` specifying how metallic the surface is.
    pub metallic: f32,
    /// A value in the range `[0..1]` specifying how rough the surface is.
    pub roughness: f32,
    pub size: Vec3,
    pub threshold: f32,
    /// The lighting model used when rendering this material
    pub lighting_model: LightingModel,
}

impl<T: TextureDataType> Material for VolumeMaterial<T> {
    fn fragment_shader_source(&self, _use_vertex_colors: bool, lights: &[&dyn Light]) -> String {
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
