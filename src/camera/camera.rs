
use crate::math::*;
use crate::core::*;

pub enum ProjectionType {
    Orthographic {
        width: f32,
        height: f32,
        depth: f32
    },
    Perspective {
        fovy: Degrees,
        aspect: f32,
        z_near: f32,
        z_far: f32
    }
}

pub struct Camera {
    projection_type: ProjectionType,
    position: Vec3,
    target: Vec3,
    up: Vec3,
    view: Mat4,
    projection: Mat4,
    screen2ray: Mat4,
    matrix_buffer: UniformBuffer,
    frustrum: [Vec4; 6]
}

impl Camera
{
    pub fn new_orthographic(context: &Context, position: Vec3, target: Vec3, up: Vec3, width: f32, height: f32, depth: f32)  -> Result<Camera, Error>
    {
        let mut camera = Camera::new(context);
        camera.set_view(position, target, up)?;
        camera.set_orthographic_projection(width, height, depth)?;
        Ok(camera)
    }

    pub fn new_perspective(context: &Context, position: Vec3, target: Vec3, up: Vec3, fovy: Degrees, aspect: f32, z_near: f32, z_far: f32) -> Result<Camera, Error>
    {
        let mut camera = Camera::new(context);
        camera.set_view(position, target, up)?;
        camera.set_perspective_projection(fovy, aspect, z_near, z_far)?;
        Ok(camera)
    }

    pub fn set_perspective_projection(&mut self, fovy: Degrees, aspect: f32, z_near: f32, z_far: f32) -> Result<(), Error>
    {
        if z_near < 0.0 || z_near > z_far { panic!("Wrong perspective camera parameters") };
        self.projection_type = ProjectionType::Perspective { fovy, aspect, z_near, z_far };
        self.projection = perspective(fovy, aspect, z_near, z_far);
        self.update_screen2ray();
        self.update_matrix_buffer()?;
        self.update_frustrum();
        Ok(())
    }

    pub fn set_orthographic_projection(&mut self, width: f32, height: f32, depth: f32) -> Result<(), Error>
    {
        self.projection_type = ProjectionType::Orthographic { width, height, depth };
        self.projection = ortho(-0.5 * width, 0.5 * width, -0.5 * height, 0.5 * height, 0.0, depth);
        self.update_screen2ray();
        self.update_matrix_buffer()?;
        self.update_frustrum();
        Ok(())
    }

    pub fn set_aspect(&mut self, value: f32) -> Result<bool, Error> {
        let mut change = false;
        match self.projection_type {
            ProjectionType::Orthographic {width, height, depth} => {
                if (width / height - value).abs() > 0.001
                {
                    self.set_orthographic_projection(height * value, height, depth)?;
                    change = true;
                }
            },
            ProjectionType::Perspective {aspect, fovy, z_near, z_far} => {
                if (aspect - value).abs() > 0.001
                {
                    self.set_perspective_projection(fovy, value, z_near, z_far)?;
                    change = true;
                }
            }
        }
        Ok(change)
    }

    pub fn set_view(&mut self, position: Vec3, target: Vec3, up: Vec3) -> Result<(), Error>
    {
        self.position = position;
        self.target = target;
        self.up = up;
        self.view = Mat4::look_at(Point::from_vec(self.position), Point::from_vec(self.target), self.up);
        self.update_screen2ray();
        self.update_matrix_buffer()?;
        self.update_frustrum();
        Ok(())
    }

    pub fn mirror_in_xz_plane(&mut self) -> Result<(), Error>
    {
        self.view[1][0] = -self.view[1][0];
        self.view[1][1] = -self.view[1][1];
        self.view[1][2] = -self.view[1][2];
        self.update_screen2ray();
        self.update_matrix_buffer()?;
        self.update_frustrum();
        Ok(())
    }

