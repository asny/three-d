use gust::*;

pub trait Camera
{
    fn get_view(&self) -> Mat4;
    fn get_projection(&self) -> Mat4;
    fn position(&self) -> &Vec3;
    fn target(&self) -> &Vec3;
    fn set_view(&mut self, position: Vec3, target: Vec3);
}

struct BaseCamera {
    position: Vec3,
    target: Vec3
}

pub struct PerspectiveCamera {
    base: BaseCamera,
    aspect: f32,
    z_near: f32,
    z_far: f32
}


impl PerspectiveCamera
{
    pub fn new(position: Vec3, target: Vec3, aspect: f32) -> PerspectiveCamera
    {
        PerspectiveCamera { base: BaseCamera { position, target }, z_near: 0.1, z_far: 1000.0, aspect }
    }

    fn set_aspect(&mut self, aspect: f32)
    {
        self.aspect = aspect;
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
        Mat4::new_perspective(self.aspect, 0.25 * ::std::f32::consts::PI, self.z_near, self.z_far)
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
}

pub struct ShadowCamera {
    base: BaseCamera,
    radius: f32
}

impl ShadowCamera
{
    pub fn new(position: Vec3, target: Vec3, radius: f32) -> ShadowCamera
    {
        ShadowCamera { base: BaseCamera { position, target }, radius }
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
        Mat4::new_orthographic(-self.radius, self.radius, -self.radius, self.radius, -self.radius, self.radius)
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
}