use std::path::Path;
use std::path::PathBuf;
use tobj;
use mesh;
use glm;

#[derive(Debug)]
pub enum Error {
    ObjLoader(tobj::LoadError),
    Mesh(mesh::Error),
    FileDoesntContainModel{message: String}
}

impl From<tobj::LoadError> for Error {
    fn from(other: tobj::LoadError) -> Self {
        Error::ObjLoader(other)
    }
}

impl From<mesh::Error> for Error {
    fn from(other: mesh::Error) -> Self {
        Error::Mesh(other)
    }
}

pub fn load_obj(name: &str) -> Result<mesh::Mesh, Error>
{
    let root_path: PathBuf = PathBuf::from("");
    let (models, _materials) = tobj::load_obj(&resource_name_to_path(&root_path,name))?;
    let m = &models.first().ok_or(Error::FileDoesntContainModel {message: format!("The file {} doesn't contain a model", name)})?.mesh;

    let no_vertices = m.positions.len()/3;

    // Create mesh
    let mut positions_vec3 = Vec::with_capacity(no_vertices);
    for vid in 0..no_vertices {
        positions_vec3.push(glm::vec3(m.positions[vid * 3], m.positions[vid * 3 + 1], m.positions[vid * 3 + 2]));
    }

    let mut mesh = match m.indices.len() > 0 {
        true => mesh::Mesh::create_indexed(m.indices.clone(), positions_vec3)?,
        false => mesh::Mesh::create(positions_vec3)?
    };

    if m.normals.len() > 0
    {
        let mut normals_vec3 = Vec::with_capacity(no_vertices);
        for vid in 0..no_vertices {
            normals_vec3.push(glm::vec3(m.normals[vid * 3], m.normals[vid * 3 + 1], m.normals[vid * 3 + 2]));
        }
        mesh.add_custom_vec3_attribute("normal", normals_vec3)?;
    }

    Ok(mesh)
}

fn resource_name_to_path(root_dir: &Path, location: &str) -> PathBuf {
    let mut path: PathBuf = root_dir.into();

    for part in location.split("/") {
        path = path.join(part);
    }

    path
}