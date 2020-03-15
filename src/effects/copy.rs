use crate::*;
use num_derive::FromPrimitive;

#[derive(Copy, Clone, FromPrimitive)]
enum Type {POSITION = 0, NORMAL = 1, COLOR = 2, DEPTH = 3, NONE = 4}

pub struct CopyEffect {
    gl: Gl,
    program: program::Program,
    full_screen_positions: VertexBuffer,
    full_screen_uvs: VertexBuffer
}

impl CopyEffect {

    pub fn new(gl: &Gl) -> Result<CopyEffect, effects::Error>
    {
        let program = program::Program::from_source(&gl,
                                                    include_str!("shaders/effect.vert"),
                                                    include_str!("shaders/copy.frag"))?;

        let positions = vec![
            -3.0, -1.0, 0.0,
            3.0, -1.0, 0.0,
            0.0, 2.0, 0.0
        ];
        let uvs = vec![
            -1.0, 0.0,
            2.0, 0.0,
            0.5, 1.5
        ];
        let full_screen_positions = VertexBuffer::new_with_static_f32(&gl, &positions).unwrap();
        let full_screen_uvs = VertexBuffer::new_with_static_f32(&gl, &uvs).unwrap();

        Ok(CopyEffect {gl: gl.clone(), program, full_screen_positions, full_screen_uvs})
    }

    pub fn apply(&self, color_texture: &Texture2DArray, depth_texture: &Texture2DArray) -> Result<(), effects::Error>
    {
        state::depth_write(&self.gl, true);
        state::depth_test(&self.gl, state::DepthTestType::None);
        state::cull(&self.gl,state::CullType::Back);
        state::blend(&self.gl, state::BlendType::None);

        self.program.use_texture(color_texture, "colorMap")?;
        self.program.use_texture(depth_texture, "depthMap")?;

        self.program.use_attribute_vec3_float(&self.full_screen_positions, "position").unwrap();
        self.program.use_attribute_vec2_float(&self.full_screen_uvs, "uv_coordinate").unwrap();
        self.program.draw_arrays(3);
        Ok(())
    }

}