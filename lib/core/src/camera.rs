
use crate::*;

pub trait Camera
{
    fn get_view(&self) -> &Mat4;
    fn get_projection(&self) -> &Mat4;
    fn position(&self) -> &Vec3;
    fn target(&self) -> &Vec3;
    fn up(&self) -> &Vec3;
    fn set_view(&mut self, position: Vec3, target: Vec3, up: Vec3);
    fn mirror_in_xz_plane(&mut self);
    fn view_direction_at(&self, screen_coordinates: (f64, f64)) -> Vec3;
    fn matrix_buffer(&self) -> Option<&UniformBuffer>;
}

pub struct BaseCamera {
    position: Vec3,
    target: Vec3,
    up: Vec3,
    view: Mat4,
    projection: Mat4,
    screen2ray: Mat4,
    matrix_buffer: Option<UniformBuffer>
}

impl BaseCamera
{
    pub fn set_view(&mut self, position: Vec3, target: Vec3, up: Vec3)
    {
        self.position = position;
        self.target = target;
        let dir = (target - position).normalize();
        self.up = dir.cross(up.normalize().cross(dir));
        self.view = Mat4::look_at(Point::from_vec(self.position), Point::from_vec(self.target), self.up);
        self.update_screen2ray();
        self.update_matrix_buffer();
    }

    pub fn mirror_in_xz_plane(&mut self)
    {
        self.view[1][0] = -self.view[1][0];
        self.view[1][1] = -self.view[1][1];
        self.view[1][2] = -self.view[1][2];
        self.update_screen2ray();
        self.update_matrix_buffer();
    }

    pub fn view_direction_at(&self, screen_coordinates: (f64, f64)) -> Vec3
    {
        let screen_pos = vec4(2. * screen_coordinates.0 as f32 - 1., 1. - 2. * screen_coordinates.1 as f32, 0., 1.);
        (self.screen2ray * screen_pos).truncate().normalize()
    }

    pub fn update_screen2ray(&mut self)
    {
        let mut v = self.view.clone();
        v[3] = vec4(0.0, 0.0, 0.0, 1.0);
        self.screen2ray = (self.projection * v).invert().unwrap();
    }

    pub fn update_matrix_buffer(&self)
    {
        let data = self.get_view().to_slice();
        self.matrix_buffer.as_ref().unwrap().fill_with(&data);
    }

    pub fn get_view(&self) -> &Mat4
    {
        &self.view
    }

    pub fn get_projection(&self) -> &Mat4
    {
        &self.projection
    }

    pub fn position(&self) -> &Vec3
    {
        &self.position
    }

    pub fn target(&self) -> &Vec3
    {
        &self.target
    }

    pub fn up(&self) -> &Vec3
    {
        &self.up
    }
}

pub struct PerspectiveCamera {
    base: BaseCamera
}

impl PerspectiveCamera
{
    pub fn new(position: Vec3, target: Vec3, up: Vec3, fovy: Degrees, aspect: f32, z_near: f32, z_far: f32) -> PerspectiveCamera
    {
        let mut camera = PerspectiveCamera { base: BaseCamera {matrix_buffer: None, position, target, up, view: Mat4::identity(), projection: Mat4::identity(), screen2ray: Mat4::identity()} };
        camera.set_view(position, target, up);
        camera.set_extent(fovy, aspect, z_near, z_far);
        camera
    }

    pub fn new_with_uniform_buffer(gl: &Gl, position: Vec3, target: Vec3, up: Vec3, fovy: Degrees, aspect: f32, z_near: f32, z_far: f32) -> PerspectiveCamera
    {
        let mut camera = PerspectiveCamera { base: BaseCamera {matrix_buffer: Some(UniformBuffer::new(gl).unwrap()), position, target, up, view: Mat4::identity(), projection: Mat4::identity(), screen2ray: Mat4::identity()} };
        camera.set_view(position, target, up);
        camera.set_extent(fovy, aspect, z_near, z_far);
        camera
    }

    pub fn set_extent(&mut self, fovy: Degrees, aspect: f32, z_near: f32, z_far: f32)
    {
        if z_near < 0.0 || z_near > z_far { panic!("Wrong perspective camera parameters") };
        self.base.projection = perspective(fovy, aspect, z_near, z_far);
        self.base.update_screen2ray();
        self.base.update_matrix_buffer();
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
        &self.base.projection
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

    fn view_direction_at(&self, screen_coordinates: (f64, f64)) -> Vec3
    {
        self.base.view_direction_at(screen_coordinates)
    }

    fn matrix_buffer(&self) -> Option<&UniformBuffer>
    {
        self.base.matrix_buffer.as_ref()
    }
}

pub struct OrthographicCamera {
    base: BaseCamera
}

impl OrthographicCamera
{
    pub fn new(position: Vec3, target: Vec3, up: Vec3, width: f32, height: f32, depth: f32) -> OrthographicCamera
    {
        let mut camera = OrthographicCamera { base: BaseCamera {matrix_buffer: None, position, target, up, view: Mat4::identity(), projection: Mat4::identity(), screen2ray: Mat4::identity()} };
        camera.set_view(position, target, up);
        camera.set_extent(width, height, depth);
        camera
    }

    pub fn new_with_uniform_buffer(gl: &Gl, position: Vec3, target: Vec3, up: Vec3, width: f32, height: f32, depth: f32) -> OrthographicCamera
    {
        let mut camera = OrthographicCamera { base: BaseCamera {matrix_buffer: Some(UniformBuffer::new(gl).unwrap()), position, target, up, view: Mat4::identity(), projection: Mat4::identity(), screen2ray: Mat4::identity()} };
        camera.set_view(position, target, up);
        camera.set_extent(width, height, depth);
        camera
    }

    fn set_extent(&mut self, width: f32, height: f32, depth: f32)
    {
        self.base.projection = ortho(-0.5 * width, 0.5 * width, -0.5 * height, 0.5 * height, -0.5 * depth, 0.5 * depth);
        self.base.update_screen2ray();
        self.base.update_matrix_buffer();
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
        &self.base.projection
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

    fn view_direction_at(&self, screen_coordinates: (f64, f64)) -> Vec3
    {
        self.base.view_direction_at(screen_coordinates)
    }

    fn matrix_buffer(&self) -> Option<&UniformBuffer>
    {
        self.base.matrix_buffer.as_ref()
    }
}