#![allow(deprecated)]

use crate::core::render_target::*;

///
/// The screen render target which is essential to get something on the screen (see the [write function](Screen::write)).
///
#[deprecated = "use RenderTarget::screen or FrameInput::screen to get the screen render target"]
pub struct Screen {}

impl Screen {
    ///
    /// Call this function and make a render call (for example one of the draw functions on [Program])
    /// in the `render` closure to render something to the screen.
    /// Before writing, the screen is cleared based on the given clear state.
    ///
    pub fn write<F: FnOnce() -> ThreeDResult<()>>(
        context: &Context,
        clear_state: ClearState,
        render: F,
    ) -> ThreeDResult<()> {
        RenderTarget::screen(context, 0, 0)
            .clear_deprecated(clear_state)?
            .write(render)?;
        Ok(())
    }

    ///
    /// Returns the RGBA color values from the screen as a list of bytes (one byte for each color channel).
    ///
    pub fn read_color(context: &Context, viewport: Viewport) -> ThreeDResult<Vec<[u8; 4]>> {
        RenderTarget::screen(context, 0, 0).read_color_viewport(viewport)
    }

    ///
    /// Returns the depth values from the screen as a list of 32-bit floats.
    /// Only available on desktop.
    ///
    #[cfg(not(target_arch = "wasm32"))]
    pub fn read_depth(context: &Context, viewport: Viewport) -> ThreeDResult<Vec<f32>> {
        RenderTarget::screen(context, 0, 0).read_depth_viewport(viewport)
    }

    ///
    /// Copies the content of the color and depth texture to the specified viewport of this render target.
    /// Only copies the channels given by the write mask.
    ///
    pub fn copy_from(
        context: &Context,
        color_texture: Option<&Texture2D>,
        depth_texture: Option<&DepthTargetTexture2D>,
        viewport: Viewport,
        write_mask: WriteMask,
    ) -> ThreeDResult<()> {
        RenderTarget::screen(context, 0, 0).copy_from(
            color_texture,
            depth_texture,
            viewport,
            write_mask,
        )?;
        Ok(())
    }

    ///
    /// Copies the content of the given layers of the color and depth array textures to the specified viewport of this render target.
    /// Only copies the channels given by the write mask.
    ///
    pub fn copy_from_array(
        context: &Context,
        color_texture: Option<(&Texture2DArray, u32)>,
        depth_texture: Option<(&DepthTargetTexture2DArray, u32)>,
        viewport: Viewport,
        write_mask: WriteMask,
    ) -> ThreeDResult<()> {
        RenderTarget::screen(context, 0, 0).copy_from_array(
            color_texture,
            depth_texture,
            viewport,
            write_mask,
        )?;
        Ok(())
    }
}
