use crate::core::rendertarget::{self, DepthRenderTarget};
use gl;
use crate::camera::{self, Camera};
use crate::*;

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
        let base = Light { color: vec3(1.0, 1.0, 1.0), intensity: 0.5 };
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

    pub fn enable_shadows(&mut self, gl: &gl::Gl, radius: f32, depth: f32) -> Result<(), Error>
    {
        self.shadow_rendertarget = Some(DepthRenderTarget::new(gl, 1024, 1024)?);
        let up = self.compute_up_direction();
        self.shadow_camera = Some(camera::OrthographicCamera::new(- self.direction, vec3(0.0, 0.0, 0.0), up,
                                                                  2.0 * radius, 2.0 * radius, 2.0 * depth));
        Ok(())
    }

    pub fn set_target(&mut self, target: &Vec3)
    {
        let up = self.compute_up_direction();
        if let Some(ref mut camera) = self.shadow_camera {
            camera.set_view(*target - self.direction, *target, up);
        }
    }

    fn compute_up_direction(&self) -> Vec3
    {
        if vec3(1.0, 0.0, 0.0).dot(self.direction).abs() > 0.9
        {
            (vec3(0.0, 1.0, 0.0).cross(self.direction)).normalize()
        }
        else {
            (vec3(1.0, 0.0, 0.0).cross(self.direction)).normalize()
        }
    }

    pub fn shadow_cast_begin(&self) -> Result<(), Error>
    {
        if let Some(ref rendertarget) = self.shadow_rendertarget
        {
            use crate::rendertarget::Rendertarget;
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

pub struct Attenuation {
    pub constant: f32,// = 0.1f;
    pub linear: f32,// = 0.01f;
    pub exp: f32// = 0.001f;
}

pub struct PointLight {
    pub base: Light,
    pub position: Vec3,
    pub attenuation: Attenuation
    // TODO: Shadows
}

impl PointLight
{
    pub fn new(position: Vec3) -> PointLight
    {
        let base = Light {color: vec3(1.0, 1.0, 1.0), intensity: 0.5};
        let attenuation = Attenuation {constant: 0.5, linear: 0.05, exp: 0.005};
        PointLight {base, position, attenuation}
    }
}

pub struct SpotLight {
    pub base: Light,
    pub direction: Vec3,
    pub position: Vec3,
    pub shadow_rendertarget: Option<DepthRenderTarget>,
    shadow_camera: Option<camera::PerspectiveCamera>,
    pub attenuation: Attenuation,
    pub cutoff: f32
}

impl SpotLight
{
    pub fn new(position: Vec3, direction: Vec3) -> SpotLight
    {
        let base = Light {color: vec3(1.0, 1.0, 1.0), intensity: 0.5};
        let attenuation = Attenuation {constant: 0.2, linear: 0.02, exp: 0.002};
        SpotLight {base, direction: direction.normalize(), position, shadow_rendertarget: None, shadow_camera: None, attenuation, cutoff: 0.1 * std::f32::consts::PI}
    }

    pub fn enable_shadows(&mut self, gl: &gl::Gl, depth: f32) -> Result<(), Error>
    {
        self.shadow_rendertarget = Some(DepthRenderTarget::new(gl, 1024, 1024)?);
        let up = self.compute_up_direction();
        self.shadow_camera = Some(camera::PerspectiveCamera::new(self.position,self.position + self.direction, up,
                                                                 degrees(45.0), 2.0 * self.cutoff, 0.1, depth));
        Ok(())
    }

    fn compute_up_direction(&self) -> Vec3
    {
        if vec3(1.0, 0.0, 0.0).dot(self.direction).abs() > 0.9
        {
            (vec3(0.0, 1.0, 0.0).cross(self.direction)).normalize()
        }
        else {
            (vec3(1.0, 0.0, 0.0).cross(self.direction)).normalize()
        }
    }

    pub fn shadow_cast_begin(&self) -> Result<(), Error>
    {
        if let Some(ref rendertarget) = self.shadow_rendertarget
        {
            use crate::rendertarget::Rendertarget;
            rendertarget.bind();
            rendertarget.clear();
            return Ok(())
        }
        Err(Error::ShadowRendertargetNotAvailable {message: format!("Shadow is not enabled for this light source")})
    }

    pub fn shadow_camera(&self) -> Result<&camera::PerspectiveCamera, Error>
    {
        if let Some(ref camera) = self.shadow_camera {
            return Ok(camera)
        }
        Err(Error::ShadowRendertargetNotAvailable {message: format!("Shadow is not enabled for this light source")})
    }
}