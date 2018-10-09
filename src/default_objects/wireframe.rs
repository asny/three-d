
use gl;
use ::*;

pub struct Wireframe {
    program: program::Program,
    surface: surface::TriangleSurface,
    pub color: Vec3
}

impl Wireframe
{
    pub fn create(gl: &gl::Gl, mesh: &Renderable) -> Wireframe
    {
        let program = program::Program::from_resource(&gl, "../Dust/examples/assets/shaders/standard").unwrap();
        let mut surface = surface::TriangleSurface::create(gl, mesh).unwrap();
        surface.add_attributes(mesh, &program, &vec!["position"]).unwrap();

        Wireframe { program, surface, color: vec3(1.0, 0.0, 0.0) }
    }

    pub fn render(&self, transformation: &Mat4, camera: &camera::Camera)
    {
        self.program.cull(state::CullType::NONE);
        self.program.depth_test(state::DepthTestType::NONE);
        self.program.depth_write(false);
        self.program.polygon_mode(state::PolygonType::Fill);

        self.program.add_uniform_vec3("color", &self.color).unwrap();
        self.program.add_uniform_mat4("modelMatrix", &transformation).unwrap();
        self.program.add_uniform_mat4("viewMatrix", &camera.get_view()).unwrap();
        self.program.add_uniform_mat4("projectionMatrix", &camera.get_projection()).unwrap();
        self.program.add_uniform_mat4("normalMatrix", &transformation.try_inverse().unwrap().transpose()).unwrap();
        self.surface.render().unwrap();
    }
}
