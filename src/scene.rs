use std::rc::Rc;
use traits;

pub struct Scene {
    pub models: Vec<Rc<traits::Reflecting>>,
    pub lights: Vec<Rc<traits::Emitting>>
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
}
