use std::rc::Rc;
use traits;
use light;

pub struct Scene {
    pub models: Vec<Rc<traits::Reflecting>>,
    pub directional_lights: Vec<light::DirectionalLight>
}

impl Scene
{
    pub fn create() -> Scene
    {
        Scene { models: Vec::new(), directional_lights: Vec::new() }
    }

    pub fn add_model(&mut self, model: Rc<traits::Reflecting>)
    {
        &self.models.push(model);
    }

    pub fn add_light(&mut self, light: light::DirectionalLight)
    {
        &self.directional_lights.push(light);
    }
}
