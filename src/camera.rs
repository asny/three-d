use gust::*;

pub trait Camera
{
    fn get_view(&self) -> Mat4;
    fn get_projection(&self) -> Mat4;
    fn position(&self) -> &Vec3;
    fn target(&self) -> &Vec3;
    fn set_view(&mut self, position: Vec3, target: Vec3);
    fn set_target_screen_size(&mut self, width: usize, height: usize);
}

struct BaseCamera {
    position: Vec3,
    target: Vec3,
    z_near: f32,
    z_far: f32
}

pub struct PerspectiveCamera {
    base: BaseCamera,
    aspect: f32
}


impl PerspectiveCamera
{
    pub fn new(position: Vec3, target: Vec3, width: usize, height: usize) -> PerspectiveCamera
    {
        PerspectiveCamera { base: BaseCamera { position, target, z_near: 0.1, z_far: 1000.0 }, aspect: (width as f32)/(height as f32) }
    }
}

impl Camera for PerspectiveCamera
{
    fn get_view(&self) -> Mat4
    {
        Mat4::look_at_rh(&na::Point::from_coordinates(self.base.position), &na::Point::from_coordinates(self.base.target), &vec3(0., 1., 0.))
    }

    fn get_projection(&self) -> Mat4
    {
        Mat4::new_perspective(self.aspect, 0.25 * ::std::f32::consts::PI, self.base.z_near, self.base.z_far)
    }

    fn position(&self) -> &Vec3
    {
        &self.base.position
    }

    fn target(&self) -> &Vec3
    {
        &self.base.target
    }

    fn set_view(&mut self, position: Vec3, target: Vec3)
    {
        self.base.position = position;
        self.base.target = target;
    }

    fn set_target_screen_size(&mut self, width: usize, height: usize)
    {
        self.aspect = (width as f32)/(height as f32);
    }
}

pub struct ShadowCamera {
    base: BaseCamera,
    half_width: usize,
    half_height: usize
}

impl ShadowCamera
{
    pub fn new(position: Vec3, target: Vec3, radius: usize) -> ShadowCamera
    {
        ShadowCamera { base: BaseCamera { position, target, z_near: 0.1, z_far: 1000.0 }, half_width: radius, half_height: radius }
    }
}

impl Camera for ShadowCamera
{
    fn get_view(&self) -> Mat4
    {
        let up = (vec3(1.0, 0.0, 0.0).cross(&(self.base.target - self.base.position))).normalize();
        Mat4::look_at_rh(&na::Point::from_coordinates(self.base.position), &na::Point::from_coordinates(self.base.target), &up)
    }

    fn get_projection(&self) -> Mat4
    {
        Mat4::new_orthographic(-(self.half_width as f32), self.half_width as f32, -(self.half_height as f32), self.half_height as f32, self.base.z_near, self.base.z_far)
    }

    fn position(&self) -> &Vec3
    {
        &self.base.position
    }

    fn target(&self) -> &Vec3
    {
        &self.base.target
    }

    fn set_view(&mut self, position: Vec3, target: Vec3)
    {
        self.base.position = position;
        self.base.target = target;
    }

    fn set_target_screen_size(&mut self, width: usize, height: usize)
    {
        self.half_width = width/2;
        self.half_height = height/2;
    }
}