use crate::core::*;
use crate::renderer::*;
use std::sync::Arc;

/// The background of the scene.
#[derive(Clone)]
pub enum Background {
    /// Environnment texture.
    Texture(Arc<TextureCubeMap>),
    /// Background color.
    Color(Srgba),
}

impl Default for Background {
    fn default() -> Self {
        Self::Color(Srgba::WHITE)
    }
}

///
/// An effect that simulates a water surface and should therefore only be applied to a water surface geometry.
/// This effect needs the rendered scene (without the water surface) in a color and depth texture to be able to add reflections and refractions.
///
#[derive(Clone)]
pub struct WaterEffect {
    /// The background of the scene which is used for reflections.
    pub background: Background,
    /// A value in the range `[0..1]` specifying how metallic the surface is.
    pub metallic: f32,
    /// A value in the range `[0..1]` specifying how rough the surface is.
    pub roughness: f32,
    /// The lighting model used when rendering this effect
    pub lighting_model: LightingModel,
}

impl Effect for WaterEffect {
    fn fragment_shader_source(
        &self,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) -> String {
        format!(
            "{}\n{}\n{}\n{}\n{}\n{}\n{}",
            match &self.background {
                Background::Color(_) => "",
                Background::Texture(_) => "#define USE_BACKGROUND_TEXTURE",
            },
            color_texture
                .expect("Must supply a color texture to apply a water effect")
                .fragment_shader_source(),
            depth_texture
                .expect("Must supply a depth texture to apply a water effect")
                .fragment_shader_source(),
            lights_shader_source(lights, self.lighting_model),
            ToneMapping::fragment_shader_source(),
            ColorMapping::fragment_shader_source(),
            include_str!("shaders/water_effect.frag")
        )
    }

    fn id(&self, color_texture: Option<ColorTexture>, depth_texture: Option<DepthTexture>) -> u16 {
        EffectMaterialId::WaterEffect(
            color_texture.expect("Must supply a color texture to apply a water effect"),
            depth_texture.expect("Must supply a depth texture to apply a water effect"),
        )
        .0
    }

    fn fragment_attributes(&self) -> FragmentAttributes {
        FragmentAttributes {
            position: true,
            normal: true,
            uv: true,
            ..FragmentAttributes::NONE
        }
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
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) {
        camera.tone_mapping.use_uniforms(program);
        camera.color_mapping.use_uniforms(program);
        color_texture
            .expect("Must supply a color texture to apply a water effect")
            .use_uniforms(program);
        depth_texture
            .expect("Must supply a depth texture to apply a water effect")
            .use_uniforms(program);
        for (i, light) in lights.iter().enumerate() {
            light.use_uniforms(program, i as u32);
        }
        program.use_uniform("viewProjection", camera.projection() * camera.view());
        program.use_uniform(
            "viewProjectionInverse",
            (camera.projection() * camera.view()).invert().unwrap(),
        );
        program.use_uniform("cameraPosition", camera.position());
        program.use_uniform(
            "screenSize",
            vec2(
                camera.viewport().width as f32,
                camera.viewport().height as f32,
            ),
        );
        program.use_uniform("metallic", self.metallic);
        program.use_uniform("roughness", self.roughness);
        match &self.background {
            Background::Color(color) => {
                program.use_uniform("environmentColor", color.to_linear_srgb())
            }
            Background::Texture(tex) => program.use_texture_cube("environmentMap", tex),
        }
    }
}

impl Default for WaterEffect {
    fn default() -> Self {
        Self {
            background: Background::default(),
            metallic: 0.0,
            roughness: 1.0,
            lighting_model: LightingModel::Blinn,
        }
    }
}
