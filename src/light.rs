use gl;
use glm;
use core::program;
use core::state;
use core::texture;
use core::texture::Texture;
use core::attributes;
use traits;

pub struct DirectionalLight {
    gl: gl::Gl,
    program: program::Program,
    direction: glm::Vec3
}

impl DirectionalLight
{
    pub fn create(gl: &gl::Gl, direction: glm::Vec3) -> Result<DirectionalLight, traits::Error>
    {
        let program = program::Program::from_resource(&gl, "examples/assets/shaders/light_pass")?;
        Ok(DirectionalLight {gl: gl.clone(), program, direction})
    }
}

impl traits::Emitting for DirectionalLight
{
    fn shine(&self, color_texture: &texture::Texture2D) -> Result<(), traits::Error>
    {
        state::depth_write(&self.gl,false);
        state::depth_test(&self.gl, false);
        state::cull_back_faces(&self.gl,true);

        color_texture.bind(0);
        self.program.add_uniform_int("colorMap", &0)?;

        attributes::Attributes::draw_full_screen_quad(&self.gl, &self.program);
        Ok(())
    }
}