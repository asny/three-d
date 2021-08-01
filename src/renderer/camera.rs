use crate::renderer::*;

pub(super) enum ProjectionType {
    Orthographic { width: f32, height: f32 },
    Perspective { field_of_view_y: Radians },
}

///
/// Used in a render call to define how to view the 3D world.
///
pub struct Camera {
    context: Context,
    viewport: Viewport,
    projection_type: ProjectionType,
    z_near: f32,
    z_far: f32,
    position: Vec3,
    target: Vec3,
    up: Vec3,
    view: Mat4,
    projection: Mat4,
    screen2ray: Mat4,
    uniform_buffer: UniformBuffer,
    frustrum: [Vec4; 6],
}

impl Camera {
    ///
    /// New camera which projects the world with an orthographic projection.
    /// See also [set_view](Self::set_view), [set_perspective_projection](Self::set_perspective_projection) and
    /// [set_orthographic_projection](Self::set_orthographic_projection).
    ///
    pub fn new_orthographic(
        context: &Context,
        viewport: Viewport,
        position: Vec3,
        target: Vec3,
        up: Vec3,
        width: f32,
        height: f32,
        depth: f32,
    ) -> Result<Camera, Error> {
        let mut camera = Camera::new(context, viewport);
        camera.set_view(position, target, up)?;
        camera.set_orthographic_projection(width, height, depth)?;
        Ok(camera)
    }

    ///
    /// New camera which projects the world with a perspective projection.
    ///
    pub fn new_perspective(
        context: &Context,
        viewport: Viewport,
        position: Vec3,
        target: Vec3,
        up: Vec3,
        field_of_view_y: impl Into<Radians>,
        z_near: f32,
        z_far: f32,
    ) -> Result<Camera, Error> {
        let mut camera = Camera::new(context, viewport);
        camera.set_view(position, target, up)?;
        camera.set_perspective_projection(field_of_view_y, z_near, z_far)?;
        Ok(camera)
    }

    ///
    /// Specify the camera to use perspective projection with the given field of view in the y-direction and near and far plane.
    ///
    pub fn set_perspective_projection(
        &mut self,
        field_of_view_y: impl Into<Radians>,
        z_near: f32,
        z_far: f32,
    ) -> Result<(), Error> {
        if z_near < 0.0 || z_near > z_far {
            panic!("Wrong perspective camera parameters")
        };
        self.z_near = z_near;
        self.z_far = z_far;
        let field_of_view_y = field_of_view_y.into();
        self.projection_type = ProjectionType::Perspective { field_of_view_y };
        self.projection = perspective(field_of_view_y, self.viewport.aspect(), z_near, z_far);
        self.update_screen2ray();
        self.update_uniform_buffer()?;
        self.update_frustrum();
        Ok(())
    }

    ///
    /// Specify the camera to use orthographic projection with the given height and depth.
    /// The view frustum height is +/- height/2
    /// The view frustum width is calculated as height * viewport.width / viewport.height.
    /// The view frustum depth is z_near to z_far.
    ///
    pub fn set_orthographic_projection(
        &mut self,
        height: f32,
        z_near: f32,
        z_far: f32,
    ) -> Result<(), Error> {
        if z_near > z_far {
            panic!("Wrong orthographic camera parameters")
        };
        self.z_near = z_near;
        self.z_far = z_far;
        let width = height * self.viewport.aspect();
        self.projection_type = ProjectionType::Orthographic { width, height };
        self.projection = ortho(
            -0.5 * width,
            0.5 * width,
            -0.5 * height,
            0.5 * height,
            z_near,
            z_far,
        );
        self.update_screen2ray();
        self.update_uniform_buffer()?;
        self.update_frustrum();
        Ok(())
    }

