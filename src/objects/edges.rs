
use crate::*;

pub struct Edges {
    program: core::Program,
    translation_buffer: VertexBuffer,
    direction_buffer: VertexBuffer,
    cylinder_index_buffer: core::ElementBuffer,
    cylinder_vertex_buffer: VertexBuffer,
    index_pairs: std::collections::HashSet<(usize, usize)>,
    no_edges: u32,
    tube_radius: f32,
    pub color: Vec3,
    pub diffuse_intensity: f32,
    pub specular_intensity: f32,
    pub specular_power: f32
}

impl Edges
{
    pub fn new(gl: &Gl, indices: &[u32], positions: &[f32], tube_radius: f32) -> Edges
    {
        let program = core::Program::from_source(&gl,
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
        let cylinder_index_buffer = ElementBuffer::new_with_u32(gl, &cylinder_indices).unwrap();
        let cylinder_vertex_buffer = VertexBuffer::new_with_static_f32(gl,&cylinder_positions).unwrap();

        let mut index_pairs = std::collections::HashSet::new();
        for f in 0..indices.len()/3 {
            let i1 = indices[3*f] as usize;
            let i2 = indices[3*f+1] as usize;
            let i3 = indices[3*f+2] as usize;
            index_pairs.insert(if i1 < i2 {(i1, i2)} else {(i2, i1)});
            index_pairs.insert(if i1 < i3 {(i1, i3)} else {(i3, i1)});
            index_pairs.insert(if i2 < i3 {(i2, i3)} else {(i3, i2)});
        }
        let no_edges = index_pairs.len() as u32;

        let (translation, direction) = Self::fill_translation_and_direction(&index_pairs, positions);
        let translation_buffer = VertexBuffer::new_with_dynamic_f32(gl, &translation).unwrap();
        let direction_buffer = VertexBuffer::new_with_dynamic_f32(gl, &direction).unwrap();

        Edges { program, translation_buffer, direction_buffer, cylinder_vertex_buffer, cylinder_index_buffer, index_pairs, no_edges, tube_radius,
            color: vec3(1.0, 0.0, 0.0), diffuse_intensity: 0.5, specular_intensity: 0.2, specular_power: 5.0 }
    }

    fn fill_translation_and_direction(index_pairs: &std::collections::HashSet<(usize, usize)>, positions: &[f32]) -> (Vec<f32>, Vec<f32>)
    {
        let mut translation = Vec::new();
        let mut direction = Vec::new();
        for (i0, i1) in index_pairs.iter() {
            for i in 0..3 {
                translation.push(positions[i0 * 3 + i]);
                direction.push(positions[i1 * 3 + i] - positions[i0 * 3 + i]);
            }
        }
        (translation, direction)
    }

    pub fn update_positions(&mut self, positions: &[f32])
    {
        let (translation, direction) = Self::fill_translation_and_direction(&self.index_pairs, positions);
        self.translation_buffer.fill_with_dynamic_f32(&translation);
        self.direction_buffer.fill_with_dynamic_f32(&direction);
    }

    pub fn render(&self, transformation: &Mat4, camera: &camera::Camera)
    {
        self.program.add_uniform_float("diffuse_intensity", &self.diffuse_intensity).unwrap();
        self.program.add_uniform_float("specular_intensity", &self.specular_intensity).unwrap();
        self.program.add_uniform_float("specular_power", &self.specular_power).unwrap();

        self.program.add_uniform_vec3("color", &self.color).unwrap();

        self.program.use_uniform_block(camera.matrix_buffer(), "Camera");
        self.program.add_uniform_float("tube_radius", &self.tube_radius).unwrap();
        self.program.add_uniform_mat4("modelMatrix", &transformation).unwrap();

        self.program.use_attribute_vec3_float_divisor(&self.translation_buffer, "translation", 1).unwrap();
        self.program.use_attribute_vec3_float_divisor(&self.direction_buffer, "direction", 1).unwrap();

        self.program.use_attribute_vec3_float(&self.cylinder_vertex_buffer, "position").unwrap();

        self.program.draw_elements_instanced(&self.cylinder_index_buffer,self.no_edges);
    }
}
