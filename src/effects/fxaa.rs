
use crate::math::*;
use crate::core::*;
use crate::effects::*;

pub struct FXAAEffect {
    pub color: Vec3,
    pub density: f32,
    pub animation: f32,
    image_effect: ImageEffect
}

impl FXAAEffect {

    pub fn new(gl: &Context) -> Result<Self, Error>
    {
        Ok(Self {color: vec3(0.8, 0.8, 0.8), density: 0.2, animation: 0.1, image_effect: ImageEffect::new(gl, include_str!("shaders/fxaa.frag"))?})
    }

    pub fn apply(&self, viewport: Viewport, color_texture: &dyn Texture) -> Result<(), Error>
    {
        let render_states = RenderStates {cull: CullType::Back, write_mask: WriteMask::color(), depth_test: DepthTestType::Always, ..Default::default()};

        self.image_effect.program().use_texture(color_texture, "colorMap")?;
        self.image_effect.program().add_uniform_vec2("resolution", &vec2(color_texture.width() as f32, color_texture.height() as f32))?;

        self.image_effect.apply(render_states, viewport)?;
        Ok(())
    }

}