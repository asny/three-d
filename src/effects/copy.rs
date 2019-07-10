use crate::*;
use num_derive::FromPrimitive;

#[derive(Copy, Clone, FromPrimitive)]
enum Type {POSITION = 0, NORMAL = 1, COLOR = 2, DEPTH = 3, NONE = 4}

pub struct CopyEffect {
    gl: Gl,
    program: program::Program
}

impl CopyEffect {

    pub fn new(gl: &Gl) -> Result<CopyEffect, effects::Error>
    {
        let program = program::Program::from_source(&gl,
                                                    include_str!("shaders/effect.vert"),
                                                    include_str!("shaders/copy.frag"))?;
        Ok(CopyEffect {gl: gl.clone(), program})
    }

    pub fn apply(&self, full_screen: &FullScreen, color_texture: &Texture, depth_texture: &Texture) -> Result<(), effects::Error>
    {
        state::depth_write(&self.gl, true);
        state::depth_test(&self.gl, state::DepthTestType::NONE);
        state::cull(&self.gl,state::CullType::BACK);
        state::blend(&self.gl, state::BlendType::NONE);

        self.program.use_texture(color_texture, "colorMap")?;
        self.program.use_texture(depth_texture, "depthMap")?;

        self.program.use_attribute_vec3_float(&full_screen.buffer(), "position", 0).unwrap();
        self.program.use_attribute_vec2_float(&full_screen.buffer(), "uv_coordinate", 1).unwrap();
        self.program.draw_arrays(3);
        Ok(())
    }

}