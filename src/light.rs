use gust::*;
use core::rendertarget::{self, DepthRenderTarget};
use gl;
use camera::{self, Camera};

#[derive(Debug)]
pub enum Error {
    Rendertarget(rendertarget::Error),
    ShadowRendertargetNotAvailable {message: String}
}

impl From<rendertarget::Error> for Error {
    fn from(other: rendertarget::Error) -> Self {
        Error::Rendertarget(other)
    }
}

pub struct Light {
    pub color: Vec3,
    pub intensity: f32
}

pub struct AmbientLight
{
    pub base: Light
}

impl AmbientLight
{
    pub fn new() -> AmbientLight
    {
        let base = Light { color: vec3(1.0, 1.0, 1.0), intensity: 0.2 };
        AmbientLight { base }
    }
}

pub struct DirectionalLight {
    pub base: Light,
    pub direction: Vec3,
    pub shadow_rendertarget: Option<DepthRenderTarget>,
    shadow_camera: Option<camera::OrthographicCamera>
}

impl DirectionalLight
{
    pub fn new(direction: Vec3) -> DirectionalLight
    {
        let base = Light {color: vec3(1.0, 1.0, 1.0), intensity: 0.5};
        DirectionalLight {direction: direction.normalize(), base, shadow_rendertarget: None, shadow_camera: None}
    }

    pub fn enable_shadows(&mut self, gl: &gl::Gl, radius: f32) -> Result<(), Error>
    {
        self.shadow_rendertarget = Some(DepthRenderTarget::create(gl, 1024, 1024)?);
        self.shadow_camera = Some(camera::OrthographicCamera::new(vec3(0.0, 0.0, 0.0), self.direction, radius));
        Ok(())
    }

    pub fn set_target(&mut self, target: &Vec3)
    {
        if let Some(ref mut camera) = self.shadow_camera {
            camera.set_view(*target, *target + self.direction);
        }
    }

    pub fn shadow_cast_begin(&self) -> Result<(), Error>
    {
        if let Some(ref rendertarget) = self.shadow_rendertarget
        {
            use rendertarget::Rendertarget;
            rendertarget.bind();
            rendertarget.clear();
            return Ok(())
        }
        Err(Error::ShadowRendertargetNotAvailable {message: format!("Shadow is not enabled for this light source")})
    }

    pub fn shadow_camera(&self) -> Result<&camera::OrthographicCamera, Error>
    {
        if let Some(ref camera) = self.shadow_camera {
            return Ok(camera)
        }
        Err(Error::ShadowRendertargetNotAvailable {message: format!("Shadow is not enabled for this light source")})
    }
}