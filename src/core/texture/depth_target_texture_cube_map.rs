use crate::context::consts;
use crate::core::texture::*;
use crate::core::*;

///
/// A depth texture cube map that can be rendered into and read from. See also [RenderTargetCubeMap].
///
pub struct DepthTargetTextureCubeMap {
    context: Context,
    id: crate::context::Texture,
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
        set_parameters(
            context,
            &id,
            consts::TEXTURE_CUBE_MAP,
            Interpolation::Nearest,
            Interpolation::Nearest,
            None,
            wrap_s,
            wrap_t,
            Some(wrap_r),
        );
        context.bind_texture(consts::TEXTURE_CUBE_MAP, &id);
        context.tex_storage_2d(
            consts::TEXTURE_CUBE_MAP,
            1,
            internal_format_from_depth(format),
            width,
            height,
        );
        Ok(Self {
            context: context.clone(),
            id,
            width,
            height,
        })
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

    pub(in crate::core) fn bind_as_depth_target(&self, side: CubeMapSide) {
        self.context.framebuffer_texture_2d(
            consts::DRAW_FRAMEBUFFER,
            consts::DEPTH_ATTACHMENT,
            side.to_const(),
            &self.id,
            0,
        );
    }
}

impl Texture for DepthTargetTextureCubeMap {
    fn bind(&self, location: u32) {
        bind_at(&self.context, &self.id, consts::TEXTURE_CUBE_MAP, location);
    }
    fn width(&self) -> u32 {
        self.width
    }
    fn height(&self) -> u32 {
        self.height
    }
    fn depth(&self) -> u32 {
        1
    }
    fn format(&self) -> Format {
        Format::R
    }
}

impl Drop for DepthTargetTextureCubeMap {
    fn drop(&mut self) {
        self.context.delete_texture(&self.id);
    }
}
