//!
//! Functionality for rendering to the screen or into textures.
//!

mod clear_state;
#[doc(inline)]
pub use clear_state::*;

mod color_target;
#[doc(inline)]
pub use color_target::*;

mod depth_target;
#[doc(inline)]
pub use depth_target::*;

mod multisample;
#[doc(inline)]
pub use multisample::*;

mod color_target_multisample;
#[doc(inline)]
pub use color_target_multisample::*;

mod depth_target_multisample;
#[doc(inline)]
pub use depth_target_multisample::*;

use crate::core::*;

use crate::context::Framebuffer;
///
/// Adds additional functionality to clear, read from and write to the screen (see [RenderTarget::screen]) or a color texture and
/// a depth texture at the same time (see [RenderTarget::new]).
/// If you only want to perform an operation on either a color texture or depth texture, see [ColorTarget] and [DepthTarget] respectively.
/// A render target purely adds functionality, so it can be created each time it is needed, the actual data is saved in the textures.
///
pub struct RenderTarget<'a> {
    id: Option<Framebuffer>,
    color: Option<ColorTarget<'a>>,
    depth: Option<DepthTarget<'a>>,
    pub(crate) context: Context,
    width: u32,
    height: u32,
}

impl<'a> RenderTarget<'a> {
    ///
    /// Returns the screen render target for this context.
    /// Write to this render target to draw something on the screen.
    ///
    pub fn screen(context: &Context, width: u32, height: u32) -> Self {
        Self {
            context: context.clone(),
            id: None,
            color: None,
            depth: None,
            width,
            height,
        }
    }

    ///
    /// Constructs a new render target that enables rendering into the given [ColorTarget] and [DepthTarget].
    ///
    pub fn new(color: ColorTarget<'a>, depth: DepthTarget<'a>) -> Self {
        let width = color.width();
        let height = color.height();
        Self {
            context: color.context.clone(),
            id: Some(new_framebuffer(&color.context)),
            color: Some(color),
            depth: Some(depth),
            width,
            height,
        }
    }

    /// The width of this target.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// The height of this target.
    pub fn height(&self) -> u32 {
        self.height
    }

    ///
    /// Clears the color and depth of this render target as defined by the given clear state.
    ///
    pub fn clear(&self, clear_state: ClearState) -> &Self {
        self.clear_partially(self.scissor_box(), clear_state)
    }

    ///
    /// Clears the color and depth of the part of this render target that is inside the given scissor box.
    ///
    pub fn clear_partially(&self, scissor_box: ScissorBox, clear_state: ClearState) -> &Self {
        self.context.set_scissor(scissor_box);
        self.bind(crate::context::DRAW_FRAMEBUFFER);
        clear_state.apply(&self.context);
        self
    }

    ///
    /// Writes whatever rendered in the `render` closure into this render target.
    ///
    pub fn write<E: std::error::Error>(
        &self,
        render: impl FnOnce() -> Result<(), E>,
    ) -> Result<&Self, E> {
        self.write_partially(self.scissor_box(), render)
    }

    ///
    /// Writes whatever rendered in the `render` closure into the part of this render target defined by the scissor box.
    ///
    pub fn write_partially<E: std::error::Error>(
        &self,
        scissor_box: ScissorBox,
        render: impl FnOnce() -> Result<(), E>,
    ) -> Result<&Self, E> {
        self.context.set_scissor(scissor_box);
        self.bind(crate::context::DRAW_FRAMEBUFFER);
        render()?;
        if let Some(ref color) = self.color {
            color.generate_mip_maps();
        }
        Ok(self)
    }

    ///
    /// Returns the colors of the pixels in this render target.
    /// The number of channels per pixel and the data format for each channel returned from this function is specified by the generic parameter `T`.
    ///
    /// **Note:**
    /// The base type of the generic parameter `T` must match the base type of the render target, for example if the render targets base type is `u8`, the base type of `T` must also be `u8`.
    ///
    /// **Web:**
    /// The generic parameter `T` is limited to:
    /// - Unsigned byte RGBA (Specify `T` as either `Vec4<u8>` or `[u8; 4]`) which works with any render target using `u8` as its base type.
    /// - 32-bit float RGBA (Specify `T` as either `Vec4<f32>` or `[f32; 4]`) which works with any render target using `f16` or `f32` as its base type.
    ///
    pub fn read_color<T: TextureDataType>(&self) -> Vec<T> {
        self.read_color_partially(self.scissor_box())
    }

