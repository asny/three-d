use std::collections::HashMap;
use std::string::String;

#[derive(Debug)]
pub enum Error {
    FailedToFindCustomAttribute {message: String}
}

pub struct Mesh {
    attributes: HashMap<String, Vec<f32>>
}


impl Mesh
{
    pub fn create(positions: Vec<f32>) -> Result<Mesh, Error>
    {
        let mut mesh = Mesh { attributes: HashMap::new() };
        mesh.add_custom_attribute("Position", positions);
        Ok(mesh)
    }

    pub fn attributes(&self) -> &HashMap<String, Vec<f32>>
    {
        &self.attributes
    }

    pub fn add_custom_attribute(&mut self, name: &str, attribute: Vec<f32>)
    {
        self.attributes.insert(String::from(name), attribute);
    }
}
