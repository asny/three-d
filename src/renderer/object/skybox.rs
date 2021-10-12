use crate::core::*;
use crate::renderer::*;

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
            include_str!("shaders/skybox.frag"),
        )?;

        let vertex_buffer = VertexBuffer::new_with_static(context, &CPUMesh::cube().positions)?;

        Ok(Skybox {
            program,
            vertex_buffer,
            texture,
        })
    }

    pub fn texture(&self) -> &TextureCubeMap {
        &self.texture
    }
}

impl Cullable for Skybox {
    fn in_frustum(&self, _camera: &Camera) -> bool {
        true
    }
}

impl Cullable for &Skybox {
    fn in_frustum(&self, camera: &Camera) -> bool {
        (*self).in_frustum(camera)
    }
}

impl Drawable for Skybox {
    fn render(&self, camera: &Camera, _lights: &Lights) -> Result<()> {
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

impl Drawable for &Skybox {
    fn render(&self, camera: &Camera, lights: &Lights) -> Result<()> {
        (*self).render(camera, lights)
    }
}

impl Object for Skybox {}
impl Object for &Skybox {}
