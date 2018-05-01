use dust::program;
use gl;
use dust::input;
use dust::material;
use gust::mesh;
use std::rc::Rc;
use dust::state;
use dust::texture;

pub struct TextureMaterial {
    program: program::Program,
    texture: texture::Texture
}

impl material::Material for TextureMaterial
{
    fn apply(&self)
    {
        self.program.set_used();
    }

    fn setup_states(&self, gl: &gl::Gl) -> Result<(), material::Error> {
        state::cull_back_faces(gl, true);
        Ok(())
    }

    fn setup_uniforms(&self, input: &input::DrawInput) -> Result<(), material::Error>
    {
        self.texture.bind_at(0);
        self.program.add_uniform_int("tex", &0)?;
        self.program.add_uniform_mat4("viewMatrix", &input.view)?;
        self.program.add_uniform_mat4("projectionMatrix", &input.projection)?;
        self.program.add_uniform_vec3("cameraPosition", &input.camera_position)?;
        Ok(())
    }

    fn setup_attributes(&self, mesh: &mesh::Mesh) -> Result<(), material::Error>
    {
        let mut list = Vec::new();
        list.push( mesh.positions());
        list.push(mesh.get("Color")?);
        self.program.add_attributes(&list)?;
        Ok(())
    }
}

impl TextureMaterial
{
    pub fn create(gl: &gl::Gl) -> Result<Rc<material::Material>, material::Error>
    {
        let shader_program = program::Program::from_resource(&gl, "examples/assets/shaders/texture")?;
        let mut texture = texture::Texture::create(&gl).unwrap();

        let tex_data: Vec<f32> = vec![
            1.0, 1.0, 0.5, 1.0, 0.5, 0.5, 1.0, 1.0, 0.5, 1.0, 1.0, 0.5, 1.0, 0.5, 0.5, 1.0
        ];

        texture.fill_with(&tex_data, 4, 4, 1);

        Ok(Rc::new(TextureMaterial { program: shader_program, texture }))
    }
}
