use core::*;
use std::f32::consts::PI;

const NO_VIEW_ANGLES: usize  = 8;

pub struct Imposter {
    gl: Gl,
    program: program::Program,
    vertex_buffer: StaticVertexBuffer,
    instance_buffer: DynamicVertexBuffer,
    instance_count: u32,
    texture: Texture2DArray
}

impl Imposter {
    pub fn new(gl: &Gl, render: &dyn Fn(&Camera), aabb: (Vec3, Vec3)) -> Self
    {
        let (min, max) = aabb;
        let width = f32::sqrt(f32::powi(max.x - min.x, 2) + f32::powi(max.z - min.z, 2));
        let height = max.y - min.y;
        let center = 0.5 * min + 0.5 * max;
        let mut camera = camera::Camera::new_orthographic(center + vec3(0.0, 0.0, -1.0),
                          center, vec3(0.0, 1.0, 0.0), width, height, 4.0*(width+height));
        camera.enable_matrix_buffer(gl);

        let texture = Texture2DArray::new_as_color_targets(gl, 1024, 1024, NO_VIEW_ANGLES*2).unwrap();
        let depth_texture = Texture2D::new_as_depth_target(gl, 1024, 1024).unwrap();
        let rendertarget = RenderTarget::new(gl, 2).unwrap();

        for i in 0..NO_VIEW_ANGLES {
            let angle = i as f32 * 2.0 * PI / NO_VIEW_ANGLES as f32;
            camera.set_view(center + width * vec3(f32::sin(-angle), 0.0, f32::cos(-angle)),
                            center, vec3(0.0, 1.0, 0.0));
            rendertarget.write_to_color_array_and_depth(&texture, &depth_texture, &|channel| { i + channel * NO_VIEW_ANGLES }).unwrap();
            rendertarget.clear_color_and_depth(&vec4(0.0, 0.0, 0.0, 0.0));
            render(&camera);
        }

        let xmin = center.x - 0.5 * width;
        let xmax = center.x + 0.5 * width;
        let ymin = min.y;
        let ymax = max.y;
        let positions = vec![
            xmin, ymin, 0.0,
            xmax, ymin, 0.0,
            xmax, ymax, 0.0,
            xmax, ymax, 0.0,
            xmin, ymax, 0.0,
            xmin, ymin, 0.0
        ];
        let uvs = vec![
            0.0, 0.0,
            1.0, 0.0,
            1.0, 1.0,
            1.0, 1.0,
            0.0, 1.0,
            0.0, 0.0
        ];

        let program = program::Program::from_source(gl,
                                                    include_str!("shaders/billboard.vert"),
                                                    include_str!("shaders/sprite.frag")).unwrap();

        let vertex_buffer = StaticVertexBuffer::new_with_vec3_vec2(&gl, &positions, &uvs).unwrap();
        let mut instance_buffer = DynamicVertexBuffer::new(gl).unwrap();
        instance_buffer.add(&[], 3);
        instance_buffer.send_data();

        Imposter {gl: gl.clone(), texture, program, vertex_buffer, instance_buffer, instance_count:0 }
    }

    pub fn update_positions(&mut self, positions: &[f32])
    {
        self.instance_buffer.update_data_at(0, positions);
        self.instance_buffer.send_data();
        self.instance_count = positions.len() as u32/3;
    }

    pub fn render(&self, camera: &camera::Camera) {

        state::depth_write(&self.gl, true);
        state::depth_test(&self.gl, state::DepthTestType::LEQUAL);
        state::cull(&self.gl,state::CullType::BACK);
        state::blend(&self.gl, state::BlendType::NONE);

        self.program.add_uniform_int("no_views", &(NO_VIEW_ANGLES as i32)).unwrap();
        self.program.use_uniform_block(camera.matrix_buffer(), "Camera");

        self.program.use_texture(&self.texture, "tex").unwrap();

        self.program.use_attribute_vec3_float(&self.vertex_buffer, "position", 0).unwrap();
        self.program.use_attribute_vec2_float(&self.vertex_buffer, "uv_coordinate", 1).unwrap();

        self.program.use_attribute_vec3_float_divisor(&self.instance_buffer, "center", 0, 1).unwrap();
        self.program.draw_arrays_instanced(6, self.instance_count);
    }
}