    ///
    /// Returns the colors of the pixels in this render target inside the given scissor box.
    /// The number of channels per pixel and the data format for each channel returned from this function is specified by the generic parameter `T`.
    ///
    /// **Note:**
    /// The base type of the generic parameter `T` must match the base type of the render target, for example if the render targets base type is `u8`, the base type of `T` must also be `u8`.
    ///
    /// **Web:**
    /// The generic parameter `T` is limited to:
    /// - Unsigned byte RGBA (Specify `T` as either `Vec4<u8>` or `[u8; 4]`) which works with any render target using `u8` as its base type.
    /// - 32-bit float RGBA (Specify `T` as either `Vec4<f32>` or `[f32; 4]`) which works with any render target using `f16` or `f32` as its base type.
    ///
    pub fn read_color_partially<T: TextureDataType>(&self, scissor_box: ScissorBox) -> Vec<T> {
        if self.id.is_some() && self.color.is_none() {
            panic!("Cannot read color from a render target without a color target");
        }
        let format = format_from_data_type::<T>();
        let data_type = T::data_type();

        // On web, the read format needs to be RGBA and f16 is not supported (see https://webglfundamentals.org/webgl/lessons/webgl-readpixels.html).
        #[cfg(target_arch = "wasm32")]
        if format != crate::context::RGBA
            || !(data_type == crate::context::UNSIGNED_BYTE || data_type == crate::context::FLOAT)
        {
            panic!("Only the texture data types `Vec4<T>` and `[T; 4]` where `T` is either `u8` or `f32` are supported when reading color from a render target on web.");
        }

        self.bind(crate::context::DRAW_FRAMEBUFFER);
        self.bind(crate::context::READ_FRAMEBUFFER);
        let data_size = std::mem::size_of::<T>();
        let mut bytes =
            vec![0u8; scissor_box.width as usize * scissor_box.height as usize * data_size];
        unsafe {
            self.context.read_pixels(
                scissor_box.x,
                scissor_box.y,
                scissor_box.width as i32,
                scissor_box.height as i32,
                format,
                data_type,
                crate::context::PixelPackData::Slice(&mut bytes),
            );
        }
        let mut pixels = from_byte_slice(&bytes).to_vec();
        flip_y(
            &mut pixels,
            scissor_box.width as usize,
            scissor_box.height as usize,
        );
        pixels
    }

    ///
    /// Returns the depth values in this render target.
    ///
    #[cfg(not(target_arch = "wasm32"))]
    pub fn read_depth(&self) -> Vec<f32> {
        self.read_depth_partially(self.scissor_box())
    }

    ///
    /// Returns the depth values in this render target inside the given scissor box.
    ///
    #[cfg(not(target_arch = "wasm32"))]
    pub fn read_depth_partially(&self, scissor_box: ScissorBox) -> Vec<f32> {
        if self.id.is_some() && self.depth.is_none() {
            panic!("cannot read depth from a render target without a depth target");
        }
        self.bind(crate::context::DRAW_FRAMEBUFFER);
        self.bind(crate::context::READ_FRAMEBUFFER);
        let mut pixels = vec![0u8; scissor_box.width as usize * scissor_box.height as usize * 4];
        unsafe {
            self.context.read_pixels(
                scissor_box.x,
                scissor_box.y,
                scissor_box.width as i32,
                scissor_box.height as i32,
                crate::context::DEPTH_COMPONENT,
                crate::context::FLOAT,
                crate::context::PixelPackData::Slice(&mut pixels),
            );
        }
        from_byte_slice(&pixels).to_vec()
    }

    ///
    /// Creates a [RenderTarget] with the given low-level [Framebuffer]. Should only be used if the [Framebuffer] is used for something else, ie. to be able
    /// to combine this crate with functionality of another crate. Also see [Self::into_framebuffer].
    ///
    pub fn from_framebuffer(
        context: &Context,
        width: u32,
        height: u32,
        framebuffer: Framebuffer,
    ) -> Self {
        Self {
            id: Some(framebuffer),
            color: None,
            depth: None,
            context: context.clone(),
            width,
            height,
        }
    }

    ///
    /// Transforms this [RenderTarget] into a low-level [Framebuffer]. Should only be used if the [Framebuffer] is used for something else, ie. to be able
    /// to combine this crate with functionality of another crate. Also see [Self::from_framebuffer].
    ///
    pub fn into_framebuffer(mut self) -> Option<Framebuffer> {
        self.id.take()
    }

