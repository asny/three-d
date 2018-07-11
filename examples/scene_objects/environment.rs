use scene_objects::terrain::Terrain;
use scene_objects::skybox::Skybox;
use dust::*;
use glm::*;

pub struct Environment
{
    skybox: Skybox,
    terrain: Terrain
}

impl Environment {
    pub fn create(gl: &gl::Gl) -> Result<Environment, traits::Error>
    {
        let terrain = Terrain::create(gl)?;
        let skybox = Skybox::create(&gl).unwrap();

        Ok(Environment {terrain, skybox})
    }

    pub fn draw_solid(&self, camera: &camera::Camera) -> Result<(), traits::Error>
    {
        self.skybox.render(&camera)?;
        self.terrain.draw_ground(camera)?;
        Ok(())
    }

    pub fn draw_transparent(&self, camera: &camera::Camera, color_texture: &core::texture::Texture) -> Result<(), traits::Error>
    {
        self.terrain.draw_water(camera, color_texture)?;
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