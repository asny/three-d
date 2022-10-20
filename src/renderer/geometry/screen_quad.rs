use crate::renderer::*;

///
/// A quad geometry that covers the entire screen.
///
pub struct ScreenQuad {
    context: Context,
    position_buffer: VertexBuffer,
    texture_transform: Mat3,
}

impl ScreenQuad {
    pub fn new(context: &Context) -> Self {
        Self {
            context: context.clone(),
            position_buffer: VertexBuffer::new_with_data(
                &context,
                &[
                    vec3(-3.0, -1.0, 0.0),
                    vec3(3.0, -1.0, 0.0),
                    vec3(0.0, 2.0, 0.0),
                ],
            ),
            texture_transform: Mat3::identity(),
        }
    }

    ///
    /// Get the texture transform applied to the uv coordinates.
    ///
    pub fn texture_transform(&mut self) -> &Mat3 {
        &self.texture_transform
    }

    ///
    /// Set the texture transform applied to the uv coordinates.
    ///
    pub fn set_texture_transform(&mut self, texture_transform: Mat3) {
        self.texture_transform = texture_transform;
    }

    fn draw(&self, program: &Program, render_states: RenderStates, camera: &Camera) {
        program.use_vertex_attribute("position", &self.position_buffer);
        program.use_uniform("textureTransform", &self.texture_transform);
        program.draw_arrays(render_states, camera.viewport(), 3);
    }
}

impl<'a> IntoIterator for &'a ScreenQuad {
    type Item = &'a dyn Geometry;
    type IntoIter = std::iter::Once<&'a dyn Geometry>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self)
    }
}

impl Geometry for ScreenQuad {
    fn render_with_material(
        &self,
        material: &dyn Material,
        camera: &Camera,
        lights: &[&dyn Light],
    ) {
        let fragment_shader_source = material.fragment_shader_source(false, lights);
        self.context
            .program(
                &include_str!("shaders/screen_quad.vert"),
                &fragment_shader_source,
                |program| {
                    material.use_uniforms(program, camera, lights);
                    self.draw(program, material.render_states(), camera);
                },
            )
            .expect("Failed compiling shader")
    }

    fn render_with_post_material(
        &self,
        material: &dyn PostMaterial,
        camera: &Camera,
        lights: &[&dyn Light],
        color_texture: ColorTexture,
        depth_texture: Option<&DepthTargetTexture2D>,
    ) {
        let fragment_shader_source =
            material.fragment_shader_source(lights, color_texture, depth_texture);
        self.context
            .program(
                &include_str!("shaders/screen_quad.vert"),
                &fragment_shader_source,
                |program| {
                    material.use_uniforms(program, camera, lights, color_texture, depth_texture);
                    self.draw(program, material.render_states(), camera);
                },
            )
            .expect("Failed compiling shader")
    }

    fn aabb(&self) -> AxisAlignedBoundingBox {
        AxisAlignedBoundingBox::INFINITE
    }
}
