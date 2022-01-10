use crate::context::{consts, DataType};
use crate::core::render_target::*;
use crate::core::*;

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
        context.bind_framebuffer(consts::DRAW_FRAMEBUFFER, None);
        clear(context, &clear_state);
        render()?;
        Ok(())
    }

    ///
    /// Returns the RGBA color values from the screen as a list of bytes (one byte for each color channel).
    ///
    pub fn read_color(context: &Context, viewport: Viewport) -> ThreeDResult<Vec<u8>> {
        let mut pixels = vec![0u8; viewport.width as usize * viewport.height as usize * 4];
        context.bind_framebuffer(consts::READ_FRAMEBUFFER, None);
        context.read_pixels_with_u8_data(
            viewport.x as u32,
            viewport.y as u32,
            viewport.width,
            viewport.height,
            consts::RGBA,
            DataType::UnsignedByte,
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
        context.bind_framebuffer(consts::READ_FRAMEBUFFER, None);
        context.read_pixels_with_f32_data(
            viewport.x as u32,
            viewport.y as u32,
            viewport.width,
            viewport.height,
            consts::DEPTH_COMPONENT,
            DataType::Float,
            &mut pixels,
        );
        Ok(pixels)
    }

    ///
    /// Copies the content of the color and depth texture to the specified viewport of this render target.
    /// Only copies the channels given by the write mask.
    ///
    pub fn copy_from(
        context: &Context,
        color_texture: Option<&impl Texture>,
        depth_texture: Option<&impl Texture>,
        viewport: Viewport,
        write_mask: WriteMask,
    ) -> ThreeDResult<()> {
        Self::write(context, ClearState::none(), || {
            copy_from(context, color_texture, depth_texture, viewport, write_mask)
        })
    }
}
