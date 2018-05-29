use dust::core::program;
use gl;
use dust::input;
use dust::traits;
use gust::mesh;
use std::rc::Rc;
use dust::core::texture;
use dust::core::texture::Texture;
use dust::core::surface;

pub struct TextureMaterial {
    program: program::Program,
    model: surface::TriangleSurface,
    texture: texture::Texture2D
}

impl traits::Reflecting for TextureMaterial
{
    fn reflect(&self, input: &input::ReflectingInput) -> Result<(), traits::Error>
    {
        self.program.cull_back_faces(true);
        self.texture.bind(0);
        self.program.add_uniform_int("texture0", &0)?;
        self.program.add_uniform_mat4("modelMatrix", &input.model)?;
        self.program.add_uniform_mat4("viewMatrix", &input.view)?;
        self.program.add_uniform_mat4("projectionMatrix", &input.projection)?;
        self.program.add_uniform_mat4("normalMatrix", &input.normal)?;

        self.model.render()?;
        Ok(())
    }

}

impl TextureMaterial
{
    pub fn create(gl: &gl::Gl, mesh: &mesh::Mesh) -> Result<Rc<traits::Reflecting>, traits::Error>
    {
        let program = program::Program::from_resource(gl, "examples/assets/shaders/texture")?;
        let model = surface::TriangleSurface::create(gl, mesh, &program)?;

        let tex_data: Vec<f32> = vec![
            1.0, 1.0, 0.5, 1.0, 0.5, 0.5, 1.0, 1.0, 0.5, 1.0, 1.0, 0.5, 1.0, 0.5, 0.5, 1.0
        ];
        let texture = texture::Texture2D::create_from_data(gl, 4, 4, &tex_data)?;

        Ok(Rc::new(TextureMaterial { program, model, texture }))
    }
}
