
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
    pub fn new(gl: &gl::Gl) -> Environment
    {
        let texture = texture::Texture3D::new_from_bytes(&gl,
                                                           include_bytes!("../assets/textures/skybox_evening/back.jpg"),
                                                           include_bytes!("../assets/textures/skybox_evening/front.jpg"),
                                                           include_bytes!("../assets/textures/skybox_evening/top.jpg"),
                                                           include_bytes!("../assets/textures/skybox_evening/left.jpg"),
                                                           include_bytes!("../assets/textures/skybox_evening/right.jpg")).unwrap();
        let skybox = objects::Skybox::new(&gl, texture);
        let terrain = Terrain::new(gl);
        let water = Water::new(gl);
        let grass = Grass::new(gl, &terrain);

        Environment {terrain, skybox, water, grass}
    }

    pub fn render_opague(&self, camera: &camera::Camera)
    {
        self.skybox.render(camera).unwrap();
        self.terrain.render(camera);
        self.grass.render(camera);
    }

    pub fn render_transparent(&self, time: f32, camera: &camera::Camera, screen_width: usize, screen_height: usize, color_texture: &core::texture::Texture, position_texture: &core::texture::Texture)
    {
        self.water.render(time, camera, screen_width, screen_height, color_texture, position_texture, self.skybox.get_texture());
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