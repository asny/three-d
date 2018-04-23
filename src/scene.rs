use model;
use input;
use gust::mesh;
use material;
use std::rc::Rc;
use gl;

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
    models: Vec<model::Model>
}


impl Scene
{
    pub fn create() -> Result<Scene, Error>
    {
        Ok(Scene { models: Vec::new() })
    }

    pub fn add_model(&mut self, gl: &gl::Gl, mesh: mesh::Mesh, material: Rc<material::Material>) -> Result<(), Error>
    {
        let model = model::Model::create(&gl, mesh, material)?;
        &self.models.push(model);
        Ok(())
    }

    pub fn draw(&self, input: &input::DrawInput) -> Result<(), Error>
    {
        for model in &self.models {
            model.draw(input)?;
        }
        Ok(())
    }
}
