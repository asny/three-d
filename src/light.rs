use glm;
use traits;

pub struct DirectionalLight {
    pub direction: glm::Vec3
}

impl DirectionalLight
{
    pub fn create(direction: glm::Vec3) -> Result<DirectionalLight, traits::Error>
    {
        Ok(DirectionalLight {direction})
    }
}

impl traits::Emitting for DirectionalLight
{
}