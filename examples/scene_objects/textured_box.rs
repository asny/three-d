extern crate image;

use gl;
use dust::*;
use std::rc::Rc;
use self::image::{GenericImage};

pub struct TexturedBox {
    program: program::Program,
    model: surface::TriangleSurface,
    texture: texture::Texture2D
}

impl traits::Reflecting for TexturedBox
{
    fn reflect(&self, transformation: &Mat4, camera: &camera::Camera) -> Result<(), traits::Error>
    {
        self.program.cull(state::CullType::BACK);
        self.texture.bind(0);
        self.program.add_uniform_int("texture0", &0)?;
        self.program.add_uniform_mat4("modelMatrix", &transformation)?;
        self.program.add_uniform_mat4("viewMatrix", &camera.get_view())?;
        self.program.add_uniform_mat4("projectionMatrix", &camera.get_projection())?;
        self.program.add_uniform_mat4("normalMatrix", &transformation.try_inverse().unwrap().transpose())?;

        self.model.render()?;
        Ok(())
    }

}

impl TexturedBox
{
    pub fn create(gl: &gl::Gl) -> Result<Rc<traits::Reflecting>, traits::Error>
    {
        let mesh = mesh_generator::create_cube().unwrap();
        let program = program::Program::from_resource(gl, "examples/assets/shaders/texture")?;
        let mut model = surface::TriangleSurface::create(gl, &mesh)?;
        model.add_attributes(&mesh, &program,&vec!["uv_coordinate", "position", "normal"])?;

        let img = image::open("examples/assets/textures/test_texture.jpg").unwrap();
        let mut texture = texture::Texture2D::create(gl)?;

        texture.fill_with_u8(img.dimensions().0 as usize, img.dimensions().1 as usize, &img.raw_pixels());

        Ok(Rc::new(TexturedBox { program, model, texture }))
    }
}
