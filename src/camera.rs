use gl;
use glm;
use scene;
use input;
use rendertarget;
use rendertarget::Rendertarget;
use num_traits::identities::One;

#[derive(Debug)]
pub enum Error {
    Scene(scene::Error)
}

impl From<scene::Error> for Error {
    fn from(other: scene::Error) -> Self {
        Error::Scene(other)
    }
}

pub struct Camera {
    gl: gl::Gl,
    position: glm::Vec3,
    target: glm::Vec3,
    z_near: f32,
    z_far: f32,
    width: usize,
    height: usize,
    screen_rendertarget: rendertarget::ScreenRendertarget,
    geometry_pass_rendertarget: rendertarget::ColorRendertarget
}


impl Camera
{
    pub fn create(gl: &gl::Gl, position: glm::Vec3, target: glm::Vec3, width: usize, height: usize) -> Result<Camera, Error>
    {
        let screen_rendertarget = rendertarget::ScreenRendertarget::create(&gl, width, height).unwrap();
        let geometry_pass_rendertarget = rendertarget::ColorRendertarget::create(&gl, width, height).unwrap();
        Ok(Camera { gl: gl.clone(), position, target, z_near: 0.1, z_far: 1000.0, width, height, screen_rendertarget, geometry_pass_rendertarget })
    }

    pub fn set_screen_size(&mut self, width: usize, height: usize)
    {
        self.screen_rendertarget = rendertarget::ScreenRendertarget::create(&self.gl, width, height).unwrap();
        self.width = width;
        self.height = height;
    }

    pub fn set_view(&mut self, position: glm::Vec3, target: glm::Vec3)
    {
        self.position = position;
        self.target = target;
    }

    pub fn position(&self) -> glm::Vec3
    {
        self.position
    }

    pub fn target(&self) -> glm::Vec3
    {
        self.target
    }

    pub fn direction(&self) -> glm::Vec3
    {
        glm::normalize(self.target - self.position)
    }

    pub fn draw(&self, scene: &scene::Scene) -> Result<(), Error>
    {
        self.deferred_pass(scene)
    }

    pub fn forward_pass(&self, scene: &scene::Scene) -> Result<(), Error>
    {
        let input = input::DrawInput{model: glm::Matrix4::one(),view: self.get_view(), projection: self.get_projection(),
            camera_position: self.position, color_texture: self.geometry_pass_rendertarget.color_texture.clone()};

        self.screen_rendertarget.bind();
        self.screen_rendertarget.clear();

        scene.draw(&input)?;
        Ok(())
    }

    pub fn deferred_pass(&self, scene: &scene::Scene) -> Result<(), Error>
    {
        let input = input::DrawInput{model: glm::Matrix4::one(),view: self.get_view(), projection: self.get_projection(),
            camera_position: self.position, color_texture: self.geometry_pass_rendertarget.color_texture.clone()};

        /*self.geometry_pass_rendertarget.bind();
        self.geometry_pass_rendertarget.clear();

        scene.draw(&input)?;*/

        self.screen_rendertarget.bind();
        self.screen_rendertarget.clear();

        scene.shine_lights(&self.gl, &input)?;
        Ok(())
    }

    fn get_view(&self) -> glm::Matrix4<f32>
    {
        glm::ext::look_at(self.position, self.target, glm::vec3(0., 1., 0.))
    }

    fn get_projection(&self) -> glm::Matrix4<f32>
    {
        glm::ext::perspective(glm::radians(45.0), (self.width as f32)/(self.height as f32), self.z_near, self.z_far)
    }
}
