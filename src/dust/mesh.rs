use std::rc::Rc;
use std::collections::HashMap;
use std::string::String;

#[derive(Debug)]
pub enum Error {
    FailedToFindCustomAttribute {message: String}
}

pub struct Mesh {
    positions: Vec<f32>,
    attributes: HashMap<String, Vec<f32>>
}


impl Mesh
{
    pub fn create(positions: Vec<f32>) -> Result<Rc<Mesh>, Error>
    {
        Ok(Rc::new(Mesh { positions, attributes: HashMap::new() }))
    }

    pub fn positions(&self) -> &Vec<f32>
    {
        &self.positions
    }

    pub fn add_custom_attribute(&mut self, name: &str, attribute: Vec<f32>)
    {
        self.attributes.insert(String::from(name), attribute);
    }

    pub fn get_custom_attribute(&self, name: &str) -> Result<&Vec<f32>, Error>
    {
        self.attributes.get(name).ok_or(Error::FailedToFindCustomAttribute {message: format!("Could not find attribute {}", name)})
    }
}
