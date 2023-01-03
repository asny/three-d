use crate::core::texture::*;

///
/// A 2D depth texture that can be rendered into and read from. See also [RenderTarget] and [DepthTarget].
///
pub struct DepthTexture2DMultisample {
    context: Context,
    id: crate::context::Renderbuffer,
    width: u32,
    height: u32,
    number_of_samples: u32,
}

impl DepthTexture2DMultisample {
    ///
    /// Constructs a new multisample 2D depth texture.
    ///
    pub fn new<T: DepthTextureDataType>(
        context: &Context,
        width: u32,
        height: u32,
        number_of_samples: u32,
    ) -> Self {
        let id = unsafe {
            context
                .create_renderbuffer()
                .expect("Failed creating render buffer")
        };
        let texture = Self {
            context: context.clone(),
            id,
            width,
            height,
            number_of_samples,
        };
        texture.bind();
        // CHECK: Omitted `set_parameters` since neither filtering, nor mipmap levels, nor clamping makes sense for multisampled textures.
        unsafe {
            context.renderbuffer_storage_multisample(
                crate::context::RENDERBUFFER,
                number_of_samples as i32,
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
    pub fn as_depth_target(&mut self) -> DepthTarget<'_> {
        DepthTarget::new_texture_2d_multisample(&self.context, self)
    }

    pub(in crate::core) fn as_depth_read(&self) -> DepthTarget<'_> {
        DepthTarget::new_texture_2d_multisample(&self.context, self)
    }

    /// The width of this texture.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// The height of this texture.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// The number of samples per fragment.
    pub fn number_of_samples(&self) -> u32 {
        self.number_of_samples
    }

    pub(in crate::core) fn bind_as_depth_target(&self) {
        unsafe {
            self.context.framebuffer_renderbuffer(
                crate::context::FRAMEBUFFER,
                crate::context::DEPTH_ATTACHMENT,
                crate::context::RENDERBUFFER,
                Some(self.id),
            );
        }
    }

    pub(in crate::core) fn bind(&self) {
        unsafe {
            self.context
                .bind_renderbuffer(crate::context::RENDERBUFFER, Some(self.id));
        }
    }
}

impl Drop for DepthTexture2DMultisample {
    fn drop(&mut self) {
        unsafe {
            self.context.delete_renderbuffer(self.id);
        }
    }
}
