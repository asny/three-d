use crate::core::*;
use crate::definition::*;

///
/// Forward pipeline based on the phong reflection model supporting a very limited amount of lights with shadows.
/// Supports colored, transparent, textured and instanced meshes.
///
/// *NOTE*: Forward rendering does not require a pipeline, so this is only necessary if you want a depth pre-pass.
///
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
