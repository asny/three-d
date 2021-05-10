use crate::core::*;
use crate::definition::*;
use crate::function::*;
use crate::math::*;

pub(super) enum ProjectionType {
    Orthographic {
        width: f32,
        height: f32,
        depth: f32,
    },
    Perspective {
        field_of_view_y: Degrees,
        aspect: f32,
        z_near: f32,
        z_far: f32,
    },
}

///
/// Used in a render call to define how to view the 3D world.
///
pub struct Camera {
    context: Context,
    projection_type: ProjectionType,
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
        position: Vec3,
        target: Vec3,
        up: Vec3,
        width: f32,
        height: f32,
        depth: f32,
    ) -> Result<Camera, Error> {
        let mut camera = Camera::new(context);
        camera.set_view(position, target, up)?;
        camera.set_orthographic_projection(width, height, depth)?;
        Ok(camera)
    }

    ///
    /// New camera which projects the world with a perspective projection.
    ///
    pub fn new_perspective(
        context: &Context,
        position: Vec3,
        target: Vec3,
        up: Vec3,
        field_of_view_y: Degrees,
        aspect: f32,
        z_near: f32,
        z_far: f32,
    ) -> Result<Camera, Error> {
        let mut camera = Camera::new(context);
        camera.set_view(position, target, up)?;
        camera.set_perspective_projection(field_of_view_y, aspect, z_near, z_far)?;
        Ok(camera)
    }

    ///
    /// Specify the camera to use perspective projection with the given field of view in the y-direction, aspect and near and far plane.
    ///
    pub fn set_perspective_projection(
        &mut self,
        field_of_view_y: Degrees,
        aspect: f32,
        z_near: f32,
        z_far: f32,
    ) -> Result<(), Error> {
        if z_near < 0.0 || z_near > z_far {
            panic!("Wrong perspective camera parameters")
        };
        self.projection_type = ProjectionType::Perspective {
            field_of_view_y,
            aspect,
            z_near,
            z_far,
        };
        self.projection = perspective(field_of_view_y, aspect, z_near, z_far);
        self.update_screen2ray();
        self.update_uniform_buffer()?;
        self.update_frustrum();
        Ok(())
    }

    ///
    /// Specify the camera to use orthographic projection with the given width, height and depth.
    /// The view frustum width is +/- width/2, height is +/- height/2 and depth is 0 to depth.
    ///
    pub fn set_orthographic_projection(
        &mut self,
        width: f32,
        height: f32,
        depth: f32,
    ) -> Result<(), Error> {
        self.projection_type = ProjectionType::Orthographic {
            width,
            height,
            depth,
        };
        self.projection = ortho(
            -0.5 * width,
            0.5 * width,
            -0.5 * height,
            0.5 * height,
            0.0,
            depth,
        );
        self.update_screen2ray();
        self.update_uniform_buffer()?;
        self.update_frustrum();
        Ok(())
    }

    ///
    /// Change the current projection to abide to the given aspect ratio.
    ///
    pub fn set_aspect(&mut self, value: f32) -> Result<bool, Error> {
        let mut change = false;
        match self.projection_type {
            ProjectionType::Orthographic {
                width,
                height,
                depth,
            } => {
                if (width / height - value).abs() > 0.001 {
                    self.set_orthographic_projection(height * value, height, depth)?;
                    change = true;
                }
            }
            ProjectionType::Perspective {
                aspect,
                field_of_view_y,
                z_near,
                z_far,
            } => {
                if (aspect - value).abs() > 0.001 {
                    self.set_perspective_projection(field_of_view_y, value, z_near, z_far)?;
                    change = true;
                }
            }
        }
        Ok(change)
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
    /// Finds the closest intersection between a ray from this camera in the direction of the given screen coordinates and the given geometries.
    /// Returns ```None``` if no geometry was hit before the given maximum depth.
    ///
    pub fn pick(
        &self,
        screen_coordinates: (f64, f64),
        max_depth: f32,
        objects: &[&dyn Geometry],
    ) -> Result<Option<Vec3>, Error> {
        let pos = *self.position();
        let dir = self.view_direction_at(screen_coordinates);
        ray_intersect(&self.context, pos, dir, max_depth, objects)
    }

    ///
    /// Returns the view direction at the given screen/image plane coordinates.
    /// The coordinates must be between 0 and 1, where (0, 0) indicate the top left corner of the screen
    /// and (1, 1) indicate the bottom right corner.
    ///
    pub fn view_direction_at(&self, screen_coordinates: (f64, f64)) -> Vec3 {
        let screen_pos = vec4(
            2. * screen_coordinates.0 as f32 - 1.,
            1. - 2. * screen_coordinates.1 as f32,
            0.,
            1.,
        );
        (self.screen2ray * screen_pos).truncate().normalize()
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
    /// Returns the up direction of this camera.
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

    fn new(context: &Context) -> Camera {
        Camera {
            context: context.clone(),
            projection_type: ProjectionType::Orthographic {
                width: 1.0,
                height: 1.0,
                depth: 1.0,
            },
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
}
