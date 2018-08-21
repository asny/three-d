use gl;
use glm::*;

use gust;

use dust::{traits, camera};
use dust::core::{buffer, program, surface, state};

pub struct Grass {
    program: program::Program,
    model: surface::TriangleSurface
}

impl Grass
{
    pub fn create(gl: &gl::Gl) -> Result<Grass, traits::Error>
    {
        let mesh = gust::models::create_cube().unwrap();
        let program = program::Program::from_resource(gl, "examples/assets/shaders/grass")?;
        let mut model = surface::TriangleSurface::create_without_adding_attributes(gl, &mesh)?;
        model.add_attributes(&[&mesh.positions].to_vec(), &program)?;

        let offsets = [0.0,0.0,0.0, 10.0,0.0,10.0];

        let mut buffer = buffer::VertexBuffer::create(gl).unwrap();
        buffer.fill_with(&offsets.to_vec());

        program.set_used();
        program.setup_attribute("offset", 3, 3, 0, 1)?;

        Ok(Grass { program, model })
    }

    pub fn render(&self, camera: &camera::Camera) -> Result<(), traits::Error>
    {
        self.program.cull(state::CullType::NONE);

        self.program.add_uniform_mat4("viewMatrix", &camera.get_view())?;
        self.program.add_uniform_mat4("projectionMatrix", &camera.get_projection())?;

        self.model.render_instances(2)?;
        Ok(())
    }
}
