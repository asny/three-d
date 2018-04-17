use dust::model;
use dust::input;

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

    pub fn add_model(&mut self, model: model::Model)
    {
        &self.models.push(model);
    }

    pub fn draw(&self, input: &input::DrawInput) -> Result<(), Error>
    {
        for model in &self.models {
            model.draw(input)?;
        }
        Ok(())
    }
}
