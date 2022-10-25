use crate::core::texture::*;

///
/// A depth texture cube map that can be rendered into and read from. See also [RenderTarget] and [DepthTarget].
///
#[deprecated = "Renamed to DepthTextureCubeMap"]
pub type DepthTargetTextureCubeMap = DepthTextureCubeMap;

///
/// A depth texture cube map that can be rendered into and read from. See also [RenderTarget] and [DepthTarget].
///
pub struct DepthTextureCubeMap {
    context: Context,
    id: crate::context::Texture,
    width: u32,
    height: u32,
}

impl DepthTextureCubeMap {
    ///
    /// Creates a new depth texture cube map.
    ///
    pub fn new<T: DepthTextureDataType>(
        context: &Context,
        width: u32,
        height: u32,
        wrap_s: Wrapping,
        wrap_t: Wrapping,
        wrap_r: Wrapping,
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
            crate::context::TEXTURE_CUBE_MAP,
            Interpolation::Nearest,
            Interpolation::Nearest,
            None,
            wrap_s,
            wrap_t,
            Some(wrap_r),
        );
        unsafe {
            context.tex_storage_2d(
                crate::context::TEXTURE_CUBE_MAP,
                1,
                T::internal_format(),
                width as i32,
                height as i32,
            );
        }
        texture
    }

    ///
    /// Returns a [DepthTarget] which can be used to clear, write to and read from the given side of this texture.
    /// Combine this together with a [ColorTarget] with [RenderTarget::new] to be able to write to both a depth and color target at the same time.
    ///
    pub fn as_depth_target<'a>(&'a mut self, side: CubeMapSide) -> DepthTarget<'a> {
        DepthTarget::new_texture_cube_map(&self.context, self, side)
    }

    /// The width of this texture.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// The height of this texture.
    pub fn height(&self) -> u32 {
        self.height
    }

    pub(in crate::core) fn bind_as_depth_target(&self, side: CubeMapSide) {
        unsafe {
            self.context.framebuffer_texture_2d(
                crate::context::DRAW_FRAMEBUFFER,
                crate::context::DEPTH_ATTACHMENT,
                side.to_const(),
                Some(self.id),
                0,
            );
        }
    }

    pub(in crate::core) fn bind(&self) {
        unsafe {
            self.context
                .bind_texture(crate::context::TEXTURE_CUBE_MAP, Some(self.id));
        }
    }
}

impl Drop for DepthTextureCubeMap {
    fn drop(&mut self) {
        unsafe {
            self.context.delete_texture(self.id);
        }
    }
}
