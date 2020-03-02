
use crate::*;

#[derive(Debug)]
pub enum Error {
    Buffer(buffer::Error),
    Rendertarget(rendertarget::Error)
}

impl From<buffer::Error> for Error {
    fn from(other: buffer::Error) -> Self {
        Error::Buffer(other)
    }
}

impl From<rendertarget::Error> for Error {
    fn from(other: rendertarget::Error) -> Self {
        Error::Rendertarget(other)
    }
}

pub const MAX_NO_LIGHTS: usize = 4;

pub struct AmbientLight
{
    color: Vec3,
    intensity: f32
}

impl AmbientLight
{
    pub fn new() -> AmbientLight
    {
        AmbientLight { color: vec3(1.0, 1.0, 1.0), intensity: 0.5 }
    }

    pub fn color(&self) -> Vec3
    {
        self.color
    }

    pub fn set_color(&mut self, color: &Vec3)
    {
        self.color = *color;
    }

    pub fn intensity(&self) -> f32
    {
        self.intensity
    }

    pub fn set_intensity(&mut self, intensity: f32)
    {
        self.intensity = intensity;
    }
}

pub struct DirectionalLight {
    gl: Gl,
    light_buffer: UniformBuffer,
    shadow_rendertarget: RenderTarget,
    shadow_texture: Option<Texture2D>,
    shadow_camera: Option<Camera>
}

impl DirectionalLight {

    pub fn new(gl: &Gl) -> Result<DirectionalLight, Error>
    {
        let uniform_sizes = [3u32, 1, 3, 1, 16];

        let mut light = DirectionalLight {
            gl: gl.clone(),
            light_buffer: UniformBuffer::new(gl, &uniform_sizes)?,
            shadow_rendertarget: RenderTarget::new(gl, 0)?,
            shadow_texture: None,
            shadow_camera: None};

        light.set_intensity(0.0);
        light.set_color(&vec3(1.0, 1.0, 1.0));
        light.set_direction(&vec3(0.0, -1.0, 0.0));
        light.clear_shadow_map();
        Ok(light)
    }

    pub fn set_color(&mut self, color: &Vec3)
    {
        self.light_buffer.update(0, &color.to_slice()).unwrap();
    }

    pub fn set_intensity(&mut self, intensity: f32)
    {
        self.light_buffer.update(1, &[intensity]).unwrap();
    }

    pub fn set_direction(&mut self, direction: &Vec3)
    {
        self.light_buffer.update(2, &direction.to_slice()).unwrap();
    }

    pub fn direction(&self) -> Vec3 {
        let d = self.light_buffer.get(2).unwrap();
        vec3(d[0], d[1], d[2])
    }

    pub fn clear_shadow_map(&mut self)
    {
        self.shadow_camera = None;
        self.shadow_texture = None;
    }

    pub fn generate_shadow_map<F>(&mut self, target: &Vec3,
                                  frustrum_width: f32, frustrum_height: f32, frustrum_depth: f32,
                                  texture_width: usize, texture_height: usize, render_scene: &F)
        where F: Fn(&Camera)
    {
        let direction = self.direction();
        let up = compute_up_direction(direction);

        self.shadow_camera = Some(Camera::new_orthographic(&self.gl, target - direction.normalize()*0.5*frustrum_depth, *target, up,
                                                           frustrum_width, frustrum_height, frustrum_depth));
        self.light_buffer.update(4, &shadow_matrix(self.shadow_camera.as_ref().unwrap()).to_slice()).unwrap();

        self.shadow_texture = Some(Texture2D::new_as_depth_target(&self.gl, texture_width, texture_height).unwrap());

        state::depth_write(&self.gl, true);
        state::depth_test(&self.gl, state::DepthTestType::LessOrEqual);

        self.shadow_rendertarget.write_to_depth(self.shadow_texture.as_ref().unwrap()).unwrap();
        self.shadow_rendertarget.clear_depth();
        render_scene(self.shadow_camera.as_ref().unwrap());
    }

    pub(crate) fn shadow_map(&self) -> Option<&Texture2D>
    {
        self.shadow_texture.as_ref()
    }

    pub(crate) fn buffer(&self) -> &UniformBuffer
    {
        &self.light_buffer
    }
}

pub struct PointLight {
    light_buffer: UniformBuffer
}

impl PointLight {

    pub fn new(gl: &Gl) -> Result<PointLight, Error>
    {
        let mut light = PointLight { light_buffer: UniformBuffer::new(gl, &[3u32, 1, 1, 1, 1, 1, 3, 1])? };

        light.set_intensity(0.0);
        light.set_color(&vec3(1.0, 1.0, 1.0));
        light.set_position(&vec3(0.0, 0.0, 0.0));
        light.set_attenuation(0.5, 0.05, 0.005);
        Ok(light)
    }

    pub fn set_color(&mut self, color: &Vec3)
    {
        self.light_buffer.update(0, &color.to_slice()).unwrap();
    }

    pub fn set_intensity(&mut self, intensity: f32)
    {
        self.light_buffer.update(1, &[intensity]).unwrap();
    }

