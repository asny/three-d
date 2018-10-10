
use gl;
use ::*;

pub struct Wireframe {
    program: program::Program,
    surface: surface::TriangleSurface,
    no_edges: usize,
    pub color: Vec3
}

impl Wireframe
{
    pub fn create(gl: &gl::Gl, mesh: &::mesh::DynamicMesh) -> Wireframe
    {
        let program = program::Program::from_resource(&gl, "../Dust/src/default_objects/shaders/wireframe").unwrap();

        let edge_mesh = ::mesh_generator::create_cylinder().unwrap();
        let mut surface = surface::TriangleSurface::create(gl, &edge_mesh).unwrap();
        surface.add_attributes(&edge_mesh, &program, &vec!["position"]).unwrap();

        let mut position_buffer = buffer::VertexBuffer::create(gl).unwrap();

        program.set_used();
        program.setup_attribute("position0", 3, 6, 0, 1).unwrap();
        program.setup_attribute("position1", 3, 6, 3, 1).unwrap();

        let mut positions = Vec::new();
        for halfedge_id in mesh.halfedge_iterator() {
            let (p0, p1) = mesh.edge_positions(&halfedge_id);
            positions.push(p0.x);
            positions.push(p0.y);
            positions.push(p0.z);
            positions.push(p1.x);
            positions.push(p1.y);
            positions.push(p1.z);
        }
        position_buffer.fill_with(positions);

        Wireframe { program, surface, no_edges: mesh.no_halfedges(), color: vec3(1.0, 0.0, 0.0) }
    }

    pub fn render(&self, camera: &camera::Camera)
    {
        self.program.cull(state::CullType::BACK);
        self.program.depth_test(state::DepthTestType::LEQUAL);
        self.program.depth_write(true);
        self.program.polygon_mode(state::PolygonType::Fill);

        self.program.add_uniform_vec3("color", &self.color).unwrap();
        self.program.add_uniform_mat4("viewMatrix", &camera.get_view()).unwrap();
        self.program.add_uniform_mat4("projectionMatrix", &camera.get_projection()).unwrap();
        self.surface.render_instances(self.no_edges).unwrap();
    }
}
