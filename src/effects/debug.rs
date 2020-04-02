use crate::*;
use num_derive::FromPrimitive;

#[derive(Debug, Copy, Clone, PartialEq, Eq, FromPrimitive)]
enum Type {POSITION = 0, NORMAL = 1, COLOR = 2, DEPTH = 3, DIFFUSE = 4, SPECULAR = 5, POWER = 6, NONE = 7}

pub struct DebugEffect {
    gl: Gl,
    debug_type: Type,
    image_effect: ImageEffect
}

impl DebugEffect {

    pub fn new(gl: &Gl) -> Result<DebugEffect, effects::Error>
    {
        Ok(DebugEffect {gl: gl.clone(), debug_type: Type::NONE, image_effect: ImageEffect::new(gl, include_str!("shaders/debug.frag"))?})
    }

    pub fn change_type(&mut self)
    {
        self.debug_type = num::FromPrimitive::from_u32(((self.debug_type as u32) + 1) % (Type::NONE as u32 + 1)).unwrap();
        println!("{:?}", self.debug_type);
    }

    pub fn apply(&self, camera: &Camera, geometry_texture: &Texture2DArray, depth_texture: &Texture2DArray) -> Result<(), effects::Error>
    {
        if self.debug_type != Type::NONE {
            state::depth_write(&self.gl,false);
            state::depth_test(&self.gl, state::DepthTestType::None);
            state::blend(&self.gl, state::BlendType::None);

            self.image_effect.program().add_uniform_mat4("viewProjectionInverse", &(camera.get_projection() * camera.get_view()).invert().unwrap())?;
            self.image_effect.program().use_texture(geometry_texture, "gbuffer")?;
            self.image_effect.program().use_texture(depth_texture, "depthMap")?;
            self.image_effect.program().add_uniform_int("type", &(self.debug_type as i32))?;

            self.image_effect.apply();
        }
        Ok(())
    }

}