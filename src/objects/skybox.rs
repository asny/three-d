use gl;
use crate::*;
use crate::surface::*;

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
    model: TriangleSurface,
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
        let indices: Vec<u32> = (0..positions.len() as u32/3).collect();
        let mut model = TriangleSurface::new(gl, &indices).unwrap();
        model.add_attributes(&program, &att!["position" => (positions, 3)]).unwrap();

        Skybox { program, model, texture }
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

        self.model.render()?;
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