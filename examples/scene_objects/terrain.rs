
use noise::{NoiseFn, Point2, SuperSimplex};
use dust::*;

pub const SIZE: f32 = 64.0;
const VERTICES_PER_UNIT: usize = 8;
const VERTICES_PER_SIDE: usize = SIZE as usize * VERTICES_PER_UNIT;
const VERTICES_IN_TOTAL: usize = VERTICES_PER_SIDE * VERTICES_PER_SIDE;
const VERTEX_DISTANCE: f32 = 1.0 / VERTICES_PER_UNIT as f32;

pub struct Terrain {
    program: program::Program,
    model: surface::TriangleSurface,
    ground_texture: texture::Texture2D,
    lake_texture: texture::Texture2D,
    noise_texture: texture::Texture2D,
    noise_generator: Box<NoiseFn<Point2<f64>>>,
    buffer: buffer::VertexBuffer,
    center: Vec3
}

impl Terrain
{
    pub fn create(gl: &gl::Gl) -> Terrain
    {
        let noise_generator = Box::new(SuperSimplex::new());
        let mesh = mesh::StaticMesh::create(indices(), att!["position" => (vec![0.0;3 * VERTICES_IN_TOTAL], 3),
                                                      "normal" => (vec![0.0;3 * VERTICES_IN_TOTAL], 3),
                                                      "uv_coordinate" => (vec![0.0;2 * VERTICES_IN_TOTAL], 2)]).unwrap();

        let program = program::Program::from_resource(gl, "examples/assets/shaders/terrain",
                                                      "examples/assets/shaders/terrain").unwrap();
        let mut model = surface::TriangleSurface::create(gl, &mesh).unwrap();
        let buffer = model.add_attributes(&mesh, &program,&vec!["uv_coordinate", "position", "normal"]).unwrap();

        let ground_texture = texture::Texture2D::new_from_file(gl,"examples/assets/textures/grass.jpg").unwrap();
        let lake_texture = texture::Texture2D::new_from_file(gl,"examples/assets/textures/bottom.png").unwrap();
        let noise_texture = texture::Texture2D::new_from_file(gl,"examples/assets/textures/grass.jpg").unwrap();

        let mut terrain = Terrain { program, model, ground_texture, lake_texture, noise_texture, buffer, center: vec3(0.0, 0.0, 0.0), noise_generator};
        terrain.set_center(&vec3(0.0, 0.0, 0.0));
        terrain
    }

    pub fn render(&self, camera: &camera::Camera) -> Result<(), traits::Error>
    {
        self.program.cull(state::CullType::BACK);
        self.program.depth_write(true);
        self.program.depth_test(state::DepthTestType::LEQUAL);

        self.ground_texture.bind(0);
        self.program.add_uniform_int("groundTexture", &0)?;

        self.lake_texture.bind(1);
        self.program.add_uniform_int("lakeTexture", &1)?;

        self.noise_texture.bind(2);
        self.program.add_uniform_int("noiseTexture", &2)?;

        let transformation = Mat4::identity();
        self.program.add_uniform_mat4("modelMatrix", &transformation)?;
        self.program.add_uniform_mat4("viewMatrix", camera.get_view())?;
        self.program.add_uniform_mat4("projectionMatrix", camera.get_projection())?;
        self.program.add_uniform_mat4("normalMatrix", &transformation.try_inverse().unwrap().transpose())?;

        self.model.render()?;

        Ok(())
    }

    pub fn get_center(&self) -> &Vec3
    {
        &self.center
    }

    pub fn set_center(&mut self, center: &Vec3)
    {
        self.center = vec3(center.x.floor(), 0.0, center.z.floor());
        const STRIDE: usize = 8;
        let mut data = vec![0.0; STRIDE * VERTICES_IN_TOTAL];

        self.update_positions(&mut data, 2, STRIDE);
        self.update_normals(&mut data, 5, STRIDE);
        self.update_uv_coordinates(&mut data, 0, STRIDE);

        self.buffer.fill_with(data);
    }

    fn update_positions(&mut self, data: &mut Vec<f32>, offset: usize, stride: usize)
    {
        for r in 0..VERTICES_PER_SIDE
        {
            for c in 0..VERTICES_PER_SIDE
            {
                let vertex_id = r*VERTICES_PER_SIDE + c;
                let x = self.center.x - SIZE/2.0 + r as f32 * VERTEX_DISTANCE;
                let z = self.center.z - SIZE/2.0 + c as f32 * VERTEX_DISTANCE;
                data[offset + vertex_id * stride] = x;
                data[offset + vertex_id * stride + 1] = self.get_height_at(x, z);
                data[offset + vertex_id * stride + 2] = z;
            }
        }
    }

    fn update_uv_coordinates(&mut self, data: &mut Vec<f32>, offset: usize, stride: usize)
    {
        let scale = 0.1;
        for r in 0..VERTICES_PER_SIDE
        {
            for c in 0..VERTICES_PER_SIDE
            {
                let vertex_id = r*VERTICES_PER_SIDE + c;
                data[offset + vertex_id * stride] = scale * (self.center.x + r as f32 * VERTEX_DISTANCE);
                data[offset + vertex_id * stride + 1] = scale * (self.center.z + c as f32 * VERTEX_DISTANCE);
            }
        }
    }

    fn update_normals(&mut self, data: &mut Vec<f32>, offset: usize, stride: usize)
    {
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
                data[offset + vertex_id * stride] = normal.x;
                data[offset + vertex_id * stride + 1] = normal.y;
                data[offset + vertex_id * stride + 2] = normal.z;
            }
        }
    }

    pub fn get_height_at(&self, x: f32, z: f32) -> f32
    {
        get_height_at(&self.noise_generator, x, z)
    }
}

fn get_height_at(noise_generator: &Box<NoiseFn<Point2<f64>>>, x: f32, z: f32) -> f32
{
    (noise_generator.get([x as f64 * 0.1, z as f64 * 0.1]) +
            0.25 * noise_generator.get([x as f64 * 0.5, z as f64 * 0.5]) +
            2.0 * noise_generator.get([x as f64 * 0.02, z as f64 * 0.02])) as f32
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