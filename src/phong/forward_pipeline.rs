
use crate::*;

pub struct PhongForwardPipeline {
    gl: Gl,
    depth_texture: Option<Texture2D>
}

impl PhongForwardPipeline {

    pub fn new(gl: &Gl) -> Result<Self, Error>
    {
        Ok(Self {
            gl: gl.clone(),
            depth_texture: Some(Texture2D::new(gl, 1, 1,
                    Interpolation::Nearest, Interpolation::Nearest, None, Wrapping::ClampToEdge,
                    Wrapping::ClampToEdge, Format::Depth32F)?)
        })
    }

    pub fn depth_pass<F: FnOnce() -> Result<(), Error>>(&mut self, width: usize, height: usize, render_scene: F) -> Result<(), Error>
    {
        state::depth_write(&self.gl, true);
        state::depth_test(&self.gl, state::DepthTestType::LessOrEqual);
        state::cull(&self.gl, state::CullType::None);
        state::blend(&self.gl, state::BlendType::None);

        self.depth_texture = Some(Texture2D::new(&self.gl, width, height,
                    Interpolation::Nearest, Interpolation::Nearest, None, Wrapping::ClampToEdge,
                    Wrapping::ClampToEdge, Format::Depth32F)?);
        RenderTarget::write_to_depth(&self.gl,0, 0, width, height,Some(1.0),self.depth_texture.as_ref(), render_scene)?;
        Ok(())
    }

    pub fn render_to_screen<F: FnOnce() -> Result<(), Error>>(&self, width: usize, height: usize, render_scene: F) -> Result<(), Error>
    {
        Ok(Screen::write(&self.gl, 0, 0, width, height,
                         Some(&vec4(0.0, 0.0, 0.0, 1.0)),
                         Some(1.0),
                         render_scene)?)
    }

    pub fn depth_texture(&self) -> &Texture2D
    {
        &self.depth_texture.as_ref().unwrap()
    }
}
