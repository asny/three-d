use gust::*;

pub trait Camera
{
    fn get_view(&self) -> Mat4;
    fn get_projection(&self) -> Mat4;
    fn position(&self) -> &Vec3;
    fn screen_width(&self) -> usize;
    fn screen_height(&self) -> usize;
}

pub struct PerspectiveCamera {
    pub position: Vec3,
    pub target: Vec3,
    z_near: f32,
    z_far: f32,
    pub width: usize,
    pub height: usize
}


impl PerspectiveCamera
{
    pub fn create(position: Vec3, target: Vec3, width: usize, height: usize) -> PerspectiveCamera
    {
        PerspectiveCamera { position, target, z_near: 0.1, z_far: 1000.0, width, height }
    }

    pub fn set_view(&mut self, position: Vec3, target: Vec3)
    {
        self.position = position;
        self.target = target;
    }

    pub fn direction(&self) -> Vec3
    {
        (self.target - self.position).normalize()
    }
}

impl Camera for PerspectiveCamera
{
    fn get_view(&self) -> Mat4
    {
        Mat4::look_at_rh(&na::Point::from_coordinates(self.position), &na::Point::from_coordinates(self.target), &vec3(0., 1., 0.))
    }

    fn get_projection(&self) -> Mat4
    {
        Mat4::new_perspective((self.width as f32)/(self.height as f32), 0.25 * ::std::f32::consts::PI, self.z_near, self.z_far)
    }

    fn position(&self) -> &Vec3
    {
        &self.position
    }

    fn screen_width(&self) -> usize
    {
        self.width
    }

    fn screen_height(&self) -> usize
    {
        self.height
    }
}
