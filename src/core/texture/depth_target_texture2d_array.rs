use crate::core::texture::*;

///
/// An array of 2D depth textures that can be rendered into and read from. See also [RenderTarget].
///
pub struct DepthTargetTexture2DArray {
    context: Context,
    id: crate::context::Texture,
    width: u32,
    height: u32,
    depth: u32,
}

impl DepthTargetTexture2DArray {
    ///
    /// Creates a new array of depth target textures.
    ///
    pub fn new(
        context: &Context,
        width: u32,
        height: u32,
        depth: u32,
        wrap_s: Wrapping,
        wrap_t: Wrapping,
        format: DepthFormat,
    ) -> ThreeDResult<Self> {
        let id = generate(context)?;
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
        )?;
        unsafe {
            context.tex_storage_3d(
                crate::context::TEXTURE_2D_ARRAY,
                1,
                internal_format_from_depth(format),
                width as i32,
                height as i32,
                depth as i32,
            );
        }
        context.error_check()?;
        Ok(texture)
    }

    ///
    /// Writes the depth of whatever rendered in the `render` closure into the depth texture defined by the input parameter `layer`.
    /// Before writing, the texture is cleared based on the given clear state.
    ///
    pub fn write<F: FnOnce() -> ThreeDResult<()>>(
        &mut self,
        layer: u32,
        clear_state: Option<f32>,
        render: F,
    ) -> ThreeDResult<()> {
        RenderTarget::new(
            &self.context.clone(),
            ColorRenderTarget::None,
            DepthRenderTarget::Texture2DArray {
                texture: self,
                layer,
            },
        )?
        .write(
            ClearState {
                depth: clear_state,
                ..ClearState::none()
            },
            render,
        )
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

    fn bind(&self) {
        unsafe {
            self.context
                .bind_texture(crate::context::TEXTURE_2D_ARRAY, Some(self.id));
        }
    }
}

impl super::internal::TextureExtensions for DepthTargetTexture2DArray {
    fn bind(&self) {
        self.bind();
    }
}

impl Texture for DepthTargetTexture2DArray {}

impl Drop for DepthTargetTexture2DArray {
    fn drop(&mut self) {
        unsafe {
            self.context.delete_texture(self.id);
        }
    }
}
