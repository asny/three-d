use crate::core::texture::*;
use glow::HasContext;

///
/// A depth texture cube map that can be rendered into and read from. See also [RenderTargetCubeMap].
///
pub struct DepthTargetTextureCubeMap {
    context: Context,
    id: glow::Texture,
    width: u32,
    height: u32,
}

impl DepthTargetTextureCubeMap {
    ///
    /// Creates a new depth target texture cube map.
    ///
    pub fn new(
        context: &Context,
        width: u32,
        height: u32,
        wrap_s: Wrapping,
        wrap_t: Wrapping,
        wrap_r: Wrapping,
        format: DepthFormat,
    ) -> ThreeDResult<Self> {
        let id = generate(context)?;
        let texture = Self {
            context: context.clone(),
            id,
            width,
            height,
        };
        texture.bind();
        set_parameters(
            context,
            glow::TEXTURE_CUBE_MAP,
            Interpolation::Nearest,
            Interpolation::Nearest,
            None,
            wrap_s,
            wrap_t,
            Some(wrap_r),
        );
        context.tex_storage_2d(
            glow::TEXTURE_CUBE_MAP,
            1,
            internal_format_from_depth(format),
            width as i32,
            height as i32,
        );
        Ok(texture)
    }

    ///
    /// Writes the depth of whatever rendered in the `render` closure into the depth texture at the cube map side given by the input parameter `side`.
    /// Before writing, the texture side is cleared based on the given clear state.
    ///
    pub fn write(
        &mut self,
        side: CubeMapSide,
        clear_state: Option<f32>,
        render: impl FnOnce() -> ThreeDResult<()>,
    ) -> ThreeDResult<()> {
        RenderTargetCubeMap::new_depth(&self.context.clone(), self)?.write(
            side,
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

    pub(in crate::core) fn bind_as_depth_target(&self, side: CubeMapSide) {
        self.context.framebuffer_texture_2d(
            glow::DRAW_FRAMEBUFFER,
            glow::DEPTH_ATTACHMENT,
            side.to_const(),
            Some(self.id),
            0,
        );
    }

    fn bind(&self) {
        self.context
            .bind_texture(glow::TEXTURE_CUBE_MAP, Some(self.id));
    }
}

impl super::internal::TextureExtensions for DepthTargetTextureCubeMap {
    fn bind(&self) {
        self.bind();
    }
}

impl Texture for DepthTargetTextureCubeMap {}

impl Drop for DepthTargetTextureCubeMap {
    fn drop(&mut self) {
        self.context.delete_texture(self.id);
    }
}
