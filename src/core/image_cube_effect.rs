use crate::core::*;

///
/// A customizable cube effect.
/// Used for rendering into all 6 sides of a cube map texture.
///
pub struct ImageCubeEffect {
    program: Program,
    positions: Buffer<Vector3<f32>>,
}

impl ImageCubeEffect {
    ///
    /// Creates a new cube effect which applies the effect defined in the given fragment shader source to a side of a cube map
    /// when calling on of the render functions.
    ///
    pub fn new(context: &Context, fragment_shader_source: &str) -> ThreeDResult<Self> {
        let program = Program::from_source(
            context,
            "uniform mat4 viewProjection;
            in vec3 position;
            out vec3 pos;
            
            void main()
            {
                pos = position;
                gl_Position = viewProjection * vec4(position, 1.0);
            }",
            fragment_shader_source,
        )?;

        let positions = Buffer::new_with_data(
            context,
            &[
                vec3(1.0, 1.0, -1.0),
                vec3(-1.0, 1.0, -1.0),
                vec3(1.0, 1.0, 1.0),
                vec3(-1.0, 1.0, 1.0),
                vec3(1.0, 1.0, 1.0),
                vec3(-1.0, 1.0, -1.0),
                vec3(-1.0, -1.0, -1.0),
                vec3(1.0, -1.0, -1.0),
                vec3(1.0, -1.0, 1.0),
                vec3(1.0, -1.0, 1.0),
                vec3(-1.0, -1.0, 1.0),
                vec3(-1.0, -1.0, -1.0),
                vec3(1.0, -1.0, -1.0),
                vec3(-1.0, -1.0, -1.0),
                vec3(1.0, 1.0, -1.0),
                vec3(-1.0, 1.0, -1.0),
                vec3(1.0, 1.0, -1.0),
                vec3(-1.0, -1.0, -1.0),
                vec3(-1.0, -1.0, 1.0),
                vec3(1.0, -1.0, 1.0),
                vec3(1.0, 1.0, 1.0),
                vec3(1.0, 1.0, 1.0),
                vec3(-1.0, 1.0, 1.0),
                vec3(-1.0, -1.0, 1.0),
                vec3(1.0, -1.0, -1.0),
                vec3(1.0, 1.0, -1.0),
                vec3(1.0, 1.0, 1.0),
                vec3(1.0, 1.0, 1.0),
                vec3(1.0, -1.0, 1.0),
                vec3(1.0, -1.0, -1.0),
                vec3(-1.0, 1.0, -1.0),
                vec3(-1.0, -1.0, -1.0),
                vec3(-1.0, 1.0, 1.0),
                vec3(-1.0, -1.0, 1.0),
                vec3(-1.0, 1.0, 1.0),
                vec3(-1.0, -1.0, -1.0),
            ],
        )?;
        Ok(Self { program, positions })
    }

    ///
    /// Applies the effect defined in the fragment shader source given at construction to the given side of a cube map.
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write].
    ///
    pub fn render(
        &self,
        side: CubeMapSide,
        render_states: RenderStates,
        viewport: Viewport,
    ) -> ThreeDResult<()> {
        let projection = perspective(degrees(90.0), viewport.aspect(), 0.1, 10.0);
        self.program
            .use_uniform("viewProjection", projection * side.view())?;
        self.program
            .use_vertex_attribute("position", &self.positions)?;
        self.program.draw_arrays(render_states, viewport, 36);
        Ok(())
    }
}

impl std::ops::Deref for ImageCubeEffect {
    type Target = Program;

    fn deref(&self) -> &Self::Target {
        &self.program
    }
}
