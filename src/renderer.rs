use camera;
use scene;
use input;
use glm;
use gl;
use num_traits::identities::One;
use core::rendertarget;
use core::rendertarget::Rendertarget;

#[derive(Debug)]
pub enum Error {
    Scene(scene::Error),
    Rendertarget(rendertarget::Error)
}

impl From<scene::Error> for Error {
    fn from(other: scene::Error) -> Self {
        Error::Scene(other)
    }
}

impl From<rendertarget::Error> for Error {
    fn from(other: rendertarget::Error) -> Self {
        Error::Rendertarget(other)
    }
}

pub struct Pipeline {
    gl: gl::Gl,
    width: usize,
    height: usize,
    screen_rendertarget: rendertarget::ScreenRendertarget,
    geometry_pass_rendertarget: rendertarget::ColorRendertarget
}


impl Pipeline
{
    pub fn create(gl: &gl::Gl, width: usize, height: usize) -> Result<Pipeline, Error>
    {
        let screen_rendertarget = rendertarget::ScreenRendertarget::create(&gl, width, height)?;
        let geometry_pass_rendertarget = rendertarget::ColorRendertarget::create(&gl, width, height)?;
        Ok(Pipeline { gl: gl.clone(), width, height, screen_rendertarget, geometry_pass_rendertarget })
    }

    pub fn set_screen_size(&mut self, width: usize, height: usize)
    {
        self.screen_rendertarget = rendertarget::ScreenRendertarget::create(&self.gl, width, height).unwrap();
        self.width = width;
        self.height = height;
    }

    pub fn draw(&self, camera: &camera::Camera, scene: &scene::Scene) -> Result<(), Error>
    {
        self.deferred_pass(camera,scene)
    }

    pub fn forward_pass(&self, camera: &camera::Camera, scene: &scene::Scene) -> Result<(), Error>
    {
        let input = input::DrawInput{model: glm::Matrix4::one(),view: camera.get_view(), projection: camera.get_projection(),
            camera_position: camera.position, color_texture: self.geometry_pass_rendertarget.color_texture.clone()};

        self.screen_rendertarget.bind();
        self.screen_rendertarget.clear();

        scene.draw(&input)?;
        Ok(())
    }

    pub fn deferred_pass(&self, camera: &camera::Camera, scene: &scene::Scene) -> Result<(), Error>
    {
        let input = input::DrawInput{model: glm::Matrix4::one(),view: camera.get_view(), projection: camera.get_projection(),
            camera_position: camera.position, color_texture: self.geometry_pass_rendertarget.color_texture.clone()};

        self.geometry_pass_rendertarget.bind();
        self.geometry_pass_rendertarget.clear();

        scene.draw(&input)?;

        self.screen_rendertarget.bind();
        self.screen_rendertarget.clear();

        scene.shine_lights(&input)?;
        Ok(())
    }
}

#[cfg(target_os = "emscripten")]
pub fn set_main_loop<F>(main_loop: F) where F: FnMut()
{
    use emscripten::{emscripten};
    emscripten::set_main_loop_callback(main_loop);
}

#[cfg(not(target_os = "emscripten"))]
pub fn set_main_loop<F>(mut main_loop: F) where F: FnMut()
{
    loop { main_loop(); }
}