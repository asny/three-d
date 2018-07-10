extern crate image;
extern crate noise;

use dust::core::program;
use gl;
use dust::traits;
use gust;
use gust::mesh::Mesh;
use dust::core::texture;
use dust::core::texture::Texture;
use dust::core::surface;
use dust::core::buffer;
use glm::*;
use dust::camera;
use dust::core::state;
use self::image::{GenericImage};
use self::noise::{NoiseFn, Point2, SuperSimplex};

const SIZE: f32 = 64.0;
const VERTICES_PER_UNIT: usize = 2;
const VERTICES_PER_SIDE: usize = SIZE as usize * VERTICES_PER_UNIT;
const VERTEX_DISTANCE: f32 = 1.0 / VERTICES_PER_UNIT as f32;

pub struct Terrain {
    terrain_program: program::Program,
    model: surface::TriangleSurface,
    ground_texture: texture::Texture2D,
    lake_texture: texture::Texture2D,
    noise_texture: texture::Texture2D,
    noise_generator: Box<NoiseFn<Point2<f64>>>,
    buffer: buffer::VertexBuffer,
    center: Vec3,
    mesh: Mesh
}

impl traits::Reflecting for Terrain
{
    fn reflect(&self, transformation: &Mat4, camera: &camera::Camera) -> Result<(), traits::Error>
    {
        self.terrain_program.cull(state::CullType::BACK);

        self.ground_texture.bind(0);
        self.terrain_program.add_uniform_int("groundTexture", &0)?;

        self.lake_texture.bind(1);
        self.terrain_program.add_uniform_int("lakeTexture", &1)?;

        self.noise_texture.bind(2);
        self.terrain_program.add_uniform_int("noiseTexture", &2)?;

        self.terrain_program.add_uniform_mat4("modelMatrix", &transformation)?;
        self.terrain_program.add_uniform_mat4("viewMatrix", &camera.get_view())?;
        self.terrain_program.add_uniform_mat4("projectionMatrix", &camera.get_projection())?;
        self.terrain_program.add_uniform_mat4("normalMatrix", &transpose(&inverse(transformation)))?;

        self.model.render()?;
        Ok(())
    }

}

impl Terrain
{
    pub fn create(gl: &gl::Gl) -> Result<Terrain, traits::Error>
    {
        let noise_generator = Box::new(SuperSimplex::new());

        let mut mesh = gust::mesh::Mesh::create_indexed(indices(), positions())?;
        mesh.add_custom_vec3_attribute("normal", normals())?;
        mesh.add_custom_vec2_attribute("uv_coordinate", uv_coordinates())?;

        let terrain_program = program::Program::from_resource(gl, "examples/assets/shaders/terrain")?;
        let water_program = program::Program::from_resource(gl, "examples/assets/shaders/water")?;
        let mut model = surface::TriangleSurface::create_without_adding_attributes(gl, &mesh)?;
        let buffer = model.add_attributes(&vec![&mesh.positions, mesh.get("normal")?, mesh.get("uv_coordinate")?], &terrain_program)?;

        let ground_texture = texture_from_img(gl,"examples/assets/textures/grass.jpg")?;
        let lake_texture = texture_from_img(gl,"examples/assets/textures/bottom.png")?;
        let noise_texture = texture_from_img(gl,"examples/assets/textures/grass.jpg")?;

        let mut terrain = Terrain { terrain_program, model, ground_texture, lake_texture, noise_texture, buffer, center: vec3(0.0, 0.0, 0.0), noise_generator, mesh};
        terrain.set_center(&vec3(0.0, 0.0, 0.0));
        Ok(terrain)
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
        self.mesh.compute_normals();

        self.buffer.fill_from(&vec![&self.mesh.positions, self.mesh.get("normal").unwrap(), self.mesh.get("uv_coordinate").unwrap()]);
    }

    fn update_heights(&mut self)
    {
        let positions = self.mesh.positions.data_mut();
        for r in 0..VERTICES_PER_SIDE+1
        {
            for c in 0..VERTICES_PER_SIDE+1
            {
                let x = self.center.x - SIZE/2.0 + r as f32 * VERTEX_DISTANCE;
                let z = self.center.z - SIZE/2.0 + c as f32 * VERTEX_DISTANCE;
                let y = get_height_at(&self.noise_generator, x, z);
                positions[3 * (r*(VERTICES_PER_SIDE+1) + c)] = x;
                positions[3 * (r*(VERTICES_PER_SIDE+1) + c) + 1] = y;
                positions[3 * (r*(VERTICES_PER_SIDE+1) + c) + 2] = z;
            }
        }
    }

    fn update_uv_coordinates(&mut self)
    {
        let uvs = self.mesh.get_mut("uv_coordinate").unwrap().data_mut();
        let scale = 0.1;
        for r in 0..VERTICES_PER_SIDE+1
        {
            for c in 0..VERTICES_PER_SIDE+1
            {
                let x = self.center.x + r as f32 * VERTEX_DISTANCE;
                let z = self.center.z + c as f32 * VERTEX_DISTANCE;
                uvs[2 * (r*(VERTICES_PER_SIDE+1) + c)] = scale * x;
                uvs[2 * (r*(VERTICES_PER_SIDE+1) + c) + 1] = scale * z;
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
    let stride = VERTICES_PER_SIDE as u32 + 1;
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

fn positions() -> Vec<f32>
{
    vec![0.0;3 * (VERTICES_PER_SIDE + 1) * (VERTICES_PER_SIDE + 1)]
}

fn normals() -> Vec<f32>
{
    vec![0.0;3 * (VERTICES_PER_SIDE + 1) * (VERTICES_PER_SIDE + 1)]
}

fn uv_coordinates() -> Vec<f32>
{
    vec![0.0;2 * (VERTICES_PER_SIDE + 1) * (VERTICES_PER_SIDE + 1)]
}