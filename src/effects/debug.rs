use crate::*;
use num_derive::FromPrimitive;

#[derive(Copy, Clone, FromPrimitive)]
enum Type {POSITION = 0, NORMAL = 1, COLOR = 2, DEPTH = 3, NONE = 4}

pub struct DebugEffect {
    gl: gl::Gl,
    program: program::Program,
    debug_type: Type
}

impl DebugEffect {

    pub fn new(gl: &gl::Gl) -> Result<DebugEffect, effects::Error>
    {
        let program = program::Program::from_source(&gl,
                                                    include_str!("shaders/effect.vert"),
                                                    include_str!("shaders/debug.frag"))?;
        Ok(DebugEffect {gl: gl.clone(), program, debug_type: Type::NONE})
    }

    pub fn change_type(&mut self)
    {
        self.debug_type = num::FromPrimitive::from_u32(((self.debug_type as u32) + 1) % (Type::NONE as u32 + 1)).unwrap();
    }

    pub fn apply(&self, color_texture: &Texture, position_texture: &Texture, normal_texture: &Texture, depth_texture: &Texture) -> Result<(), effects::Error>
    {
        state::depth_write(&self.gl,false);
        state::depth_test(&self.gl, state::DepthTestType::NONE);
        state::blend(&self.gl, state::BlendType::SRC_ALPHA__ONE_MINUS_SRC_ALPHA);

        color_texture.bind(0);
        self.program.add_uniform_int("colorMap", &0)?;

        position_texture.bind(1);
        self.program.add_uniform_int("positionMap", &1)?;

        normal_texture.bind(2);
        self.program.add_uniform_int("normalMap", &2)?;

        depth_texture.bind(3);
        self.program.add_uniform_int("depthMap", &3)?;

        self.program.add_uniform_int("type", &(self.debug_type as i32))?;

        full_screen_quad::render(&self.gl, &self.program);
        Ok(())
    }

}