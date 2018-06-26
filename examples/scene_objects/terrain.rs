extern crate image;
extern crate noise;

use dust::core::program;
use gl;
use dust::traits;
use gust;
use gust::mesh::Mesh;
use std::rc::Rc;
use dust::core::texture;
use dust::core::texture::Texture;
use dust::core::surface;
use glm;
use dust::camera;
use dust::core::state;
use self::image::{GenericImage};
use self::noise::{NoiseFn, Point2, SuperSimplex};

const SIZE: f32 = 16.0;
const VERTICES_PER_UNIT: usize = 8;
const VERTICES_PER_SIDE: usize = SIZE as usize * VERTICES_PER_UNIT;
const VERTEX_DISTANCE: f32 = 1.0 / VERTICES_PER_UNIT as f32;

const PATCH_RADIUS: usize = 1;
const PATCH_SIDE_LENGTH: usize = 2 * PATCH_RADIUS + 1;
const PATCH_VERTICES_PER_SIDE: usize = VERTICES_PER_SIDE * PATCH_SIDE_LENGTH;
const PATCH_SIZE: f32 = SIZE * PATCH_SIDE_LENGTH as f32;

pub struct Terrain {
    program: program::Program,
    model: surface::TriangleSurface,
    texture: texture::Texture2D
}

impl traits::Reflecting for Terrain
{
    fn reflect(&self, transformation: &glm::Mat4, camera: &camera::Camera) -> Result<(), traits::Error>
    {
        self.program.cull(state::CullType::BACK);

        self.texture.bind(0);
        self.program.add_uniform_int("texture0", &0)?;
        self.program.add_uniform_mat4("modelMatrix", &transformation)?;
        self.program.add_uniform_mat4("viewMatrix", &camera.get_view())?;
        self.program.add_uniform_mat4("projectionMatrix", &camera.get_projection())?;
        self.program.add_uniform_mat4("normalMatrix", &glm::transpose(&glm::inverse(transformation)))?;

        self.model.render()?;
        Ok(())
    }

}

impl Terrain
{
    pub fn create(gl: &gl::Gl) -> Result<Rc<traits::Reflecting>, traits::Error>
    {
        let mut heightmap = Heightmap::create();
        heightmap.initialize(glm::vec3(-SIZE/2.0, 0.0, -SIZE/2.0));

        let mut mesh = gust::mesh::Mesh::create_indexed(Heightmap::indices(), heightmap.positions().clone())?;
        mesh.add_custom_vec3_attribute("normal", heightmap.normals())?;
        mesh.add_custom_vec2_attribute("uv_coordinate", Heightmap::uv_coordinates())?;

        let program = program::Program::from_resource(gl, "examples/assets/shaders/texture")?;
        let model = surface::TriangleSurface::create(gl, &mesh, &program)?;

        let img = image::open("examples/assets/textures/grass.jpg").unwrap();
        let mut texture = texture::Texture2D::create(gl)?;

        texture.fill_with_u8(img.dimensions().0 as usize, img.dimensions().1 as usize, &img.raw_pixels());

        Ok(Rc::new(Terrain { program, model, texture }))
    }
}

struct Heightmap
{
    origo: glm::Vec3,
    positions: Vec<glm::Vec3>,
    noise_generator: Box<NoiseFn<Point2<f64>>>
}

impl Heightmap
{
    pub fn create() -> Heightmap
    {
        let positions = vec![glm::vec3(0.0, 0.0, 0.0);(VERTICES_PER_SIDE + 1) * (VERTICES_PER_SIDE + 1)];
        let noise_generator = Box::new(SuperSimplex::new());
        Heightmap {origo: glm::vec3(0.0,0.0,0.0), positions, noise_generator}
    }

    pub fn initialize(&mut self, _origo: glm::Vec3)
    {
        self.origo = _origo;

        self.set_height(SIZE, 0, 0, vec![]);
        self.set_height(SIZE, 0, VERTICES_PER_SIDE, vec![]);
        self.set_height(SIZE, VERTICES_PER_SIDE, 0, vec![]);
        self.set_height(SIZE, VERTICES_PER_SIDE, VERTICES_PER_SIDE, vec![]);
        self.subdivide(0, 0, VERTICES_PER_SIDE);
    }

    pub fn indices() -> Vec<u32>
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

    pub fn positions(&self) -> &Vec<glm::Vec3>
    {
        &self.positions
    }

