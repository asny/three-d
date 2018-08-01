extern crate image;

use gl;
use dust::traits;
use gust;
use dust::core::program;
use dust::core::texture;
use dust::core::texture::Texture;
use dust::core::surface;
use dust::core::state;
use dust::camera;
use self::image::{GenericImage};

pub struct Skybox {
    program: program::Program,
    model: surface::TriangleSurface,
    texture: texture::Texture3D
}

impl Skybox
{
    pub fn create(gl: &gl::Gl) -> Result<Skybox, traits::Error>
    {
        let mesh = gust::loader::load_obj("examples/assets/models/box.obj").unwrap();
        let program = program::Program::from_resource(gl, "examples/assets/shaders/skybox")?;
        let model = surface::TriangleSurface::create(gl, &mesh, &program)?;

        let back = image::open("examples/assets/textures/skybox_evening/back.jpg").unwrap();
        let front = image::open("examples/assets/textures/skybox_evening/front.jpg").unwrap();
        let top = image::open("examples/assets/textures/skybox_evening/top.jpg").unwrap();
        let left = image::open("examples/assets/textures/skybox_evening/left.jpg").unwrap();
        let right = image::open("examples/assets/textures/skybox_evening/right.jpg").unwrap();
        let mut texture = texture::Texture3D::create(gl)?;
        texture.fill_with(back.dimensions().0 as usize, back.dimensions().1 as usize,
                          [&right.raw_pixels(), &left.raw_pixels(), &top.raw_pixels(),
                              &top.raw_pixels(), &front.raw_pixels(), &back.raw_pixels()]);

        Ok(Skybox { program, model, texture })
    }

    pub fn render(&self, camera: &camera::Camera) -> Result<(), traits::Error>
    {
        self.program.cull(state::CullType::FRONT);
        self.program.depth_write(true);
        self.program.depth_test(state::DepthTestType::LEQUAL);

        self.texture.bind(0);
        self.program.add_uniform_int("texture0", &0)?;
        self.program.add_uniform_mat4("viewMatrix", &camera.get_view())?;
        self.program.add_uniform_mat4("projectionMatrix", &camera.get_projection())?;
        self.program.add_uniform_vec3("cameraPosition", &camera.position)?;

        self.model.render()?;
        Ok(())
    }

    pub fn get_texture(&self) -> &texture::Texture3D
    {
        &self.texture
    }
}
