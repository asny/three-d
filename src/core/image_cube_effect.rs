#![allow(deprecated)]
use crate::core::*;

///
/// A customizable cube effect.
/// Used for rendering into all 6 sides of a cube map texture.
///
#[deprecated = "Use apply_cube_effect instead"]
pub struct ImageCubeEffect {
    program: Program,
    positions: VertexBuffer,
}

impl ImageCubeEffect {
    ///
    /// Creates a new cube effect which applies the effect defined in the given fragment shader source to a side of a cube map
    /// when calling on of the render functions.
    ///
    pub fn new(context: &Context, fragment_shader_source: &str) -> Result<Self, CoreError> {
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

        let positions = VertexBuffer::new_with_data(
            context,
            &three_d_asset::TriMesh::cube().positions.to_f32(),
        );
        Ok(Self { program, positions })
    }

    ///
    /// Applies the effect defined in the fragment shader source given at construction to the given side of a cube map.
    /// Must be called in the callback given as input to a [RenderTarget], [ColorTarget] or [DepthTarget] write method.
    ///
    pub fn render(&self, side: CubeMapSide, render_states: RenderStates, viewport: Viewport) {
        let projection = cgmath::perspective(degrees(90.0), viewport.aspect(), 0.1, 10.0);
        self.program.use_uniform(
            "viewProjection",
            projection
                * Mat4::look_at_rh(
                    Point3::new(0.0, 0.0, 0.0),
                    Point3::new(side.direction().x, side.direction().y, side.direction().z),
                    side.up(),
                ),
        );
        self.program
            .use_vertex_attribute("position", &self.positions);
        self.program.draw_arrays(render_states, viewport, 36);
    }
}

impl std::ops::Deref for ImageCubeEffect {
    type Target = Program;

    fn deref(&self) -> &Self::Target {
        &self.program
    }
}