    pub fn uv_coordinates() -> Vec<glm::Vec2>
    {
        let mut uvs = Vec::new();
        let scale = 1.0 / VERTICES_PER_SIDE as f32;
        for r in 0..VERTICES_PER_SIDE+1
        {
            for c in 0..VERTICES_PER_SIDE+1
            {
                uvs.push(glm::vec2(r as f32 * scale, c as f32 * scale));
            }
        }
        uvs
    }

    pub fn normals(&self) -> Vec<glm::Vec3>
    {
        let mut normals = Vec::new();
        for r in 0..VERTICES_PER_SIDE+1
        {
            for c in 0..VERTICES_PER_SIDE+1
            {
                if c == 0 || r == 0 || c == VERTICES_PER_SIDE || r == VERTICES_PER_SIDE
                {
                    normals.push(glm::vec3(0.0, 1.0, 0.0));
                }
                else {
                    let mut n = glm::vec3(self.get_height(r-1,c) - self.get_height(r+1,c),
                                          2.0 * VERTEX_DISTANCE,
                                          self.get_height(r,c-1) - self.get_height(r,c+1));
                    glm::normalize(n);
                    normals.push(n)
                }
            }
        }
        normals
    }

    fn set_height(&mut self, scale: f32, r: usize, c: usize, neighbour_heights: Vec<f32>)
    {
        let mut pos = glm::vec3(self.origo.x + r as f32 * VERTEX_DISTANCE, 0.0,
                            self.origo.z + c as f32 * VERTEX_DISTANCE);
        let noise_val = self.noise_generator.get([pos.x as f64, pos.z as f64]);
        pos.y = average(&neighbour_heights) + 0.15 * scale * noise_val as f32;
        self.positions[r*(VERTICES_PER_SIDE+1) + c] = pos;
    }

    fn get_height(&self, r: usize, c: usize) -> f32
    {
        self.positions[r*(VERTICES_PER_SIDE+1) + c].y
    }

    fn subdivide(&mut self, origo_r: usize, origo_c: usize, size: usize)
    {
        let half_size = size/2;
        if half_size >= 1
        {
            let scale = size as f32 * VERTEX_DISTANCE;

            let mut neighbour_heights = vec![self.get_height(origo_r,origo_c), self.get_height(origo_r + size,origo_c)];
            self.set_height(scale, origo_r + half_size, origo_c, neighbour_heights);
            neighbour_heights = vec![self.get_height(origo_r,origo_c), self.get_height(origo_r,origo_c + size)];
            self.set_height(scale, origo_r, origo_c + half_size, neighbour_heights);
            neighbour_heights = vec![self.get_height(origo_r + size,origo_c + size), self.get_height(origo_r,origo_c + size)];
            self.set_height(scale, origo_r + half_size, origo_c + size, neighbour_heights);
            neighbour_heights = vec![self.get_height(origo_r + size,origo_c + size), self.get_height(origo_r + size,origo_c)];
            self.set_height(scale, origo_r + size, origo_c + half_size, neighbour_heights);
            neighbour_heights = vec![self.get_height(origo_r + half_size,origo_c), self.get_height(origo_r,origo_c + half_size),
                            self.get_height(origo_r + half_size,origo_c + size), self.get_height(origo_r + size,origo_c + half_size)];
            self.set_height(scale, origo_r + half_size, origo_c + half_size, neighbour_heights);

            self.subdivide(origo_r, origo_c, half_size);
            self.subdivide(origo_r + half_size, origo_c, half_size);
            self.subdivide(origo_r, origo_c + half_size, half_size);
            self.subdivide(origo_r + half_size, origo_c + half_size, half_size);
        }
    }

    pub fn get_height_at(&self, position: glm::Vec3) -> f32
    {
        let vec = position - self.origo;

        let r = (vec.x * VERTICES_PER_UNIT as f32).floor() as usize;
        let c = (vec.z * VERTICES_PER_UNIT as f32).floor() as usize;

        let tx = vec.x * VERTICES_PER_UNIT as f32 - r as f32;
        let tz = vec.z * VERTICES_PER_UNIT as f32 - c as f32;

        let mut height = (1. - tx) * (1. - tz) * self.get_height(r,c);
        height += tx * (1. - tz) * self.get_height(r+1,c);
        height += (1. - tx) * tz * self.get_height(r,c+1);
        height += tx * tz * self.get_height(r+1,c+1);
        return height;
    }

    pub fn get_origo(&self) -> glm::Vec3
    {
        return self.origo;
    }
}

fn average(heights: &Vec<f32>) -> f32
{
    if heights.len() == 0
    {
        return 0.0;
    }
    let mut sum = 0.0;
    for height in heights {
        sum += height;
    }
    return sum / heights.len() as f32;
}