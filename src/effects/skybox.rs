
use crate::*;
use crate::core::Error;
use image::{DynamicImage, GenericImageView};

pub struct Skybox {
    gl: Gl,
    program: program::Program,
    vertex_buffer: VertexBuffer,
    texture: texture::TextureCubeMap
}

impl Skybox
{
    pub fn new(gl: &Gl, right: &DynamicImage, left: &DynamicImage, top: &DynamicImage, front: &DynamicImage, back: &DynamicImage) -> Result<Skybox, Error>
    {
        let texture = TextureCubeMap::new_with_u8(&gl,
                                                  Interpolation::Linear, Interpolation::Linear, None,
                                                  Wrapping::ClampToEdge, Wrapping::ClampToEdge, Wrapping::ClampToEdge,
                                                  right.width(), right.height(),
                                                  [&right.to_bytes(), &left.to_bytes(), &top.to_bytes(), &top.to_bytes(), &front.to_bytes(), &back.to_bytes()])?;
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

    pub fn apply(&self, camera: &camera::Camera) -> Result<(), Error>
    {
        state::depth_write(&self.gl, true);
        state::depth_test(&self.gl, state::DepthTestType::LessOrEqual);
        state::cull(&self.gl, state::CullType::Front);
        state::blend(&self.gl, state::BlendType::None);

        self.program.use_texture(&self.texture, "texture0")?;
        self.program.use_uniform_block(camera.matrix_buffer(), "Camera");

        self.program.use_attribute_vec3_float(&self.vertex_buffer, "position")?;

        self.program.draw_arrays(36);
        state::cull(&self.gl, state::CullType::None);
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