use input;
use std::rc::Rc;
use light;
use material;

#[derive(Debug)]
pub enum Error {
    Model(material::Error),
    Light(light::Error)
}

impl From<material::Error> for Error {
    fn from(other: material::Error) -> Self {
        Error::Model(other)
    }
}

impl From<light::Error> for Error {
    fn from(other: light::Error) -> Self {
        Error::Light(other)
    }
}

pub struct Scene {
    models: Vec<Rc<material::Reflecting>>,
    lights: Vec<Rc<light::Emitting>>
}


impl Scene
{
    pub fn create() -> Scene
    {
        Scene { models: Vec::new(), lights: Vec::new() }
    }

    pub fn add_model(&mut self, model: Rc<material::Reflecting>)
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
            model.apply();
            model.setup_states()?;
            model.setup_uniforms(&input)?;
            model.reflect(input);
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
