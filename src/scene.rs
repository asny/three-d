use model;
use input;
use std::rc::Rc;
use light;

#[derive(Debug)]
pub enum Error {
    Model(model::Error),
    Light(light::Error)
}

impl From<model::Error> for Error {
    fn from(other: model::Error) -> Self {
        Error::Model(other)
    }
}

impl From<light::Error> for Error {
    fn from(other: light::Error) -> Self {
        Error::Light(other)
    }
}

pub struct Scene {
    models: Vec<model::Model>,
    lights: Vec<Rc<light::Emitting>>
}


impl Scene
{
    pub fn create() -> Scene
    {
        Scene { models: Vec::new(), lights: Vec::new() }
    }

    pub fn add_model(&mut self, model: model::Model)
    {
        &self.models.push(model);
    }

    pub fn add_light(&mut self, light: Rc<light::Emitting>)
    {
        &self.lights.push(light);
    }

    pub fn draw(&self, input: &input::DrawInput) -> Result<(), Error>
    {
        for model in &self.models {
            model.draw(input)?;
        }
        Ok(())
    }

    pub fn shine_lights(&self, input: &input::DrawInput) -> Result<(), Error>
    {
        for light in &self.lights {
            light.shine(input)?;
        }
        Ok(())
    }
}
