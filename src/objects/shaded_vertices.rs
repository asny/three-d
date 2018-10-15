
use gl;
use ::*;

pub struct ShadedVertices {
    program: program::Program,
    surface: surface::TriangleSurface,
    no_vertices: usize,
    pub color: Vec3,
    pub diffuse_intensity: f32,
    pub specular_intensity: f32,
    pub specular_power: f32,
    pub scale: f32
}

impl ShadedVertices
{
    pub fn create(gl: &gl::Gl, mesh: &::mesh::DynamicMesh) -> ShadedVertices
    {
        let program = program::Program::from_resource(&gl, "../Dust/src/objects/shaders/vertex_shaded",
                                                      "../Dust/src/objects/shaders/shaded").unwrap();

        let vertex_mesh = ::mesh_generator::create_cylinder().unwrap();
        let mut surface = surface::TriangleSurface::create(gl, &vertex_mesh).unwrap();
        surface.add_attributes(&vertex_mesh, &program, &vec!["position"]).unwrap();

        let mut instance_buffer = buffer::VertexBuffer::create(gl).unwrap();

        program.set_used();
        program.setup_attribute("translation", 3, 3, 0, 1).unwrap();

        let mut data = Vec::new();
        for vertex_id in mesh.vertex_iterator() {
            for val in mesh.position(&vertex_id).iter() {
                data.push(*val);
            }
        }
        instance_buffer.fill_with(data);

        ShadedVertices { program, surface, no_vertices: mesh.no_vertices(), color: vec3(1.0, 0.0, 0.0), diffuse_intensity: 0.5, specular_intensity: 0.2, specular_power: 5.0, scale: 1.0 }
    }

    pub fn render(&self, camera: &camera::Camera)
    {
        self.program.cull(state::CullType::BACK);
        self.program.depth_test(state::DepthTestType::LEQUAL);
        self.program.depth_write(true);
        self.program.polygon_mode(state::PolygonType::Fill);

        self.program.add_uniform_float("diffuse_intensity", &self.diffuse_intensity).unwrap();
        self.program.add_uniform_float("specular_intensity", &self.specular_intensity).unwrap();
        self.program.add_uniform_float("specular_power", &self.specular_power).unwrap();

        self.program.add_uniform_int("use_texture", &0).unwrap();
        self.program.add_uniform_vec3("color", &self.color).unwrap();

        self.program.add_uniform_float("scale", &self.scale).unwrap();

        self.program.add_uniform_mat4("viewMatrix", camera.get_view()).unwrap();
        self.program.add_uniform_mat4("projectionMatrix", camera.get_projection()).unwrap();
        self.surface.render_instances(self.no_vertices).unwrap();
    }
}
