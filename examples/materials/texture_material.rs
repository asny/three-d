use dust::core::program;
use gl;
use dust::input;
use dust::traits;
use gust::mesh;
use std::rc::Rc;
use dust::core::texture;
use dust::core::texture::Texture;
use dust::core::attributes;

pub struct TextureMaterial {
    program: program::Program,
    model: attributes::Attributes,
    texture: texture::Texture2D
}

impl traits::Reflecting for TextureMaterial
{
    fn reflect(&self, input: &input::DrawInput) -> Result<(), traits::Error>
    {
        self.program.cull_back_faces(true);
        self.texture.bind(0);
        self.program.add_uniform_int("tex", &0)?;
        self.program.add_uniform_mat4("viewMatrix", &input.view)?;
        self.program.add_uniform_mat4("projectionMatrix", &input.projection)?;
        self.program.add_uniform_vec3("cameraPosition", &input.camera_position)?;

        self.model.draw(input);
        Ok(())
    }

}

impl TextureMaterial
{
    pub fn create(gl: &gl::Gl, mesh: &mesh::Mesh) -> Result<Rc<traits::Reflecting>, traits::Error>
    {
        let shader_program = program::Program::from_resource(gl, "examples/assets/shaders/texture")?;
        let attributes = attributes::Attributes::create(gl, mesh, &shader_program).unwrap();

        let tex_data: Vec<f32> = vec![
            1.0, 1.0, 0.5, 1.0, 0.5, 0.5, 1.0, 1.0, 0.5, 1.0, 1.0, 0.5, 1.0, 0.5, 0.5, 1.0
        ];
        let texture = texture::Texture2D::create_from_data(gl, 4, 4, &tex_data).unwrap();

        Ok(Rc::new(TextureMaterial { program: shader_program, model: attributes, texture }))
    }
}
