use gl;
use crate::*;
use crate::static_mesh::Attribute;

pub struct Skybox {
    program: program::Program,
    model: surface::TriangleSurface,
    texture: texture::Texture3D
}

impl Skybox
{
    pub fn create(gl: &gl::Gl, texture: texture::Texture3D) -> Skybox
    {
        let program = program::Program::from_resource(gl, "../Dust/src/objects/shaders/skybox",
                                                      "../Dust/src/objects/shaders/skybox").unwrap();

        let positions = get_positions();
        let indices: Vec<u32> = (0..positions.len() as u32/3).collect();
        let attributes = vec![Attribute::new("position", 3, positions)];
        let mut model = surface::TriangleSurface::create(gl, &indices).unwrap();
        model.add_attributes(&program, &attributes).unwrap();

        Skybox { program, model, texture }
    }

    pub fn render(&self, camera: &camera::Camera) -> Result<(), traits::Error>
    {
        self.program.cull(state::CullType::FRONT);
        self.program.depth_write(true);
        self.program.depth_test(state::DepthTestType::LEQUAL);
        self.program.polygon_mode(state::PolygonType::Fill);

        self.texture.bind(0);
        self.program.add_uniform_int("texture0", &0)?;
        self.program.add_uniform_mat4("viewMatrix", camera.get_view())?;
        self.program.add_uniform_mat4("projectionMatrix", camera.get_projection())?;
        self.program.add_uniform_vec3("cameraPosition", camera.position())?;

        self.model.render()?;
        Ok(())
    }

    pub fn get_texture(&self) -> &texture::Texture3D
    {
        &self.texture
    }
}

fn get_positions() -> Vec<f32>
{
    vec![
        1.0, 1.0, -1.0,
        -1.0, 1.0, -1.0,
        1.0, 1.0, 1.0,
        -1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
        -1.0, 1.0, -1.0,

        -1.0, -1.0, -1.0,
        1.0, -1.0, -1.0,
        1.0, -1.0, 1.0,
        1.0, -1.0, 1.0,
        -1.0, -1.0, 1.0,
        -1.0, -1.0, -1.0,

        1.0, -1.0, -1.0,
        -1.0, -1.0, -1.0,
        1.0, 1.0, -1.0,
        -1.0, 1.0, -1.0,
        1.0, 1.0, -1.0,
        -1.0, -1.0, -1.0,

        -1.0, -1.0, 1.0,
        1.0, -1.0, 1.0,
        1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
        -1.0, 1.0, 1.0,
        -1.0, -1.0, 1.0,

        1.0, -1.0, -1.0,
        1.0, 1.0, -1.0,
        1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
        1.0, -1.0, 1.0,
        1.0, -1.0, -1.0,

        -1.0, 1.0, -1.0,
        -1.0, -1.0, -1.0,
        -1.0, 1.0, 1.0,
        -1.0, -1.0, 1.0,
        -1.0, 1.0, 1.0,
        -1.0, -1.0, -1.0
    ]
}