use dust::model;

#[derive(Debug)]
pub enum Error {
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

    pub fn draw(&self)
    {
        for model in &self.models {
            model.draw();
        }
    }
}
