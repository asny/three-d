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
    /// Writes the depth of whatever rendered in the `render` closure into the depth texture defined by the input parameter `depth_layer`.
    /// Before writing, the texture is cleared based on the given clear state.
    ///
    pub fn write<F: FnOnce() -> ThreeDResult<()>>(
        &self,
        depth_layer: u32,
        clear_state: Option<f32>,
        render: F,
    ) -> ThreeDResult<()> {
        RenderTargetCubeMap::<u8>::new_depth(&self.context, &self)?.write(
            0,
            depth_layer,
            ClearState {
                depth: clear_state,
                ..ClearState::none()
            },
            render,
        )
    }

    pub(in crate::core) fn bind_as_depth_target(&self, layer: u32) {
        self.context.framebuffer_texture_2d(
            consts::DRAW_FRAMEBUFFER,
            consts::DEPTH_ATTACHMENT,
            consts::TEXTURE_CUBE_MAP_POSITIVE_X + layer,
            &self.id,
            0,
        );
    }
}

impl TextureCube for DepthTargetTextureCubeMap {
    fn bind(&self, location: u32) {
        bind_at(&self.context, &self.id, consts::TEXTURE_CUBE_MAP, location);
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
    fn is_hdr(&self) -> bool {
        false
    }
}

impl Drop for DepthTargetTextureCubeMap {
    fn drop(&mut self) {
        self.context.delete_texture(&self.id);
    }
}
