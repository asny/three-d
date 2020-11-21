
use crate::*;
use crate::core::Error;

pub struct Skybox {
    gl: Gl,
    program: program::Program,
    vertex_buffer: VertexBuffer,
    texture: texture::TextureCubeMap
}

impl Skybox
{
    pub fn new(gl: &Gl, loaded: &Loaded, back_path: &str, front_path: &str, top_path: &str, left_path: &str, right_path: &str) -> Result<Skybox, Error>
    {
        let texture = TextureCubeMap::new_from_bytes(&gl, Interpolation::Linear, Interpolation::Linear, None, Wrapping::ClampToEdge, Wrapping::ClampToEdge, Wrapping::ClampToEdge,
                                                           loaded.get(back_path).ok_or(
            Error::FailedToCreateTexture {message:format!("Tried to use a texture which was not loaded: {}", back_path)})?.as_ref().map_err(
            |_| Error::FailedToCreateTexture {message:format!("Could not load texture: {}", back_path)})?,
                                                           loaded.get(front_path).ok_or(
            Error::FailedToCreateTexture {message:format!("Tried to use a texture which was not loaded: {}", front_path)})?.as_ref().map_err(
            |_| Error::FailedToCreateTexture {message:format!("Could not load texture: {}", front_path)})?,
                                                           loaded.get(top_path).ok_or(
            Error::FailedToCreateTexture {message:format!("Tried to use a texture which was not loaded: {}", top_path)})?.as_ref().map_err(
            |_| Error::FailedToCreateTexture {message:format!("Could not load texture: {}", top_path)})?,
                                                           loaded.get(top_path).ok_or(
            Error::FailedToCreateTexture {message:format!("Tried to use a texture which was not loaded: {}", top_path)})?.as_ref().map_err(
            |_| Error::FailedToCreateTexture {message:format!("Could not load texture: {}", top_path)})?,
                                                           loaded.get(left_path).ok_or(
            Error::FailedToCreateTexture {message:format!("Tried to use a texture which was not loaded: {}", left_path)})?.as_ref().map_err(
            |_| Error::FailedToCreateTexture {message:format!("Could not load texture: {}", left_path)})?,
                                                           loaded.get(right_path).ok_or(
            Error::FailedToCreateTexture {message:format!("Tried to use a texture which was not loaded: {}", right_path)})?.as_ref().map_err(
            |_| Error::FailedToCreateTexture {message:format!("Could not load texture: {}", right_path)})?)?;
        Self::new_with_texture(gl, texture)
    }

    pub fn new_with_texture(gl: &Gl, texture: texture::TextureCubeMap) -> Result<Skybox, Error>
    {
        let program = program::Program::from_source(gl,
                                                    include_str!("shaders/skybox.vert"),
                                                    include_str!("shaders/skybox.frag"))?;

        let vertex_buffer = VertexBuffer::new_with_static_f32(gl, &get_positions())?;

        Ok(Skybox { gl: gl.clone(), program, vertex_buffer, texture })
    }

    pub fn render(&self, camera: &camera::Camera) -> Result<(), Error>
    {
        state::depth_write(&self.gl, false);
        state::depth_test(&self.gl, state::DepthTestType::None);
        state::cull(&self.gl, state::CullType::Front);
        state::blend(&self.gl, state::BlendType::None);

        self.program.use_texture(&self.texture, "texture0")?;
        self.program.use_uniform_block(camera.matrix_buffer(), "Camera");

        self.program.use_attribute_vec3_float(&self.vertex_buffer, "position")?;

        self.program.draw_arrays(36);
        Ok(())
    }

    pub fn get_texture(&self) -> &texture::TextureCubeMap
    {
        &self.texture
    }
}

fn get_positions() -> Vec<f32>
{
    vec![
        1.0, 1.0, -1.0,
        -1.0, 1.0, -1.0,
        1.0, 1.0, 1.0,
        -1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
        -1.0, 1.0, -1.0,

        -1.0, -1.0, -1.0,
        1.0, -1.0, -1.0,
        1.0, -1.0, 1.0,
        1.0, -1.0, 1.0,
        -1.0, -1.0, 1.0,
        -1.0, -1.0, -1.0,

        1.0, -1.0, -1.0,
        -1.0, -1.0, -1.0,
        1.0, 1.0, -1.0,
        -1.0, 1.0, -1.0,
        1.0, 1.0, -1.0,
        -1.0, -1.0, -1.0,

        -1.0, -1.0, 1.0,
        1.0, -1.0, 1.0,
        1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
        -1.0, 1.0, 1.0,
        -1.0, -1.0, 1.0,

        1.0, -1.0, -1.0,
        1.0, 1.0, -1.0,
        1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
        1.0, -1.0, 1.0,
        1.0, -1.0, -1.0,

        -1.0, 1.0, -1.0,
        -1.0, -1.0, -1.0,
        -1.0, 1.0, 1.0,
        -1.0, -1.0, 1.0,
        -1.0, 1.0, 1.0,
        -1.0, -1.0, -1.0
    ]
}