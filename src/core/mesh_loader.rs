use std::path::PathBuf;
use tobj;
use crate::static_mesh;

#[derive(Debug)]
pub enum Error {
    ObjLoader(tobj::LoadError),
    Mesh(static_mesh::Error),
    FileDoesntContainModel{message: String}
}

impl From<tobj::LoadError> for Error {
    fn from(other: tobj::LoadError) -> Self {
        Error::ObjLoader(other)
    }
}

impl From<static_mesh::Error> for Error {
    fn from(other: static_mesh::Error) -> Self {
        Error::Mesh(other)
    }
}

pub fn load_obj(name: &str) -> Result<Vec<static_mesh::StaticMesh>, Error>
{
    let mut result = Vec::new();

    let (models, _materials) = tobj::load_obj(&PathBuf::from(name))?;
    if models.is_empty()
    {
        return Err(Error::FileDoesntContainModel {message: format!("The file {} doesn't contain a model", name)})
    }

    for m in models {
        let indices = match m.mesh.indices.len() > 0 { true => m.mesh.indices.clone(), false => (0..m.mesh.positions.len() as u32/3).collect() };
        let attributes;
        if m.mesh.normals.len() > 0
        {
            attributes = att!["position" => (m.mesh.positions.clone(), 3), "normal" => (m.mesh.normals.clone(), 3)];
        }
        else {
            attributes = att!["position" => (m.mesh.positions.clone(), 3)];
        }
        let mesh = static_mesh::StaticMesh::create(indices, attributes)?;
        result.push(mesh);
    }
    Ok(result)
}