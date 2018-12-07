use geo_proc::*;

pub trait Camera
{
    fn get_view(&self) -> &Mat4;
    fn get_projection(&self) -> &Mat4;
    fn position(&self) -> &Vec3;
    fn target(&self) -> &Vec3;
    fn up(&self) -> &Vec3;
    fn set_view(&mut self, position: Vec3, target: Vec3, up: Vec3);
    fn mirror_in_xz_plane(&mut self);
}

struct BaseCamera {
    position: Vec3,
    target: Vec3,
    up: Vec3,
    view: Mat4
}

impl BaseCamera
{
    pub fn set_view(&mut self, position: Vec3, target: Vec3, up: Vec3)
    {
        self.position = position;
        self.target = target;
        let dir = (target - position).normalize();
        self.up = dir.cross(&up.normalize().cross(&dir));
        self.view = Mat4::look_at_rh(&Point::from_coordinates(self.position), &Point::from_coordinates(self.target), &self.up);
    }

    pub fn mirror_in_xz_plane(&mut self)
    {
        self.view[(0,1)] = -self.view[(0,1)];
        self.view[(1,1)] = -self.view[(1,1)];
        self.view[(2,1)] = -self.view[(2,1)];
    }
}

pub struct PerspectiveCamera {
    base: BaseCamera,
    projection: Mat4
}

impl PerspectiveCamera
{
    pub fn new(position: Vec3, target: Vec3, up: Vec3, aspect: f32, fovy: f32, z_near: f32, z_far: f32) -> PerspectiveCamera
    {
        let mut camera = PerspectiveCamera { base: BaseCamera {position, target, up, view: Mat4::identity()}, projection: Mat4::identity() };
        camera.set_view(position, target, up);
        camera.set_extent(aspect, fovy, z_near, z_far);
        camera
    }

    pub fn set_extent(&mut self, aspect: f32, fovy: f32, z_near: f32, z_far: f32)
    {
        if z_near < 0.0 || z_near > z_far { panic!("Wrong perspective camera parameters") };
        self.projection = Mat4::new_perspective(aspect, fovy, z_near, z_far);
    }
}

impl Camera for PerspectiveCamera
{
    fn get_view(&self) -> &Mat4
    {
        &self.base.view
    }

    fn get_projection(&self) -> &Mat4
    {
        &self.projection
    }

    fn position(&self) -> &Vec3
    {
        &self.base.position
    }

    fn target(&self) -> &Vec3
    {
        &self.base.target
    }

    fn up(&self) -> &Vec3
    {
        &self.base.up
    }

    fn set_view(&mut self, position: Vec3, target: Vec3, up: Vec3)
    {
        self.base.set_view(position, target, up);
    }

    fn mirror_in_xz_plane(&mut self)
    {
        self.base.mirror_in_xz_plane();
    }
}

pub struct OrthographicCamera {
    base: BaseCamera,
    projection: Mat4
}

impl OrthographicCamera
{
    pub fn new(position: Vec3, target: Vec3, up: Vec3, width: f32, height: f32, depth: f32) -> OrthographicCamera
    {
        let mut camera = OrthographicCamera { base: BaseCamera {position, target, up, view: Mat4::identity()}, projection: Mat4::identity() };
        camera.set_view(position, target, up);
        camera.set_extent(width, height, depth);
        camera
    }

    fn set_extent(&mut self, width: f32, height: f32, depth: f32)
    {
        self.projection = Mat4::new_orthographic(-0.5 * width, 0.5 * width, -0.5 * height, 0.5 * height, -0.5 * depth, 0.5 * depth);
    }
}

impl Camera for OrthographicCamera
{
    fn get_view(&self) -> &Mat4
    {
        &self.base.view
    }

    fn get_projection(&self) -> &Mat4
    {
        &self.projection
    }

    fn position(&self) -> &Vec3
    {
        &self.base.position
    }

    fn target(&self) -> &Vec3
    {
        &self.base.target
    }

    fn up(&self) -> &Vec3
    {
        &self.base.up
    }

    fn set_view(&mut self, position: Vec3, target: Vec3, up: Vec3)
    {
        self.base.set_view(position, target, up);
    }

    fn mirror_in_xz_plane(&mut self)
    {
        self.base.mirror_in_xz_plane();
    }
}