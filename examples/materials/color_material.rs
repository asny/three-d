use dust::core::program;
use gl;
use dust::input;
use dust::traits;
use gust::mesh;
use dust::core::attributes;
use std::rc::Rc;

pub struct ColorMaterial {
    program: program::Program,
    model: attributes::Attributes
}

impl traits::Reflecting for ColorMaterial
{
    fn reflect(&self, input: &input::DrawInput) -> Result<(), traits::Error>
    {
        self.program.add_uniform_mat4("viewMatrix", &input.view)?;
        self.program.add_uniform_mat4("projectionMatrix", &input.projection)?;
        self.model.draw(input);
        Ok(())
    }
}

impl ColorMaterial
{
    pub fn create(gl: &gl::Gl, mesh: &mesh::Mesh) -> Result<Rc<traits::Reflecting>, traits::Error>
    {
        let shader_program = program::Program::from_resource(&gl, "examples/assets/shaders/color")?;
        let attributes = attributes::Attributes::create(gl, mesh, &shader_program).unwrap();

        Ok(Rc::new(ColorMaterial { program: shader_program, model: attributes }))
    }
}