    // false if fully outside, true if inside or intersects
    pub fn in_frustum(&self, aabb: &AxisAlignedBoundingBox) -> bool
    {
        // check box outside/inside of frustum
        for i in 0..6
        {
            let mut out = 0;
            if self.frustrum[i].dot(vec4(aabb.min.x, aabb.min.y, aabb.min.z, 1.0)) < 0.0 {out += 1};
            if self.frustrum[i].dot(vec4(aabb.max.x, aabb.min.y, aabb.min.z, 1.0)) < 0.0 {out += 1};
            if self.frustrum[i].dot(vec4(aabb.min.x, aabb.max.y, aabb.min.z, 1.0)) < 0.0 {out += 1};
            if self.frustrum[i].dot(vec4(aabb.max.x, aabb.max.y, aabb.min.z, 1.0)) < 0.0 {out += 1};
            if self.frustrum[i].dot(vec4(aabb.min.x, aabb.min.y, aabb.max.z, 1.0)) < 0.0 {out += 1};
            if self.frustrum[i].dot(vec4(aabb.max.x, aabb.min.y, aabb.max.z, 1.0)) < 0.0 {out += 1};
            if self.frustrum[i].dot(vec4(aabb.min.x, aabb.max.y, aabb.max.z, 1.0)) < 0.0 {out += 1};
            if self.frustrum[i].dot(vec4(aabb.max.x, aabb.max.y, aabb.max.z, 1.0)) < 0.0 {out += 1};
            if out == 8 {return false;}
        }
        // TODO: Test the frustum corners against the box planes (http://www.iquilezles.org/www/articles/frustumcorrect/frustumcorrect.htm)

        return true;
    }

    pub fn view_direction_at(&self, screen_coordinates: (f64, f64)) -> Vec3
    {
        let screen_pos = vec4(2. * screen_coordinates.0 as f32 - 1., 1. - 2. * screen_coordinates.1 as f32, 0., 1.);
        (self.screen2ray * screen_pos).truncate().normalize()
    }

    pub fn projection_type(&self) -> &ProjectionType {
        &self.projection_type
    }

    pub fn view(&self) -> &Mat4
    {
        &self.view
    }

    pub fn projection(&self) -> &Mat4
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

    fn new(context: &Context) -> Camera
    {
        Camera {
            projection_type: ProjectionType::Orthographic {width: 1.0, height: 1.0, depth: 1.0},
            matrix_buffer: UniformBuffer::new(context, &vec![16, 16, 16, 3, 1]).unwrap(),
            frustrum: [vec4(0.0, 0.0, 0.0, 0.0); 6],
            position: vec3(0.0, 0.0, 5.0), target: vec3(0.0, 0.0, 0.0), up: vec3(0.0, 1.0, 0.0),
            view: Mat4::identity(), projection: Mat4::identity(), screen2ray: Mat4::identity()}
    }

    fn update_screen2ray(&mut self)
    {
        let mut v = self.view.clone();
        v[3] = vec4(0.0, 0.0, 0.0, 1.0);
        self.screen2ray = (self.projection * v).invert().unwrap();
    }

    fn update_matrix_buffer(&mut self) -> Result<(), Error>
    {
        self.matrix_buffer.update(0, &(self.projection * self.view).to_slice())?;
        self.matrix_buffer.update(1, &self.view.to_slice())?;
        self.matrix_buffer.update(2, &self.projection.to_slice())?;
        self.matrix_buffer.update(3, &self.position.to_slice())?;
        Ok(())
    }

    fn update_frustrum(&mut self)
    {
        let m = self.projection * self.view;
        self.frustrum = [vec4(m.x.w + m.x.x, m.y.w + m.y.x, m.z.w + m.z.x, m.w.w + m.w.x),
         vec4(m.x.w - m.x.x, m.y.w - m.y.x, m.z.w - m.z.x, m.w.w - m.w.x),
         vec4(m.x.w + m.x.y, m.y.w + m.y.y,m.z.w + m.z.y, m.w.w + m.w.y),
         vec4(m.x.w - m.x.y, m.y.w - m.y.y,m.z.w - m.z.y, m.w.w - m.w.y),
         vec4(m.x.w + m.x.z,m.y.w + m.y.z,m.z.w + m.z.z, m.w.w + m.w.z),
         vec4(m.x.w - m.x.z,m.y.w - m.y.z,m.z.w - m.z.z, m.w.w - m.w.z)];
    }
}