extern crate image;
extern crate noise;

use dust::core::program;
use gl;
use dust::traits;
use gust;
use gust::ids::*;
use gust::mesh::Mesh;
use dust::*;
use dust::core::*;
use dust::core::texture::Texture;
use glm::*;
use self::image::{GenericImage};
use self::noise::{NoiseFn, Point2, SuperSimplex};
use num_traits::identities::One;

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
    center: Vec3,
    mesh: Mesh
}

impl Terrain
{
    pub fn create(gl: &gl::Gl) -> Result<Terrain, traits::Error>
    {
        let noise_generator = Box::new(SuperSimplex::new());

        let mut mesh = gust::mesh::Mesh::create_indexed(indices(), vec![0.0;3 * VERTICES_IN_TOTAL])?;
        mesh.add_custom_vec3_attribute("normal", vec![0.0;3 * VERTICES_IN_TOTAL])?;
        mesh.add_custom_vec2_attribute("uv_coordinate", vec![0.0;2 * VERTICES_IN_TOTAL])?;

        let program = program::Program::from_resource(gl, "examples/assets/shaders/terrain")?;
        let mut model = surface::TriangleSurface::create_without_adding_attributes(gl, &mesh)?;
        let buffer = model.add_attributes(&mesh, &program, &vec!["position", "normal"], &vec!["uv_coordinate"])?;

        let ground_texture = texture_from_img(gl,"examples/assets/textures/grass.jpg")?;
        let lake_texture = texture_from_img(gl,"examples/assets/textures/bottom.png")?;
        let noise_texture = texture_from_img(gl,"examples/assets/textures/grass.jpg")?;

        let mut terrain = Terrain { program, model, ground_texture, lake_texture, noise_texture, buffer, center: vec3(0.0, 0.0, 0.0), noise_generator, mesh};
        terrain.set_center(&vec3(0.0, 0.0, 0.0));
        Ok(terrain)
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

        let transformation = Matrix4::one();
        self.program.add_uniform_mat4("modelMatrix", &transformation)?;
        self.program.add_uniform_mat4("viewMatrix", &camera.get_view())?;
        self.program.add_uniform_mat4("projectionMatrix", &camera.get_projection())?;
        self.program.add_uniform_mat4("normalMatrix", &transpose(&inverse(&transformation)))?;

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
        self.update_heights();
        self.update_uv_coordinates();
        self.mesh.update_normals().unwrap();

        self.buffer.fill_from_attributes(&self.mesh, &vec!["uv_coordinate"], &vec!["position", "normal"]);
    }

    fn update_heights(&mut self)
    {
        for r in 0..VERTICES_PER_SIDE
        {
            for c in 0..VERTICES_PER_SIDE
            {
                let x = self.center.x - SIZE/2.0 + r as f32 * VERTEX_DISTANCE;
                let z = self.center.z - SIZE/2.0 + c as f32 * VERTEX_DISTANCE;
                let y = get_height_at(&self.noise_generator, x, z);
                self.mesh.positions.set(&VertexID::new(r*VERTICES_PER_SIDE + c), &vec3(x, y, z));
            }
        }
    }

    fn update_uv_coordinates(&mut self)
    {
        let scale = 0.1;
        for r in 0..VERTICES_PER_SIDE
        {
            for c in 0..VERTICES_PER_SIDE
            {
                let x = self.center.x + r as f32 * VERTEX_DISTANCE;
                let z = self.center.z + c as f32 * VERTEX_DISTANCE;
                self.mesh.set_vec2_attribute_at("uv_coordinate", &VertexID::new(r*VERTICES_PER_SIDE + c), &vec2(scale * x, scale * z)).unwrap();
            }
        }

    }

    pub fn get_height_at(&self, x: f32, z: f32) -> f32
    {
        get_height_at(&self.noise_generator, x, z)
    }
}

fn texture_from_img(gl: &gl::Gl, name: &str) -> Result<texture::Texture2D, traits::Error>
{
    let img = image::open(name).unwrap();
    let mut texture = texture::Texture2D::create(gl)?;
    texture.fill_with_u8(img.dimensions().0 as usize, img.dimensions().1 as usize, &img.raw_pixels());
    Ok(texture)
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