use crate::core::texture::*;

///
/// An array of 2D depth textures that can be rendered into and read from. See also [RenderTarget] and [DepthTarget].
///
#[deprecated = "Renamed to DepthTexture2DArray"]
pub type DepthTargetTexture2DArray = DepthTexture2DArray;

///
/// An array of 2D depth textures that can be rendered into and read from. See also [RenderTarget] and [DepthTarget].
///
pub struct DepthTexture2DArray {
    context: Context,
    id: crate::context::Texture,
    width: u32,
    height: u32,
    depth: u32,
}

impl DepthTexture2DArray {
    ///
    /// Creates a new array of depth textures.
    ///
    pub fn new<T: DepthTextureDataType>(
        context: &Context,
        width: u32,
        height: u32,
        depth: u32,
        wrap_s: Wrapping,
        wrap_t: Wrapping,
    ) -> Self {
        let id = generate(context);
        let texture = Self {
            context: context.clone(),
            id,
            width,
            height,
            depth,
        };
        texture.bind();
        set_parameters(
            context,
            crate::context::TEXTURE_2D_ARRAY,
            Interpolation::Nearest,
            Interpolation::Nearest,
            None,
            wrap_s,
            wrap_t,
            None,
        );
        unsafe {
            context.tex_storage_3d(
                crate::context::TEXTURE_2D_ARRAY,
                1,
                T::internal_format(),
                width as i32,
                height as i32,
                depth as i32,
            );
        }
        texture
    }

    ///
    /// Returns a [DepthTarget] which can be used to clear, write to and read from the given layer of this texture.
    /// Combine this together with a [ColorTarget] with [RenderTarget::new] to be able to write to both a depth and color target at the same time.
    ///
    pub fn as_depth_target<'a>(&'a mut self, layer: u32) -> DepthTarget<'a> {
        DepthTarget::new_texture_2d_array(&self.context, self, layer)
    }

    /// The width of this texture.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// The height of this texture.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// The number of layers.
    pub fn depth(&self) -> u32 {
        self.depth
    }

    pub(in crate::core) fn bind_as_depth_target(&self, layer: u32) {
        unsafe {
            self.context.framebuffer_texture_layer(
                crate::context::DRAW_FRAMEBUFFER,
                crate::context::DEPTH_ATTACHMENT,
                Some(self.id),
                0,
                layer as i32,
            );
        }
    }

    pub(in crate::core) fn bind(&self) {
        unsafe {
            self.context
                .bind_texture(crate::context::TEXTURE_2D_ARRAY, Some(self.id));
        }
    }
}

impl Drop for DepthTexture2DArray {
    fn drop(&mut self) {
        unsafe {
            self.context.delete_texture(self.id);
        }
    }
}
