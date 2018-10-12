extern crate image;

use self::image::{GenericImage};
use scene_objects::terrain::Terrain;
use scene_objects::water::Water;
use scene_objects::grass::Grass;
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
        let back = image::open("examples/assets/textures/skybox_evening/back.jpg").unwrap();
        let front = image::open("examples/assets/textures/skybox_evening/front.jpg").unwrap();
        let top = image::open("examples/assets/textures/skybox_evening/top.jpg").unwrap();
        let left = image::open("examples/assets/textures/skybox_evening/left.jpg").unwrap();
        let right = image::open("examples/assets/textures/skybox_evening/right.jpg").unwrap();
        let mut texture = texture::Texture3D::create(gl).unwrap();
        texture.fill_with(back.dimensions().0 as usize, back.dimensions().1 as usize,
                          [&right.raw_pixels(), &left.raw_pixels(), &top.raw_pixels(),
                              &top.raw_pixels(), &front.raw_pixels(), &back.raw_pixels()]);

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
        if (*self.terrain.get_center() - *position).norm() > 10.0
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