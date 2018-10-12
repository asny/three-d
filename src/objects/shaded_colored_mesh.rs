
use gl;
use ::*;

pub struct ShadedColoredMesh {
    program: program::Program,
    model: surface::TriangleSurface,
    pub color: Vec3
}

impl ShadedColoredMesh
{
    pub fn create(gl: &gl::Gl, mesh: &Renderable) -> ShadedColoredMesh
    {
        let program = program::Program::from_resource(&gl, "../Dust/src/objects/shaders/mesh_shaded",
                                                      "../Dust/src/objects/shaders/shaded_colored").unwrap();
        let mut model = surface::TriangleSurface::create(gl, mesh).unwrap();
        model.add_attributes(mesh, &program, &vec!["position", "normal"]).unwrap();

        ShadedColoredMesh { program, model, color: vec3(1.0, 1.0, 1.0) }
    }

    pub fn render(&self, transformation: &Mat4, camera: &camera::Camera)
    {
        self.program.cull(state::CullType::BACK);
        self.program.depth_test(state::DepthTestType::LEQUAL);
        self.program.depth_write(true);
        self.program.polygon_mode(state::PolygonType::Fill);

        self.program.add_uniform_vec3("color", &self.color).unwrap();
        self.program.add_uniform_mat4("modelMatrix", &transformation).unwrap();
        self.program.add_uniform_mat4("viewMatrix", &camera.get_view()).unwrap();
        self.program.add_uniform_mat4("projectionMatrix", &camera.get_projection()).unwrap();
        self.program.add_uniform_mat4("normalMatrix", &transformation.try_inverse().unwrap().transpose()).unwrap();
        self.model.render().unwrap();
    }
}
