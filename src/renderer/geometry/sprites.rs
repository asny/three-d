use crate::core::*;
use crate::renderer::*;

///
/// A set of sprites, ie. a set of quads that orients itself towards the camera.
///
/// The sprites will always orient themselves towards the camera, but if a direction is specified, the sprite normals will also always be orthogonal to that direction.
/// For example, if the up direction is specified, the sprites will rotate around the up direction trying to face the camera.
/// Sprites are also known as billboards in the case where no direction is specified.
///
pub struct Sprites {
    context: Context,
    position_buffer: VertexBuffer<Vec3>,
    uv_buffer: VertexBuffer<Vec2>,
    center_buffer: InstanceBuffer<Vec3>,
    transformation: Mat4,
    direction: Option<Vec3>,
}

impl Sprites {
    ///
    /// Create a new set of [Sprites] with the given centers. The centers also determines the number of sprites.
    /// The sprites will always orient themselves towards the camera, but if a direction is specified, the sprite normals will always be orthogonal to that direction.
    ///
    pub fn new(context: &Context, centers: &[Vec3], direction: Option<Vec3>) -> Self {
        let position_buffer = VertexBuffer::new_with_data(
            context,
            &[
                vec3(-1.0, -1.0, 0.0),
                vec3(1.0, -1.0, 0.0),
                vec3(1.0, 1.0, 0.0),
                vec3(1.0, 1.0, 0.0),
                vec3(-1.0, 1.0, 0.0),
                vec3(-1.0, -1.0, 0.0),
            ],
        );
        let uv_buffer = VertexBuffer::new_with_data(
            context,
            &[
                vec2(0.0, 0.0),
                vec2(1.0, 0.0),
                vec2(1.0, 1.0),
                vec2(1.0, 1.0),
                vec2(0.0, 1.0),
                vec2(0.0, 0.0),
            ],
        );
        Self {
            context: context.clone(),
            position_buffer,
            uv_buffer,
            center_buffer: InstanceBuffer::new_with_data(context, centers),
            transformation: Mat4::identity(),
            direction,
        }
    }

    ///
    /// Returns the local to world transformation applied to all sprites.
    ///
    pub fn transformation(&self) -> Mat4 {
        self.transformation
    }

    ///
    /// Set the local to world transformation applied to all sprites.
    ///
    pub fn set_transformation(&mut self, transformation: Mat4) {
        self.transformation = transformation;
    }

    ///
    /// Set a direction the sprite normals are always orthogonal to.
    ///
    pub fn set_direction(&mut self, direction: Option<Vec3>) {
        self.direction = direction;
    }

    ///
    /// Set the centers of the sprites. The centers also determines the number of sprites.
    ///
    pub fn set_centers(&mut self, centers: &[Vec3]) {
        self.center_buffer.fill(centers);
    }

    fn draw(&self, program: &Program, render_states: RenderStates, viewer: &dyn Viewer) {
        program.use_uniform("eye", viewer.position());
        program.use_uniform("viewProjection", viewer.projection() * viewer.view());
        program.use_uniform("transformation", self.transformation);
        program.use_vertex_attribute("position", &self.position_buffer);
        if program.requires_attribute("uv_coordinate") {
            program.use_vertex_attribute("uv_coordinate", &self.uv_buffer);
        }
        program.use_instance_attribute("center", &self.center_buffer);
        program.use_uniform("direction", self.direction.unwrap_or(vec3(0.0, 0.0, 0.0)));
        program.draw_arrays_instanced(
            render_states,
            viewer.viewport(),
            6,
            self.center_buffer.instance_count(),
        )
    }
}

impl<'a> IntoIterator for &'a Sprites {
    type Item = &'a dyn Geometry;
    type IntoIter = std::iter::Once<&'a dyn Geometry>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self)
    }
}

impl Geometry for Sprites {
    fn draw(&self, viewer: &dyn Viewer, program: &Program, render_states: RenderStates) {
        self.draw(program, render_states, viewer);
    }

    fn vertex_shader_source(&self) -> String {
        include_str!("shaders/sprites.vert").to_owned()
    }

    fn id(&self) -> GeometryId {
        GeometryId::Sprites
    }

    fn render_with_material(
        &self,
        material: &dyn Material,
        viewer: &dyn Viewer,
        lights: &[&dyn Light],
    ) {
        render_with_material(&self.context, viewer, &self, material, lights);
    }

    fn render_with_effect(
        &self,
        material: &dyn Effect,
        viewer: &dyn Viewer,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) {
        render_with_effect(
            &self.context,
            viewer,
            self,
            material,
            lights,
            color_texture,
            depth_texture,
        )
    }

    fn aabb(&self) -> AxisAlignedBoundingBox {
        AxisAlignedBoundingBox::INFINITE
    }
}
