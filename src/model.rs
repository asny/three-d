use gl;
use std;
use std::rc::Rc;
use material;
use gust::mesh;
use input;
use core::buffer;
use glm;
use core::program;
use core::attributes;

#[derive(Debug)]
pub enum Error {
    Material(material::Error)
}

impl From<material::Error> for Error {
    fn from(other: material::Error) -> Self {
        Error::Material(other)
    }
}

pub struct Model {
    material: Rc<material::Reflecting>,
    mesh: mesh::Mesh,
    attributes: attributes::Attributes
}

impl Model
{
    pub fn create(gl: &gl::Gl, mesh: mesh::Mesh, material: Rc<material::Reflecting>) -> Result<Model, Error>
    {
        let attributes = attributes::Attributes::create(gl, mesh, material.program).unwrap();
        let model = Model { material, mesh, attributes };
        Ok(model)
    }

    pub fn draw(&self, input: &input::DrawInput) -> Result<(), Error>
    {
        self.material.apply();
        self.material.setup_states(&self.gl)?;
        self.material.setup_uniforms(&input)?;

        self.attributes.draw(input);
        Ok(())
    }
}