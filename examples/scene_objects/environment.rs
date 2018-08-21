use scene_objects::terrain::Terrain;
use scene_objects::water::Water;
use scene_objects::skybox::Skybox;
use scene_objects::grass::Grass;
use dust::*;
use glm::*;

pub struct Environment
{
    skybox: Skybox,
    terrain: Terrain,
    water: Water,
    grass: Grass
}

impl Environment {
    pub fn create(gl: &gl::Gl) -> Result<Environment, traits::Error>
    {
        let skybox = Skybox::create(&gl).unwrap();
        let terrain = Terrain::create(gl)?;
        let water = Water::create(gl)?;
        let grass = Grass::create(gl, &terrain)?;

        Ok(Environment {terrain, skybox, water, grass})
    }

    pub fn render_opague(&self, camera: &camera::Camera) -> Result<(), traits::Error>
    {
        self.skybox.render(&camera)?;
        self.terrain.render(camera)?;
        Ok(())
    }

    pub fn render_transparent(&self, time: f32, camera: &camera::Camera, color_texture: &core::texture::Texture, position_texture: &core::texture::Texture) -> Result<(), traits::Error>
    {
        self.water.render(time, camera, color_texture, position_texture, self.skybox.get_texture())?;
        self.grass.render(camera)?;
        Ok(())
    }

    pub fn set_position(&mut self, position: &Vec3)
    {
        if length(*self.terrain.get_center() - *position) > 10.0
        {
            self.terrain.set_center(position);
            self.water.set_center(position);
            self.grass.create_straws(&self.terrain);
        }
    }

    pub fn get_height_at(&self, x: f32, z: f32) -> f32
    {
        self.terrain.get_height_at(x, z)
    }
}