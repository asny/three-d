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

pub struct ForwardPipeline {
    gl: gl::Gl,
    width: usize,
    height: usize,
    rendertarget: rendertarget::ScreenRendertarget
}

impl ForwardPipeline
{
    pub fn create(gl: &gl::Gl, width: usize, height: usize) -> Result<ForwardPipeline, Error>
    {
        let rendertarget = rendertarget::ScreenRendertarget::create(gl, width, height)?;
        Ok(ForwardPipeline {gl: gl.clone(), width, height, rendertarget})
    }

    pub fn resize(&mut self, width: usize, height: usize) -> Result<(), Error>
    {
        self.rendertarget = rendertarget::ScreenRendertarget::create(&self.gl, width, height)?;
        self.width = width;
        self.height = height;
        Ok(())
    }

    pub fn draw(&self, camera: &camera::Camera, scene: &scene::Scene) -> Result<(), Error>
    {
        let input = input::ReflectingInput { model: glm::Matrix4::one(),view: camera.get_view(), projection: camera.get_projection(),
            normal: glm::Matrix4::one(), camera_position: camera.position };

        self.rendertarget.bind();
        self.rendertarget.clear();

        scene.draw(&input)?;
        Ok(())
    }
}

pub struct DeferredPipeline {
    gl: gl::Gl,
    pub width: usize,
    pub height: usize,
    rendertarget: rendertarget::ScreenRendertarget,
    geometry_pass_rendertarget: rendertarget::ColorRendertarget
}


impl DeferredPipeline
{
    pub fn create(gl: &gl::Gl, width: usize, height: usize) -> Result<DeferredPipeline, Error>
    {
        let rendertarget = rendertarget::ScreenRendertarget::create(gl, width, height)?;
        let geometry_pass_rendertarget = rendertarget::ColorRendertarget::create(&gl, width, height, 3)?;
        Ok(DeferredPipeline { gl: gl.clone(), width, height, rendertarget, geometry_pass_rendertarget })
    }

    pub fn resize(&mut self, width: usize, height: usize) -> Result<(), Error>
    {
        self.rendertarget = rendertarget::ScreenRendertarget::create(&self.gl, width, height)?;
        self.geometry_pass_rendertarget = rendertarget::ColorRendertarget::create(&self.gl, width, height, 3)?;
        self.width = width;
        self.height = height;
        Ok(())
    }

    pub fn draw(&self, camera: &camera::Camera, scene: &scene::Scene) -> Result<(), Error>
    {
        let reflecting_input = input::ReflectingInput { model: glm::Matrix4::one(),view: camera.get_view(), projection: camera.get_projection(),
            normal: glm::Matrix4::one(), camera_position: camera.position };

        self.geometry_pass_rendertarget.bind();
        self.geometry_pass_rendertarget.clear();

        scene.draw(&reflecting_input)?;

        self.rendertarget.bind();
        self.rendertarget.clear();

        scene.shine_lights(&self.geometry_pass_rendertarget.targets[0])?;
        Ok(())
    }
}