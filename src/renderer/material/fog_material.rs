use crate::renderer::*;

///
/// A material that simulates fog, ie. the area where it is applied gets hazy when objects are far away.
///
#[derive(Clone)]
pub struct FogMaterial {
    /// The color of the fog.
    pub color: Color,
    /// The density of the fog.
    pub density: f32,
    /// Determines the variation on the density as a function of time.
    pub animation: f32,
    pub time: f64,
}

impl FogMaterial {
    ///
    /// Constructs a new fog material.
    ///
    pub fn new(color: Color, density: f32, animation: f32) -> Self {
        Self {
            color,
            density,
            animation,
            time: 0.0,
        }
    }
}

impl PostMaterial for FogMaterial {
    fn fragment_shader_source(
        &self,
        _lights: &[&dyn Light],
        _color_texture: ColorTexture,
        depth_texture: DepthTexture,
    ) -> String {
        format!(
            "{}\n{}\n{}",
            include_str!("../../core/shared.frag"),
            depth_texture
                .fragment_shader_source()
                .expect("Must supply a depth texture to apply a fog effect"),
            include_str!("shaders/fog_material.frag")
        )
    }

    fn use_uniforms(
        &self,
        program: &Program,
        camera: &Camera,
        _lights: &[&dyn Light],
        _color_texture: ColorTexture,
        depth_texture: DepthTexture,
    ) {
        depth_texture.use_uniforms(program);
        program.use_uniform(
            "viewProjectionInverse",
            (camera.projection() * camera.view()).invert().unwrap(),
        );
        program.use_uniform("fogColor", self.color);
        program.use_uniform("fogDensity", self.density);
        program.use_uniform("animation", self.animation);
        program.use_uniform("time", 0.001 * self.time as f32);
        program.use_uniform("eyePosition", camera.position());
    }

    fn render_states(&self) -> RenderStates {
        RenderStates {
            write_mask: WriteMask::COLOR,
            blend: Blend::TRANSPARENCY,
            cull: Cull::Back,
            ..Default::default()
        }
    }
}