    ///
    /// Set the current [viewport](crate::Viewport).
    /// Returns whether or not the viewport actually changed.
    ///
    pub fn set_viewport(&mut self, viewport: Viewport) -> Result<bool, Error> {
        if self.viewport != viewport {
            self.viewport = viewport;
            match self.projection_type {
                ProjectionType::Orthographic { width: _, height } => {
                    self.set_orthographic_projection(height, self.z_near, self.z_far)?;
                }
                ProjectionType::Perspective { field_of_view_y } => {
                    self.set_perspective_projection(field_of_view_y, self.z_near, self.z_far)?;
                }
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }

    ///
    /// Change the view of the camera.
    /// The camera is placed at the given position, looking at the given target and with the given up direction.
    ///
    pub fn set_view(&mut self, position: Vec3, target: Vec3, up: Vec3) -> Result<(), Error> {
        self.position = position;
        self.target = target;
        self.up = up;
        self.view = Mat4::look_at(
            Point::from_vec(self.position),
            Point::from_vec(self.target),
            self.up,
        );
        self.update_screen2ray();
        self.update_uniform_buffer()?;
        self.update_frustrum();
        Ok(())
    }

    ///
    /// Change the camera view such that it is mirrored in the xz-plane.
    ///
    pub fn mirror_in_xz_plane(&mut self) -> Result<(), Error> {
        self.view[1][0] = -self.view[1][0];
        self.view[1][1] = -self.view[1][1];
        self.view[1][2] = -self.view[1][2];
        self.update_screen2ray();
        self.update_uniform_buffer()?;
        self.update_frustrum();
        Ok(())
    }

    ///
    /// Returns whether or not the given bounding box is within the camera frustum.
    /// It returns false if it is fully outside and true if it is inside or intersects.
    ///
    pub fn in_frustum(&self, aabb: &AxisAlignedBoundingBox) -> bool {
        // check box outside/inside of frustum
        for i in 0..6 {
            let mut out = 0;
            if self.frustrum[i].dot(vec4(aabb.min().x, aabb.min().y, aabb.min().z, 1.0)) < 0.0 {
                out += 1
            };
            if self.frustrum[i].dot(vec4(aabb.max().x, aabb.min().y, aabb.min().z, 1.0)) < 0.0 {
                out += 1
            };
            if self.frustrum[i].dot(vec4(aabb.min().x, aabb.max().y, aabb.min().z, 1.0)) < 0.0 {
                out += 1
            };
            if self.frustrum[i].dot(vec4(aabb.max().x, aabb.max().y, aabb.min().z, 1.0)) < 0.0 {
                out += 1
            };
            if self.frustrum[i].dot(vec4(aabb.min().x, aabb.min().y, aabb.max().z, 1.0)) < 0.0 {
                out += 1
            };
            if self.frustrum[i].dot(vec4(aabb.max().x, aabb.min().y, aabb.max().z, 1.0)) < 0.0 {
                out += 1
            };
            if self.frustrum[i].dot(vec4(aabb.min().x, aabb.max().y, aabb.max().z, 1.0)) < 0.0 {
                out += 1
            };
            if self.frustrum[i].dot(vec4(aabb.max().x, aabb.max().y, aabb.max().z, 1.0)) < 0.0 {
                out += 1
            };
            if out == 8 {
                return false;
            }
        }
        // TODO: Test the frustum corners against the box planes (http://www.iquilezles.org/www/articles/frustumcorrect/frustumcorrect.htm)

        true
    }

    ///
    /// Finds the closest intersection between a ray from this camera in the given pixel coordinate and the given geometries.
    /// The pixel coordinate must be in physical pixels, where (viewport.x, viewport.y) indicate the top left corner of the viewport
    /// and (viewport.x + viewport.width, viewport.y + viewport.height) indicate the bottom right corner.
    /// Returns ```None``` if no geometry was hit before the given maximum depth.
    ///
    pub fn pick(
        &self,
        pixel: (f32, f32),
        max_depth: f32,
        objects: &[&dyn Geometry],
    ) -> Result<Option<Vec3>, Error> {
        let pos = self.position_at_pixel(pixel);
        let dir = self.view_direction_at_pixel(pixel);
        ray_intersect(&self.context, pos, dir, max_depth, objects)
    }

    ///
    /// Returns the 3D position at the given pixel coordinate.
    /// The pixel coordinate must be in physical pixels, where (viewport.x, viewport.y) indicate the top left corner of the viewport
    /// and (viewport.x + viewport.width, viewport.y + viewport.height) indicate the bottom right corner.
    ///
    pub fn position_at_pixel(&self, pixel: (f32, f32)) -> Vec3 {
        match self.projection_type() {
            ProjectionType::Orthographic { width, height, .. } => {
                let coords = self.uv_coordinates_at_pixel(pixel);
                self.position() + vec3((coords.0 - 0.5) * width, (0.5 - coords.1) * height, 0.0)
            }
            ProjectionType::Perspective { .. } => *self.position(),
        }
    }

    ///
    /// Returns the 3D view direction at the given pixel coordinate.
    /// The pixel coordinate must be in physical pixels, where (viewport.x, viewport.y) indicate the top left corner of the viewport
    /// and (viewport.x + viewport.width, viewport.y + viewport.height) indicate the bottom right corner.
    ///
    pub fn view_direction_at_pixel(&self, pixel: (f32, f32)) -> Vec3 {
        match self.projection_type() {
            ProjectionType::Orthographic { .. } => self.view_direction(),
            ProjectionType::Perspective { .. } => {
                let coords = self.uv_coordinates_at_pixel(pixel);
                let screen_pos = vec4(2. * coords.0 - 1., 1. - 2. * coords.1, 0., 1.);
                (self.screen2ray * screen_pos).truncate().normalize()
            }
        }
    }

    ///
    /// Returns the uv coordinate for the given pixel coordinate.
    /// The pixel coordinate must be in physical pixels, where (viewport.x, viewport.y) indicate the top left corner of the viewport
    /// and (viewport.x + viewport.width, viewport.y + viewport.height) indicate the bottom right corner.
    /// The returned uv coordinate are between 0 and 1 where (0,0) indicate the top left corner of the viewport and (1,1) indicate the bottom right corner.
    ///
    pub fn uv_coordinates_at_pixel(&self, pixel: (f32, f32)) -> (f32, f32) {
        (
            (pixel.0 - self.viewport.x as f32) / self.viewport.width as f32,
            (pixel.1 - self.viewport.y as f32) / self.viewport.height as f32,
        )
    }

    ///
    /// Returns the uv coordinate for the given world position.
    /// The returned uv coordinate are between 0 and 1 where (0,0) indicate a position that maps to the top left corner of the viewport
    /// and (1,1) indicate a position that maps to the bottom right corner.
    ///
    pub fn uv_coordinates_at_position(&self, position: Vec3) -> (f32, f32) {
        let proj = self.projection() * self.view() * position.extend(1.0);
        (
            0.5 * (proj.x / proj.w.abs() + 1.0),
            0.5 * (proj.y / proj.w.abs() + 1.0),
        )
    }

    pub(super) fn projection_type(&self) -> &ProjectionType {
        &self.projection_type
    }

    ///
    /// Returns the view matrix, ie. the matrix that transforms objects from world space (as placed in the world) to view space (as seen from this camera).
    ///
    pub fn view(&self) -> &Mat4 {
        &self.view
    }

    ///
    /// Returns the projection matrix, ie. the matrix that projects objects in view space onto this cameras image plane.
    ///
    pub fn projection(&self) -> &Mat4 {
        &self.projection
    }

    ///
    /// Returns the viewport.
    ///
    pub fn viewport(&self) -> Viewport {
        self.viewport
    }

    ///
    /// Returns the distance to the near plane of the camera frustum.
    ///
    pub fn z_near(&self) -> f32 {
        self.z_near
    }

    ///
    /// Returns the distance to the far plane of the camera frustum.
    ///
    pub fn z_far(&self) -> f32 {
        self.z_far
    }

    ///
    /// Returns the position of this camera.
    ///
    pub fn position(&self) -> &Vec3 {
        &self.position
    }

    ///
    /// Returns the target of this camera, ie the point that this camera looks towards.
    ///
    pub fn target(&self) -> &Vec3 {
        &self.target
    }

    ///
    /// Returns the up direction of this camera (might not be orthogonal to the view direction).
    ///
    pub fn up(&self) -> &Vec3 {
        &self.up
    }

    ///
    /// Returns the view direction of this camera, ie. the direction the camera is looking.
    ///
    pub fn view_direction(&self) -> Vec3 {
        (self.target - self.position).normalize()
    }

    ///
    /// Returns the right direction of this camera.
    ///
    pub fn right_direction(&self) -> Vec3 {
        self.view_direction().cross(self.up)
    }

    ///
    /// Returns an uniform buffer containing camera information which makes it easy to transfer all necessary camera information to a shader.
    ///
    /// Use this buffer in your [Program](crate::Program) like this `program.use_uniform_block(camera.uniform_buffer(), "Camera");` and add the following to your shader code:
    ///
    /// ```ignore
    /// layout (std140) uniform Camera
    /// {
    ///     mat4 viewProjection;
    ///     mat4 view;
    ///     mat4 projection;
    ///     vec3 position;
    ///     float padding;
    /// } camera;
    /// ```
    ///
    pub fn uniform_buffer(&self) -> &UniformBuffer {
        &self.uniform_buffer
    }

    fn new(context: &Context, viewport: Viewport) -> Camera {
        Camera {
            context: context.clone(),
            viewport,
            projection_type: ProjectionType::Orthographic {
                width: 1.0,
                height: 1.0,
            },
            z_near: 0.0,
            z_far: 0.0,
            uniform_buffer: UniformBuffer::new(context, &[16, 16, 16, 3, 1]).unwrap(),
            frustrum: [vec4(0.0, 0.0, 0.0, 0.0); 6],
            position: vec3(0.0, 0.0, 5.0),
            target: vec3(0.0, 0.0, 0.0),
            up: vec3(0.0, 1.0, 0.0),
            view: Mat4::identity(),
            projection: Mat4::identity(),
            screen2ray: Mat4::identity(),
        }
    }

    fn update_screen2ray(&mut self) {
        let mut v = self.view;
        v[3] = vec4(0.0, 0.0, 0.0, 1.0);
        self.screen2ray = (self.projection * v).invert().unwrap();
    }

    fn update_uniform_buffer(&mut self) -> Result<(), Error> {
        self.uniform_buffer
            .update(0, &(self.projection * self.view).to_slice())?;
        self.uniform_buffer.update(1, &self.view.to_slice())?;
        self.uniform_buffer.update(2, &self.projection.to_slice())?;
        self.uniform_buffer.update(3, &self.position.to_slice())?;
        Ok(())
    }

    fn update_frustrum(&mut self) {
        let m = self.projection * self.view;
        self.frustrum = [
            vec4(m.x.w + m.x.x, m.y.w + m.y.x, m.z.w + m.z.x, m.w.w + m.w.x),
            vec4(m.x.w - m.x.x, m.y.w - m.y.x, m.z.w - m.z.x, m.w.w - m.w.x),
            vec4(m.x.w + m.x.y, m.y.w + m.y.y, m.z.w + m.z.y, m.w.w + m.w.y),
            vec4(m.x.w - m.x.y, m.y.w - m.y.y, m.z.w - m.z.y, m.w.w - m.w.y),
            vec4(m.x.w + m.x.z, m.y.w + m.y.z, m.z.w + m.z.z, m.w.w + m.w.z),
            vec4(m.x.w - m.x.z, m.y.w - m.y.z, m.z.w - m.z.z, m.w.w - m.w.z),
        ];
    }

    ///
    /// Translate the camera by the given change while keeping the same view and up directions.
    ///
    pub fn translate(&mut self, change: &Vec3) -> Result<(), Error> {
        self.set_view(self.position + change, self.target + change, self.up)?;
        Ok(())
    }

    pub fn pitch(&mut self, delta: impl Into<Radians>) -> Result<(), Error> {
        let target = (self.view.invert().unwrap()
            * Mat4::from_angle_x(delta)
            * self.view
            * self.target.extend(1.0))
        .truncate();
        if (target - self.position).normalize().dot(self.up).abs() < 0.999 {
            self.set_view(self.position, target, self.up)?;
        }
        Ok(())
    }

    pub fn yaw(&mut self, delta: impl Into<Radians>) -> Result<(), Error> {
        let target = (self.view.invert().unwrap()
            * Mat4::from_angle_y(delta)
            * self.view
            * self.target.extend(1.0))
        .truncate();
        self.set_view(self.position, target, self.up)?;
        Ok(())
    }

    pub fn roll(&mut self, delta: impl Into<Radians>) -> Result<(), Error> {
        let up = (self.view.invert().unwrap()
            * Mat4::from_angle_z(delta)
            * self.view
            * (self.up + self.position).extend(1.0))
        .truncate()
            - self.position;
        self.set_view(self.position, self.target, up.normalize())?;
        Ok(())
    }

    ///
    /// Rotate the camera around the given point while keeping the same distance to the point.
    /// The input `x` specifies the amount of rotation in the left direction and `y` specifies the amount of rotation in the up direction.
    /// If you want the camera up direction to stay fixed, use the [rotate_around_with_fixed_up](crate::Camera::rotate_around_with_fixed_up) function instead.
    ///
    pub fn rotate_around(&mut self, point: &Vec3, x: f32, y: f32) -> Result<(), Error> {
        let dir = (point - self.position()).normalize();
        let right = dir.cross(*self.up());
        let up = right.cross(dir);
        let new_dir = (point - self.position() + right * x - up * y).normalize();
        let rotation = rotation_matrix_from_dir_to_dir(dir, new_dir);
        let new_position = (rotation * (self.position() - point).extend(1.0)).truncate() + point;
        let new_target = (rotation * (self.target() - point).extend(1.0)).truncate() + point;
        self.set_view(new_position, new_target, up)?;
        Ok(())
    }

    ///
    /// Rotate the camera around the given point while keeping the same distance to the point and the same up direction.
    /// The input `x` specifies the amount of rotation in the left direction and `y` specifies the amount of rotation in the up direction.
    ///
    pub fn rotate_around_with_fixed_up(
        &mut self,
        point: &Vec3,
        x: f32,
        y: f32,
    ) -> Result<(), Error> {
        let dir = (point - self.position()).normalize();
        let right = dir.cross(*self.up());
        let mut up = right.cross(dir);
        let new_dir = (point - self.position() + right * x - up * y).normalize();
        up = *self.up();
        if new_dir.dot(up).abs() < 0.999 {
            let rotation = rotation_matrix_from_dir_to_dir(dir, new_dir);
            let new_position =
                (rotation * (self.position() - point).extend(1.0)).truncate() + point;
            let new_target = (rotation * (self.target() - point).extend(1.0)).truncate() + point;
            self.set_view(new_position, new_target, up)?;
        }
        Ok(())
    }

    ///
    /// Moves the camera towards the given point by the amount delta while keeping the given minimum and maximum distance to the point.
    ///
    pub fn zoom_towards(
        &mut self,
        point: &Vec3,
        delta: f32,
        minimum_distance: f32,
        maximum_distance: f32,
    ) -> Result<(), Error> {
        if minimum_distance <= 0.0 {
            return Err(Error::CameraError {
                message: "Zoom towards cannot take as input a negative minimum distance."
                    .to_string(),
            });
        }
        if maximum_distance < minimum_distance {
            return Err(Error::CameraError {
                message: "Zoom towards cannot take as input a maximum distance which is smaller than the minimum distance."
                    .to_string(),
            });
        }
        let position = *self.position();
        let distance = point.distance(position);
        let direction = (point - position).normalize();
        let target = *self.target();
        let up = *self.up();
        let new_distance = (distance - delta)
            .max(minimum_distance)
            .min(maximum_distance);
        let new_position = point - direction * new_distance;
        self.set_view(new_position, new_position + (target - position), up)?;
        match self.projection_type() {
            ProjectionType::Orthographic { width: _, height } => {
                let h = new_distance * height / distance;
                let z_near = self.z_near();
                let z_far = self.z_far();
                self.set_orthographic_projection(h, z_near, z_far)?;
            }
            _ => {}
        }
        Ok(())
    }
}
