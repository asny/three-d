use gl;
use glm;
use program;
use state;
use input;
use mesh;
use std;
use buffer;
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


pub trait Shining {
    fn shine(&self, gl: &gl::Gl, input: &input::DrawInput);
}

pub struct DirectionalLight {
    program: program::Program,
    direction: glm::Vec3
}

impl DirectionalLight
{
    pub fn create(gl: &gl::Gl, direction: glm::Vec3) -> Result<DirectionalLight, Error>
    {
        let program = program::Program::from_resource(&gl, "examples/assets/shaders/light_pass")?;
        Ok(DirectionalLight {program, direction})
    }
}

impl Shining for DirectionalLight
{
    fn shine(&self, gl: &gl::Gl, input: &input::DrawInput)
    {
        state::depth_write(gl,false);
        state::depth_test(gl, false);
        state::cull_back_faces(gl,true);

        input.color_texture.bind(0);
        self.program.add_uniform_int("colorMap", &0);

        model::Model::draw_full_screen_quad(gl, &self.program);
    }
}