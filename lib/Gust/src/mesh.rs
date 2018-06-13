use attribute;
use std::string::String;
use glm;

#[derive(Debug)]
pub enum Error {
    FailedToFindCustomAttribute {message: String},
    WrongSizeOfAttribute {message: String},
    Attribute(attribute::Error)
}

impl From<attribute::Error> for Error {
    fn from(other: attribute::Error) -> Self {
        Error::Attribute(other)
    }
}

pub struct Mesh {
    pub no_vertices: usize,
    pub indices: Option<Vec<u16>>,
    pub attributes: Vec<attribute::Attribute>
}


impl Mesh
{
    pub fn create_with_normals(positions: Vec<glm::Vec3>, normals: Vec<glm::Vec3>) -> Result<Mesh, Error>
    {
        let mut mesh = Mesh::create(positions)?;
        let normal_attribute = attribute::Attribute::create_vec3_attribute("normal", normals)?;
        mesh.attributes.push(normal_attribute);
        Ok(mesh)
    }

    pub fn create(positions: Vec<glm::Vec3>) -> Result<Mesh, Error>
    {
        let no_vertices = positions.len();
        let position_attribute = attribute::Attribute::create_vec3_attribute("position", positions)?;
        let mut mesh = Mesh { no_vertices, indices: None, attributes: Vec::new() };
        mesh.attributes.push(position_attribute);
        Ok(mesh)
    }

    pub fn create_unsafe_with_normals(indices: &Vec<u32>, positions: &Vec<f32>, normals: &Vec<f32>) -> Result<Mesh, Error>
    {
        let mut mesh = Mesh::create_unsafe(indices, positions)?;
        let no_vertices = normals.len()/3;
        let mut normals_vec3 = Vec::with_capacity(no_vertices);
        for vid in 0..no_vertices {
            normals_vec3.push(glm::vec3(normals[vid * 3], normals[vid * 3 + 1], normals[vid * 3 + 2]));
        }
        let normal_attribute = attribute::Attribute::create_vec3_attribute("normal", normals_vec3)?;
        mesh.attributes.push(normal_attribute);
        Ok(mesh)
    }

    pub fn create_unsafe(indices: &Vec<u32>, positions: &Vec<f32>) -> Result<Mesh, Error>
    {
        let no_vertices = positions.len()/3;
        let mut positions_vec3 = Vec::with_capacity(no_vertices);
        for vid in 0..no_vertices {
            positions_vec3.push(glm::vec3(positions[vid * 3], positions[vid * 3 + 1], positions[vid * 3 + 2]));
        }
        Mesh::create_indexed(&indices, positions_vec3)
    }

    pub fn create_indexed(indices: &Vec<u32>, positions: Vec<glm::Vec3>) -> Result<Mesh, Error>
    {
        let no_vertices = positions.len();
        let mut indices_u16 = Vec::with_capacity(indices.len());
        for i in 0..indices.len() {
            indices_u16.push(indices[i] as u16);
        }

        let position_attribute = attribute::Attribute::create_vec3_attribute("position", positions)?;
        let mut mesh = Mesh { no_vertices, indices: Some(indices_u16), attributes: Vec::new() };
        mesh.attributes.push(position_attribute);
        Ok(mesh)
    }

    pub fn positions(&self) -> Result<&attribute::Attribute, Error>
    {
        self.get("position")
    }

    pub fn get(&self, name: &str) -> Result<&attribute::Attribute, Error>
    {
        for attribute in self.attributes.iter() {
            if attribute.name() == name
            {
                return Ok(attribute)
            }
        }
        Err(Error::FailedToFindCustomAttribute{message: format!("Failed to find {} attribute", name)})
    }

    pub fn add_custom_vec2_attribute(&mut self, name: &str, data: Vec<glm::Vec2>) -> Result<(), Error>
    {
        if self.no_vertices != data.len() {
            return Err(Error::WrongSizeOfAttribute {message: format!("The data for {} does not have the correct size, it should be {}", name, self.no_vertices)})
        }
        let custom_attribute = attribute::Attribute::create_vec2_attribute(name, data)?;
        self.attributes.push(custom_attribute);
        Ok(())
    }

    pub fn add_custom_vec3_attribute(&mut self, name: &str, data: Vec<glm::Vec3>) -> Result<(), Error>
    {
        if self.no_vertices != data.len() {
            return Err(Error::WrongSizeOfAttribute {message: format!("The data for {} does not have the correct size, it should be {}", name, self.no_vertices)})
        }
        let custom_attribute = attribute::Attribute::create_vec3_attribute(name, data)?;
        self.attributes.push(custom_attribute);
        Ok(())
    }

    pub fn add_custom_int_attribute(&mut self, name: &str, data: &Vec<u32>) -> Result<(), Error>
    {
        if self.no_vertices != data.len() {
            return Err(Error::WrongSizeOfAttribute {message: format!("The data for {} does not have the correct size, it should be {}", name, self.no_vertices)})
        }
        let custom_attribute = attribute::Attribute::create_int_attribute(name, data)?;
        self.attributes.push(custom_attribute);
        Ok(())
    }
}
