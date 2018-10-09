
use gl;
use ::*;

pub struct Standard {
    program: program::Program,
    model: surface::TriangleSurface,
    pub color: Vec3
}

impl Standard
{
    pub fn create(gl: &gl::Gl, mesh: &Renderable) -> Standard
    {
        let program = program::Program::from_resource(&gl, "../Dust/src/default_objects/shaders/standard").unwrap();
        let mut model = surface::TriangleSurface::create(gl, mesh).unwrap();
        model.add_attributes(mesh, &program, &vec!["position"]).unwrap();

        Standard { program, model, color: vec3(1.0, 1.0, 1.0) }
    }

    pub fn render(&self, transformation: &Mat4, camera: &camera::Camera)
    {
        self.program.add_uniform_vec3("color", &self.color).unwrap();
        self.program.add_uniform_mat4("modelMatrix", &transformation).unwrap();
        self.program.add_uniform_mat4("viewMatrix", &camera.get_view()).unwrap();
        self.program.add_uniform_mat4("projectionMatrix", &camera.get_projection()).unwrap();
        self.program.add_uniform_mat4("normalMatrix", &transformation.try_inverse().unwrap().transpose()).unwrap();
        self.model.render().unwrap();
    }
}
