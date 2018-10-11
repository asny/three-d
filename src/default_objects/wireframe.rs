
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

        let mut instance_buffer = buffer::VertexBuffer::create(gl).unwrap();

        program.set_used();
        program.setup_attribute("local2worldX", 3, 12, 0, 1).unwrap();
        program.setup_attribute("local2worldY", 3, 12, 3, 1).unwrap();
        program.setup_attribute("local2worldZ", 3, 12, 6, 1).unwrap();
        program.setup_attribute("translation", 3, 12, 9, 1).unwrap();

        let mut data = Vec::new();
        for halfedge_id in mesh.halfedge_iterator() {
            let (p0, p1) = mesh.edge_positions(&halfedge_id);

            let local_to_world = Mat4::identity();
            let normal_matrix = local_to_world.try_inverse().unwrap().transpose();

            for i in 0..4 {
                for j in 0..3 {
                    data.push(*local_to_world.column(i).row(j).iter().next().unwrap());
                }
            }

        }
        instance_buffer.fill_with(data);

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
