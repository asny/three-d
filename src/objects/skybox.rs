
use crate::*;
use crate::core::Error;

pub struct Skybox {
    program: program::Program,
    vertex_buffer: VertexBuffer,
    texture: texture::Texture3D
}

impl Skybox
{
    pub fn new(gl: &Gl, texture: texture::Texture3D) -> Skybox
    {
        let program = program::Program::from_source(gl,
                                                    include_str!("shaders/skybox.vert"),
                                                    include_str!("shaders/skybox.frag")).unwrap();

        let vertex_buffer = VertexBuffer::new_with_one_static_attribute(gl, &get_positions()).unwrap();

        Skybox { program, vertex_buffer, texture }
    }

    pub fn render(&self, camera: &camera::Camera) -> Result<(), Error>
    {
        self.program.use_texture(&self.texture, "texture0")?;
        self.program.use_uniform_block(camera.matrix_buffer(), "Camera");

        self.program.use_attribute_vec3_float(&self.vertex_buffer, "position", 0)?;

        self.program.draw_arrays(36);
        Ok(())
    }

    pub fn get_texture(&self) -> &texture::Texture3D
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