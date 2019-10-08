
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
    pub(crate) fn new() -> AmbientLight
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
    shadow_rendertarget: DepthRenderTargetArray,
    shadow_cameras: [Option<Camera>; MAX_NO_LIGHTS],
    index: usize
}

impl DirectionalLight {

    pub(crate) fn new(gl: &Gl) -> Result<DirectionalLight, Error>
    {
        let uniform_sizes: Vec<u32> = [3u32, 1, 3, 1, 16].iter().cloned().cycle().take(5*MAX_NO_LIGHTS).collect();
        let mut lights = DirectionalLight {
            gl: gl.clone(),
            shadow_rendertarget: DepthRenderTargetArray::new(gl, 1024, 1024, MAX_NO_LIGHTS)?,
            light_buffer: UniformBuffer::new(gl, &uniform_sizes)?,
            shadow_cameras: [None, None, None, None],
            index: 0};

        for light_id in 0..MAX_NO_LIGHTS {
            let light = lights.light_at(light_id);
            light.set_intensity(0.0);
            light.set_color(&vec3(1.0, 1.0, 1.0));
            light.set_direction(&vec3(0.0, -1.0, 0.0));
        }
        Ok(lights)
    }

    pub fn set_color(&mut self, color: &Vec3)
    {
        self.light_buffer.update(self.index_at(0), &color.to_slice()).unwrap();
    }

    pub fn set_intensity(&mut self, intensity: f32)
    {
        self.light_buffer.update(self.index_at(1), &[intensity]).unwrap();
    }

    pub fn set_direction(&mut self, direction: &Vec3)
    {
        self.light_buffer.update(self.index_at(2), &direction.to_slice()).unwrap();
        self.update_shadows(vec3(0.0, 0.0, 0.0), 4.0, 20.0);
    }

    pub fn direction(&self) -> Vec3 {
        let d = self.light_buffer.get(self.index_at(2)).unwrap();
        vec3(d[0], d[1], d[2])
    }

    pub fn is_shadows_enabled(&self) -> bool {
        self.shadow_cameras[self.index].is_some()
    }

    pub fn enable_shadows(&mut self)
    {
        self.shadow_cameras[self.index] = Some(Camera::new());
        self.shadow_cameras[self.index].as_mut().unwrap().enable_matrix_buffer(&self.gl);
        self.update_shadows(vec3(0.0, 0.0, 0.0), 4.0, 20.0);
    }

    pub fn update_shadows(&mut self, target: Vec3, size: f32, depth: f32) {
        let direction = self.direction();
        if let Some(ref mut camera) = self.shadow_cameras[self.index]
        {
            let up = compute_up_direction(direction);
            camera.set_view(target - direction, target, up);
            camera.set_orthographic_projection(size, size, depth);

            let bias_matrix = crate::Mat4::new(
                                 0.5, 0.0, 0.0, 0.0,
                                 0.0, 0.5, 0.0, 0.0,
                                 0.0, 0.0, 0.5, 0.0,
                                 0.5, 0.5, 0.5, 1.0);
            let shadow_matrix = bias_matrix * camera.get_projection() * camera.get_view();
            self.light_buffer.update(self.index_at(4), &shadow_matrix.to_slice()).unwrap();
        }
    }

    pub fn disable_shadows(&mut self)
    {
        self.shadow_cameras[self.index] = None;
        self.light_buffer.update(self.index_at(4), &Mat4::from_value(0.0).to_slice()).unwrap();
    }

    pub(crate) fn shadow_pass<F>(&self, render_scene: &F)
        where F: Fn(&Camera)
    {
        for light_id in 0..MAX_NO_LIGHTS {
            if let Some(ref camera) = self.shadow_cameras[light_id]
            {
                self.shadow_rendertarget.bind(light_id);
                self.shadow_rendertarget.clear();
                render_scene(camera);
            }
        }
    }

    pub(crate) fn shadow_maps(&self) -> &Texture2DArray
    {
        &self.shadow_rendertarget.target
    }

