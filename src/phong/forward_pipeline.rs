use crate::core::*;
use crate::definition::*;

pub struct PhongForwardPipeline {
    context: Context,
    depth_texture: Option<DepthTargetTexture2D>,
}

impl PhongForwardPipeline {
    pub fn new(context: &Context) -> Result<Self, Error> {
        Ok(Self {
            context: context.clone(),
            depth_texture: None,
        })
    }

    pub fn depth_pass<F: FnOnce() -> Result<(), Error>>(
        &mut self,
        width: usize,
        height: usize,
        render: F,
    ) -> Result<(), Error> {
        self.depth_texture = Some(DepthTargetTexture2D::new(
            &self.context,
            width,
            height,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
            DepthFormat::Depth32F,
        )?);
        self.depth_texture
            .as_ref()
            .unwrap()
            .write(Some(1.0), render)?;
        Ok(())
    }

    pub fn depth_texture(&self) -> &DepthTargetTexture2D {
        self.depth_texture.as_ref().unwrap()
    }
}
