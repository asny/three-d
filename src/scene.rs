use model;
use input;
use gust::mesh;
use material;
use std::rc::Rc;
use gl;
use light;

#[derive(Debug)]
pub enum Error {
    Model(model::Error)
}

impl From<model::Error> for Error {
    fn from(other: model::Error) -> Self {
        Error::Model(other)
    }
}

pub struct Scene {
    models: Vec<model::Model>,
    lights: Vec<Rc<light::Emitting>>
}


impl Scene
{
    pub fn create() -> Result<Scene, Error>
    {
        Ok(Scene { models: Vec::new(), lights: Vec::new() })
    }

    pub fn add_model(&mut self, gl: &gl::Gl, mesh: mesh::Mesh, material: Rc<material::Reflecting>) -> Result<(), Error>
    {
        let model = model::Model::create(&gl, mesh, material)?;
        &self.models.push(model);
        Ok(())
    }

    pub fn add_light(&mut self, gl: &gl::Gl, light: Rc<light::Emitting>) -> Result<(), Error>
    {
        &self.lights.push(light);
        Ok(())
    }

    pub fn draw(&self, input: &input::DrawInput) -> Result<(), Error>
    {
        for model in &self.models {
            model.draw(input)?;
        }
        Ok(())
    }

    pub fn shine_lights(&self, gl: &gl::Gl, input: &input::DrawInput) -> Result<(), Error>
    {
        for light in &self.lights {
            light.shine(gl, input);
        }
        Ok(())
    }
}
