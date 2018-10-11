
use gl;
use ::*;

pub struct Wireframe {
    program: program::Program,
    surface: surface::TriangleSurface,
    no_edges: usize,
    pub color: Vec3
}

fn rotation(axis: Vec3, cos_angle: f32) -> Mat4
{
    let c = cos_angle;
    let s = (1.0 - c*c).sqrt();
    let oc = 1.0 - c;
    return Mat4::new(oc * axis.x * axis.x + c,           oc * axis.x * axis.y - axis.z * s,  oc * axis.z * axis.x + axis.y * s,  0.0,
                oc * axis.x * axis.y + axis.z * s,  oc * axis.y * axis.y + c,           oc * axis.y * axis.z - axis.x * s,  0.0,
                oc * axis.z * axis.x - axis.y * s,  oc * axis.y * axis.z + axis.x * s,  oc * axis.z * axis.z + c,           0.0,
                0.0,                                0.0,                                0.0,                                1.0);
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
        program.setup_attribute("local2worldX", 3, 21, 0, 1).unwrap();
        program.setup_attribute("local2worldY", 3, 21, 3, 1).unwrap();
        program.setup_attribute("local2worldZ", 3, 21, 6, 1).unwrap();
        program.setup_attribute("translation", 3, 21, 9, 1).unwrap();
        program.setup_attribute("normalMatrixX", 3, 21, 12, 1).unwrap();
        program.setup_attribute("normalMatrixY", 3, 21, 15, 1).unwrap();
        program.setup_attribute("normalMatrixZ", 3, 21, 18, 1).unwrap();

        let mut data = Vec::new();
        for halfedge_id in mesh.halfedge_iterator() {
            let (p0, p1) = mesh.edge_positions(&halfedge_id);

            let length = (p1 - p0).norm();
            let dir = (p1 - p0)/length;
            let cos_angle = vec3(1.0, 0.0, 0.0).dot(&dir);
            let rotation_axis = if cos_angle.abs() > 0.999 {
                vec3(0.0, 1.0, 0.0)
            }
            else {
                vec3(1.0, 0.0, 0.0).cross(&dir).normalize()
            };
            let local_to_world = rotation(rotation_axis, cos_angle) * Mat4::new_nonuniform_scaling(&vec3(length, 0.01, 0.01));
            let normal_matrix = local_to_world.try_inverse().unwrap().transpose();

            for i in 0..3 {
                for j in 0..3 {
                    data.push(*local_to_world.column(i).row(j).iter().next().unwrap());
                }
            }

            for val in p0.iter() {
                data.push(*val);
            }

            for i in 0..3 {
                for j in 0..3 {
                    data.push(*normal_matrix.column(i).row(j).iter().next().unwrap());
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
