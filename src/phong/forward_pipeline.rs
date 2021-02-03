
use crate::core::*;

pub struct PhongForwardPipeline {
    gl: Context,
    depth_texture: Option<Texture2D>
}

impl PhongForwardPipeline {

    pub fn new(gl: &Context) -> Result<Self, Error>
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
        self.depth_texture = Some(Texture2D::new(&self.gl, width, height,
                    Interpolation::Nearest, Interpolation::Nearest, None, Wrapping::ClampToEdge,
                    Wrapping::ClampToEdge, Format::Depth32F)?);
        RenderTarget::write_to_depth(&self.gl,Some(1.0),self.depth_texture.as_ref(), render_scene)?;
        Ok(())
    }

    pub fn depth_texture(&self) -> &Texture2D
    {
        &self.depth_texture.as_ref().unwrap()
    }
}