    pub(crate) fn buffer(&self) -> &UniformBuffer
    {
        &self.light_buffer
    }

    pub(crate) fn light_at(&mut self, index: usize) -> &mut Self
    {
        self.index = index;
        self
    }

    fn index_at(&self, index: usize) -> usize
    {
        self.index * 5 + index
    }
}

pub struct PointLight {
    light_buffer: UniformBuffer,
    index: usize
}

impl PointLight {

    pub(crate) fn new(gl: &Gl) -> Result<PointLight, Error>
    {
        let uniform_sizes: Vec<u32> = [3u32, 1, 1, 1, 1, 1, 3, 1].iter().cloned().cycle().take(8*MAX_NO_LIGHTS).collect();
        let mut lights = PointLight {
            light_buffer: UniformBuffer::new(gl, &uniform_sizes)?,
            index: 0};

        for light_id in 0..MAX_NO_LIGHTS {
            let light = lights.light_at(light_id);
            light.set_intensity(0.0);
            light.set_color(&vec3(1.0, 1.0, 1.0));
            light.set_position(&vec3(0.0, 0.0, 0.0));
            light.set_attenuation(0.5, 0.05, 0.005);
        }
        Ok(lights)
    }

    pub fn set_color(&mut self, color: &Vec3)
    {
        self.light_buffer.update(self.index_at(0), &color.to_slice()).unwrap();
    }

    pub fn set_intensity(&mut self, intensity: f32)
    {
        self.light_buffer.update(self.index_at(1), &[intensity]).unwrap();
    }

    pub fn set_attenuation(&mut self, constant: f32, linear: f32, exponential: f32)
    {
        self.light_buffer.update(self.index_at(2), &[constant]).unwrap();
        self.light_buffer.update(self.index_at(3), &[linear]).unwrap();
        self.light_buffer.update(self.index_at(4), &[exponential]).unwrap();
    }

    pub fn set_position(&mut self, position: &Vec3)
    {
        self.light_buffer.update(self.index_at(6), &position.to_slice()).unwrap();
    }

    pub(crate) fn buffer(&self) -> &UniformBuffer
    {
        &self.light_buffer
    }

    pub(crate) fn light_at(&mut self, index: usize) -> &mut Self
    {
        self.index = index;
        self
    }

    fn index_at(&self, index: usize) -> usize
    {
        self.index * 8 + index
    }
}

pub struct SpotLight {
    gl: Gl,
    light_buffer: UniformBuffer,
    shadow_rendertarget: DepthRenderTargetArray,
    shadow_cameras: [Option<Camera>; MAX_NO_LIGHTS],
    index: usize
}

impl SpotLight {

    pub(crate) fn new(gl: &Gl) -> Result<SpotLight, Error>
    {
        let uniform_sizes: Vec<u32> = [3u32, 1, 1, 1, 1, 1, 3, 1, 3, 1, 16].iter().cloned().cycle().take(11*MAX_NO_LIGHTS).collect();
        let mut lights = SpotLight {
            gl: gl.clone(),
            shadow_rendertarget: DepthRenderTargetArray::new(gl, 1024, 1024, MAX_NO_LIGHTS)?,
            light_buffer: UniformBuffer::new(gl, &uniform_sizes)?,
            shadow_cameras: [None, None, None, None],
            index: 0};

        for light_id in 0..MAX_NO_LIGHTS {
            let light = lights.light_at(light_id);
            light.set_intensity(0.0);
            light.set_color(&vec3(1.0, 1.0, 1.0));
            light.set_direction(&vec3(0.0, -1.0, 0.0));
            light.set_position(&vec3(0.0, 0.0, 0.0));
            light.set_attenuation(0.5, 0.05, 0.005);
            light.set_cutoff(0.1 * std::f32::consts::PI);
        }
        Ok(lights)
    }

    pub fn set_color(&mut self, color: &Vec3)
    {
        self.light_buffer.update(self.index_at(0), &color.to_slice()).unwrap();
    }

