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

    pub fn apply<T: TextureDataType>(
        &self,
        render_target: &RenderTargetCubeMap<T>,
        side: CubeMapSide,
        clear_state: ClearState,
        render_states: RenderStates,
        projection: Mat4,
        viewport: Viewport,
    ) -> ThreeDResult<()> {
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

    pub fn write_to_mip_level<T: TextureDataType>(
        &self,
        render_target: &RenderTargetCubeMap<T>,
        side: CubeMapSide,
        mip_level: u32,
        clear_state: ClearState,
        render_states: RenderStates,
        projection: Mat4,
        viewport: Viewport,
    ) -> ThreeDResult<()> {
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

    pub fn apply_all<T: TextureDataType>(
        &self,
        render_target: &RenderTargetCubeMap<T>,
        clear_state: ClearState,
        render_states: RenderStates,
        projection: Mat4,
        viewport: Viewport,
    ) -> ThreeDResult<()> {
        self.apply(
            render_target,
            CubeMapSide::Right,
            clear_state,
            render_states,
            projection,
            viewport,
        )?;
        self.apply(
            render_target,
            CubeMapSide::Left,
            clear_state,
            render_states,
            projection,
            viewport,
        )?;
        self.apply(
            render_target,
            CubeMapSide::Back,
            clear_state,
            render_states,
            projection,
            viewport,
        )?;
        self.apply(
            render_target,
            CubeMapSide::Front,
            clear_state,
            render_states,
            projection,
            viewport,
        )?;
        self.apply(
            render_target,
            CubeMapSide::Bottom,
            clear_state,
            render_states,
            projection,
            viewport,
        )?;
        self.apply(
            render_target,
            CubeMapSide::Top,
            clear_state,
            render_states,
            projection,
            viewport,
        )
    }

    pub fn write_all_to_mip_level<T: TextureDataType>(
        &self,
        render_target: &RenderTargetCubeMap<T>,
        mip_level: u32,
        clear_state: ClearState,
        render_states: RenderStates,
        projection: Mat4,
        viewport: Viewport,
    ) -> ThreeDResult<()> {
        self.write_to_mip_level(
            render_target,
            CubeMapSide::Right,
            mip_level,
            clear_state,
            render_states,
            projection,
            viewport,
        )?;
        self.write_to_mip_level(
            render_target,
            CubeMapSide::Left,
            mip_level,
            clear_state,
            render_states,
            projection,
            viewport,
        )?;
        self.write_to_mip_level(
            render_target,
            CubeMapSide::Back,
            mip_level,
            clear_state,
            render_states,
            projection,
            viewport,
        )?;
        self.write_to_mip_level(
            render_target,
            CubeMapSide::Front,
            mip_level,
            clear_state,
            render_states,
            projection,
            viewport,
        )?;
        self.write_to_mip_level(
            render_target,
            CubeMapSide::Bottom,
            mip_level,
            clear_state,
            render_states,
            projection,
            viewport,
        )?;
        self.write_to_mip_level(
            render_target,
            CubeMapSide::Top,
            mip_level,
            clear_state,
            render_states,
            projection,
            viewport,
        )
    }
}

impl std::ops::Deref for ImageCubeEffect {
    type Target = Program;

    fn deref(&self) -> &Self::Target {
        &self.program
    }
}
