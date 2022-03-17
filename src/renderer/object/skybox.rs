use crate::core::*;
use crate::renderer::*;

///
/// An illusion of a sky.
///
pub struct Skybox {
    context: Context,
    vertex_buffer: VertexBuffer<Vec3>,
    material: SkyboxMaterial,
}

impl Skybox {
    ///
    /// Creates a new skybox with the given cpu-side version of a [TextureCubeMap].
    ///
    pub fn new(context: &Context, cpu_texture: &CpuTextureCube) -> ThreeDResult<Self> {
        let texture = TextureCubeMap::new(&context, cpu_texture)?;
        Self::new_with_texture(context, texture)
    }

    ///
    /// Creates a new skybox with a cube texture generated from the equirectangular texture given as input.
    ///
    pub fn new_from_equirectangular(
        context: &Context,
        cpu_texture: &CpuTexture,
    ) -> ThreeDResult<Self> {
        let texture = match cpu_texture.data {
            TextureData::RgbaU8(_)
            | TextureData::RgbU8(_)
            | TextureData::RgU8(_)
            | TextureData::RU8(_) => {
                TextureCubeMap::new_from_equirectangular::<Vector3<u8>>(context, cpu_texture)?
            }
            TextureData::RgbaF16(_)
            | TextureData::RgbF16(_)
            | TextureData::RgF16(_)
            | TextureData::RF16(_) => {
                TextureCubeMap::new_from_equirectangular::<Vector3<f16>>(context, cpu_texture)?
            }
            TextureData::RgbaF32(_)
            | TextureData::RgbF32(_)
            | TextureData::RgF32(_)
            | TextureData::RF32(_) => {
                TextureCubeMap::new_from_equirectangular::<Vector3<f32>>(context, cpu_texture)?
            }
        };

        Self::new_with_texture(context, texture)
    }

    ///
    /// Creates a new skybox with the given [TextureCubeMap].
    ///
    pub fn new_with_texture(context: &Context, texture: TextureCubeMap) -> ThreeDResult<Self> {
        let vertex_buffer = VertexBuffer::new_with_data(
            context,
            &[
                vec3(1.0, 1.0, -1.0),
                vec3(-1.0, 1.0, -1.0),
                vec3(1.0, 1.0, 1.0),
                vec3(-1.0, 1.0, 1.0),
                vec3(1.0, 1.0, 1.0),
                vec3(-1.0, 1.0, -1.0),
                vec3(-1.0, -1.0, -1.0),
                vec3(1.0, -1.0, -1.0),
                vec3(1.0, -1.0, 1.0),
                vec3(1.0, -1.0, 1.0),
                vec3(-1.0, -1.0, 1.0),
                vec3(-1.0, -1.0, -1.0),
                vec3(1.0, -1.0, -1.0),
                vec3(-1.0, -1.0, -1.0),
                vec3(1.0, 1.0, -1.0),
                vec3(-1.0, 1.0, -1.0),
                vec3(1.0, 1.0, -1.0),
                vec3(-1.0, -1.0, -1.0),
                vec3(-1.0, -1.0, 1.0),
                vec3(1.0, -1.0, 1.0),
                vec3(1.0, 1.0, 1.0),
                vec3(1.0, 1.0, 1.0),
                vec3(-1.0, 1.0, 1.0),
                vec3(-1.0, -1.0, 1.0),
                vec3(1.0, -1.0, -1.0),
                vec3(1.0, 1.0, -1.0),
                vec3(1.0, 1.0, 1.0),
                vec3(1.0, 1.0, 1.0),
                vec3(1.0, -1.0, 1.0),
                vec3(1.0, -1.0, -1.0),
                vec3(-1.0, 1.0, -1.0),
                vec3(-1.0, -1.0, -1.0),
                vec3(-1.0, 1.0, 1.0),
                vec3(-1.0, -1.0, 1.0),
                vec3(-1.0, 1.0, 1.0),
                vec3(-1.0, -1.0, -1.0),
            ],
        )?;

        Ok(Skybox {
            context: context.clone(),
            vertex_buffer,
            material: SkyboxMaterial { texture },
        })
    }

    ///
    /// Returns a reference to the cube map texture
    ///
    pub fn texture(&self) -> &TextureCubeMap {
        &self.material.texture
    }
}

impl Geometry for Skybox {
    fn aabb(&self) -> AxisAlignedBoundingBox {
        AxisAlignedBoundingBox::INFINITE
    }

    fn render_with_material(
        &self,
        material: &dyn Material,
        camera: &Camera,
        lights: &[&dyn Light],
    ) -> ThreeDResult<()> {
        let fragment_shader_source = material.fragment_shader_source(false, lights);
        self.context.program(
            &include_str!("shaders/skybox.vert"),
            &fragment_shader_source,
            |program| {
                material.use_uniforms(program, camera, lights)?;
                program.use_uniform_block("Camera", camera.uniform_buffer())?;
                program.use_vertex_attribute("position", &self.vertex_buffer)?;
                program.draw_arrays(material.render_states(), camera.viewport(), 36)?;
                Ok(())
            },
        )
    }
}

impl Object for Skybox {
    fn render(&self, camera: &Camera, lights: &[&dyn Light]) -> ThreeDResult<()> {
        self.render_with_material(&self.material, camera, lights)
    }

    fn is_transparent(&self) -> bool {
        false
    }
}
