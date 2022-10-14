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
    /// A value in the range `[0..1]` specifying how metallic the surface is.
    pub metallic: f32,
    /// A value in the range `[0..1]` specifying how rough the surface is.
    pub roughness: f32,
    /// The lighting model used when rendering this material
    pub lighting_model: LightingModel,
}

impl EffectMaterial for WaterMaterial<'_> {
    fn fragment_shader_source(&self, lights: &[&dyn Light]) -> String {
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

    fn use_uniforms(
        &self,
        program: &Program,
        camera: &Camera,
        lights: &[&dyn Light],
        color_texture: Option<&Texture2D>,
        depth_texture: Option<&DepthTargetTexture2D>,
    ) {
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
        program.use_texture(
            "colorMap",
            color_texture.expect("Must supply a color texture to apply a water effect"),
        );
        program.use_depth_texture(
            "depthMap",
            depth_texture.expect("Must supply a depth texture to apply a water effect"),
        );
        program.use_texture_cube("environmentMap", self.environment_texture);
    }
}
