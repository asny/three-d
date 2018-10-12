use gust::*;

pub trait Camera
{
    fn get_view(&self) -> Mat4;
    fn get_projection(&self) -> Mat4;
    fn position(&self) -> &Vec3;
    fn target(&self) -> &Vec3;
    fn screen_width(&self) -> usize;
    fn screen_height(&self) -> usize;
    fn set_view(&mut self, position: Vec3, target: Vec3);
}

struct BaseCamera {
    pub position: Vec3,
    pub target: Vec3,
    z_near: f32,
    z_far: f32
}

pub struct PerspectiveCamera {
    base: BaseCamera,
    width: usize,
    height: usize
}


impl PerspectiveCamera
{
    pub fn new(position: Vec3, target: Vec3, width: usize, height: usize) -> PerspectiveCamera
    {
        PerspectiveCamera { base: BaseCamera { position, target, z_near: 0.1, z_far: 1000.0 }, width, height }
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
        Mat4::new_perspective((self.width as f32)/(self.height as f32), 0.25 * ::std::f32::consts::PI, self.base.z_near, self.base.z_far)
    }

    fn position(&self) -> &Vec3
    {
        &self.base.position
    }

    fn target(&self) -> &Vec3
    {
        &self.base.target
    }

    fn screen_width(&self) -> usize
    {
        self.width
    }

    fn screen_height(&self) -> usize
    {
        self.height
    }

    fn set_view(&mut self, position: Vec3, target: Vec3)
    {
        self.base.position = position;
        self.base.target = target;
    }
}

pub struct ShadowCamera {
    base: BaseCamera,
    radius: usize
}

impl ShadowCamera
{
    pub fn new(position: Vec3, target: Vec3, radius: usize) -> ShadowCamera
    {
        ShadowCamera { base: BaseCamera { position, target, z_near: 0.1, z_far: 1000.0 }, radius }
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
        Mat4::new_orthographic(-(self.radius as f32), self.radius as f32, -(self.radius as f32), self.radius as f32, self.base.z_near, self.base.z_far)
    }

    fn position(&self) -> &Vec3
    {
        &self.base.position
    }

    fn target(&self) -> &Vec3
    {
        &self.base.target
    }

    fn screen_width(&self) -> usize
    {
        2 * self.radius
    }

    fn screen_height(&self) -> usize
    {
        2 * self.radius
    }

    fn set_view(&mut self, position: Vec3, target: Vec3)
    {
        self.base.position = position;
        self.base.target = target;
    }
}