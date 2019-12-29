use core::*;


pub struct Imposter {
    gl: Gl,
    program: program::Program,
    vertex_buffer: StaticVertexBuffer,
    rendertarget: ColorRendertargetArray
}

impl Imposter {
    pub fn new(gl: &Gl, render: &dyn Fn(&Camera)) -> Self
    {
        let mut camera = camera::Camera::new_orthographic(vec3(0.0, 8.0, -5.0),
                          vec3(0.0, 8.0, 0.0), vec3(0.0, 1.0, 0.0), 20.0, 20.0, 20.0);
        camera.enable_matrix_buffer(gl);
        let rendertarget = ColorRendertargetArray::new(gl, 1024, 1024, 2, false).unwrap();
        rendertarget.bind();
        rendertarget.clear(&vec4(0.0, 0.0, 0.0, 0.0));
        render(&camera);

        let positions = vec![
            -3.0, -1.0, 0.0,
            3.0, -1.0, 0.0,
            0.0, 2.0, 0.0
        ];
        let uvs = vec![
            -1.0, 0.0,
            2.0, 0.0,
            0.5, 1.5
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
        self.program.draw_arrays(3);
    }
}