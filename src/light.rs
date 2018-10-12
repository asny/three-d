use gust::*;

pub struct Light {
    pub color: Vec3,
    pub ambient_intensity: f32,
    pub diffuse_intensity: f32
}

pub struct DirectionalLight {
    pub base: Light,
    pub direction: Vec3
}

impl DirectionalLight
{
    pub fn new(direction: Vec3) -> DirectionalLight
    {
        let color = vec3(1., 1., 1.);
        let ambient_intensity = 0.2;
        let diffuse_intensity = 0.5;
        let base = Light {color, ambient_intensity, diffuse_intensity};
        DirectionalLight {direction, base}
    }
}