    pub fn set_intensity(&mut self, intensity: f32)
    {
        self.light_buffer.update(self.index_at(1), &[intensity]).unwrap();
    }

    pub fn set_attenuation(&mut self, constant: f32, linear: f32, exponential: f32)
    {
        self.light_buffer.update(self.index_at(2), &[constant]).unwrap();
        self.light_buffer.update(self.index_at(3), &[linear]).unwrap();
        self.light_buffer.update(self.index_at(4), &[exponential]).unwrap();
    }

    pub fn set_position(&mut self, position: &Vec3)
    {
        self.light_buffer.update(self.index_at(6), &position.to_slice()).unwrap();
        self.update_shadow_camera();
    }

    pub fn position(&self) -> Vec3
    {
        let p = self.light_buffer.get(self.index_at(6)).unwrap();
        vec3(p[0], p[1], p[2])
    }

    pub fn set_cutoff(&mut self, cutoff: f32)
    {
        self.light_buffer.update(self.index_at(7), &[cutoff]).unwrap();
        self.update_shadow_camera();
    }

    pub fn set_direction(&mut self, direction: &Vec3)
    {
        self.light_buffer.update(self.index_at(8), &direction.to_slice()).unwrap();
        self.update_shadow_camera();
    }

    pub fn direction(&self) -> Vec3
    {
        let d = self.light_buffer.get(self.index_at(8)).unwrap();
        vec3(d[0], d[1], d[2])
    }

    fn update_shadow_camera(&mut self)
    {
        let position = self.position();
        let direction = self.direction();
        let up = compute_up_direction(direction);

        let depth = 200.0;
        let cutoff = self.light_buffer.get(self.index_at(7)).unwrap()[0];

        if let Some(ref mut camera) = self.shadow_cameras[self.index]
        {
            camera.set_view(position, position + direction, up);
            camera.set_perspective_projection(degrees(45.0), 2.0 * cutoff, 0.1, depth);
            let shadow_matrix = shadow_matrix(camera);
            self.light_buffer.update(self.index_at(10), &shadow_matrix.to_slice()).unwrap();
        }
    }

    pub fn is_shadows_enabled(&self) -> bool {
        self.shadow_cameras[self.index].is_some()
    }

    pub fn enable_shadows(&mut self)
    {
        self.shadow_cameras[self.index] = Some(Camera::new());
        self.shadow_cameras[self.index].as_mut().unwrap().enable_matrix_buffer(&self.gl);
        self.update_shadow_camera();
    }

    pub fn disable_shadows(&mut self)
    {
        self.shadow_cameras[self.index] = None;
        self.light_buffer.update(self.index_at(10), &Mat4::from_value(0.0).to_slice()).unwrap();
    }

    pub(crate) fn shadow_pass<F>(&self, render_scene: &F)
        where F: Fn(&Camera)
    {
        for light_id in 0..MAX_NO_LIGHTS {
            if let Some(ref camera) = self.shadow_cameras[light_id]
            {
                self.shadow_rendertarget.bind(light_id);
                self.shadow_rendertarget.clear();
                render_scene(camera);
            }
        }
    }

    pub(crate) fn shadow_maps(&self) -> &Texture2DArray
    {
        &self.shadow_rendertarget.target
    }

    pub(crate) fn buffer(&self) -> &UniformBuffer
    {
        &self.light_buffer
    }

    pub(crate) fn light_at(&mut self, index: usize) -> &mut Self
    {
        self.index = index;
        self
    }

