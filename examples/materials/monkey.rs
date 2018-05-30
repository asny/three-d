use dust::core::program;
use gl;
use glm;
use dust::traits;
use gust;
use dust::core::surface;

pub struct Monkey {
    program: program::Program,
    model: surface::TriangleSurface
}

impl traits::Reflecting for Monkey
{
    fn reflect(&self, input: &traits::ReflectingInput) -> Result<(), traits::Error>
    {
        self.program.add_uniform_vec3("color", &glm::vec3(1.0, 1.0, 1.0))?;
        self.program.add_uniform_mat4("modelMatrix", &input.model)?;
        self.program.add_uniform_mat4("viewMatrix", &input.view)?;
        self.program.add_uniform_mat4("projectionMatrix", &input.projection)?;
        self.program.add_uniform_mat4("normalMatrix", &input.normal)?;
        self.model.render()?;
        Ok(())
    }
}

impl Monkey
{
    pub fn create(gl: &gl::Gl) -> Result<Monkey, traits::Error>
    {
        let mesh = gust::loader::load_obj("/examples/assets/models/suzanne.obj").unwrap();
        let program = program::Program::from_resource(&gl, "examples/assets/shaders/standard")?;
        let model = surface::TriangleSurface::create(gl, &mesh, &program)?;

        Ok(Monkey { program, model })
    }
}
