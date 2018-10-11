
use gl;
use ::*;

pub struct Phong {
    program: program::Program,
    model: surface::TriangleSurface,
    pub color: Vec3
}

impl Phong
{
    pub fn create(gl: &gl::Gl, mesh: &Renderable) -> Phong
    {
        let program = program::Program::from_resource(&gl, "../Dust/src/objects/shaders/mesh",
                                                      "../Dust/src/objects/shaders/shaded_color").unwrap();
        let mut model = surface::TriangleSurface::create(gl, mesh).unwrap();
        model.add_attributes(mesh, &program, &vec!["position", "normal"]).unwrap();

        Phong { program, model, color: vec3(1.0, 1.0, 1.0) }
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
