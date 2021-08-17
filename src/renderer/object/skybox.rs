use crate::core::*;

///
/// An illusion of a sky.
///
pub struct Skybox {
    program: Program,
    vertex_buffer: VertexBuffer,
    texture: TextureCubeMap,
}

impl Skybox {
    pub fn new<T: TextureDataType>(
        context: &Context,
        cpu_texture: &mut CPUTexture<T>,
    ) -> Result<Skybox> {
        cpu_texture.wrap_t = Wrapping::ClampToEdge;
        cpu_texture.wrap_s = Wrapping::ClampToEdge;
        cpu_texture.wrap_r = Wrapping::ClampToEdge;
        cpu_texture.mip_map_filter = None;
        let texture = TextureCubeMap::new(&context, cpu_texture)?;
        Self::new_with_texture(context, texture)
    }

    pub fn new_with_texture(context: &Context, texture: TextureCubeMap) -> Result<Skybox> {
        let program = Program::from_source(
            context,
            include_str!("shaders/skybox.vert"),
            &format!(
                "{}{}",
                include_str!("../../core/shared.frag"),
                include_str!("shaders/skybox.frag")
            ),
        )?;

        let vertex_buffer = VertexBuffer::new_with_static(context, &get_positions())?;

        Ok(Skybox {
            program,
            vertex_buffer,
            texture,
        })
    }

    ///
    /// Render the skybox.
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    ///
    pub fn render(&self, camera: &Camera) -> Result<()> {
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

    pub fn texture(&self) -> &TextureCubeMap {
        &self.texture
    }
}

fn get_positions() -> Vec<f32> {
    vec![
        1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0,
        -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, -1.0, -1.0, 1.0,
        -1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0,
        1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, -1.0, 1.0, 1.0, -1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 1.0, 1.0,
        -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, -1.0, -1.0,
    ]
}
