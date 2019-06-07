
use crate::*;

pub struct Camera {
    position: Vec3,
    target: Vec3,
    up: Vec3,
    view: Mat4,
    projection: Mat4,
    screen2ray: Mat4,
    matrix_buffer: UniformBuffer
}

impl Camera
{
    pub fn new(gl: &Gl, position: Vec3, target: Vec3, up: Vec3) -> Camera
    {
        let mut camera = Camera {matrix_buffer: UniformBuffer::new(gl, &vec![16, 16, 16]).unwrap(), position, target, up, view: Mat4::identity(), projection: Mat4::identity(), screen2ray: Mat4::identity()};
        camera.set_view(position, target, up);
        camera
    }

    pub fn new_orthographic(gl: &Gl, position: Vec3, target: Vec3, up: Vec3, width: f32, height: f32, depth: f32) -> Camera
    {
        let mut camera = Camera::new(gl, position, target, up);
        camera.set_orthographic_projection(width, height, depth);
        camera
    }

    pub fn new_perspective(gl: &Gl, position: Vec3, target: Vec3, up: Vec3, fovy: Degrees, aspect: f32, z_near: f32, z_far: f32) -> Camera
    {
        let mut camera = Camera::new(gl, position, target, up);
        camera.set_perspective_projection(fovy, aspect, z_near, z_far);
        camera
    }

    pub fn set_perspective_projection(&mut self, fovy: Degrees, aspect: f32, z_near: f32, z_far: f32)
    {
        if z_near < 0.0 || z_near > z_far { panic!("Wrong perspective camera parameters") };
        self.projection = perspective(fovy, aspect, z_near, z_far);
        self.update_screen2ray();
        self.update_matrix_buffer();
    }

    pub fn set_orthographic_projection(&mut self, width: f32, height: f32, depth: f32)
    {
        self.projection = ortho(-0.5 * width, 0.5 * width, -0.5 * height, 0.5 * height, -0.5 * depth, 0.5 * depth);
        self.update_screen2ray();
        self.update_matrix_buffer();
    }

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

    pub fn matrix_buffer(&self) -> &UniformBuffer
    {
        &self.matrix_buffer
    }

    fn update_screen2ray(&mut self)
    {
        let mut v = self.view.clone();
        v[3] = vec4(0.0, 0.0, 0.0, 1.0);
        self.screen2ray = (self.projection * v).invert().unwrap();
    }

    fn update_matrix_buffer(&mut self)
    {
        self.matrix_buffer.update(0, &self.view.to_slice()).unwrap();
        self.matrix_buffer.update(1, &self.projection.to_slice()).unwrap();
        self.matrix_buffer.update(2, &(self.projection * self.view).to_slice()).unwrap();
    }
}