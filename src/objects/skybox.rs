use crate::core::*;

pub struct Skybox {
    program: program::Program,
    vertex_buffer: VertexBuffer,
    texture: texture::TextureCubeMap
}

impl Skybox
{
    pub fn new(context: &Context, right: &mut CPUTexture, left: &CPUTexture, top: &CPUTexture, front: &CPUTexture, back: &CPUTexture) -> Result<Skybox, Error>
    {
        right.wrap_t = Wrapping::ClampToEdge;
        right.wrap_s = Wrapping::ClampToEdge;
        right.mip_map_filter = None;
        let texture = TextureCubeMap::new_with_u8(&context, right, left, top, top, front, back)?;
        Self::new_with_texture(context, texture)
    }

    pub fn new_with_texture(context: &Context, texture: texture::TextureCubeMap) -> Result<Skybox, Error>
    {
        let program = program::Program::from_source(context,
                                                    include_str!("shaders/skybox.vert"),
                                                    include_str!("shaders/skybox.frag"))?;

        let vertex_buffer = VertexBuffer::new_with_static_f32(context, &get_positions())?;

        Ok(Skybox { program, vertex_buffer, texture })
    }

    pub fn render(&self, viewport: Viewport, camera: &camera::Camera) -> Result<(), Error>
    {
        let render_states = RenderStates {cull: CullType::Front, depth_test: DepthTestType::LessOrEqual, ..Default::default()};

        self.program.use_texture(&self.texture, "texture0")?;
        self.program.use_uniform_block(camera.matrix_buffer(), "Camera");

        self.program.use_attribute_vec3_float(&self.vertex_buffer, "position")?;

        self.program.draw_arrays(render_states, viewport, 36);
        Ok(())
    }

    pub fn get_texture(&self) -> &texture::TextureCubeMap
    {
        &self.texture
    }
}

fn get_positions() -> Vec<f32>
{
    vec![
        1.0, 1.0, -1.0,
        -1.0, 1.0, -1.0,
        1.0, 1.0, 1.0,
        -1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
        -1.0, 1.0, -1.0,

        -1.0, -1.0, -1.0,
        1.0, -1.0, -1.0,
        1.0, -1.0, 1.0,
        1.0, -1.0, 1.0,
        -1.0, -1.0, 1.0,
        -1.0, -1.0, -1.0,

        1.0, -1.0, -1.0,
        -1.0, -1.0, -1.0,
        1.0, 1.0, -1.0,
        -1.0, 1.0, -1.0,
        1.0, 1.0, -1.0,
        -1.0, -1.0, -1.0,

        -1.0, -1.0, 1.0,
        1.0, -1.0, 1.0,
        1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
        -1.0, 1.0, 1.0,
        -1.0, -1.0, 1.0,

        1.0, -1.0, -1.0,
        1.0, 1.0, -1.0,
        1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
        1.0, -1.0, 1.0,
        1.0, -1.0, -1.0,

        -1.0, 1.0, -1.0,
        -1.0, -1.0, -1.0,
        -1.0, 1.0, 1.0,
        -1.0, -1.0, 1.0,
        -1.0, 1.0, 1.0,
        -1.0, -1.0, -1.0
    ]
}