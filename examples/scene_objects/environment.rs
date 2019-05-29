
use crate::scene_objects::terrain::Terrain;
use crate::scene_objects::water::Water;
//use crate::scene_objects::grass::Grass;
use dust::*;

pub struct Environment
{
    skybox: objects::Skybox,
    terrain: Terrain,
    water: Water,
    //grass: Grass,
    volume_effect: VolumeEffect
}

impl Environment {
    pub fn new(gl: &Gl) -> Environment
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
        //let grass = Grass::new(gl, &terrain);

        Environment {terrain, skybox, water, volume_effect: VolumeEffect::new(gl).unwrap()}
    }

    pub fn render_opague(&self, camera: &camera::Camera)
    {
        self.skybox.render(camera).unwrap();
        self.terrain.render(camera);
        //self.grass.render(camera);
    }

    pub fn render_transparent(&self, full_screen: &objects::FullScreen, time: f32, camera: &camera::Camera, screen_width: usize, screen_height: usize, color_texture: &core::texture::Texture, position_texture: &core::texture::Texture)
    {
        self.volume_effect.apply(full_screen, time, camera, color_texture, position_texture).unwrap();
        self.water.render(time, camera, screen_width, screen_height, color_texture, position_texture, self.skybox.get_texture());
    }

    pub fn set_position(&mut self, position: &Vec3)
    {
        if (*self.terrain.get_center() - *position).magnitude() > 10.0
        {
            self.terrain.set_center(position);
            self.water.set_center(position);
            //self.grass.create_straws(&self.terrain);
        }
    }

    pub fn get_height_at(&self, x: f32, z: f32) -> f32
    {
        self.terrain.get_height_at(x, z)
    }
}

pub struct VolumeEffect {
    gl: Gl,
    program: program::Program
}

impl VolumeEffect {

    pub fn new(gl: &Gl) -> Result<VolumeEffect, effects::Error>
    {
        let program = program::Program::from_source(&gl,
                                                    include_str!("../assets/shaders/effect.vert"),
                                                    include_str!("../assets/shaders/volume_effect.frag"))?;
        Ok(VolumeEffect {gl: gl.clone(), program})
    }

    pub fn apply(&self, full_screen: &objects::FullScreen, time: f32, camera: &camera::Camera, shaded_color_texture: &Texture, position_texture: &Texture) -> Result<(), effects::Error>
    {
        let color = vec3(0.8, 0.8, 0.8);
        let density = 0.08;
        let no_fog_height = 6.;
        let animation = 0.1;

        state::depth_write(&self.gl,false);
        state::depth_test(&self.gl, state::DepthTestType::NONE);
        state::blend(&self.gl, state::BlendType::SRC_ALPHA__ONE_MINUS_SRC_ALPHA);

        shaded_color_texture.bind(0);
        self.program.add_uniform_int("colorMap", &0)?;

        position_texture.bind(1);
        self.program.add_uniform_int("positionMap", &1)?;

        self.program.add_uniform_vec3("fogColor", &color)?;
        self.program.add_uniform_float("fogDensity", &density)?;
        self.program.add_uniform_float("noFogHeight", &no_fog_height)?;
        self.program.add_uniform_float("animation", &animation)?;
        self.program.add_uniform_float("time", &(0.001 * time))?;
        self.program.add_uniform_vec3("eyePosition", camera.position())?;

        full_screen.render(&self.program);
        Ok(())
    }
}