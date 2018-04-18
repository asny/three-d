use gl;
use std;
use std::rc::Rc;
use dust::material;
use dust::mesh;
use dust::input;

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
    gl: gl::Gl,
    id: gl::types::GLuint,
    material: Rc<material::Material>
}

impl Model
{
    pub fn create(gl: &gl::Gl, material: Rc<material::Material>, mesh: &mesh::Mesh) -> Result<Model, Error>
    {
        let mut vao: gl::types::GLuint = 0;
        unsafe {
            gl.GenVertexArrays(1, &mut vao);
            gl.BindVertexArray(vao);
        }
        let model = Model { gl: gl.clone(), id: vao, material: material.clone() };

        model.add_custom_attribute("Position", mesh.positions())?;

        Ok(model)
    }

    pub fn add_custom_attribute(&self, name: &str, data: &Vec<f32>) -> Result<(), Error>
    {
        let location = self.material.get_attribute_location(name)? as gl::types::GLuint;
        let mut vbo: gl::types::GLuint = 0;
        unsafe {
            self.gl.GenBuffers(1, &mut vbo);
            self.gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
            self.gl.BufferData(
                gl::ARRAY_BUFFER, // target
                (data.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
                data.as_ptr() as *const gl::types::GLvoid, // pointer to data
                gl::STATIC_DRAW, // usage
            );

            self.gl.EnableVertexAttribArray(location);
            self.gl.VertexAttribPointer(
                location, // index of the generic vertex attribute
                3, // the number of components per generic vertex attribute
                gl::FLOAT, // data type
                gl::FALSE, // normalized (int-to-float conversion)
                (3 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
                std::ptr::null() // offset of the first component
            );
            self.gl.BindBuffer(gl::ARRAY_BUFFER, 0);
        }
        Ok(())
    }

    pub fn draw(&self, input: &input::DrawInput) -> Result<(), Error>
    {
        self.material.apply();
        self.material.setup_uniforms(&input)?;
        unsafe {
            self.gl.BindVertexArray(self.id);
            self.gl.DrawArrays(
                gl::TRIANGLES, // mode
                0, // starting index in the enabled arrays
                3 // number of indices to be rendered
            );
        }
        Ok(())
    }
}
