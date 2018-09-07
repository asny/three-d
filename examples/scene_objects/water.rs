extern crate image;

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
use num_traits::identities::One;

const SIZE: f32 = 64.0;
const VERTICES_PER_UNIT: usize = 2;
const VERTICES_PER_SIDE: usize = SIZE as usize * VERTICES_PER_UNIT;
const VERTICES_IN_TOTAL: usize = VERTICES_PER_SIDE * VERTICES_PER_SIDE;
const VERTEX_DISTANCE: f32 = 1.0 / VERTICES_PER_UNIT as f32;

pub struct Water {
    program: program::Program,
    model: surface::TriangleSurface,
    foam_texture: texture::Texture2D,
    buffer: buffer::VertexBuffer,
    center: Vec3,
    mesh: Mesh
}

impl Water
{
    pub fn create(gl: &gl::Gl) -> Result<Water, traits::Error>
    {
        let mut mesh = gust::mesh::Mesh::create_indexed(indices(), vec![0.0;3 * VERTICES_IN_TOTAL])?;
        mesh.add_custom_vec2_attribute("uv_coordinate", vec![0.0;2 * VERTICES_IN_TOTAL])?;

        let program = program::Program::from_resource(gl, "examples/assets/shaders/water")?;
        let mut model = surface::TriangleSurface::create_without_adding_attributes(gl, &mesh)?;
        let buffer = model.add_attributes(&mesh, &program, &vec!["uv_coordinate"], &vec!["position"])?;

        let foam_texture = texture_from_img(gl,"examples/assets/textures/grass.jpg")?;

        let mut water = Water { program, model, foam_texture, buffer, center: vec3(0.0, 0.0, 0.0), mesh};
        water.set_center(&vec3(0.0, 0.0, 0.0));
        Ok(water)
    }

    pub fn render(&self, time: f32, camera: &camera::Camera, color_texture: &Texture, position_texture: &Texture, skybox_texture: &Texture) -> Result<(), traits::Error>
    {
        self.program.blend(state::BlendType::SRC_ALPHA__ONE_MINUS_SRC_ALPHA);
        self.program.cull(state::CullType::NONE);
        self.program.depth_write(false);
        self.program.depth_test(state::DepthTestType::LEQUAL);

        self.program.add_uniform_mat4("modelMatrix", &Matrix4::one())?;
        self.program.add_uniform_mat4("viewMatrix", &camera.get_view())?;
        self.program.add_uniform_mat4("projectionMatrix", &camera.get_projection())?;

        self.program.add_uniform_vec3("eyePosition", &camera.position)?;
        self.program.add_uniform_vec2("screenSize", &vec2(camera.width as f32, camera.height as f32))?;

        self.program.add_uniform_float("time", &time)?;

        color_texture.bind(0);
        self.program.add_uniform_int("colorMap", &0)?;

        position_texture.bind(1);
        self.program.add_uniform_int("positionMap", &1)?;

        skybox_texture.bind(2);
        self.program.add_uniform_int("environmentMap", &2)?;

        self.model.render()?;
        Ok(())
    }

    pub fn set_center(&mut self, center: &Vec3)
    {
        self.center = vec3(center.x.floor(), 0.0, center.z.floor());
        self.update_positions();
        self.update_uv_coordinates();

        self.buffer.fill_from_attributes(&self.mesh, &vec!["uv_coordinate"], &vec!["position"]);
    }

    fn update_positions(&mut self)
    {
        for r in 0..VERTICES_PER_SIDE
        {
            for c in 0..VERTICES_PER_SIDE
            {
                let x = self.center.x - SIZE/2.0 + r as f32 * VERTEX_DISTANCE;
                let z = self.center.z - SIZE/2.0 + c as f32 * VERTEX_DISTANCE;
                self.mesh.positions.set(&VertexID::new(r*VERTICES_PER_SIDE + c), &vec3(x, 0.0, z));
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
}

fn texture_from_img(gl: &gl::Gl, name: &str) -> Result<texture::Texture2D, traits::Error>
{
    let img = image::open(name).unwrap();
    let mut texture = texture::Texture2D::create(gl)?;
    texture.fill_with_u8(img.dimensions().0 as usize, img.dimensions().1 as usize, &img.raw_pixels());
    Ok(texture)
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