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
use dust::core::buffer;
use glm;
use dust::camera;
use dust::core::state;
use self::image::{GenericImage};
use self::noise::{NoiseFn, Point2, SuperSimplex};

const SIZE: f32 = 32.0;
const VERTICES_PER_UNIT: usize = 8;
const VERTICES_PER_SIDE: usize = SIZE as usize * VERTICES_PER_UNIT;
const VERTEX_DISTANCE: f32 = 1.0 / VERTICES_PER_UNIT as f32;

pub struct Terrain {
    program: program::Program,
    model: surface::TriangleSurface,
    texture: texture::Texture2D,
    noise_generator: Box<NoiseFn<Point2<f64>>>,
    buffer: buffer::VertexBuffer,
    origo: glm::Vec3,
    mesh: Mesh
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
        let noise_generator = Box::new(SuperSimplex::new());

        let mut mesh = gust::mesh::Mesh::create_indexed(indices(), positions())?;
        mesh.add_custom_vec3_attribute("normal", normals())?;
        mesh.add_custom_vec2_attribute("uv_coordinate", uv_coordinates())?;

        let program = program::Program::from_resource(gl, "examples/assets/shaders/texture")?;
        let mut model = surface::TriangleSurface::create_without_adding_attributes(gl, &mesh, &program)?;
        let buffer = model.add_attributes(&mesh.get_vec(&vec!["position", "normal"])?, &program)?;
        model.add_attributes(&mesh.get_vec(&vec!["uv_coordinate"])?, &program)?;

        let img = image::open("examples/assets/textures/grass.jpg").unwrap();
        let mut texture = texture::Texture2D::create(gl)?;

        texture.fill_with_u8(img.dimensions().0 as usize, img.dimensions().1 as usize, &img.raw_pixels());

        let mut terrain = Terrain { program, model, texture, buffer, origo: glm::vec3(0.0, 0.0, 0.0), noise_generator, mesh};
        terrain.set_center(&glm::vec3(0.0, 0.0, 0.0));
        Ok(Rc::new(terrain))
    }

    pub fn set_center(&mut self, center: &glm::Vec3)
    {
        self.origo = glm::vec3(center.x - SIZE/2.0, 0.0, center.z - SIZE/2.0);
        self.update_heights();
        self.update_normals();

        let mut attributes = Vec::new();
        for name in vec!["position", "normal"] {
            attributes.push(self.mesh.get(name).unwrap());
        }
        self.buffer.fill_from(&attributes);
    }

    fn update_heights(&mut self) -> &Vec<f32>
    {
        let position_attribute = self.mesh.get_mut("position").unwrap();
        let positions = position_attribute.data_mut();
        for r in 0..VERTICES_PER_SIDE+1
        {
            for c in 0..VERTICES_PER_SIDE+1
            {
                let x = self.origo.x + r as f32 * VERTEX_DISTANCE;
                let z = self.origo.z + c as f32 * VERTEX_DISTANCE;
                let y = (self.noise_generator.get([x as f64 * 0.1, z as f64 * 0.1]) +
                        0.25 * self.noise_generator.get([x as f64 * 0.5, z as f64 * 0.5]) +
                        2.0 * self.noise_generator.get([x as f64 * 0.02, z as f64 * 0.02])) as f32;
                positions[3 * (r*(VERTICES_PER_SIDE+1) + c)] = x;
                positions[3 * (r*(VERTICES_PER_SIDE+1) + c) + 1] = y;
                positions[3 * (r*(VERTICES_PER_SIDE+1) + c) + 2] = z;
            }
        }
        positions
    }

    fn update_normals(&mut self)
    {
        let normal_attribute = self.mesh.get_mut("normal").unwrap();
        let normals = normal_attribute.data_mut();
        for r in 0..VERTICES_PER_SIDE+1
        {
            for c in 0..VERTICES_PER_SIDE+1
            {
                let mut n = glm::vec3(0.0, 1.0, 0.0);
                /*if c != 0 && r != 0 && c != VERTICES_PER_SIDE && r != VERTICES_PER_SIDE
                {
                    n = glm::vec3(get_height(positions,r-1,c) - get_height(positions,r+1,c),
                                          2.0 * VERTEX_DISTANCE,
                                          get_height(positions,r,c-1) - get_height(positions,r,c+1));
                    n = glm::normalize(n);
                }*/
                normals[3 * (r*(VERTICES_PER_SIDE+1) + c)] = n.x;
                normals[3 * (r*(VERTICES_PER_SIDE+1) + c) + 1] = n.y;
                normals[3 * (r*(VERTICES_PER_SIDE+1) + c) + 2] = n.z;
            }
        }
    }

    /*pub fn get_height_at(&self, position: glm::Vec3) -> f32
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
    }*/
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
    let mut uvs = Vec::new();
    let scale = 1.0 / VERTICES_PER_SIDE as f32;
    for r in 0..VERTICES_PER_SIDE+1
    {
        for c in 0..VERTICES_PER_SIDE+1
        {
            uvs.push(r as f32 * scale);
            uvs.push(c as f32 * scale);
        }
    }
    uvs
}

fn get_height(positions: &Vec<f32>, r: usize, c: usize) -> f32
{
    positions[3 * (r*(VERTICES_PER_SIDE+1) + c) + 1]
}