    pub fn set_attenuation(&mut self, constant: f32, linear: f32, exponential: f32)
    {
        self.light_buffer.update(2, &[constant]).unwrap();
        self.light_buffer.update(3, &[linear]).unwrap();
        self.light_buffer.update(4, &[exponential]).unwrap();
    }

    pub fn set_position(&mut self, position: &Vec3)
    {
        self.light_buffer.update(6, &position.to_slice()).unwrap();
    }

    pub(crate) fn buffer(&self) -> &UniformBuffer
    {
        &self.light_buffer
    }
}

pub struct SpotLight {
    gl: Gl,
    light_buffer: UniformBuffer,
    shadow_rendertarget: RenderTarget,
    shadow_texture: Option<Texture2D>,
    shadow_camera: Option<Camera>
}

impl SpotLight {

    pub fn new(gl: &Gl) -> Result<SpotLight, Error>
    {
        let uniform_sizes = [3u32, 1, 1, 1, 1, 1, 3, 1, 3, 1, 16];
        let mut light = SpotLight {
            gl: gl.clone(),
            light_buffer: UniformBuffer::new(gl, &uniform_sizes)?,
            shadow_rendertarget: RenderTarget::new(gl, 0)?,
            shadow_texture: None,
            shadow_camera: None
        };
        light.set_intensity(0.0);
        light.set_color(&vec3(1.0, 1.0, 1.0));
        light.set_cutoff(0.1 * std::f32::consts::PI);
        light.set_direction(&vec3(0.0, -1.0, 0.0));
        light.set_position(&vec3(0.0, 0.0, 0.0));
        light.set_attenuation(0.5, 0.05, 0.005);
        Ok(light)
    }

    pub fn set_color(&mut self, color: &Vec3)
    {
        self.light_buffer.update(0, &color.to_slice()).unwrap();
    }

    pub fn set_intensity(&mut self, intensity: f32)
    {
        self.light_buffer.update(1, &[intensity]).unwrap();
    }

    pub fn set_attenuation(&mut self, constant: f32, linear: f32, exponential: f32)
    {
        self.light_buffer.update(2, &[constant]).unwrap();
        self.light_buffer.update(3, &[linear]).unwrap();
        self.light_buffer.update(4, &[exponential]).unwrap();
    }

    pub fn set_position(&mut self, position: &Vec3)
    {
        self.light_buffer.update(6, &position.to_slice()).unwrap();
    }

    pub fn position(&self) -> Vec3
    {
        let p = self.light_buffer.get(6).unwrap();
        vec3(p[0], p[1], p[2])
    }

    pub fn set_cutoff(&mut self, cutoff: f32)
    {
        self.light_buffer.update(7, &[cutoff]).unwrap();
    }

    pub fn set_direction(&mut self, direction: &Vec3)
    {
        self.light_buffer.update(8, &direction.normalize().to_slice()).unwrap();
    }

    pub fn direction(&self) -> Vec3
    {
        let d = self.light_buffer.get(8).unwrap();
        vec3(d[0], d[1], d[2])
    }

    pub fn clear_shadow_map(&mut self)
    {
        self.shadow_camera = None;
        self.shadow_texture = None;
    }

    pub fn generate_shadow_map<F>(&mut self, frustrum_depth: f32,
                                  texture_width: usize, texture_height: usize, render_scene: &F)
        where F: Fn(&Camera)
    {
        let position = self.position();
        let direction = self.direction();
        let up = compute_up_direction(direction);
        let cutoff = self.light_buffer.get(7).unwrap()[0];

        self.shadow_camera = Some(Camera::new_perspective(&self.gl, position, position + direction, up,
                                                          degrees(45.0), 2.0 * cutoff, 0.1, frustrum_depth));
        self.light_buffer.update(10, &shadow_matrix(self.shadow_camera.as_ref().unwrap()).to_slice()).unwrap();
        self.shadow_texture = Some(Texture2D::new_as_depth_target(&self.gl, texture_width, texture_height).unwrap());

        state::depth_write(&self.gl, true);
        state::depth_test(&self.gl, state::DepthTestType::LessOrEqual);

        self.shadow_rendertarget.write_to_depth(self.shadow_texture.as_ref().unwrap()).unwrap();
        self.shadow_rendertarget.clear_depth();
        render_scene(self.shadow_camera.as_ref().unwrap());
    }

    pub(crate) fn shadow_map(&self) -> Option<&Texture2D>
    {
        self.shadow_texture.as_ref()
    }

    pub(crate) fn buffer(&self) -> &UniformBuffer
    {
        &self.light_buffer
    }
}

fn shadow_matrix(camera: &Camera) -> Mat4
{
    let bias_matrix = crate::Mat4::new(
                         0.5, 0.0, 0.0, 0.0,
                         0.0, 0.5, 0.0, 0.0,
                         0.0, 0.0, 0.5, 0.0,
                         0.5, 0.5, 0.5, 1.0);
    bias_matrix * camera.get_projection() * camera.get_view()
}

fn compute_up_direction(direction: Vec3) -> Vec3
{
    if vec3(1.0, 0.0, 0.0).dot(direction).abs() > 0.9
    {
        (vec3(0.0, 1.0, 0.0).cross(direction)).normalize()
    }
    else {
        (vec3(1.0, 0.0, 0.0).cross(direction)).normalize()
    }
}