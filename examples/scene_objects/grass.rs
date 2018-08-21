use gl;
use glm::*;

use gust;

use dust::{traits, camera};
use dust::core::{program, surface, state};

use num_traits::identities::One;

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

        Ok(Grass { program, model })
    }

    pub fn render(&self, camera: &camera::Camera) -> Result<(), traits::Error>
    {
        let transformation = Matrix4::one();

        self.program.cull(state::CullType::NONE);

        self.program.add_uniform_mat4("viewMatrix", &camera.get_view())?;
        self.program.add_uniform_mat4("projectionMatrix", &camera.get_projection())?;

        self.model.render_instances(2)?;
        Ok(())
    }
}
