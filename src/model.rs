use gl;
use std;
use std::rc::Rc;
use material;
use gust::mesh;
use input;
use buffer;
use glm;
use program;

#[derive(Debug)]
pub enum Error {
    Material(material::Error),
    Buffer(buffer::Error)
}

impl From<material::Error> for Error {
    fn from(other: material::Error) -> Self {
        Error::Material(other)
    }
}

impl From<buffer::Error> for Error {
    fn from(other: buffer::Error) -> Self {
        Error::Buffer(other)
    }
}

pub struct Model {
    gl: gl::Gl,
    id: gl::types::GLuint,
    material: Rc<material::Material>,
    mesh: mesh::Mesh
}

impl Model
{
    pub fn draw_full_screen_quad(gl: &gl::Gl, program: &program::Program)
    {
        unsafe {
            static mut FULL_SCREEN__QUAD_ID: gl::types::GLuint = std::u32::MAX;
            if std::u32::MAX == FULL_SCREEN__QUAD_ID
            {
                // Generate and bind array
                gl.GenVertexArrays(1, &mut FULL_SCREEN__QUAD_ID);
                bind(gl, FULL_SCREEN__QUAD_ID);

                let positions: Vec<glm::Vec3> = vec![
                    glm::vec3(-3.0, -1.0, 0.0),
                    glm::vec3(3.0,  -1.0, 0.0),
                    glm::vec3(0.0, 2.0, 0.0)
                ];
                let uv_coordinates: Vec<glm::Vec2> = vec![
                    glm::vec2(-1.0, 0.0),
                    glm::vec2(2.0, 0.0),
                    glm::vec2(0.5, 1.5)
                ];
                let mut mesh = mesh::Mesh::create(&vec![0, 1, 2], positions).unwrap();
                mesh.add_custom_vec2_attribute("uv_coordinate", uv_coordinates).unwrap();

                let index_buffer = buffer::ElementBuffer::create(gl).unwrap();
                index_buffer.fill_with(&vec![0, 1, 2]);

                let mut list = Vec::new();
                list.push( mesh.positions());
                list.push(mesh.get("uv_coordinate").unwrap());
                program.add_attributes(&list).unwrap();
            }
            bind(gl, FULL_SCREEN__QUAD_ID);
            gl.DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_SHORT, std::ptr::null());
        }
    }

    pub fn create(gl: &gl::Gl, mesh: mesh::Mesh, material: Rc<material::Material>) -> Result<Model, Error>
    {
        let mut id: gl::types::GLuint = 0;
        unsafe {
            gl.GenVertexArrays(1, &mut id);
        }
        let model = Model { gl: gl.clone(), id, material, mesh };
        model.bind();

        let index_buffer = buffer::ElementBuffer::create(&model.gl)?;
        index_buffer.fill_with(&model.mesh.indices());

        model.material.setup_attributes(&model.mesh)?;
        Ok(model)
    }

    pub fn update_attributes(&self) -> Result<(), Error>
    {
        // TODO: Update the attributes in the relevant vertex buffers
        Ok(())
    }

    pub fn draw(&self, input: &input::DrawInput) -> Result<(), Error>
    {
        self.material.apply();
        self.material.setup_states(&self.gl)?;
        self.material.setup_uniforms(&input)?;

        self.bind();
        let draw_mode = self.get_draw_mode();
        unsafe {
            self.gl.DrawElements(
                draw_mode, // mode
                self.mesh.indices().len() as i32, // number of indices to be rendered
                gl::UNSIGNED_SHORT,
                std::ptr::null() // starting index in the enabled arrays
            );
        }
        Ok(())
    }

    fn bind(&self)
    {
        bind(&self.gl, self.id);
    }

    fn get_draw_mode(&self) -> u32
    {
        gl::TRIANGLES
    }
}

fn bind(gl: &gl::Gl, id: u32)
{
    unsafe {
        static mut CURRENTLY_USED: gl::types::GLuint = std::u32::MAX;
        if id != CURRENTLY_USED
        {
            gl.BindVertexArray(id);
            CURRENTLY_USED = id;
        }
    }
}