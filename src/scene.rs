use input;
use std::rc::Rc;
use traits;

#[derive(Debug)]
pub enum Error {
    Traits(traits::Error)
}

impl From<traits::Error> for Error {
    fn from(other: traits::Error) -> Self {
        Error::Traits(other)
    }
}

pub struct Scene {
    models: Vec<Rc<traits::Reflecting>>,
    lights: Vec<Rc<traits::Emitting>>
}


impl Scene
{
    pub fn create() -> Scene
    {
        Scene { models: Vec::new(), lights: Vec::new() }
    }

    pub fn add_model(&mut self, model: Rc<traits::Reflecting>)
    {
        &self.models.push(model);
    }

    pub fn add_light(&mut self, light: Rc<traits::Emitting>)
    {
        &self.lights.push(light);
    }

    pub fn draw(&self, input: &input::DrawInput) -> Result<(), Error>
    {
        for model in &self.models {
            model.reflect(input)?;
        }
        Ok(())
    }

    pub fn shine_lights(&self, input: &input::EmittingInput) -> Result<(), Error>
    {
        for light in &self.lights {
            light.shine(input)?;
        }
        Ok(())
    }
}
