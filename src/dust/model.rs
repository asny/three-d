use gl;
use dust::material;
use dust::mesh;
use dust::attribute;

#[derive(Debug)]
pub enum Error {
}

pub struct Model {
    gl: gl::Gl,
    id: gl::types::GLuint,
    material: material::Material
}


impl Model
{
    pub fn create(gl: &gl::Gl, material: &material::Material, mesh: &mesh::Mesh) -> Result<Model, Error>
    {
        let mut vao: gl::types::GLuint = 0;
        unsafe {
            gl.GenVertexArrays(1, &mut vao);
            gl.BindVertexArray(vao);
        }

        attribute::Attribute::create(&gl, "Position", &material.program(), mesh.positions()).unwrap();

        Ok(Model { gl: gl.clone(), id: vao, material: material.clone() })
    }

    pub fn add_attribute(&self, name: &str, data: &Vec<f32>) -> Result<attribute::Attribute, attribute::Error>{

        attribute::Attribute::create(&self.gl, name, &self.material.program(), data)
    }

    pub fn draw(&self)
    {
        self.material.apply();
        unsafe {
            self.gl.BindVertexArray(self.id);
            self.gl.DrawArrays(
                gl::TRIANGLES, // mode
                0, // starting index in the enabled arrays
                3 // number of indices to be rendered
            );
        }
    }
}
