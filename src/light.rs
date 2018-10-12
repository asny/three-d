use gust::*;
use core::rendertarget::DepthRenderTarget;
use gl;
use camera::{self, Camera};

pub struct Light {
    pub color: Vec3,
    pub ambient_intensity: f32,
    pub diffuse_intensity: f32
}

pub struct DirectionalLight {
    pub base: Light,
    pub direction: Vec3,
    pub shadow_render_target: Option<DepthRenderTarget>,
    shadow_camera: Option<camera::ShadowCamera>
}

impl DirectionalLight
{
    pub fn new(direction: Vec3) -> DirectionalLight
    {
        let color = vec3(1., 1., 1.);
        let ambient_intensity = 0.2;
        let diffuse_intensity = 0.5;
        let base = Light {color, ambient_intensity, diffuse_intensity};
        DirectionalLight {direction, base, shadow_render_target: None, shadow_camera: None}
    }

    pub fn new_that_cast_shadow(gl: &gl::Gl, direction: Vec3, radius: f32) -> DirectionalLight
    {
        let mut light = DirectionalLight::new(direction);
        light.shadow_render_target = Some(DepthRenderTarget::create(gl, 1024, 1024).unwrap());
        light.shadow_camera = Some(camera::ShadowCamera::new(vec3(0.0, 0.0, 0.0),light.direction, radius));
        light
    }

    pub fn set_target(&mut self, target: &Vec3)
    {
        if let Some(ref mut camera) = self.shadow_camera {
            camera.set_view(*target, *target + self.direction);
        }
    }

    pub fn shadow_cast_begin(&self)
    {
        let rendertarget = self.shadow_render_target.as_ref().unwrap();

        use rendertarget::Rendertarget;
        rendertarget.bind();
        rendertarget.clear();
    }

    pub fn shadow_camera(&self) -> &camera::ShadowCamera
    {
        self.shadow_camera.as_ref().unwrap()
    }
}