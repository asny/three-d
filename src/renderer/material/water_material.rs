use crate::core::*;
use crate::renderer::*;

///
/// A material that simulates a water surface.
/// This material needs the rendered scene (without the water surface) in a color and depth texture to be able to add reflections/refractions.
/// Therefore, the material needs to be updated/constructed each frame.
///
#[derive(Clone)]
pub struct WaterMaterial<'a> {
    /// A reference to the environnment texture of the scene which is used for reflections.
    pub environment_texture: &'a TextureCubeMap,
    /// A reference to a color texture that contains a render of the entire scene without the water surface. Used for reflections/refractions.
    pub color_texture: &'a Texture2D,
    /// A reference to a depth texture that contains a render of the entire scene without the water surface. Used for reflections/refractions.
    pub depth_texture: &'a DepthTargetTexture2D,
    /// A value in the range `[0..1]` specifying how metallic the surface is.
    pub metallic: f32,
    /// A value in the range `[0..1]` specifying how rough the surface is.
    pub roughness: f32,
    /// The lighting model used when rendering this material
    pub lighting_model: LightingModel,
}

impl Material for WaterMaterial<'_> {
    fn fragment_shader_source(&self, _use_vertex_colors: bool, lights: &[&dyn Light]) -> String {
        format!(
            "{}\n{}",
            lights_shader_source(lights, self.lighting_model),
            include_str!("shaders/water_material.frag")
        )
    }

    fn render_states(&self) -> RenderStates {
        RenderStates {
            blend: Blend::TRANSPARENCY,
            ..Default::default()
        }
    }

    fn material_type(&self) -> MaterialType {
        MaterialType::Opaque
    }

    fn use_uniforms(&self, program: &Program, camera: &Camera, lights: &[&dyn Light]) {
        for (i, light) in lights.iter().enumerate() {
            light.use_uniforms(program, i as u32);
        }
        program.use_uniform("viewProjection", camera.projection() * camera.view());
        program.use_uniform(
            "viewProjectionInverse",
            &(camera.projection() * camera.view()).invert().unwrap(),
        );
        program.use_uniform("cameraPosition", camera.position());
        program.use_uniform(
            "screenSize",
            &vec2(
                camera.viewport().width as f32,
                camera.viewport().height as f32,
            ),
        );
        program.use_uniform("metallic", self.metallic);
        program.use_uniform("roughness", self.roughness);
        program.use_texture("colorMap", self.color_texture);
        program.use_depth_texture("depthMap", self.depth_texture);
        program.use_texture_cube("environmentMap", self.environment_texture);
    }
}
