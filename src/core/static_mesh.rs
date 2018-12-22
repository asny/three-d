
use crate::surface::*;
#[derive(Debug)]
pub enum Error {
    FailedToFindCustomAttribute {message: String},
    FailedToFindEntryForVertexID {message: String},
    WrongSizeOfAttribute {message: String},
    NeedPositionAttributeToCreateMesh {message: String}
}

#[derive(Clone, Debug)]
pub struct StaticMesh
{
    indices: Vec<u32>,
    attributes: Vec<Attribute>
}

impl StaticMesh
{
    pub fn create(indices: Vec<u32>, attributes: Vec<Attribute>) -> Result<StaticMesh, Error>
    {
        if attributes.len() == 0 {
            return Err(Error::NeedPositionAttributeToCreateMesh {message: format!("Need at least the position attribute to create a mesh.")})
        }
        Ok(StaticMesh { indices, attributes })
    }

    pub fn indices(&self) -> &Vec<u32>
    {
        &self.indices
    }

    pub fn attribute(&self, name: &str) -> Option<&Attribute>
    {
        self.attributes.iter().find(|att| att.name == name).and_then(|att| Some(att))
    }

    pub fn no_vertices(&self) -> usize
    {
        let att = self.attributes.first().unwrap();
        att.data.len()/att.no_components
    }

    pub fn no_faces(&self) -> usize
    {
        self.indices.len()/3
    }
}