use crate::*;
use num_derive::FromPrimitive;

#[derive(Copy, Clone, FromPrimitive)]
enum Type {POSITION = 0, NORMAL = 1, COLOR = 2, DEPTH = 3, NONE = 4}

pub struct CopyEffect {
    gl: Gl,
    program: program::Program,
    buffer: VertexBuffer
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
        let buffer = VertexBuffer::new_with_two_static_attributes(&gl, &positions, &uvs).unwrap();

        Ok(CopyEffect {gl: gl.clone(), program, buffer})
    }

    pub fn apply(&self, color_texture: &Texture, depth_texture: &Texture) -> Result<(), effects::Error>
    {
        state::depth_write(&self.gl, true);
        state::depth_test(&self.gl, state::DepthTestType::NONE);
        state::cull(&self.gl,state::CullType::BACK);
        state::blend(&self.gl, state::BlendType::NONE);

        self.program.use_texture(color_texture, "colorMap")?;
        self.program.use_texture(depth_texture, "depthMap")?;

        self.program.use_attribute_vec3_float(&self.buffer, "position", 0).unwrap();
        self.program.use_attribute_vec2_float(&self.buffer, "uv_coordinate", 1).unwrap();
        self.program.draw_arrays(3);
        Ok(())
    }

}