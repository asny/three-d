use crate::core::*;
use crate::renderer::*;
use std::rc::Rc;

///
/// A material that renders a [Geometry] in a color defined by multiplying a color with an optional texture and optional per vertex colors.
/// This material is not affected by lights.
///
#[derive(Clone, Default)]
pub struct ColorMaterial {
    /// A color applied everywhere.
    pub color: Color,
    /// An optional texture which is samples using uv coordinates (requires that the [Geometry] supports uv coordinates).
    pub texture: Option<Rc<Texture2D<u8>>>,
    /// Render states
    pub render_states: RenderStates,

    pub is_transparent: bool,
}
impl ColorMaterial {
    ///
    /// Constructs a new color material from a [CpuMaterial].
    /// Tries to infer whether this material is transparent or opaque from the alpha value of the albedo color and the alpha values in the albedo texture.
    /// Since this is not always correct, it is preferred to use [ColorMaterial::new_opaque] or [ColorMaterial::new_transparent].
    ///
    pub fn new(context: &Context, cpu_material: &CpuMaterial) -> ThreeDResult<Self> {
        let is_transparent = cpu_material.albedo.a == 255
            || cpu_material
                .albedo_texture
                .as_ref()
                .map(|t| t.is_transparent())
                .unwrap_or(false);

        if is_transparent {
            Self::new_transparent(context, cpu_material)
        } else {
            Self::new_opaque(context, cpu_material)
        }
    }

    /// Constructs a new opaque color material from a [CpuMaterial].
    pub fn new_opaque(context: &Context, cpu_material: &CpuMaterial) -> ThreeDResult<Self> {
        let texture = if let Some(ref cpu_texture) = cpu_material.albedo_texture {
            Some(Rc::new(Texture2D::new(&context, cpu_texture)?))
        } else {
            None
        };
        Ok(Self {
            color: cpu_material.albedo,
            texture,
            is_transparent: false,
            render_states: RenderStates::default(),
        })
    }

    /// Constructs a new transparent color material from a [CpuMaterial].
    pub fn new_transparent(context: &Context, cpu_material: &CpuMaterial) -> ThreeDResult<Self> {
        let texture = if let Some(ref cpu_texture) = cpu_material.albedo_texture {
            Some(Rc::new(Texture2D::new(&context, cpu_texture)?))
        } else {
            None
        };
        Ok(Self {
            color: cpu_material.albedo,
            texture,
            is_transparent: true,
            render_states: RenderStates {
                write_mask: WriteMask::COLOR,
                blend: Blend::TRANSPARENCY,
                ..Default::default()
            },
        })
    }

    pub fn from_physical_material(physical_material: &PhysicalMaterial) -> Self {
        Self {
            color: physical_material.albedo,
            texture: physical_material.albedo_texture.clone(),
            render_states: physical_material.render_states,
            is_transparent: physical_material.is_transparent,
        }
    }
}

impl Material for ColorMaterial {
    fn fragment_shader_source(&self, use_vertex_colors: bool, _lights: &[&dyn Light]) -> String {
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
        _lights: &[&dyn Light],
    ) -> ThreeDResult<()> {
        program.use_uniform_vec4("surfaceColor", &self.color.to_vec4())?;
        if let Some(ref tex) = self.texture {
            program.use_texture("tex", &**tex)?
        }
        Ok(())
    }
    fn render_states(&self) -> RenderStates {
        self.render_states
    }
    fn is_transparent(&self) -> bool {
        self.is_transparent
    }
}
