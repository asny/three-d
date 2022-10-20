use crate::core::*;

///
/// A customizable 2D effect.
/// Can for example be used for adding an effect on top of a rendered image.
///
pub struct ImageEffect {
    program: Program,
    positions: VertexBuffer,
    uvs: VertexBuffer,
    texture_transform: Mat3,
}

impl ImageEffect {
    ///
    /// Creates a new image effect which applies the calculations defined in the given fragment shader source when calling the [ImageEffect::apply] function.
    ///
    pub fn new(context: &Context, fragment_shader: &str) -> Result<Self, CoreError> {
        let program = Program::from_source(
            &context,
            "
                uniform mat3 textureTransform;
                in vec3 position;
                in vec2 uv_coordinates;
                out vec2 uvs;
                void main()
                {
                    uvs = (textureTransform * vec3(uv_coordinates, 1.0)).xy;
                    gl_Position = vec4(position, 1.0);
                }
            ",
            fragment_shader,
        )?;

        let positions = vec![
            vec3(-3.0, -1.0, 0.0),
            vec3(3.0, -1.0, 0.0),
            vec3(0.0, 2.0, 0.0),
        ];
        let uvs = vec![vec2(-1.0, 0.0), vec2(2.0, 0.0), vec2(0.5, 1.5)];
        let positions = VertexBuffer::new_with_data(&context, &positions);
        let uvs = VertexBuffer::new_with_data(&context, &uvs);

        Ok(Self {
            program,
            positions,
            uvs,
            texture_transform: Mat3::identity(),
        })
    }

    ///
    /// Get the texture transform applied to the uv coordinates of the image effect.
    ///
    pub fn texture_transform(&mut self) -> &Mat3 {
        &self.texture_transform
    }

    ///
    /// Set the texture transform applied to the uv coordinates of the image effect.
    ///
    pub fn set_texture_transform(&mut self, texture_transform: Mat3) {
        self.texture_transform = texture_transform;
    }

    ///
    /// Applies the calculations defined in the fragment shader given at construction and output it to the current screen/render target.
    /// Must be called in the callback given as input to a [RenderTarget], [ColorTarget] or [DepthTarget] write method.
    ///
    pub fn render(&self, render_states: RenderStates, viewport: Viewport) {
        self.program
            .use_vertex_attribute("position", &self.positions);
        self.program
            .use_vertex_attribute("uv_coordinates", &self.uvs);
        self.program
            .use_uniform("textureTransform", &self.texture_transform);
        self.program.draw_arrays(render_states, viewport, 3);
    }

    ///
    /// Applies the calculations defined in the fragment shader given at construction and output it to the current screen/render target.
    /// Must be called in the callback given as input to a [RenderTarget], [ColorTarget] or [DepthTarget] write method.
    ///
    pub fn apply(&self, render_states: RenderStates, viewport: Viewport) {
        self.render(render_states, viewport)
    }
}

impl std::ops::Deref for ImageEffect {
    type Target = Program;

    fn deref(&self) -> &Self::Target {
        &self.program
    }
}
