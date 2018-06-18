extern crate image;

use dust::core::program;
use gl;
use dust::traits;
use gust;
use std::rc::Rc;
use dust::core::texture;
use dust::core::texture::Texture;
use dust::core::surface;
use glm;
use dust::camera;
use self::image::{GenericImage};

pub struct Skybox {
    program: program::Program,
    model: surface::TriangleSurface,
    texture: texture::Texture3D
}

impl traits::Reflecting for Skybox
{
    fn reflect(&self, transformation: &glm::Mat4, camera: &camera::Camera) -> Result<(), traits::Error>
    {
        self.program.cull_back_faces(false);
        self.program.depth_write(true);
        self.program.depth_test(true);

        self.texture.bind(0);
        self.program.add_uniform_int("texture0", &0)?;
        self.program.add_uniform_mat4("modelMatrix", &transformation)?;
        self.program.add_uniform_mat4("viewMatrix", &camera.get_view())?;
        self.program.add_uniform_mat4("projectionMatrix", &camera.get_projection())?;
        self.program.add_uniform_vec3("cameraPosition", &camera.position)?;

        self.model.render()?;
        Ok(())
    }

}

impl Skybox
{
    pub fn create(gl: &gl::Gl) -> Result<Rc<traits::Reflecting>, traits::Error>
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

        Ok(Rc::new(Skybox { program, model, texture }))
    }
}
