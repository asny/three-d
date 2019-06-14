
use crate::*;

#[derive(Debug)]
pub enum Error {
    ShadowRendertargetNotAvailable {message: String}
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
    pub color: Vec3,
    pub intensity: f32,
    pub direction: Vec3,
    pub shadow_camera: Camera
}

impl DirectionalLight
{
    pub fn new(gl: &Gl) -> DirectionalLight
    {
        DirectionalLight::new_with_direction(gl, vec3(0.0, -1.0, 0.0))
    }

    pub fn new_with_direction(gl: &Gl, direction: Vec3) -> DirectionalLight
    {
        let up = Self::compute_up_direction(direction);
        let radius = 2.0;
        let depth = 10.0;
        let shadow_camera = Camera::new_orthographic(gl, - direction, vec3(0.0, 0.0, 0.0), up,
                                                                  2.0 * radius, 2.0 * radius, 2.0 * depth);
        DirectionalLight {color: vec3(1.0, 1.0, 1.0), intensity: 0.5, direction: direction.normalize(), shadow_camera}
    }

    pub fn set_target(&mut self, target: &Vec3)
    {
        let up = Self::compute_up_direction(self.direction);
        self.shadow_camera.set_view(*target - self.direction, *target, up);
    }

    fn compute_up_direction(direction: Vec3) -> Vec3
    {
        if vec3(1.0, 0.0, 0.0).dot(direction).abs() > 0.9
        {
            (vec3(0.0, 1.0, 0.0).cross(direction)).normalize()
        }
        else {
            (vec3(1.0, 0.0, 0.0).cross(direction)).normalize()
        }
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
    shadow_camera: Option<Camera>,
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

    pub fn enable_shadows(&mut self, gl: &Gl, depth: f32) -> Result<(), Error>
    {
        self.shadow_rendertarget = Some(DepthRenderTarget::new(gl, 512, 512).unwrap());
        let up = self.compute_up_direction();
        self.shadow_camera = Some(Camera::new_perspective(gl, self.position,self.position + self.direction, up,
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
            rendertarget.bind();
            rendertarget.clear();
            return Ok(())
        }
        Err(Error::ShadowRendertargetNotAvailable {message: format!("Shadow is not enabled for this light source")})
    }

    pub fn shadow_camera(&self) -> Result<&Camera, Error>
    {
        if let Some(ref camera) = self.shadow_camera {
            return Ok(camera)
        }
        Err(Error::ShadowRendertargetNotAvailable {message: format!("Shadow is not enabled for this light source")})
    }
}