use scene_objects::terrain;
use dust::*;
use glm::*;

pub struct Environment
{
    terrain: terrain::Terrain
}

impl Environment {
    pub fn create(gl: &gl::Gl) -> Result<Environment, traits::Error>
    {
        let terrain = terrain::Terrain::create(gl)?;
        Ok(Environment {terrain})
    }

    pub fn draw_solid(&self, camera: &camera::Camera) -> Result<(), traits::Error>
    {
        self.terrain.draw_ground(camera)?;
        Ok(())
    }

    pub fn draw_transparent(&self, camera: &camera::Camera) -> Result<(), traits::Error>
    {
        self.terrain.draw_water(camera)?;
        Ok(())
    }

    pub fn set_position(&mut self, position: &Vec3)
    {
        if length(*self.terrain.get_center() - *position) > 10.0
        {
            self.terrain.set_center(position);
        }
    }

    pub fn get_height_at(&self, x: f32, z: f32) -> f32
    {
        self.terrain.get_height_at(x, z)
    }
}