use crate::core::*;

///
/// A customizable cube effect.
/// Can be used for rendering into all 6 sides of a cube map texture.
///
pub struct ImageCubeEffect {
    program: Program,
    positions: VertexBuffer,
}

impl ImageCubeEffect {
    ///
    /// Creates a new image effect which applies the calculations defined in the given fragment shader source when calling the [ImageEffect::apply] function.
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

        let positions = VertexBuffer::new_with_static(context, &CPUMesh::cube().positions)?;
        Ok(Self { program, positions })
    }

    pub fn render<T: TextureDataType>(
        &self,
        render_target: &RenderTargetCubeMap<T>,
        side: CubeMapSide,
        clear_state: ClearState,
        render_states: RenderStates,
    ) -> ThreeDResult<()> {
        let viewport = Viewport::new_at_origo(render_target.width(), render_target.height());
        let projection = perspective(degrees(90.0), viewport.aspect(), 0.1, 10.0);
        render_target.write(side, clear_state, || {
            self.program
                .use_uniform_mat4("viewProjection", &(projection * side.view()))?;
            self.program
                .use_attribute_vec3("position", &self.positions)?;
            self.program.draw_arrays(render_states, viewport, 36);
            Ok(())
        })?;
        Ok(())
    }

    pub fn render_to_mip_level<T: TextureDataType>(
        &self,
        render_target: &RenderTargetCubeMap<T>,
        side: CubeMapSide,
        mip_level: u32,
        clear_state: ClearState,
        render_states: RenderStates,
    ) -> ThreeDResult<()> {
        let viewport = Viewport::new_at_origo(
            render_target.width() / 2u32.pow(mip_level),
            render_target.height() / 2u32.pow(mip_level),
        );
        let projection = perspective(degrees(90.0), viewport.aspect(), 0.1, 10.0);
        render_target.write_to_mip_level(side, mip_level, clear_state, || {
            self.program
                .use_uniform_mat4("viewProjection", &(projection * side.view()))?;
            self.program
                .use_attribute_vec3("position", &self.positions)?;
            self.program.draw_arrays(render_states, viewport, 36);
            Ok(())
        })?;
        Ok(())
    }
}

impl std::ops::Deref for ImageCubeEffect {
    type Target = Program;

    fn deref(&self) -> &Self::Target {
        &self.program
    }
}
