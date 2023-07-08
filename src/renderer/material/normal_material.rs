use crate::core::*;
use crate::renderer::*;

///
/// Render the object with colors that reflect its normals which primarily is used for debug purposes.
/// A normal with an x value of -1 yields 0.0 in the red channel and an x value of 1 yields 1.0 in the red channel.
/// The same mapping is applied from y value to green channel and z value to blue channel.
///
#[derive(Clone)]
pub struct NormalMaterial {
    /// A scalar multiplier applied to each normal vector of the [Self::normal_texture].
    pub normal_scale: f32,
    /// A tangent space normal map, also known as bump map.
    pub normal_texture: Option<Texture2DRef>,
    /// Render states.
    pub render_states: RenderStates,
}

impl NormalMaterial {
    /// Constructs a new normal material from a [CpuMaterial] where only relevant information is used.
    pub fn new(context: &Context, cpu_material: &CpuMaterial) -> Self {
        let normal_texture = cpu_material
            .normal_texture
            .as_ref()
            .map(|cpu_texture| Texture2DRef::from_cpu_texture(context, cpu_texture));
        Self {
            normal_scale: cpu_material.normal_scale,
            normal_texture,
            render_states: RenderStates::default(),
        }
    }

    /// Creates a normal material from a [PhysicalMaterial].
    pub fn from_physical_material(physical_material: &PhysicalMaterial) -> Self {
        Self {
            normal_scale: physical_material.normal_scale,
            normal_texture: physical_material.normal_texture.clone(),
            render_states: RenderStates {
                write_mask: WriteMask::default(),
                blend: Blend::Disabled,
                ..physical_material.render_states
            },
        }
    }
}

impl FromCpuMaterial for NormalMaterial {
    fn from_cpu_material(context: &Context, cpu_material: &CpuMaterial) -> Self {
        Self::new(context, cpu_material)
    }
}

impl Material for NormalMaterial {
    fn id(&self) -> u16 {
        if self.normal_texture.is_some() {
            0b1u16 << 15 | 0b110u16
        } else {
            0b1u16 << 15 | 0b111u16
        }
    }

    fn fragment_shader_source(&self, _lights: &[&dyn Light]) -> String {
        let mut attributes = FragmentAttributes {
            normal: true,
            ..FragmentAttributes::NONE
        };
        let mut source = String::new();
        if self.normal_texture.is_some() {
            attributes.uv = true;
            attributes.tangents = true;
            source.push_str("#define USE_TEXTURE\nin vec2 uvs;\nin vec3 tang;\nin vec3 bitang;\n");
        }
        source.push_str(include_str!("shaders/normal_material.frag"));
        source
    }

    fn fragment_attributes(&self) -> FragmentAttributes {
        FragmentAttributes {
            normal: true,
            uv: self.normal_texture.is_some(),
            tangents: self.normal_texture.is_some(),
            ..FragmentAttributes::NONE
        }
    }

    fn use_uniforms(&self, program: &Program, _camera: &Camera, _lights: &[&dyn Light]) {
        if let Some(ref tex) = self.normal_texture {
            program.use_uniform("normalScale", self.normal_scale);
            program.use_uniform("textureTransformation", tex.transformation);
            program.use_texture("normalTexture", tex);
        }
    }
    fn render_states(&self) -> RenderStates {
        self.render_states
    }
    fn material_type(&self) -> MaterialType {
        MaterialType::Opaque
    }
}

impl Default for NormalMaterial {
    fn default() -> Self {
        Self {
            normal_texture: None,
            normal_scale: 1.0,
            render_states: RenderStates::default(),
        }
    }
}
