use crate::core::render_target::*;
use glow::HasContext;

///
/// The screen render target which is essential to get something on the screen (see the [write function](Screen::write)).
///
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
        context.bind_framebuffer(glow::DRAW_FRAMEBUFFER, None);
        clear(context, &clear_state);
        render()?;
        Ok(())
    }

    ///
    /// Returns the RGBA color values from the screen as a list of bytes (one byte for each color channel).
    ///
    pub fn read_color(context: &Context, viewport: Viewport) -> ThreeDResult<Vec<u8>> {
        let mut pixels = vec![0u8; viewport.width as usize * viewport.height as usize * 4];
        context.bind_framebuffer(glow::READ_FRAMEBUFFER, None);
        context.read_pixels_with_u8_data(
            viewport.x as u32,
            viewport.y as u32,
            viewport.width,
            viewport.height,
            glow::RGBA,
            glow::UNSIGNED_BYTE,
            &mut pixels,
        );
        Ok(pixels)
    }

    ///
    /// Returns the depth values from the screen as a list of 32-bit floats.
    /// Only available on desktop.
    ///
    #[cfg(not(target_arch = "wasm32"))]
    pub fn read_depth(context: &Context, viewport: Viewport) -> ThreeDResult<Vec<f32>> {
        let mut pixels = vec![0f32; viewport.width as usize * viewport.height as usize];
        context.bind_framebuffer(glow::READ_FRAMEBUFFER, None);
        context.read_pixels_with_f32_data(
            viewport.x as u32,
            viewport.y as u32,
            viewport.width,
            viewport.height,
            glow::DEPTH_COMPONENT,
            glow::FLOAT,
            &mut pixels,
        );
        Ok(pixels)
    }

    ///
    /// Copies the content of the color and depth texture to the specified viewport of this render target.
    /// Only copies the channels given by the write mask.
    ///
    pub fn copy_from<T: TextureDataType>(
        context: &Context,
        color_texture: Option<&Texture2D<T>>,
        depth_texture: Option<&DepthTargetTexture2D>,
        viewport: Viewport,
        write_mask: WriteMask,
    ) -> ThreeDResult<()> {
        Self::write(context, ClearState::none(), || {
            copy_from(context, color_texture, depth_texture, viewport, write_mask)
        })
    }

    ///
    /// Copies the content of the given layers of the color and depth array textures to the specified viewport of this render target.
    /// Only copies the channels given by the write mask.
    ///
    pub fn copy_from_array<T: TextureDataType>(
        context: &Context,
        color_texture: Option<(&Texture2DArray<T>, u32)>,
        depth_texture: Option<(&DepthTargetTexture2DArray, u32)>,
        viewport: Viewport,
        write_mask: WriteMask,
    ) -> ThreeDResult<()> {
        Self::write(context, ClearState::none(), || {
            copy_from_array(context, color_texture, depth_texture, viewport, write_mask)
        })
    }
}
