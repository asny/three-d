
use dust::*;
use noise::{NoiseFn, Point2, SuperSimplex};

pub const SIZE: f32 = 128.0;
const VERTICES_PER_UNIT: usize = 8;
const VERTICES_PER_SIDE: usize = SIZE as usize * VERTICES_PER_UNIT;
const VERTICES_IN_TOTAL: usize = VERTICES_PER_SIDE * VERTICES_PER_SIDE;
const VERTEX_DISTANCE: f32 = 1.0 / VERTICES_PER_UNIT as f32;

pub struct Terrain {
    program: program::Program,
    ground_texture: texture::Texture2D,
    lake_texture: texture::Texture2D,
    noise_texture: texture::Texture2D,
    noise_generator: Box<NoiseFn<Point2<f64>>>,
    buffer: DynamicVertexBuffer,
    index_buffer: ElementBuffer,
    center: Vec3
}

impl Terrain
{
    pub fn new(gl: &Gl) -> Terrain
    {
        let noise_generator = Box::new(SuperSimplex::new());
        let program = program::Program::from_source(gl, include_str!("../assets/shaders/terrain.vert"),
                                                      include_str!("../assets/shaders/terrain.frag")).unwrap();
        let index_buffer = buffer::ElementBuffer::new_with(gl, &indices()).unwrap();
        let mut buffer = DynamicVertexBuffer::new(gl).unwrap();
        buffer.add(&vec![0.0; 3 * VERTICES_IN_TOTAL], 3);
        buffer.add(&vec![0.0; 3 * VERTICES_IN_TOTAL], 3);
        buffer.add(&vec![0.0; 2 * VERTICES_IN_TOTAL], 2);
        buffer.send_data();

        let ground_texture = texture::Texture2D::new_from_bytes(&gl, include_bytes!("../assets/textures/grass.jpg")).unwrap();
        let lake_texture = texture::Texture2D::new_from_bytes(&gl, include_bytes!("../assets/textures/bottom.png")).unwrap();

        let mut terrain = Terrain { program, ground_texture, lake_texture, noise_texture: new_noise_texture(gl), buffer, index_buffer, center: vec3(0.0, 0.0, 0.0), noise_generator};
        terrain.set_center(&vec3(0.0, 0.0, 0.0));
        terrain
    }

    pub fn render(&self, camera: &camera::Camera)
    {
        self.program.cull(state::CullType::BACK);
        self.program.depth_write(true);
        self.program.depth_test(state::DepthTestType::LEQUAL);

        self.ground_texture.bind(0);
        self.program.add_uniform_int("groundTexture", &0).unwrap();

        self.lake_texture.bind(1);
        self.program.add_uniform_int("lakeTexture", &1).unwrap();

        self.noise_texture.bind(2);
        self.program.add_uniform_int("noiseTexture", &2).unwrap();

        let transformation = Mat4::identity();
        self.program.add_uniform_mat4("modelMatrix", &transformation).unwrap();
        self.program.add_uniform_mat4("viewMatrix", camera.get_view()).unwrap();
        self.program.add_uniform_mat4("projectionMatrix", camera.get_projection()).unwrap();
        self.program.add_uniform_mat4("normalMatrix", &transformation.invert().unwrap().transpose()).unwrap();

        self.program.use_attribute_vec3_float(&self.buffer, "position", 0).unwrap();
        self.program.use_attribute_vec3_float(&self.buffer, "normal", 1).unwrap();
        self.program.use_attribute_vec2_float(&self.buffer, "uv_coordinate", 2).unwrap();
        self.program.draw_elements(&self.index_buffer);
    }

    pub fn get_center(&self) -> &Vec3
    {
        &self.center
    }

    pub fn set_center(&mut self, center: &Vec3)
    {
        self.center = vec3(center.x.floor(), 0.0, center.z.floor());

        self.update_positions();
        self.update_normals();
        self.update_uv_coordinates();

        self.buffer.send_data();

    }

    fn update_positions(&mut self)
    {
        let mut data = vec![0.0; 3 * VERTICES_IN_TOTAL];
        for r in 0..VERTICES_PER_SIDE
        {
            for c in 0..VERTICES_PER_SIDE
            {
                let vertex_id = r*VERTICES_PER_SIDE + c;
                let x = self.center.x - SIZE/2.0 + r as f32 * VERTEX_DISTANCE;
                let z = self.center.z - SIZE/2.0 + c as f32 * VERTEX_DISTANCE;
                data[vertex_id * 3] = x;
                data[vertex_id * 3 + 1] = self.get_height_at(x, z);
                data[vertex_id * 3 + 2] = z;
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
        self.buffer.update_data_at(2, &data);
    }

    fn update_normals(&mut self)
    {
        let mut data = vec![0.0; 3 * VERTICES_IN_TOTAL];
        let h = VERTEX_DISTANCE;
        for r in 0..VERTICES_PER_SIDE
        {
            for c in 0..VERTICES_PER_SIDE
            {
                let vertex_id = r*VERTICES_PER_SIDE + c;
                let x = self.center.x - SIZE/2.0 + r as f32 * VERTEX_DISTANCE;
                let z = self.center.z - SIZE/2.0 + c as f32 * VERTEX_DISTANCE;
                let dx = self.get_height_at(x + 0.5 * h, z) - self.get_height_at(x - 0.5 * h, z);
                let dz = self.get_height_at(x, z + 0.5 * h) - self.get_height_at(x, z - 0.5 * h);
                let normal = vec3(-dx, h, -dz).normalize();
                data[vertex_id * 3] = normal.x;
                data[vertex_id * 3 + 1] = normal.y;
                data[vertex_id * 3 + 2] = normal.z;
            }
        }
        self.buffer.update_data_at(1, &data);
    }

    pub fn get_height_at(&self, x: f32, z: f32) -> f32
    {

        (self.noise_generator.get([x as f64 * 0.1, z as f64 * 0.1]) +
                0.25 * self.noise_generator.get([x as f64 * 0.5, z as f64 * 0.5]) +
                2.0 * self.noise_generator.get([x as f64 * 0.02, z as f64 * 0.02])) as f32
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

fn new_noise_texture(gl: &Gl) -> texture::Texture2D
{
    use rand::prelude::*;
    let noise_size = 128;
    let mut noise: Vec<f32> = Vec::with_capacity(noise_size * noise_size);
    for _ in 0..noise_size * noise_size
    {
        noise.push(random::<f32>());
    }
    let mut noise_texture = texture::Texture2D::new(gl).unwrap();
    noise_texture.fill_with_f32(noise_size, noise_size, &noise);
    noise_texture
}