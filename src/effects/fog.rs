use crate::*;

pub struct FogEffect {
    gl: gl::Gl,
    program: program::Program,
    pub color: Vec3,
    pub density: f32,
    pub no_fog_height: f32,
    pub animation: f32
}

impl FogEffect {

    pub fn new(gl: &gl::Gl) -> Result<FogEffect, effects::Error>
    {
        let program = program::Program::from_source(&gl,
                                                    include_str!("shaders/effect.vert"),
                                                    include_str!("shaders/fog.frag"))?;
        Ok(FogEffect {gl: gl.clone(), program, color: vec3(0.8, 0.8, 0.8), density: 0.2, no_fog_height: 3.0, animation: 0.1})
    }

    pub fn apply(&self, time: f32, camera: &camera::Camera, position_texture: &Texture) -> Result<(), effects::Error>
    {
        state::depth_write(&self.gl,false);
        state::depth_test(&self.gl, state::DepthTestType::NONE);
        state::blend(&self.gl, state::BlendType::SRC_ALPHA__ONE_MINUS_SRC_ALPHA);

        position_texture.bind(0);
        self.program.add_uniform_int("positionMap", &0)?;

        self.program.add_uniform_vec3("fogColor", &self.color)?;
        self.program.add_uniform_float("fogDensity", &self.density)?;
        self.program.add_uniform_float("noFogHeight", &self.no_fog_height)?;
        self.program.add_uniform_float("animation", &self.animation)?;
        self.program.add_uniform_float("time", &(0.001 * time))?;
        self.program.add_uniform_vec3("eyePosition", camera.position())?;

        full_screen_quad::render(&self.gl, &self.program);
        Ok(())
    }

}