    pub(in crate::core) fn blit_to(&self, target: &RenderTarget) {
        self.bind(crate::context::DRAW_FRAMEBUFFER);
        target.bind(crate::context::DRAW_FRAMEBUFFER);
        let target_is_screen = target.color.is_none() && target.depth.is_none();
        let mask = if self.color.is_some() && (target.color.is_some() || target_is_screen) {
            let mut mask = crate::context::COLOR_BUFFER_BIT;
            if self.depth.is_some() && (target.depth.is_some() || target_is_screen) {
                mask |= crate::context::DEPTH_BUFFER_BIT;
            }
            mask
        } else if self.depth.is_some() && (target.depth.is_some() || target_is_screen) {
            crate::context::DEPTH_BUFFER_BIT
        } else {
            unreachable!()
        };
        unsafe {
            self.context
                .bind_framebuffer(crate::context::READ_FRAMEBUFFER, self.id);

            self.context.blit_framebuffer(
                0,
                0,
                self.width as i32,
                self.height as i32,
                0,
                0,
                target.width as i32,
                target.height as i32,
                mask,
                crate::context::NEAREST,
            );
        }
    }

    fn new_color(color: ColorTarget<'a>) -> Self {
        let width = color.width();
        let height = color.height();
        Self {
            context: color.context.clone(),
            id: Some(new_framebuffer(&color.context)),
            color: Some(color),
            depth: None,
            width,
            height,
        }
    }

    fn new_depth(depth: DepthTarget<'a>) -> Self {
        let width = depth.width();
        let height = depth.height();
        Self {
            context: depth.context.clone(),
            id: Some(new_framebuffer(&depth.context)),
            depth: Some(depth),
            color: None,
            width,
            height,
        }
    }

    fn bind(&self, target: u32) {
        unsafe {
            self.context.bind_framebuffer(target, self.id);
        }
        if let Some(ref color) = self.color {
            color.bind(&self.context);
        }
        if let Some(ref depth) = self.depth {
            depth.bind();
        }
    }
}

impl Drop for RenderTarget<'_> {
    fn drop(&mut self) {
        unsafe {
            if let Some(id) = self.id {
                self.context.delete_framebuffer(id);
            }
        }
    }
}

fn size_with_mip(size: u32, mip: Option<u32>) -> u32 {
    if let Some(mip) = mip {
        size / 2u32.pow(mip)
    } else {
        size
    }
}

fn new_framebuffer(context: &Context) -> crate::context::Framebuffer {
    unsafe {
        context
            .create_framebuffer()
            .expect("Failed creating frame buffer")
    }
}

#[cfg(debug_assertions)]
fn multisample_sanity_check(context: &Context, number_of_samples: u32) {
    let max_samples: u32 = unsafe {
        context
            .get_parameter_i32(crate::context::MAX_SAMPLES)
            .try_into()
            .unwrap()
    };
    if number_of_samples > max_samples {
        panic!("number_of_samples ({}) for multisample target is larger than supported number of samples: {}", number_of_samples, max_samples);
    }
    if (number_of_samples != 0) && number_of_samples & (number_of_samples - 1) != 0 {
        panic!("number_of_samples ({}) for multisample target must be a power of 2 (and larger than 0).", number_of_samples);
    }
}

macro_rules! impl_render_target_core_extensions_body {
    () => {
        ///
        /// Returns the scissor box that encloses the entire target.
        ///
        pub fn scissor_box(&self) -> ScissorBox {
            ScissorBox::new_at_origo(self.width(), self.height())
        }

        ///
        /// Returns the viewport that encloses the entire target.
        ///
        pub fn viewport(&self) -> Viewport {
            Viewport::new_at_origo(self.width(), self.height())
        }
    };
}

macro_rules! impl_render_target_core_extensions {
    // 2 generic arguments with bounds
    ($name:ident < $a:ident : $ta:tt , $b:ident : $tb:tt >) => {
        impl<$a: $ta, $b: $tb> $name<$a, $b> {
            impl_render_target_core_extensions_body!();
        }
    };
    // 1 generic argument with bound
    ($name:ident < $a:ident : $ta:tt >) => {
        impl<$a: $ta> $name<$a> {
            impl_render_target_core_extensions_body!();
        }
    };
    // 1 liftetime argument
    ($name:ident < $lt:lifetime >) => {
        impl<$lt> $name<$lt> {
            impl_render_target_core_extensions_body!();
        }
    };
    // without any arguments
    ($name:ty) => {
        impl $name {
            impl_render_target_core_extensions_body!();
        }
    };
}

impl_render_target_core_extensions!(RenderTarget<'a>);
impl_render_target_core_extensions!(ColorTarget<'a>);
impl_render_target_core_extensions!(DepthTarget<'a>);
impl_render_target_core_extensions!(
    RenderTargetMultisample<C: TextureDataType, D: DepthTextureDataType>
);
impl_render_target_core_extensions!(ColorTargetMultisample<C: TextureDataType>);
impl_render_target_core_extensions!(DepthTargetMultisample<D: DepthTextureDataType>);
