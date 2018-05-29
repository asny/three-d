use dust::core::program;
use gl;
use dust::input;
use dust::traits;
use gust::mesh;
use dust::core::surface;
use std::rc::Rc;

pub struct ColorMaterial {
    program: program::Program,
    model: surface::TriangleSurface
}

impl traits::Reflecting for ColorMaterial
{
    fn reflect(&self, input: &input::ReflectingInput) -> Result<(), traits::Error>
    {
        self.program.add_uniform_mat4("viewMatrix", &input.view)?;
        self.program.add_uniform_mat4("projectionMatrix", &input.projection)?;
        self.model.render()?;
        Ok(())
    }
}

impl ColorMaterial
{
    pub fn create(gl: &gl::Gl, mesh: &mesh::Mesh) -> Result<Rc<traits::Reflecting>, traits::Error>
    {
        let program = program::Program::from_resource(&gl, "examples/assets/shaders/color")?;
        let model = surface::TriangleSurface::create(gl, mesh, &program)?;
        program.add_attribute(mesh.get("color")?)?;

        Ok(Rc::new(ColorMaterial { program, model }))
    }
}
