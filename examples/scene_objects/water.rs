extern crate image;

use dust::core::program;
use gl;
use dust::traits;
use gust;
use gust::mesh::Attribute;
use gust::ids::*;
use gust::static_mesh::StaticMesh;
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
    center: Vec3
}

impl Water
{
    pub fn create(gl: &gl::Gl) -> Water
    {
        let mut mesh = StaticMesh::create(indices(), vec![Attribute::new("position", 3, vec![0.0;3 * VERTICES_IN_TOTAL]),
                                                          Attribute::new("uv_coordinate", 2, vec![0.0;2 * VERTICES_IN_TOTAL])]).unwrap();

        let program = program::Program::from_resource(gl, "examples/assets/shaders/water").unwrap();
        let mut model = surface::TriangleSurface::create(gl, &mesh).unwrap();
        let buffer = model.add_attributes(&mesh, &program, &vec!["uv_coordinate", "position"]).unwrap();

        let foam_texture = texture_from_img(gl,"examples/assets/textures/grass.jpg").unwrap();

        let mut water = Water { program, model, foam_texture, buffer, center: vec3(0.0, 0.0, 0.0)};
        water.set_center(&vec3(0.0, 0.0, 0.0));
        water
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
        let mut data = vec![0.0; 5 * VERTICES_IN_TOTAL];

        self.update_positions(&mut data, 2, 5);
        self.update_uv_coordinates(&mut data, 0, 5);

        self.buffer.fill_with(data);
    }

    fn update_positions(&mut self, data: &mut Vec<f32>, offset: usize, stride: usize)
    {
        for r in 0..VERTICES_PER_SIDE
        {
            for c in 0..VERTICES_PER_SIDE
            {
                let vertex_id = r*VERTICES_PER_SIDE + c;
                data[offset + vertex_id * stride] = self.center.x - SIZE/2.0 + r as f32 * VERTEX_DISTANCE;
                data[offset + vertex_id * stride + 2] = self.center.z - SIZE/2.0 + c as f32 * VERTEX_DISTANCE;
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