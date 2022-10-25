use crate::core::texture::*;

///
/// A 2D depth texture that can be rendered into and read from. See also [RenderTarget] and [DepthTarget].
///
#[deprecated = "Renamed to DepthTexture2D"]
pub type DepthTargetTexture2D = DepthTexture2D;

///
/// A 2D depth texture that can be rendered into and read from. See also [RenderTarget] and [DepthTarget].
///
pub struct DepthTexture2D {
    context: Context,
    id: crate::context::Texture,
    width: u32,
    height: u32,
}

impl DepthTexture2D {
    ///
    /// Constructs a new 2D depth texture.
    ///
    pub fn new<T: DepthTextureDataType>(
        context: &Context,
        width: u32,
        height: u32,
        wrap_s: Wrapping,
        wrap_t: Wrapping,
    ) -> Self {
        let id = generate(context);
        let texture = Self {
            context: context.clone(),
            id,
            width,
            height,
        };
        texture.bind();
        set_parameters(
            context,
            crate::context::TEXTURE_2D,
            Interpolation::Nearest,
            Interpolation::Nearest,
            None,
            wrap_s,
            wrap_t,
            None,
        );
        unsafe {
            context.tex_storage_2d(
                crate::context::TEXTURE_2D,
                1,
                T::internal_format(),
                width as i32,
                height as i32,
            );
        }
        texture
    }

    ///
    /// Returns a [DepthTarget] which can be used to clear, write to and read from this texture.
    /// Combine this together with a [ColorTarget] with [RenderTarget::new] to be able to write to both a depth and color target at the same time.
    ///
    pub fn as_depth_target<'a>(&'a mut self) -> DepthTarget<'a> {
        DepthTarget::new_texture2d(&self.context, self)
    }

    /// The width of this texture.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// The height of this texture.
    pub fn height(&self) -> u32 {
        self.height
    }

    pub(in crate::core) fn bind_as_depth_target(&self) {
        unsafe {
            self.context.framebuffer_texture_2d(
                crate::context::FRAMEBUFFER,
                crate::context::DEPTH_ATTACHMENT,
                crate::context::TEXTURE_2D,
                Some(self.id),
                0,
            );
        }
    }

    pub(in crate::core) fn bind(&self) {
        unsafe {
            self.context
                .bind_texture(crate::context::TEXTURE_2D, Some(self.id));
        }
    }
}

impl Drop for DepthTexture2D {
    fn drop(&mut self) {
        unsafe {
            self.context.delete_texture(self.id);
        }
    }
}
