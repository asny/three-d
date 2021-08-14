use crate::context::{consts, Context};
use crate::core::texture::*;
use crate::core::*;

///
/// Type of formats for depth render targets ([DepthTargetTexture2D] and
/// [DepthTargetTexture2DArray]).
///
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum DepthFormat {
    Depth16,
    Depth24,
    Depth32F,
}

///
/// A 2D depth texture that can be rendered into and read from. See also [RenderTarget].
///
pub struct DepthTargetTexture2D {
    context: Context,
    id: crate::context::Texture,
    width: u32,
    height: u32,
}

impl DepthTargetTexture2D {
    ///
    /// Constructs a new 2D depth target texture.
    ///
    pub fn new(
        context: &Context,
        width: u32,
        height: u32,
        wrap_s: Wrapping,
        wrap_t: Wrapping,
        format: DepthFormat,
    ) -> Result<Self, Error> {
        let id = generate(context)?;
        set_parameters(
            context,
            &id,
            consts::TEXTURE_2D,
            Interpolation::Nearest,
            Interpolation::Nearest,
            None,
            wrap_s,
            wrap_t,
            None,
        );
        context.tex_storage_2d(
            consts::TEXTURE_2D,
            1,
            internal_format_from_depth(format),
            width as u32,
            height as u32,
        );
        Ok(Self {
            context: context.clone(),
            id,
            width,
            height,
        })
    }

    ///
    /// Write the depth of whatever rendered in the `render` closure into the texture.
    /// Before writing, the texture is cleared based on the given clear state.
    ///
    pub fn write<F: FnOnce() -> Result<(), Error>>(
        &self,
        clear_state: Option<f32>,
        render: F,
    ) -> Result<(), Error> {
        RenderTarget::<f32>::new_depth(&self.context, &self)?.write(
            ClearState {
                depth: clear_state,
                ..ClearState::none()
            },
            render,
        )
    }

    ///
    /// Copies the content of the depth texture to the specified destination at the given viewport.
    ///
    /// # Errors
    /// Will return an error if the destination is a color texture.
    ///
    pub fn copy_to<T: TextureDataType>(
        &self,
        destination: CopyDestination<T>,
        viewport: Viewport,
    ) -> Result<(), Error> {
        RenderTarget::new_depth(&self.context, &self)?.copy_to(
            destination,
            viewport,
            WriteMask::DEPTH,
        )
    }

    pub(in crate::core) fn bind_as_depth_target(&self) {
        self.context.framebuffer_texture_2d(
            consts::FRAMEBUFFER,
            consts::DEPTH_ATTACHMENT,
            consts::TEXTURE_2D,
            &self.id,
            0,
        );
    }
}

impl Texture for DepthTargetTexture2D {
    fn bind(&self, location: u32) {
        bind_at(&self.context, &self.id, consts::TEXTURE_2D, location);
    }
    fn width(&self) -> u32 {
        self.width
    }
    fn height(&self) -> u32 {
        self.height
    }
    fn format(&self) -> Format {
        Format::R
    }
    fn is_transparent(&self) -> bool {
        false
    }
}

impl Drop for DepthTargetTexture2D {
    fn drop(&mut self) {
        self.context.delete_texture(&self.id);
    }
}
