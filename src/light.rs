use glm;
use traits;
use core::program;

pub struct Light {
    pub color: glm::Vec3,
    pub ambient_intensity: f32,
    pub diffuse_intensity: f32
}

pub struct DirectionalLight {
    pub base: Light,
    pub direction: glm::Vec3
}

impl DirectionalLight
{
    pub fn create(direction: glm::Vec3) -> Result<DirectionalLight, traits::Error>
    {
        let color = glm::vec3(1., 1., 1.);
        let ambient_intensity = 0.2;
        let diffuse_intensity = 0.5;
        let base = Light {color, ambient_intensity, diffuse_intensity};
        Ok(DirectionalLight {direction, base})
    }
}

impl traits::Emitting for DirectionalLight
{
    fn emit(&self, program: &program::Program) -> Result<(), traits::Error>
    {
        program.add_uniform_int("lightType", &1)?;
        program.add_uniform_vec3("directionalLight.direction", &self.direction)?;
        program.add_uniform_vec3("directionalLight.base.color", &self.base.color)?;
        program.add_uniform_float("directionalLight.base.ambientIntensity", &self.base.ambient_intensity)?;
        program.add_uniform_float("directionalLight.base.diffuseIntensity", &self.base.diffuse_intensity)?;
        Ok(())
    }
}