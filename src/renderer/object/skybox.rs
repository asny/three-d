use crate::core::*;
use crate::renderer::*;

///
/// An illusion of a sky.
///
pub struct Skybox<T: TextureCube> {
    context: Context,
    vertex_buffer: VertexBuffer,
    material: SkyboxMaterial<T>,
}

impl<T: TextureDataType> Skybox<TextureCubeMap<T>> {
    ///
    /// Creates a new skybox with the given cpu-side version of a [TextureCubeMap].
    ///
    pub fn new(
        context: &Context,
        cpu_texture: &CpuTextureCube<T>,
    ) -> ThreeDResult<Skybox<TextureCubeMap<T>>> {
        let texture = TextureCubeMap::new(&context, cpu_texture)?;
        Self::new_with_texture(context, texture)
    }

    ///
    /// Creates a new skybox with a cube texture generated from the equirectangular texture given as input.
    ///
    pub fn new_from_equirectangular(
        context: &Context,
        cpu_texture: &CpuTexture<T>,
    ) -> ThreeDResult<Skybox<TextureCubeMap<T>>> {
        let texture = TextureCubeMap::new_from_equirectangular(context, cpu_texture)?;
        Self::new_with_texture(context, texture)
    }
}

impl<T: TextureCube> Skybox<T> {
    ///
    /// Creates a new skybox with the given [TextureCubeMap].
    ///
    pub fn new_with_texture(context: &Context, texture: T) -> ThreeDResult<Skybox<T>> {
        let vertex_buffer = VertexBuffer::new_with_static(context, &CpuMesh::cube().positions)?;

        Ok(Skybox {
            context: context.clone(),
            vertex_buffer,
            material: SkyboxMaterial { texture },
        })
    }

    ///
    /// Returns a reference to the cube map texture
    ///
    pub fn texture(&self) -> &impl TextureCube {
        &self.material.texture
    }
}

impl<T: TextureCube> Geometry for Skybox<T> {
    fn aabb(&self) -> AxisAlignedBoundingBox {
        AxisAlignedBoundingBox::INFINITE
    }

    fn transformation(&self) -> Mat4 {
        Mat4::identity()
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
                program.use_uniform_block("Camera", camera.uniform_buffer());
                program.use_attribute_vec3("position", &self.vertex_buffer)?;
                program.draw_arrays(material.render_states(), camera.viewport(), 36);
                Ok(())
            },
        )
    }
}

impl<T: TextureCube> Object for Skybox<T> {
    fn render(&self, camera: &Camera, lights: &[&dyn Light]) -> ThreeDResult<()> {
        self.render_with_material(&self.material, camera, lights)
    }

    fn is_transparent(&self) -> bool {
        false
    }
}

struct SkyboxMaterial<T: TextureCube> {
    pub texture: T,
}

impl<T: TextureCube> Material for SkyboxMaterial<T> {
    fn fragment_shader_source(&self, _use_vertex_colors: bool, _lights: &[&dyn Light]) -> String {
        format!(
            "{}{}",
            include_str!("../../core/shared.frag"),
            include_str!("shaders/skybox.frag")
        )
    }

    fn use_uniforms(
        &self,
        program: &Program,
        camera: &Camera,
        _lights: &[&dyn Light],
    ) -> ThreeDResult<()> {
        program.use_uniform_int("isHDR", if self.texture.is_hdr() { &1 } else { &0 })?;
        program.use_texture_cube("texture0", &self.texture)?;
        program.use_uniform_block("Camera", camera.uniform_buffer());
        Ok(())
    }

    fn render_states(&self) -> RenderStates {
        RenderStates {
            depth_test: DepthTest::LessOrEqual,
            cull: Cull::Front,
            ..Default::default()
        }
    }

    fn is_transparent(&self) -> bool {
        false
    }
}
