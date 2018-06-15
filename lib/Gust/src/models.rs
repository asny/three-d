use mesh;
use glm;

#[derive(Debug)]
pub enum Error {
    Mesh(mesh::Error)
}

impl From<mesh::Error> for Error {
    fn from(other: mesh::Error) -> Self {
        Error::Mesh(other)
    }
}

pub fn create_cube() -> Result<mesh::Mesh, Error>
{
    let positions: Vec<glm::Vec3> = vec![
        glm::vec3(1.0, 1.0, -1.0),
        glm::vec3(-1.0, 1.0, -1.0),
        glm::vec3(1.0, 1.0, 1.0),
        glm::vec3(-1.0, 1.0, 1.0),
        glm::vec3(1.0, 1.0, 1.0),
        glm::vec3(-1.0, 1.0, -1.0),

        glm::vec3(-1.0, -1.0, -1.0),
        glm::vec3(1.0, -1.0, -1.0),
        glm::vec3(1.0, -1.0, 1.0),
        glm::vec3(1.0, -1.0, 1.0),
        glm::vec3(-1.0, -1.0, 1.0),
        glm::vec3(-1.0, -1.0, -1.0),

        glm::vec3(1.0, -1.0, -1.0),
        glm::vec3(-1.0, -1.0, -1.0),
        glm::vec3(1.0, 1.0, -1.0),
        glm::vec3(-1.0, 1.0, -1.0),
        glm::vec3(1.0, 1.0, -1.0),
        glm::vec3(-1.0, -1.0, -1.0),

        glm::vec3(-1.0, -1.0, 1.0),
        glm::vec3(1.0, -1.0, 1.0),
        glm::vec3(1.0, 1.0, 1.0),
        glm::vec3(1.0, 1.0, 1.0),
        glm::vec3(-1.0, 1.0, 1.0),
        glm::vec3(-1.0, -1.0, 1.0),

        glm::vec3(1.0, -1.0, -1.0),
        glm::vec3(1.0, 1.0, -1.0),
        glm::vec3(1.0, 1.0, 1.0),
        glm::vec3(1.0, 1.0, 1.0),
        glm::vec3(1.0, -1.0, 1.0),
        glm::vec3(1.0, -1.0, -1.0),

        glm::vec3(-1.0, 1.0, -1.0),
        glm::vec3(-1.0, -1.0, -1.0),
        glm::vec3(-1.0, 1.0, 1.0),
        glm::vec3(-1.0, -1.0, 1.0),
        glm::vec3(-1.0, 1.0, 1.0),
        glm::vec3(-1.0, -1.0, -1.0)
    ];
    let normals: Vec<glm::Vec3> = vec![
        glm::vec3(0.0, 1.0, 0.0),
        glm::vec3(0.0, 1.0, 0.0),
        glm::vec3(0.0, 1.0, 0.0),
        glm::vec3(0.0, 1.0, 0.0),
        glm::vec3(0.0, 1.0, 0.0),
        glm::vec3(0.0, 1.0, 0.0),

        glm::vec3(0.0, -1.0, 0.0),
        glm::vec3(0.0, -1.0, 0.0),
        glm::vec3(0.0, -1.0, 0.0),
        glm::vec3(0.0, -1.0, 0.0),
        glm::vec3(0.0, -1.0, 0.0),
        glm::vec3(0.0, -1.0, 0.0),

        glm::vec3(0.0, 0.0, -1.0),
        glm::vec3(0.0, 0.0, -1.0),
        glm::vec3(0.0, 0.0, -1.0),
        glm::vec3(0.0, 0.0, -1.0),
        glm::vec3(0.0, 0.0, -1.0),
        glm::vec3(0.0, 0.0, -1.0),

        glm::vec3(0.0, 0.0, 1.0),
        glm::vec3(0.0, 0.0, 1.0),
        glm::vec3(0.0, 0.0, 1.0),
        glm::vec3(0.0, 0.0, 1.0),
        glm::vec3(0.0, 0.0, 1.0),
        glm::vec3(0.0, 0.0, 1.0),

        glm::vec3(1.0, 0.0, 0.0),
        glm::vec3(1.0, 0.0, 0.0),
        glm::vec3(1.0, 0.0, 0.0),
        glm::vec3(1.0, 0.0, 0.0),
        glm::vec3(1.0, 0.0, 0.0),
        glm::vec3(1.0, 0.0, 0.0),

        glm::vec3(-1.0, 0.0, 0.0),
        glm::vec3(-1.0, 0.0, 0.0),
        glm::vec3(-1.0, 0.0, 0.0),
        glm::vec3(-1.0, 0.0, 0.0),
        glm::vec3(-1.0, 0.0, 0.0),
        glm::vec3(-1.0, 0.0, 0.0)
    ];

    let uvs: Vec<glm::Vec2> = vec![
        glm::vec2(1.0, 0.0),
        glm::vec2(0.0, 0.0),
        glm::vec2(1.0, 1.0),
        glm::vec2(0.0, 1.0),
        glm::vec2(1.0, 1.0),
        glm::vec2(0.0, 0.0),

        glm::vec2(0.0, 0.0),
        glm::vec2(0.0, 0.0),
        glm::vec2(0.0, 0.0),
        glm::vec2(0.0, 0.0),
        glm::vec2(0.0, 0.0),
        glm::vec2(0.0, 0.0),

        glm::vec2(0.0, 0.0),
        glm::vec2(0.0, 0.0),
        glm::vec2(0.0, 0.0),
        glm::vec2(0.0, 0.0),
        glm::vec2(0.0, 0.0),
        glm::vec2(0.0, 0.0),

        glm::vec2(0.0, 0.0),
        glm::vec2(0.0, 0.0),
        glm::vec2(0.0, 0.0),
        glm::vec2(0.0, 0.0),
        glm::vec2(0.0, 0.0),
        glm::vec2(0.0, 0.0),

        glm::vec2(0.0, 0.0),
        glm::vec2(0.0, 0.0),
        glm::vec2(0.0, 0.0),
        glm::vec2(0.0, 0.0),
        glm::vec2(0.0, 0.0),
        glm::vec2(0.0, 0.0),

        glm::vec2(0.0, 0.0),
        glm::vec2(0.0, 0.0),
        glm::vec2(0.0, 0.0),
        glm::vec2(0.0, 0.0),
        glm::vec2(0.0, 0.0),
        glm::vec2(0.0, 0.0)
    ];

    let mut mesh = mesh::Mesh::create(positions)?;
    mesh.add_custom_vec3_attribute("normal", normals)?;
    mesh.add_custom_vec2_attribute("uv_coordinate", uvs)?;
    Ok(mesh)
}