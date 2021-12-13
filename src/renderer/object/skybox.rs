use crate::core::*;

///
/// An illusion of a sky.
///
pub struct Skybox<T: TextureCube> {
    program: Program,
    vertex_buffer: VertexBuffer,
    texture: T,
}

impl Skybox<TextureCubeMap> {
    ///
    /// Creates a new skybox with the given cpu-side version of a [TextureCubeMap].
    ///
    pub fn new<T: TextureDataType>(
        context: &Context,
        cpu_texture: &mut CPUTexture<T>,
    ) -> ThreeDResult<Skybox<TextureCubeMap>> {
        cpu_texture.wrap_t = Wrapping::ClampToEdge;
        cpu_texture.wrap_s = Wrapping::ClampToEdge;
        cpu_texture.wrap_r = Wrapping::ClampToEdge;
        cpu_texture.mip_map_filter = None;
        let texture = TextureCubeMap::new(&context, cpu_texture)?;
        Self::new_with_texture(context, texture)
    }

    ///
    /// Creates a new skybox with the given [TextureCubeMap].
    ///
    pub fn new_with_texture(
        context: &Context,
        texture: TextureCubeMap,
    ) -> ThreeDResult<Skybox<TextureCubeMap>> {
        let program = Program::from_source(
            context,
            include_str!("shaders/skybox.vert"),
            &format!(
                "{}{}",
                include_str!("../../core/shared.frag"),
                include_str!("shaders/skybox.frag")
            ),
        )?;

        let vertex_buffer = VertexBuffer::new_with_static(context, &CPUMesh::cube().positions)?;

        Ok(Skybox {
            program,
            vertex_buffer,
            texture,
        })
    }
}

impl<T: TextureCube> Skybox<T> {
    ///
    /// Returns a reference to the cube map texture
    ///
    pub fn texture(&self) -> &impl TextureCube {
        &self.texture
    }

    ///
    /// Render the skybox.
    ///
    pub fn render(&self, camera: &Camera) -> ThreeDResult<()> {
        let render_states = RenderStates {
            depth_test: DepthTest::LessOrEqual,
            cull: Cull::Front,
            ..Default::default()
        };

        self.program.use_texture_cube("texture0", &self.texture)?;
        self.program
            .use_uniform_block("Camera", camera.uniform_buffer());

        self.program
            .use_attribute_vec3("position", &self.vertex_buffer)?;

        self.program
            .draw_arrays(render_states, camera.viewport(), 36);
        Ok(())
    }
}
