use crate::core::texture::*;

///
/// A multisampled 2D texture.
///
pub struct Texture2DMultisample {
    context: Context,
    id: crate::context::Texture,
    width: u32,
    height: u32,
    number_of_samples: u32,
}

impl Texture2DMultisample {
    ///
    /// Constructs a new empty 2D texture with the given parameters.
    /// The format is determined by the generic [TextureDataType] parameter
    /// (for example, if [u8; 4] is specified, the format is RGBA and the data type is byte).
    ///
    pub fn new<T: TextureDataType>(
        context: &Context,
        width: u32,
        height: u32,
        number_of_samples: u32,
    ) -> Self {
        let id = generate(context);
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
            context.tex_storage_2d_multisample(
                crate::context::TEXTURE_2D_MULTISAMPLE,
                number_of_samples as i32,
                T::internal_format(),
                width as i32,
                height as i32,
                false,
            );
        }
        texture
    }

    ///
    /// Returns a [ColorTarget] which can be used to clear, write to and read from the texture.
    /// Combine this together with a [DepthTarget] with [RenderTarget::new] to be able to write to both a depth and color target at the same time.
    ///
    /// **Note:** [DepthTest] is disabled if not also writing to a depth texture.
    ///
    pub fn as_color_target<'a>(&'a mut self) -> ColorTarget<'a> {
        ColorTarget::new_texture_2d_multisample(&self.context, self)
    }

    /// The width of this texture.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// The height of this texture.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// The number of samples for each fragment.
    pub fn number_of_samples(&self) -> u32 {
        self.number_of_samples
    }

    pub(in crate::core) fn bind_as_color_target(&self, channel: u32) {
        unsafe {
            self.context.framebuffer_texture_2d(
                crate::context::FRAMEBUFFER,
                crate::context::COLOR_ATTACHMENT0 + channel,
                crate::context::TEXTURE_2D_MULTISAMPLE,
                Some(self.id),
                0,
            );
        }
    }
    pub(in crate::core) fn bind(&self) {
        unsafe {
            self.context
                .bind_texture(crate::context::TEXTURE_2D_MULTISAMPLE, Some(self.id));
        }
    }
}

impl Drop for Texture2DMultisample {
    fn drop(&mut self) {
        unsafe {
            self.context.delete_texture(self.id);
        }
    }
}
