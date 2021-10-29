use crate::core::*;
use crate::renderer::*;
use std::rc::Rc;

///
/// A material that renders a [Shadable] object in a color defined by multiplying a color with an optional texture and optional per vertex colors.
/// This material is not affected by lights.
///
#[derive(Clone)]
pub struct ColorMaterial {
    /// A color applied everywhere.
    pub color: Color,
    /// An optional texture which is samples using uv coordinates (requires that the [Shadable] object supports uv coordinates).
    pub texture: Option<Rc<Texture2D>>,
    /// Render states used when the color is opaque (has a maximal alpha value).
    pub opaque_render_states: RenderStates,
    /// Render states used when the color is transparent (does not have a maximal alpha value).
    pub transparent_render_states: RenderStates,
}
impl ColorMaterial {
    /// Constructs a new color material from a [CPUMaterial].
    pub fn new(context: &Context, cpu_material: &CPUMaterial) -> ThreeDResult<Self> {
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

    pub fn from_physical_material(physical_material: &PhysicalMaterial) -> Self {
        Self {
            color: physical_material.albedo,
            texture: physical_material.albedo_texture.clone(),
            opaque_render_states: physical_material.opaque_render_states,
            transparent_render_states: physical_material.transparent_render_states,
        }
    }
}

impl ForwardMaterial for ColorMaterial {
    fn fragment_shader_source(&self, use_vertex_colors: bool, _lights: &Lights) -> String {
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
    fn use_uniforms(
        &self,
        program: &Program,
        _camera: &Camera,
        _lights: &Lights,
    ) -> ThreeDResult<()> {
        program.use_uniform_vec4("color", &self.color.to_vec4())?;
        if let Some(ref tex) = self.texture {
            program.use_texture("tex", &**tex)?
        }
        Ok(())
    }
    fn render_states(&self) -> RenderStates {
        if self.is_transparent() {
            self.transparent_render_states
        } else {
            self.opaque_render_states
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
            opaque_render_states: RenderStates::default(),
            transparent_render_states: RenderStates {
                write_mask: WriteMask::COLOR,
                blend: Blend::TRANSPARENCY,
                ..Default::default()
            },
        }
    }
}
