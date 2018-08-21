use gl;
use glm::*;

use gust;

use scene_objects::terrain::Terrain;
use dust::{traits, camera};
use dust::core::{buffer, program, surface, state};

pub struct Grass {
    program: program::Program,
    model: surface::TriangleSurface,
    position_buffer: buffer::VertexBuffer
}

const NO_STRAWS: usize = 2;

impl Grass
{
    pub fn create(gl: &gl::Gl, terrain: &Terrain) -> Result<Grass, traits::Error>
    {
        let mesh = gust::models::create_cube().unwrap();
        let program = program::Program::from_resource(gl, "examples/assets/shaders/grass")?;
        let mut model = surface::TriangleSurface::create_without_adding_attributes(gl, &mesh)?;
        model.add_attributes(&[&mesh.positions].to_vec(), &program)?;

        let mut position_buffer = buffer::VertexBuffer::create(gl).unwrap();

        program.set_used();
        program.setup_attribute("root_position", 3, 3, 0, 1)?;

        let mut grass = Grass { program, model, position_buffer };
        grass.create_straws(terrain);
        Ok(grass)
    }

    pub fn create_straws(&mut self, terrain: &Terrain)
    {
        let offsets = [0.0,0.0,0.0, 10.0,0.0,10.0];
        self.position_buffer.fill_with(&offsets.to_vec());
    }

    pub fn render(&self, camera: &camera::Camera) -> Result<(), traits::Error>
    {
        self.program.cull(state::CullType::NONE);

        self.program.add_uniform_mat4("viewMatrix", &camera.get_view())?;
        self.program.add_uniform_mat4("projectionMatrix", &camera.get_projection())?;

        self.model.render_instances(NO_STRAWS)?;
        Ok(())
    }
}
