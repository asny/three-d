use crate::*;
use num_derive::FromPrimitive;

#[derive(Copy, Clone, FromPrimitive)]
enum Type {POSITION = 0, NORMAL = 1, COLOR = 2, DEPTH = 3, NONE = 4}

pub struct CopyEffect {
    gl: Gl,
    program: program::Program,
    debug_type: Type
}

impl CopyEffect {

    pub fn new(gl: &Gl) -> Result<CopyEffect, effects::Error>
    {
        let program = program::Program::from_source(&gl,
                                                    include_str!("shaders/effect.vert"),
                                                    include_str!("shaders/copy.frag"))?;
        Ok(CopyEffect {gl: gl.clone(), program, debug_type: Type::NONE})
    }

    pub fn change_type(&mut self)
    {
        self.debug_type = num::FromPrimitive::from_u32(((self.debug_type as u32) + 1) % (Type::NONE as u32 + 1)).unwrap();
    }

    pub fn apply(&self, full_screen: &FullScreen, color_texture: &Texture, depth_texture: &Texture) -> Result<(), effects::Error>
    {
        state::depth_write(&self.gl, true);
        state::depth_test(&self.gl, state::DepthTestType::LEQUAL);
        state::cull(&self.gl,state::CullType::BACK);
        state::blend(&self.gl, state::BlendType::NONE);

        self.program.use_texture(color_texture, "colorMap")?;
        self.program.use_texture(depth_texture, "depthMap")?;

        full_screen.render(&self.program);
        Ok(())
    }

}