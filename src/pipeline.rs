use camera;
use scene;
use input;
use glm;
use gl;
use num_traits::identities::One;
use core::rendertarget;
use core::rendertarget::Rendertarget;
use core::state;
use core::attributes;
use core::texture::Texture;
use core::program;
use traits;

#[derive(Debug)]
pub enum Error {
    Program(program::Error),
    Rendertarget(rendertarget::Error),
    Traits(traits::Error)
}

impl From<traits::Error> for Error {
    fn from(other: traits::Error) -> Self {
        Error::Traits(other)
    }
}

impl From<program::Error> for Error {
    fn from(other: program::Error) -> Self {
        Error::Program(other)
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

        for model in &scene.models {
            model.reflect(&input)?;
        }

        Ok(())
    }
}

pub struct DeferredPipeline {
    gl: gl::Gl,
    pub width: usize,
    pub height: usize,
    light_pass_program: program::Program,
    rendertarget: rendertarget::ScreenRendertarget,
    geometry_pass_rendertarget: rendertarget::ColorRendertarget
}


impl DeferredPipeline
{
    pub fn create(gl: &gl::Gl, width: usize, height: usize) -> Result<DeferredPipeline, Error>
    {
        let light_pass_program = program::Program::from_resource(&gl, "examples/assets/shaders/light_pass")?;
        let rendertarget = rendertarget::ScreenRendertarget::create(gl, width, height)?;
        let geometry_pass_rendertarget = rendertarget::ColorRendertarget::create(&gl, width, height, 3)?;
        Ok(DeferredPipeline { gl: gl.clone(), width, height, light_pass_program, rendertarget, geometry_pass_rendertarget })
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
        let input = input::ReflectingInput { model: glm::Matrix4::one(),view: camera.get_view(),
            projection: camera.get_projection(), normal: glm::Matrix4::one(), camera_position: camera.position };

        self.geometry_pass_rendertarget.bind();
        self.geometry_pass_rendertarget.clear();

        for model in &scene.models {
            model.reflect(&input)?;
        }

        self.rendertarget.bind();
        self.rendertarget.clear();

        state::depth_write(&self.gl,false);
        state::depth_test(&self.gl, false);
        state::cull_back_faces(&self.gl,true);

        self.geometry_pass_rendertarget.targets[0].bind(0);
        self.light_pass_program.add_uniform_int("colorMap", &0)?;

        self.geometry_pass_rendertarget.targets[1].bind(1);
        self.light_pass_program.add_uniform_int("positionMap", &1)?;

        self.geometry_pass_rendertarget.targets[2].bind(2);
        self.light_pass_program.add_uniform_int("normalMap", &2)?;

        self.geometry_pass_rendertarget.depth_target.bind(3);
        self.light_pass_program.add_uniform_int("depthMap", &3)?;

        attributes::Attributes::draw_full_screen_quad(&self.gl, &self.light_pass_program);
        Ok(())
    }
}