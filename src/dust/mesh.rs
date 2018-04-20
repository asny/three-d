use dust::attribute;
use std::string::String;
use glm;

#[derive(Debug)]
pub enum Error {
    FailedToFindCustomAttribute {message: String},
    Attribute(attribute::Error)
}

impl From<attribute::Error> for Error {
    fn from(other: attribute::Error) -> Self {
        Error::Attribute(other)
    }
}

pub struct Mesh {
    positions: attribute::Attribute,
    custom_attributes: Vec<attribute::Attribute>
}


impl Mesh
{
    pub fn create(positions: Vec<glm::Vec3>) -> Result<Mesh, Error>
    {
        let position_attribute = attribute::Attribute::create_vec3_attribute("Position", positions)?;
        let mesh = Mesh { positions: position_attribute, custom_attributes: Vec::new() };
        Ok(mesh)
    }

    pub fn positions(&self) -> &attribute::Attribute
    {
        &self.positions
    }

    pub fn get(&self, name: &str) -> Result<&attribute::Attribute, Error>
    {
        for attribute in self.custom_attributes.iter() {
            if attribute.name() == name
            {
                return Ok(attribute)
            }
        }
        Err(Error::FailedToFindCustomAttribute{message: format!("Failed to find {} attribute", name)})
    }

    pub fn add_custom_attribute(&mut self, attribute: attribute::Attribute)
    {
        self.custom_attributes.push(attribute);
    }
}
