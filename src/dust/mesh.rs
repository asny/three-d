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
    positions: attribute::Attribute
}


impl Mesh
{
    pub fn create(positions: Vec<glm::Vec3>) -> Result<Mesh, Error>
    {
        let position_attribute = attribute::Attribute::create_vec3_attribute("Position", positions)?;
        let mesh = Mesh { positions: position_attribute };
        Ok(mesh)
    }

    pub fn positions(&self) -> &attribute::Attribute
    {
        &self.positions
    }
}
