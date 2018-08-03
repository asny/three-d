use attribute;
use glm;
use std::string::String;

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
    pub no_faces: usize,
    pub indices: Option<Vec<u32>>,
    pub positions: attribute::Attribute,
    attributes: Vec<attribute::Attribute>
}


impl Mesh
{
    pub fn create(positions: Vec<f32>) -> Result<Mesh, Error>
    {
        let no_vertices = positions.len()/3;
        let position_attribute = attribute::Attribute::create_vec3_attribute("position", positions)?;
        Ok(Mesh { no_vertices, no_faces: no_vertices/3, indices: None, positions: position_attribute, attributes: Vec::new() })
    }

    pub fn create_indexed(indices: Vec<u32>, positions: Vec<f32>) -> Result<Mesh, Error>
    {
        let no_vertices = positions.len()/3;
        let position_attribute = attribute::Attribute::create_vec3_attribute("position", positions)?;

        Ok(Mesh { no_vertices, no_faces: indices.len()/3, indices: Some(indices), positions: position_attribute, attributes: Vec::new() })
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

    pub fn get_attributes(&self) -> Vec<&attribute::Attribute>
    {
        let mut att = Vec::new();
        att.push(&self.positions);
        for attribute in self.attributes.iter() {
            att.push(attribute);
        }
        att
    }

    pub fn get_mut(&mut self, name: &str) -> Result<&mut attribute::Attribute, Error>
    {
        for attribute in self.attributes.iter_mut() {
            if attribute.name() == name
            {
                return Ok(attribute)
            }
        }
        Err(Error::FailedToFindCustomAttribute{message: format!("Failed to find {} attribute", name)})
    }

    pub fn add_custom_vec2_attribute(&mut self, name: &str, data: Vec<f32>) -> Result<(), Error>
    {
        if self.no_vertices != data.len()/2 {
            return Err(Error::WrongSizeOfAttribute {message: format!("The data for {} does not have the correct size, it should be {}", name, self.no_vertices)})
        }
        let custom_attribute = attribute::Attribute::create_vec2_attribute(name, data)?;
        self.attributes.push(custom_attribute);
        Ok(())
    }

    pub fn add_custom_vec3_attribute(&mut self, name: &str, data: Vec<f32>) -> Result<(), Error>
    {
        if self.no_vertices != data.len()/3 {
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

    fn position(&self, index: usize) -> glm::Vec3
    {
        glm::vec3(self.positions.data()[3 * index], self.positions.data()[3 * index+1], self.positions.data()[3 * index+2])
    }

    fn indices_of(&self, face_id: usize) -> [usize; 3]
    {
        let index0: usize;
        let index1: usize;
        let index2: usize;
        match self.indices {
            Some(ref indices) => {
                index0 = indices[face_id*3] as usize;
                index1 = indices[face_id*3+1] as usize;
                index2 = indices[face_id*3+2] as usize;
            },
            None => {
                index0 = face_id;
                index1 = face_id+1;
                index2 = face_id+2;
            }
        }
        [index0, index1, index2]
    }

    fn normal_of(&self, face_id: usize) -> glm::Vec3
    {
        let indices = self.indices_of(face_id);
        let p0 = self.position(indices[0]);
        let p1 = self.position(indices[1]);
        let p2 = self.position(indices[2]);

        glm::normalize(glm::cross(p1 - p0, p2 - p0))
    }

    pub fn compute_normals(&mut self)
    {
        //let normals = self.get_mut("normal").unwrap();
        let mut normals = vec![0.0; 3 * self.no_vertices];
        {
            for face_id in 0..self.no_faces {
                let normal = self.normal_of(face_id);
                let indices = self.indices_of(face_id);
                for index in indices.iter() {
                    normals[3 * *index] += normal.x;
                    normals[3 * *index+1] += normal.y;
                    normals[3 * *index+2] += normal.z;
                }
            }
        }
        {
            let normals_dest = self.get_mut("normal").unwrap().data_mut();
            for i in 0..normals.len()/3 {
                let n = glm::normalize(glm::vec3(normals[i*3], normals[i*3+1], normals[i*3+2]));
                normals_dest[i*3] = n[0];
                normals_dest[i*3+1] = n[1];
                normals_dest[i*3+2] = n[2];
            }
        }
    }
}
