
use crate::core::*;

pub struct PhongForwardPipeline {
    context: Context,
    depth_texture: Option<DepthTargetTexture2D>
}

impl PhongForwardPipeline {

    pub fn new(context: &Context) -> Result<Self, Error>
    {
        Ok(Self {
            context: context.clone(),
            depth_texture: Some(DepthTargetTexture2D::new(context, 1, 1,
                    Interpolation::Nearest, Interpolation::Nearest, None, Wrapping::ClampToEdge,
                    Wrapping::ClampToEdge, DepthFormat::Depth32F)?)
        })
    }

    pub fn depth_pass<F: FnOnce() -> Result<(), Error>>(&mut self, width: usize, height: usize, render_scene: F) -> Result<(), Error>
    {
        self.depth_texture = Some(DepthTargetTexture2D::new(&self.context, width, height,
                    Interpolation::Nearest, Interpolation::Nearest, None, Wrapping::ClampToEdge,
                    Wrapping::ClampToEdge, DepthFormat::Depth32F)?);
        RenderTarget::new_depth(&self.context,self.depth_texture.as_ref().unwrap())?
            .write(None, Some(1.0), render_scene)?;
        Ok(())
    }

    pub fn depth_texture(&self) -> &dyn Texture
    {
        self.depth_texture.as_ref().unwrap()
    }
}
