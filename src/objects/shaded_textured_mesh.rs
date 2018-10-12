
use gl;
use ::*;

pub struct ShadedTexturedMesh {
    program: program::Program,
    model: surface::TriangleSurface,
    texture: texture::Texture2D
}

impl ShadedTexturedMesh
{
    pub fn create(gl: &gl::Gl, mesh: &mesh::StaticMesh, texture: texture::Texture2D) -> ShadedTexturedMesh
    {
        let program = program::Program::from_resource(&gl, "../Dust/src/objects/shaders/mesh_shaded_textured",
                                                      "../Dust/src/objects/shaders/shaded_textured").unwrap();
        let mut model = surface::TriangleSurface::create(gl, mesh).unwrap();
        model.add_attributes(mesh, &program, &vec!["position", "normal", "uv_coordinate"]).unwrap();

        ShadedTexturedMesh { program, model, texture }
    }

    pub fn render(&self, transformation: &Mat4, camera: &camera::PerspectiveCamera)
    {
        self.texture.bind(0);
        self.program.add_uniform_int("texture0", &0).unwrap();
        self.program.add_uniform_mat4("modelMatrix", &transformation).unwrap();
        self.program.add_uniform_mat4("viewMatrix", &camera.get_view()).unwrap();
        self.program.add_uniform_mat4("projectionMatrix", &camera.get_projection()).unwrap();
        self.program.add_uniform_mat4("normalMatrix", &transformation.try_inverse().unwrap().transpose()).unwrap();
        self.model.render().unwrap();
    }
}
