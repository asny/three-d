use gl;
use crate::*;
use crate::buffer::*;

#[derive(Debug)]
pub enum Error {
    Program(program::Error),
    Model(surface::Error)
}

impl From<program::Error> for Error {
    fn from(other: program::Error) -> Self {
        Error::Program(other)
    }
}

impl From<surface::Error> for Error {
    fn from(other: surface::Error) -> Self {
        Error::Model(other)
    }
}

pub struct Skybox {
    program: program::Program,
    vertex_buffer: VertexBuffer,
    texture: texture::Texture3D
}

impl Skybox
{
    pub fn new(gl: &gl::Gl, texture: texture::Texture3D) -> Skybox
    {
        let program = program::Program::from_source(gl,
                                                    include_str!("shaders/skybox.vert"),
                                                    include_str!("shaders/skybox.frag")).unwrap();

        let positions = get_positions();
        let vertex_buffer = VertexBuffer::new_from_attributes(gl, &att!["position" => (positions, 3)]).unwrap();

        Skybox { program, vertex_buffer, texture }
    }

    pub fn render(&self, camera: &camera::Camera) -> Result<(), Error>
    {
        self.program.cull(state::CullType::FRONT);
        self.program.depth_write(true);
        self.program.depth_test(state::DepthTestType::LEQUAL);

        self.texture.bind(0);
        self.program.add_uniform_int("texture0", &0)?;
        self.program.add_uniform_mat4("viewMatrix", camera.get_view())?;
        self.program.add_uniform_mat4("projectionMatrix", camera.get_projection())?;
        self.program.add_uniform_vec3("cameraPosition", camera.position())?;

        self.program.use_attribute_vec3_float(&self.vertex_buffer, "position", 0)?;
        self.program.draw_arrays(self.vertex_buffer.count() as u32);
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