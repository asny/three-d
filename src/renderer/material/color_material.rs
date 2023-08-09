use crate::core::*;
use crate::renderer::*;

///
/// A material that renders a [Geometry] in a color defined by multiplying a color with an optional texture and optional per vertex colors.
/// This material is not affected by lights.
///
#[derive(Clone, Default)]
pub struct ColorMaterial {
    /// Base surface color.
    pub color: Srgba,
    /// An optional texture which is samples using uv coordinates (requires that the [Geometry] supports uv coordinates).
    /// The colors are assumed to be in linear sRGB (`RgbU8`), linear sRGB with an alpha channel (`RgbaU8`) or HDR color space.
    pub texture: Option<Texture2DRef>,
    /// Render states.
    pub render_states: RenderStates,
    /// Whether this material should be treated as a transparent material (An object needs to be rendered differently depending on whether it is transparent or opaque).
    pub is_transparent: bool,
}

impl ColorMaterial {
    ///
    /// Constructs a new color material from a [CpuMaterial].
    /// Tries to infer whether this material is transparent or opaque from the alpha value of the albedo color and the alpha values in the albedo texture.
    /// Since this is not always correct, it is preferred to use [ColorMaterial::new_opaque] or [ColorMaterial::new_transparent].
    ///
    pub fn new(context: &Context, cpu_material: &CpuMaterial) -> Self {
        if super::is_transparent(cpu_material) {
            Self::new_transparent(context, cpu_material)
        } else {
            Self::new_opaque(context, cpu_material)
        }
    }

    /// Constructs a new opaque color material from a [CpuMaterial].
    pub fn new_opaque(context: &Context, cpu_material: &CpuMaterial) -> Self {
        let texture =
            cpu_material
                .albedo_texture
                .as_ref()
                .map(|cpu_texture| match &cpu_texture.data {
                    TextureData::RgbU8(_) | TextureData::RgbaU8(_) => {
                        let mut cpu_texture = cpu_texture.clone();
                        cpu_texture.data.to_linear_srgb();
                        Texture2DRef::from_cpu_texture(context, &cpu_texture)
                    }
                    _ => Texture2DRef::from_cpu_texture(context, cpu_texture),
                });
        Self {
            color: cpu_material.albedo,
            texture,
            is_transparent: false,
            render_states: RenderStates::default(),
        }
    }

    /// Constructs a new transparent color material from a [CpuMaterial].
    pub fn new_transparent(context: &Context, cpu_material: &CpuMaterial) -> Self {
        let texture =
            cpu_material
                .albedo_texture
                .as_ref()
                .map(|cpu_texture| match &cpu_texture.data {
                    TextureData::RgbU8(_) | TextureData::RgbaU8(_) => {
                        let mut cpu_texture = cpu_texture.clone();
                        cpu_texture.data.to_linear_srgb();
                        Texture2DRef::from_cpu_texture(context, &cpu_texture)
                    }
                    _ => Texture2DRef::from_cpu_texture(context, cpu_texture),
                });
        Self {
            color: cpu_material.albedo,
            texture,
            is_transparent: true,
            render_states: RenderStates {
                write_mask: WriteMask::COLOR,
                blend: Blend::TRANSPARENCY,
                ..Default::default()
            },
        }
    }

    /// Creates a color material from a [PhysicalMaterial].
    pub fn from_physical_material(physical_material: &PhysicalMaterial) -> Self {
        Self {
            color: physical_material.albedo,
            texture: physical_material.albedo_texture.clone(),
            render_states: physical_material.render_states,
            is_transparent: physical_material.is_transparent,
        }
    }
}

impl FromCpuMaterial for ColorMaterial {
    fn from_cpu_material(context: &Context, cpu_material: &CpuMaterial) -> Self {
        Self::new(context, cpu_material)
    }
}

impl Material for ColorMaterial {
    fn id(&self) -> u16 {
        if self.texture.is_some() {
            0b1u16 << 15
        } else {
            0b1u16 << 15 | 0b1u16
        }
    }

    fn fragment_shader_source(&self, _lights: &[&dyn Light]) -> String {
        let mut shader = String::new();
        if self.texture.is_some() {
            shader.push_str("#define USE_TEXTURE\nin vec2 uvs;\n");
        }
        shader.push_str(include_str!("../../core/shared.frag"));
        shader.push_str(ColorMapping::fragment_shader_source());
        shader.push_str(include_str!("shaders/color_material.frag"));
        shader
    }

    fn fragment_attributes(&self) -> FragmentAttributes {
        FragmentAttributes {
            color: true,
            uv: self.texture.is_some(),
            ..FragmentAttributes::NONE
        }
    }

    fn use_uniforms(&self, program: &Program, camera: &Camera, _lights: &[&dyn Light]) {
        camera.color_mapping.use_uniforms(program);
        program.use_uniform("surfaceColor", self.color.to_linear_srgb());
        if let Some(ref tex) = self.texture {
            program.use_uniform("textureTransformation", tex.transformation);
            program.use_texture("tex", tex);
        }
    }
    fn render_states(&self) -> RenderStates {
        self.render_states
    }
    fn material_type(&self) -> MaterialType {
        if self.is_transparent {
            MaterialType::Transparent
        } else {
            MaterialType::Opaque
        }
    }
}
