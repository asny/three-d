use gl;
use glm;
use program;
use state;
use input;
use texture::Texture;
use model;

#[derive(Debug)]
pub enum Error {
    Program(program::Error)
}

impl From<program::Error> for Error {
    fn from(other: program::Error) -> Self {
        Error::Program(other)
    }
}


pub trait Emitting {
    fn shine(&self, input: &input::DrawInput) -> Result<(), Error>;
}

pub struct DirectionalLight {
    gl: gl::Gl,
    program: program::Program,
    direction: glm::Vec3
}

impl DirectionalLight
{
    pub fn create(gl: &gl::Gl, direction: glm::Vec3) -> Result<DirectionalLight, Error>
    {
        let program = program::Program::from_resource(&gl, "examples/assets/shaders/light_pass")?;
        Ok(DirectionalLight {gl: gl.clone(), program, direction})
    }
}

impl Emitting for DirectionalLight
{
    fn shine(&self, input: &input::DrawInput) -> Result<(), Error>
    {
        state::depth_write(&self.gl,false);
        state::depth_test(&self.gl, false);
        state::cull_back_faces(&self.gl,true);

        input.color_texture.bind(0);
        self.program.add_uniform_int("colorMap", &0)?;

        model::Model::draw_full_screen_quad(&self.gl, &self.program);
        Ok(())
    }
}