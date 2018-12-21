
use crate::scene_objects::terrain::Terrain;
use crate::scene_objects::water::Water;
use crate::scene_objects::grass::Grass;
use dust::*;

pub struct Environment
{
    skybox: objects::Skybox,
    terrain: Terrain,
    water: Water,
    grass: Grass
}

impl Environment {
    pub fn create(gl: &gl::Gl) -> Environment
    {
        let texture = texture::Texture3D::new_from_files(&gl, "examples/assets/textures/skybox_evening/",
                                                       "back.jpg", "front.jpg", "top.jpg", "left.jpg", "right.jpg").unwrap();

        let skybox = objects::Skybox::create(&gl, texture);
        let terrain = Terrain::create(gl);
        let water = Water::create(gl);
        let grass = Grass::create(gl, &terrain);

        Environment {terrain, skybox, water, grass}
    }

    pub fn render_opague(&self, camera: &camera::Camera) -> Result<(), traits::Error>
    {
        self.skybox.render(camera)?;
        self.terrain.render(camera)?;
        self.grass.render(camera)?;
        Ok(())
    }

    pub fn render_transparent(&self, time: f32, camera: &camera::Camera, screen: &screen::Screen, color_texture: &core::texture::Texture, position_texture: &core::texture::Texture) -> Result<(), traits::Error>
    {
        self.water.render(time, camera, screen, color_texture, position_texture, self.skybox.get_texture())?;
        Ok(())
    }

    pub fn set_position(&mut self, position: &Vec3)
    {
        if (*self.terrain.get_center() - *position).magnitude() > 10.0
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