    fn index_at(&self, index: usize) -> usize
    {
        self.index * 11 + index
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

/*#[derive(Debug)]
pub enum Error {
    ShadowRendertargetNotAvailable {message: String}
}

pub struct Light {
    pub color: Vec3,
    pub intensity: f32
}

pub struct AmbientLight
{
    pub base: Light
}

impl AmbientLight
{
    pub fn new() -> AmbientLight
    {
        let base = Light { color: vec3(1.0, 1.0, 1.0), intensity: 0.5 };
        AmbientLight { base }
    }
}

pub struct DirectionalLight {
    pub color: Vec3,
    pub intensity: f32,
    pub direction: Vec3,
    pub shadow_camera: Camera
}

impl DirectionalLight
{
    pub fn new(gl: &Gl) -> DirectionalLight
    {
        DirectionalLight::new_with_direction(gl, vec3(0.0, -1.0, 0.0))
    }

    pub fn new_with_direction(gl: &Gl, direction: Vec3) -> DirectionalLight
    {
        let up = Self::compute_up_direction(direction);
        let radius = 2.0;
        let depth = 10.0;
        let shadow_camera = Camera::new_orthographic(gl, - direction, vec3(0.0, 0.0, 0.0), up,
                                                                  2.0 * radius, 2.0 * radius, 2.0 * depth);
        DirectionalLight {color: vec3(1.0, 1.0, 1.0), intensity: 0.5, direction: direction.normalize(), shadow_camera}
    }

    pub fn set_target(&mut self, target: &Vec3)
    {
        let up = Self::compute_up_direction(self.direction);
        self.shadow_camera.set_view(*target - self.direction, *target, up);
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
}

pub struct Attenuation {
    pub constant: f32,// = 0.1f;
    pub linear: f32,// = 0.01f;
    pub exp: f32// = 0.001f;
}

pub struct PointLight {
    pub base: Light,
    pub position: Vec3,
    pub attenuation: Attenuation
    // TODO: Shadows
}

impl PointLight
{
    pub fn new(position: Vec3) -> PointLight
    {
        let base = Light {color: vec3(1.0, 1.0, 1.0), intensity: 0.5};
        let attenuation = Attenuation {constant: 0.5, linear: 0.05, exp: 0.005};
        PointLight {base, position, attenuation}
    }
}

pub struct SpotLight {
    pub base: Light,
    pub direction: Vec3,
    pub position: Vec3,
    pub shadow_rendertarget: Option<DepthRenderTarget>,
    shadow_camera: Option<Camera>,
    pub attenuation: Attenuation,
    pub cutoff: f32
}

impl SpotLight
{
    pub fn new(position: Vec3, direction: Vec3) -> SpotLight
    {
        let base = Light {color: vec3(1.0, 1.0, 1.0), intensity: 0.5};
        let attenuation = Attenuation {constant: 0.2, linear: 0.02, exp: 0.002};
        SpotLight {base, direction: direction.normalize(), position, shadow_rendertarget: None, shadow_camera: None, attenuation, cutoff: 0.1 * std::f32::consts::PI}
    }

    pub fn enable_shadows(&mut self, gl: &Gl, depth: f32) -> Result<(), Error>
    {
        self.shadow_rendertarget = Some(DepthRenderTarget::new(gl, 512, 512).unwrap());
        let up = self.compute_up_direction();
        self.shadow_camera = Some(Camera::new_perspective(gl, self.position,self.position + self.direction, up,
                                                                 degrees(45.0), 2.0 * self.cutoff, 0.1, depth));
        Ok(())
    }

    fn compute_up_direction(&self) -> Vec3
    {
        if vec3(1.0, 0.0, 0.0).dot(self.direction).abs() > 0.9
        {
            (vec3(0.0, 1.0, 0.0).cross(self.direction)).normalize()
        }
        else {
            (vec3(1.0, 0.0, 0.0).cross(self.direction)).normalize()
        }
    }

    pub fn shadow_cast_begin(&self) -> Result<(), Error>
    {
        if let Some(ref rendertarget) = self.shadow_rendertarget
        {
            rendertarget.bind();
            rendertarget.clear();
            return Ok(())
        }
        Err(Error::ShadowRendertargetNotAvailable {message: format!("Shadow is not enabled for this light source")})
    }

    pub fn shadow_camera(&self) -> Result<&Camera, Error>
    {
        if let Some(ref camera) = self.shadow_camera {
            return Ok(camera)
        }
        Err(Error::ShadowRendertargetNotAvailable {message: format!("Shadow is not enabled for this light source")})
    }
}*/