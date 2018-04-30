use dust::program;
use gl;
use dust::input;
use dust::material;
use dust::texture;
use gust::mesh;
use std::rc::Rc;

pub struct SimulationMaterial {
    program: program::Program,
    texture: texture::Texture
}

impl material::Material for SimulationMaterial
{
    fn apply(&self)
    {
        self.program.set_used();
    }

    fn setup_states(&self, _gl: &gl::Gl) -> Result<(), material::Error> {
        Ok(())
    }

    fn setup_uniforms(&self, input: &input::DrawInput) -> Result<(), material::Error>
    {
        self.texture.bind_at(0);
        self.program.add_uniform_int("tex", &0)?;
        self.program.add_uniform_mat4("viewMatrix", &input.view)?;
        self.program.add_uniform_mat4("projectionMatrix", &input.projection)?;
        Ok(())
    }

    fn setup_attributes(&self, mesh: &mesh::Mesh) -> Result<(), material::Error>
    {
        self.program.add_attribute(&mesh.positions())?;
        Ok(())
    }
}

impl SimulationMaterial
{
    pub fn create(gl: &gl::Gl) -> Result<Rc<material::Material>, material::Error>
    {
        let shader_program = program::Program::from_resource(&gl, "examples/assets/shaders/simulation")?;

        let mut texture = texture::Texture::create(&gl).unwrap();
        let tex_data: Vec<f32> = vec![
            1.0, 1.0, 0.5, 1.0, 0.5, 0.5, 1.0, 1.0, 0.5, 1.0, 1.0, 0.5, 1.0, 0.5, 0.5, 1.0
        ];
        texture.fill_with(&tex_data, 4, 4);

        Ok(Rc::new(SimulationMaterial { program: shader_program, texture }))
    }
}
