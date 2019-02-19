
use gl;
use crate::*;

pub struct ShadedEdges {
    program: program::Program,
    instance_buffer: buffer::VertexBuffer,
    surface: surface::TriangleSurface,
    index_pairs: std::collections::HashSet<(usize, usize)>,
    no_edges: usize,
    tube_radius: f32,
    pub color: Vec3,
    pub diffuse_intensity: f32,
    pub specular_intensity: f32,
    pub specular_power: f32
}

impl ShadedEdges
{
    pub fn new(gl: &gl::Gl, indices: &[u32], positions: &[f32], tube_radius: f32) -> ShadedEdges
    {
        let program = program::Program::from_source(&gl,
                                                    include_str!("shaders/line_shaded.vert"),
                                                    include_str!("shaders/shaded.frag")).unwrap();

        let x_subdivisions = 1;
        let angle_subdivisions = 10;
        let mut cylinder_positions = Vec::new();
        let mut cylinder_indices = Vec::new();
        for i in 0..x_subdivisions+1 {
            let x = i as f32 / x_subdivisions as f32;
            for j in 0..angle_subdivisions {
                let angle = 2.0 * std::f32::consts::PI * j as f32 / angle_subdivisions as f32;

                cylinder_positions.push(x);
                cylinder_positions.push(angle.cos());
                cylinder_positions.push(angle.sin());
            }
        }
        for i in 0..x_subdivisions as u32 {
            for j in 0..angle_subdivisions as u32 {
                cylinder_indices.push(i * angle_subdivisions as u32 + j);
                cylinder_indices.push(i * angle_subdivisions as u32 + (j+1)%angle_subdivisions as u32);
                cylinder_indices.push((i+1) * angle_subdivisions as u32 + (j+1)%angle_subdivisions as u32);

                cylinder_indices.push(i * angle_subdivisions as u32 + j);
                cylinder_indices.push((i+1) * angle_subdivisions as u32 + (j+1)%angle_subdivisions as u32);
                cylinder_indices.push((i+1) * angle_subdivisions as u32 + j);
            }
        }
        let mut surface = surface::TriangleSurface::new(gl, &cylinder_indices).unwrap();
        surface.add_attributes(&program, &att!["position" => (cylinder_positions, 3)]).unwrap();

        let instance_buffer = buffer::VertexBuffer::new(gl).unwrap();

        program.setup_attribute(&instance_buffer,"local2worldX", 3, 21, 0, 1).unwrap();
        program.setup_attribute(&instance_buffer,"local2worldY", 3, 21, 3, 1).unwrap();
        program.setup_attribute(&instance_buffer,"local2worldZ", 3, 21, 6, 1).unwrap();
        program.setup_attribute(&instance_buffer,"translation", 3, 21, 9, 1).unwrap();
        program.setup_attribute(&instance_buffer,"normalMatrixX", 3, 21, 12, 1).unwrap();
        program.setup_attribute(&instance_buffer,"normalMatrixY", 3, 21, 15, 1).unwrap();
        program.setup_attribute(&instance_buffer,"normalMatrixZ", 3, 21, 18, 1).unwrap();

        let mut index_pairs = std::collections::HashSet::new();
        for f in 0..indices.len()/3 {
            let i1 = indices[3*f] as usize;
            let i2 = indices[3*f+1] as usize;
            let i3 = indices[3*f+2] as usize;
            index_pairs.insert(if i1 < i2 {(i1, i2)} else {(i2, i1)});
            index_pairs.insert(if i1 < i3 {(i1, i3)} else {(i3, i1)});
            index_pairs.insert(if i2 < i3 {(i2, i3)} else {(i3, i2)});
        }
        let no_edges = index_pairs.len();

        let mut object = ShadedEdges { program, instance_buffer, surface, index_pairs, no_edges, tube_radius, color: vec3(1.0, 0.0, 0.0), diffuse_intensity: 0.5, specular_intensity: 0.2, specular_power: 5.0 };
        object.update_positions(positions);
        object
    }

    pub fn update_positions(&mut self, positions: &[f32])
    {
        let mut data = Vec::new();
        for (i0, i1) in self.index_pairs.iter() {
            let p0 = vec3(positions[i0 * 3], positions[i0 * 3+1], positions[i0 * 3+2]);
            let p1 = vec3(positions[i1 * 3], positions[i1 * 3+1], positions[i1 * 3+2]);

            let length = (p1 - p0).magnitude();
            let dir = (p1 - p0)/length;
            let local_to_world = rotation_matrix_from_dir_to_dir(vec3(1.0, 0.0, 0.0), dir) * Mat4::from_nonuniform_scale(length, self.tube_radius, self.tube_radius);
            let normal_matrix = local_to_world.invert().unwrap().transpose();

            for i in 0..3 {
                for j in 0..3 {
                    data.push(local_to_world[i][j]);
                }
            }

            for i in 0..3 {
                data.push(p0[i]);
            }

            for i in 0..3 {
                for j in 0..3 {
                    data.push(normal_matrix[i][j]);
                }
            }

        }
        self.instance_buffer.fill_with(&data);
    }

    pub fn render(&self, camera: &camera::Camera)
    {
        self.program.cull(state::CullType::BACK);
        self.program.depth_test(state::DepthTestType::LEQUAL);
        self.program.depth_write(true);

        self.program.add_uniform_float("diffuse_intensity", &self.diffuse_intensity).unwrap();
        self.program.add_uniform_float("specular_intensity", &self.specular_intensity).unwrap();
        self.program.add_uniform_float("specular_power", &self.specular_power).unwrap();

        self.program.add_uniform_int("use_texture", &0).unwrap();
        self.program.add_uniform_vec3("color", &self.color).unwrap();

        self.program.add_uniform_mat4("viewMatrix", camera.get_view()).unwrap();
        self.program.add_uniform_mat4("projectionMatrix", camera.get_projection()).unwrap();
        self.surface.render_instances(self.no_edges).unwrap();
    }
}
