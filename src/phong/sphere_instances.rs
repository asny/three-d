
use crate::*;

pub struct SphereInstances {
    program: Program,
    instance_buffer: VertexBuffer,
    ball_index_buffer: ElementBuffer,
    ball_vertex_buffer: VertexBuffer,
    no_vertices: u32,
    pub material: PhongMaterial,
    pub ball_radius: f32
}

impl SphereInstances
{
    pub(crate) fn new(gl: &Gl, positions: &[f32], ball_radius: f32, material: &PhongMaterial) -> Result<Self, Error>
    {
        let program = Program::from_source(&gl,
                                                    include_str!("shaders/sphere.vert"),
                                                    &format!("{}\n{}",
                                                             include_str!("shaders/deferred_objects_shared.frag"),
                                                             include_str!("shaders/colored_deferred.frag")))?;

        let x = 0.525731112119133606;
        let z = 0.850650808352039932;

        let ball_positions = vec!(
           -x, 0.0, z, x, 0.0, z, -x, 0.0, -z, x, 0.0, -z,
           0.0, z, x, 0.0, z, -x, 0.0, -z, x, 0.0, -z, -x,
           z, x, 0.0, -z, x, 0.0, z, -x, 0.0, -z, -x, 0.0
        );
        let ball_indices = vec!(
           0,1,4, 0,4,9, 9,4,5, 4,8,5, 4,1,8,
           8,1,10, 8,10,3, 5,8,3, 5,3,2, 2,3,7,
           7,3,10, 7,10,6, 7,6,11, 11,6,0, 0,6,1,
           6,10,1, 9,11,0, 9,2,11, 9,5,2, 7,11,2
        );
        let ball_index_buffer = ElementBuffer::new_with_u32(gl, &ball_indices)?;
        let ball_vertex_buffer = VertexBuffer::new_with_static_f32(gl, &ball_positions)?;
        let instance_buffer = VertexBuffer::new_with_dynamic_f32(gl, positions)?;

        Ok(SphereInstances { program, instance_buffer, ball_index_buffer, ball_vertex_buffer, no_vertices: positions.len() as u32/3,
            material: material.clone(), ball_radius })
    }

    pub fn update_positions(&mut self, positions: &[f32])
    {
        self.instance_buffer.fill_with_dynamic_f32(positions);
    }

    pub fn render(&self, transformation: &Mat4, camera: &camera::Camera) -> Result<(), Error>
    {
        self.program.add_uniform_float("diffuse_intensity", &self.material.diffuse_intensity)?;
        self.program.add_uniform_float("specular_intensity", &self.material.specular_intensity)?;
        self.program.add_uniform_float("specular_power", &self.material.specular_power)?;

        let color = if let ColorSource::Color(c) = self.material.color_source {c} else {vec4(1.0, 0.0, 0.0, 1.0)};
        self.program.add_uniform_vec4("color", &color)?;

        self.program.add_uniform_float("scale", &self.ball_radius)?;
        self.program.add_uniform_mat4("modelMatrix", &transformation)?;
        self.program.use_uniform_block(camera.matrix_buffer(), "Camera");

        self.program.use_attribute_vec3_float_divisor(&self.instance_buffer, "translation", 1)?;

        self.program.use_attribute_vec3_float(&self.ball_vertex_buffer, "position")?;

        self.program.draw_elements_instanced(&self.ball_index_buffer, self.no_vertices);
        Ok(())
    }
}
