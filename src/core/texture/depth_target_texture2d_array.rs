use crate::context::consts;
use crate::core::texture::*;
use crate::core::*;

///
/// An array of 2D depth textures that can be rendered into and read from. See also [RenderTargetArray].
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
        set_parameters(
            context,
            &id,
            consts::TEXTURE_2D_ARRAY,
            Interpolation::Nearest,
            Interpolation::Nearest,
            None,
            wrap_s,
            wrap_t,
            None,
        );
        context.bind_texture(consts::TEXTURE_2D_ARRAY, &id);
        context.tex_storage_3d(
            consts::TEXTURE_2D_ARRAY,
            1,
            internal_format_from_depth(format),
            width,
            height,
            depth,
        );
        Ok(Self {
            context: context.clone(),
            id,
            width,
            height,
            depth,
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
        RenderTargetArray::new_depth(&self.context, &self)?.write(
            &[],
            depth_layer,
            ClearState {
                depth: clear_state,
                ..ClearState::none()
            },
            render,
        )
    }

    ///
    /// Copies the content of the depth texture at the given layer to the specified destination at the given viewport.
    ///
    /// # Errors
    /// Will return an error if the destination is a color texture.
    ///
    pub fn copy_to<T: TextureDataType>(
        &self,
        depth_layer: u32,
        destination: CopyDestination<T>,
        viewport: Viewport,
    ) -> ThreeDResult<()> {
        RenderTargetArray::new_depth_internal(&self.context, &self)?.copy_to(
            0,
            depth_layer,
            destination,
            viewport,
            WriteMask::DEPTH,
        )
    }

    pub(in crate::core) fn bind_as_depth_target(&self, layer: u32) {
        self.context.framebuffer_texture_layer(
            consts::DRAW_FRAMEBUFFER,
            consts::DEPTH_ATTACHMENT,
            &self.id,
            0,
            layer as u32,
        );
    }
}

impl TextureArray for DepthTargetTexture2DArray {
    fn bind(&self, location: u32) {
        bind_at(&self.context, &self.id, consts::TEXTURE_2D_ARRAY, location);
    }
    fn width(&self) -> u32 {
        self.width
    }
    fn height(&self) -> u32 {
        self.height
    }
    fn depth(&self) -> u32 {
        self.depth
    }
    fn format(&self) -> Format {
        Format::R
    }
}

impl Drop for DepthTargetTexture2DArray {
    fn drop(&mut self) {
        self.context.delete_texture(&self.id);
    }
}
