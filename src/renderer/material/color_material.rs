use crate::core::*;
use crate::renderer::*;
use std::rc::Rc;

#[derive(Clone)]
pub struct ColorMaterial {
    pub color: Color,
    pub texture: Option<Rc<Texture2D>>,
    pub render_states: RenderStates,
    pub transparent_render_states: RenderStates,
}
impl ColorMaterial {
    pub fn new(context: &Context, cpu_material: &CPUMaterial) -> Result<Self> {
        let texture = if let Some(ref cpu_texture) = cpu_material.albedo_texture {
            Some(Rc::new(Texture2D::new(&context, cpu_texture)?))
        } else {
            None
        };
        Ok(Self {
            color: cpu_material.albedo,
            texture,
            ..Default::default()
        })
    }
}

impl ForwardMaterial for ColorMaterial {
    fn fragment_shader_source(&self, _lights: &[&dyn Light], use_vertex_colors: bool) -> String {
        let mut shader = String::new();
        if self.texture.is_some() {
            shader.push_str("#define USE_TEXTURE\nin vec2 uvs;\n");
        }
        if use_vertex_colors {
            shader.push_str("#define USE_VERTEX_COLORS\nin vec4 col;\n");
        }
        shader.push_str(include_str!("../../core/shared.frag"));
        shader.push_str(include_str!("shaders/color_material.frag"));
        shader
    }
    fn bind(&self, program: &Program, _camera: &Camera, _lights: &[&dyn Light]) -> Result<()> {
        program.use_uniform_vec4("color", &self.color.to_vec4())?;
        if let Some(ref tex) = self.texture {
            program.use_texture("tex", &**tex)?
        }
        Ok(())
    }
    fn render_states(&self, transparent: bool) -> RenderStates {
        if transparent || self.is_transparent() {
            self.transparent_render_states
        } else {
            self.render_states
        }
    }
    fn is_transparent(&self) -> bool {
        self.color.a != 255u8
            || self
                .texture
                .as_ref()
                .map(|t| t.is_transparent())
                .unwrap_or(false)
    }
}

impl Default for ColorMaterial {
    fn default() -> Self {
        Self {
            color: Color::default(),
            texture: None,
            render_states: RenderStates::default(),
            transparent_render_states: RenderStates {
                write_mask: WriteMask::COLOR,
                blend: Blend::TRANSPARENCY,
                ..Default::default()
            },
        }
    }
}
