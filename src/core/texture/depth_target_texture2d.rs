use crate::core::texture::*;

///
/// Type of formats for depth render targets ([DepthTargetTexture2D] and
/// [DepthTargetTexture2DArray]).
///
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum DepthFormat {
    /// 16 bit per pixel.
    Depth16,
    /// 24 bit per pixel.
    Depth24,
    /// 32 bit per pixel.
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
                internal_format_from_depth(format),
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
    pub fn as_depth_target(&mut self) -> DepthTarget {
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

impl Drop for DepthTargetTexture2D {
    fn drop(&mut self) {
        unsafe {
            self.context.delete_texture(self.id);
        }
    }
}
