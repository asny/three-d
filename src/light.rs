use gust::*;
use core::rendertarget::DepthRenderTarget;
use gl;

pub struct Light {
    pub color: Vec3,
    pub ambient_intensity: f32,
    pub diffuse_intensity: f32
}

pub struct DirectionalLight {
    pub base: Light,
    pub direction: Vec3,
    pub shadow_render_target: Option<DepthRenderTarget>
}

impl DirectionalLight
{
    pub fn new(direction: Vec3) -> DirectionalLight
    {
        let color = vec3(1., 1., 1.);
        let ambient_intensity = 0.2;
        let diffuse_intensity = 0.5;
        let base = Light {color, ambient_intensity, diffuse_intensity};
        DirectionalLight {direction, base, shadow_render_target: None}
    }

    pub fn new_that_cast_shadow(gl: &gl::Gl, direction: Vec3) -> DirectionalLight
    {
        let mut light = DirectionalLight::new(direction);
        light.shadow_render_target = Some(DepthRenderTarget::create(gl, 1024, 1024).unwrap());
        light
    }
}