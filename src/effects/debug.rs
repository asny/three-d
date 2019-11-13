use crate::*;
use num_derive::FromPrimitive;

#[derive(Copy, Clone, PartialEq, Eq, FromPrimitive)]
enum Type {POSITION = 0, NORMAL = 1, COLOR = 2, DEPTH = 3, NONE = 4}

pub struct DebugEffect {
    gl: Gl,
    program: program::Program,
    debug_type: Type
}

impl DebugEffect {

    pub fn new(gl: &Gl) -> Result<DebugEffect, effects::Error>
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

    pub fn apply(&self, full_screen: &FullScreen, camera: &Camera, geometry_texture: &Texture, depth_texture: &Texture) -> Result<(), effects::Error>
    {
        if self.debug_type != Type::NONE {
            state::depth_write(&self.gl,false);
            state::depth_test(&self.gl, state::DepthTestType::NONE);
            state::blend(&self.gl, state::BlendType::SRC_ALPHA__ONE_MINUS_SRC_ALPHA);

            self.program.add_uniform_mat4("viewInverse", &camera.get_view().invert().unwrap())?;
            self.program.add_uniform_mat4("projectionInverse", &camera.get_projection().invert().unwrap())?;

            geometry_texture.bind(0);
            self.program.add_uniform_int("gbuffer", &0)?;

            depth_texture.bind(1);
            self.program.add_uniform_int("depthMap", &1)?;

            self.program.add_uniform_int("type", &(self.debug_type as i32))?;

            self.program.use_attribute_vec3_float(&full_screen.buffer(), "position", 0).unwrap();
            self.program.use_attribute_vec2_float(&full_screen.buffer(), "uv_coordinate", 1).unwrap();
            self.program.draw_arrays(3);
        }
        Ok(())
    }

}