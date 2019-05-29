
use dust::*;

const SIZE: f32 = 128.0;
const VERTICES_PER_UNIT: usize = 2;
const VERTICES_PER_SIDE: usize = SIZE as usize * VERTICES_PER_UNIT;
const VERTICES_IN_TOTAL: usize = VERTICES_PER_SIDE * VERTICES_PER_SIDE;
const VERTEX_DISTANCE: f32 = 1.0 / VERTICES_PER_UNIT as f32;

pub struct Water {
    program: program::Program,
    index_buffer: buffer::ElementBuffer,
    foam_texture: texture::Texture2D,
    buffer: buffer::DynamicVertexBuffer,
    center: Vec3
}

impl Water
{
    pub fn new(gl: &Gl) -> Water
    {
        let program = program::Program::from_source(gl, include_str!("../assets/shaders/water.vert"),
                                                      include_str!("../assets/shaders/water.frag")).unwrap();
        let index_buffer = buffer::ElementBuffer::new_with(gl, &indices()).unwrap();
        let mut buffer = DynamicVertexBuffer::new(gl).unwrap();
        buffer.add(&vec![0.0; 3 * VERTICES_IN_TOTAL], 3);
        buffer.add(&vec![0.0; 2 * VERTICES_IN_TOTAL], 2);
        buffer.send_data();

        let foam_texture = texture::Texture2D::new_from_bytes(&gl, include_bytes!("../assets/textures/grass.jpg")).unwrap();

        let mut water = Water { program, index_buffer, foam_texture, buffer, center: vec3(0.0, 0.0, 0.0)};
        water.set_center(&vec3(0.0, 0.0, 0.0));
        water
    }

    pub fn render(&self, time: f32, camera: &camera::Camera, screen_width: usize, screen_height: usize, color_texture: &Texture, position_texture: &Texture, skybox_texture: &Texture)
    {
        self.program.blend(state::BlendType::SRC_ALPHA__ONE_MINUS_SRC_ALPHA);
        self.program.cull(state::CullType::NONE);
        self.program.depth_write(false);
        self.program.depth_test(state::DepthTestType::LEQUAL);

        self.program.add_uniform_mat4("modelMatrix", &Mat4::identity()).unwrap();
        self.program.add_uniform_mat4("viewMatrix", camera.get_view()).unwrap();
        self.program.add_uniform_mat4("projectionMatrix", camera.get_projection()).unwrap();

        self.program.add_uniform_vec3("eyePosition", camera.position()).unwrap();
        self.program.add_uniform_vec2("screenSize", &vec2(screen_width as f32, screen_height as f32)).unwrap();

        self.program.add_uniform_float("time", &(time * 0.001)).unwrap();

        color_texture.bind(0);
        self.program.add_uniform_int("colorMap", &0).unwrap();

        position_texture.bind(1);
        self.program.add_uniform_int("positionMap", &1).unwrap();

        skybox_texture.bind(2);
        self.program.add_uniform_int("environmentMap", &2).unwrap();

        self.program.use_attribute_vec3_float(&self.buffer, "position", 0).unwrap();
        self.program.use_attribute_vec2_float(&self.buffer, "uv_coordinate", 1).unwrap();
        self.program.draw_elements(&self.index_buffer);
    }

    pub fn set_center(&mut self, center: &Vec3)
    {
        self.center = vec3(center.x.floor(), 0.0, center.z.floor());

        self.update_positions();
        self.update_uv_coordinates();
    }

    fn update_positions(&mut self)
    {
        let mut data = vec![0.0; 3 * VERTICES_IN_TOTAL];
        for r in 0..VERTICES_PER_SIDE
        {
            for c in 0..VERTICES_PER_SIDE
            {
                let vertex_id = r*VERTICES_PER_SIDE + c;
                data[vertex_id * 3] = self.center.x - SIZE/2.0 + r as f32 * VERTEX_DISTANCE;
                data[vertex_id * 3 + 2] = self.center.z - SIZE/2.0 + c as f32 * VERTEX_DISTANCE;
            }
        }
        self.buffer.update_data_at(0, &data);
    }

    fn update_uv_coordinates(&mut self)
    {
        let mut data = vec![0.0; 2 * VERTICES_IN_TOTAL];
        let scale = 0.1;
        for r in 0..VERTICES_PER_SIDE
        {
            for c in 0..VERTICES_PER_SIDE
            {
                let vertex_id = r*VERTICES_PER_SIDE + c;
                data[vertex_id * 2] = scale * (self.center.x + r as f32 * VERTEX_DISTANCE);
                data[vertex_id * 2 + 1] = scale * (self.center.z + c as f32 * VERTEX_DISTANCE);
            }
        }
        self.buffer.update_data_at(1, &data);
    }
}

fn indices() -> Vec<u32>
{
    let mut indices: Vec<u32> = Vec::new();
    let stride = VERTICES_PER_SIDE as u32;
    for r in 0..stride-1
    {
        for c in 0..stride-1
        {
            indices.push(r + c * stride);
            indices.push(r + 1 + c * stride);
            indices.push(r + (c + 1) * stride);
            indices.push(r + (c + 1) * stride);
            indices.push(r + 1 + c * stride);
            indices.push(r + 1 + (c + 1) * stride);

        }
    }
    indices
}