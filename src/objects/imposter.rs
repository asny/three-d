use core::*;
use std::f32::consts::PI;


pub struct Imposter {
    gl: Gl,
    program: program::Program,
    vertex_buffer: StaticVertexBuffer,
    rendertarget: ColorRendertargetArray
}

impl Imposter {
    pub fn new(gl: &Gl, render: &dyn Fn(&Camera), aabb: (Vec3, Vec3)) -> Self
    {
        let no_views = 4;
        let (min, max) = aabb;
        let width = f32::sqrt(f32::powi(max.x - min.x, 2) + f32::powi(max.z - min.z, 2));
        let height = max.y - min.y;
        let center = 0.5 * min + 0.5 * max;
        let mut camera = camera::Camera::new_orthographic(center + vec3(0.0, 0.0, -1.0),
                          center, vec3(0.0, 1.0, 0.0), width, height, width+1.0);
        camera.enable_matrix_buffer(gl);
        let rendertarget = ColorRendertargetArray::new(gl, 1024, 1024, no_views*2, false).unwrap();
        rendertarget.bind();
        rendertarget.clear(&vec4(0.0, 0.0, 0.0, 0.0));

        for i in 0..no_views {
            let angle = i as f32 * 2.0 * PI / no_views as f32;
            camera.set_view(center + vec3(f32::sin(angle), 0.0, f32::cos(angle)),
                            center, vec3(0.0, 1.0, 0.0));

            rendertarget.targets.bind_to_framebuffer(i, 0);
            rendertarget.targets.bind_to_framebuffer(no_views + i, 1);
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

        Imposter {gl: gl.clone(), rendertarget, program, vertex_buffer }
    }

    pub fn render(&self, transformation: &Mat4, camera: &camera::Camera) {

        state::depth_write(&self.gl, true);
        state::depth_test(&self.gl, state::DepthTestType::LEQUAL);
        state::cull(&self.gl,state::CullType::BACK);
        state::blend(&self.gl, state::BlendType::NONE);

        self.program.add_uniform_mat4("modelMatrix", transformation).unwrap();
        self.program.use_uniform_block(camera.matrix_buffer(), "Camera");

        self.program.use_texture(&self.rendertarget.targets, "tex").unwrap();

        self.program.use_attribute_vec3_float(&self.vertex_buffer, "position", 0).unwrap();
        self.program.use_attribute_vec2_float(&self.vertex_buffer, "uv_coordinate", 1).unwrap();
        self.program.draw_arrays(6);
    }
}