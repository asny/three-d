use crate::core::render_target::*;

pub enum Target<'a> {
    Screen {
        width: u32,
        height: u32,
    },
    Color(ColorTarget<'a>),
    Depth(DepthTarget<'a>),
    ColorAndDepth {
        color: ColorTarget<'a>,
        depth: DepthTarget<'a>,
    },
}

impl<'a> Target<'a> {
    fn width(&self) -> u32 {
        match self {
            Self::ColorAndDepth { color, .. } => color.width(),
            Self::Color(color) => color.width(),
            Self::Depth(depth) => depth.width(),
            Self::Screen { width, .. } => *width,
        }
    }

    fn height(&self) -> u32 {
        match self {
            Self::ColorAndDepth { color, .. } => color.height(),
            Self::Color(color) => color.height(),
            Self::Depth(depth) => depth.height(),
            Self::Screen { height, .. } => *height,
        }
    }

    fn bind(&self, context: &Context) {
        match self {
            Self::ColorAndDepth { color, depth } => {
                color.bind(context);
                depth.bind();
            }
            Self::Color(color) => color.bind(context),
            Self::Depth(depth) => depth.bind(),
            Self::Screen { .. } => {}
        }
    }
}

pub enum ColorTarget<'a> {
    Texture2D {
        texture: &'a mut Texture2D,
        mip_level: Option<u32>,
    },
    Texture2DArray {
        texture: &'a mut Texture2DArray,
        layers: &'a [u32],
        mip_level: Option<u32>,
    },
    TextureCubeMap {
        texture: &'a mut TextureCubeMap,
        side: CubeMapSide,
        mip_level: Option<u32>,
    },
}

impl<'a> ColorTarget<'a> {
    fn width(&self) -> u32 {
        match self {
            Self::Texture2D { texture, .. } => texture.width(),
            Self::Texture2DArray { texture, .. } => texture.width(),
            Self::TextureCubeMap { texture, .. } => texture.width(),
        }
    }

    fn height(&self) -> u32 {
        match self {
            Self::Texture2D { texture, .. } => texture.height(),
            Self::Texture2DArray { texture, .. } => texture.height(),
            Self::TextureCubeMap { texture, .. } => texture.height(),
        }
    }

    fn generate_mip_maps(&self) {
        match self {
            Self::Texture2D { texture, mip_level } => {
                if mip_level.is_none() {
                    texture.generate_mip_maps()
                }
            }
            Self::Texture2DArray {
                texture, mip_level, ..
            } => {
                if mip_level.is_none() {
                    texture.generate_mip_maps()
                }
            }
            Self::TextureCubeMap {
                texture, mip_level, ..
            } => {
                if mip_level.is_none() {
                    texture.generate_mip_maps()
                }
            }
        }
    }

    fn bind(&self, context: &Context) {
        match self {
            Self::Texture2D { texture, mip_level } => unsafe {
                context.draw_buffers(&[crate::context::COLOR_ATTACHMENT0]);
                texture.bind_as_color_target(0, mip_level.unwrap_or(0));
            },
            Self::Texture2DArray {
                texture,
                layers,
                mip_level,
            } => unsafe {
                context.draw_buffers(
                    &(0..layers.len())
                        .map(|i| crate::context::COLOR_ATTACHMENT0 + i as u32)
                        .collect::<Vec<u32>>(),
                );
                for channel in 0..layers.len() {
                    texture.bind_as_color_target(
                        layers[channel],
                        channel as u32,
                        mip_level.unwrap_or(0),
                    );
                }
            },
            Self::TextureCubeMap {
                texture,
                side,
                mip_level,
            } => unsafe {
                context.draw_buffers(&[crate::context::COLOR_ATTACHMENT0]);
                texture.bind_as_color_target(*side, 0, mip_level.unwrap_or(0));
            },
        }
    }
}

pub enum DepthTarget<'a> {
    Texture2D {
        texture: &'a mut DepthTargetTexture2D,
    },
    Texture2DArray {
        texture: &'a mut DepthTargetTexture2DArray,
        layer: u32,
    },
    TextureCubeMap {
        texture: &'a mut DepthTargetTextureCubeMap,
        side: CubeMapSide,
    },
}

impl<'a> DepthTarget<'a> {
    fn width(&self) -> u32 {
        match self {
            Self::Texture2D { texture, .. } => texture.width(),
            Self::Texture2DArray { texture, .. } => texture.width(),
            Self::TextureCubeMap { texture, .. } => texture.width(),
        }
    }

    fn height(&self) -> u32 {
        match self {
            Self::Texture2D { texture, .. } => texture.height(),
            Self::Texture2DArray { texture, .. } => texture.height(),
            Self::TextureCubeMap { texture, .. } => texture.height(),
        }
    }

    fn bind(&self) {
        match self {
            Self::Texture2D { texture } => {
                texture.bind_as_depth_target();
            }
            Self::Texture2DArray { texture, layer } => {
                texture.bind_as_depth_target(*layer);
            }
            Self::TextureCubeMap { texture, side } => {
                texture.bind_as_depth_target(*side);
            }
        }
    }
}

///
/// Adds additional functionality to read from, write to and copy from both a color texture (see [ColorTarget]) and
/// a depth texture (see [DepthTarget]) at the same time.
/// If you only want to perform an operation on either a color texture or depth texture,
/// use the functionality directly on the texture structs (for example [Texture2D]).
/// It purely adds functionality, so it can be created each time it is needed, the actual data is saved in the textures.
///
pub struct RenderTarget<'a> {
    context: Context,
    id: Option<crate::context::Framebuffer>,
    target: Target<'a>,
}

impl<'a> RenderTarget<'a> {
    ///
    /// Returns the screen render target for this context.
    /// Write to this render target to draw something on the screen.
    ///
    pub fn screen(context: &Context, width: u32, height: u32) -> Self {
        Self::new(context, Target::Screen { width, height }).unwrap()
    }

    ///
    /// Constructs a new render target that enables rendering into the given [ColorTarget] and [DepthTarget].
    ///
    pub fn new(context: &Context, target: Target<'a>) -> ThreeDResult<Self> {
        Ok(Self {
            context: context.clone(),
            id: if let Target::Screen { .. } = target {
                None
            } else {
                Some(new_framebuffer(context)?)
            },
            target,
        })
    }

    ///
    /// Constructs a new render target that enables rendering into the given [Texture2D].
    ///
    #[deprecated = "use new with no depth target or call the render_target metod on Texture2D"]
    pub fn new_color(context: &Context, texture: &'a mut Texture2D) -> ThreeDResult<Self> {
        Self::new(
            context,
            Target::Color(ColorTarget::Texture2D {
                texture,
                mip_level: None,
            }),
        )
    }

    ///
    /// Constructs a new render target that enables rendering into the given [DepthTargetTexture2D].
    ///
    #[deprecated = "use new with no color target or call the render_target method on DepthTargetTexture2D"]
    pub fn new_depth(
        context: &Context,
        texture: &'a mut DepthTargetTexture2D,
    ) -> ThreeDResult<Self> {
        Self::new(context, Target::Depth(DepthTarget::Texture2D { texture }))
    }

    pub fn clear(&self, mut clear_state: ClearState) -> ThreeDResult<&Self> {
        self.bind(crate::context::DRAW_FRAMEBUFFER)?;
        match self.target {
            Target::Depth { .. } => {
                clear_state.red = None;
                clear_state.green = None;
                clear_state.blue = None;
                clear_state.alpha = None;
            }
            Target::Color { .. } => clear_state.depth = None,
            _ => {}
        }
        clear(&self.context, &clear_state);
        self.context.error_check()?;
        Ok(self)
    }

    ///
    /// Renders whatever rendered in the `render` closure into the textures defined at construction.
    /// Before writing, the textures are cleared based on the given clear state.
    ///
    pub fn write(&self, render: impl FnOnce() -> ThreeDResult<()>) -> ThreeDResult<&Self> {
        self.write_to_area(self.area(), render)
    }

    pub fn write_to_area(
        &self,
        area: Rectangle,
        render: impl FnOnce() -> ThreeDResult<()>,
    ) -> ThreeDResult<&Self> {
        set_scissor(&self.context, area);
        self.bind(crate::context::DRAW_FRAMEBUFFER)?;
        render()?;
        if let Target::Color(ref color) = self.target {
            color.generate_mip_maps();
        } else if let Target::ColorAndDepth { ref color, .. } = self.target {
            color.generate_mip_maps();
        }
        self.context.error_check()?;
        Ok(self)
    }

    ///
    /// Returns the values of the pixels in this texture inside the given viewport.
    /// The number of channels per pixel and the data format for each channel is specified by the generic parameter.
    ///
    /// **Note:** On web, the data format needs to match the data format of the color texture.
    ///
    pub fn read_color<T: TextureDataType>(&self) -> ThreeDResult<Vec<T>> {
        self.read_color_area(self.area())
    }

    pub fn read_color_area<T: TextureDataType>(&self, area: Rectangle) -> ThreeDResult<Vec<T>> {
        if let Target::Depth { .. } = self.target {
            Err(CoreError::RenderTargetRead("color".to_string()))?;
        }
        self.bind(crate::context::DRAW_FRAMEBUFFER)?;
        self.bind(crate::context::READ_FRAMEBUFFER)?;
        let mut data_size = std::mem::size_of::<T>();
        // On web, the format needs to be RGBA if the data type is byte.
        if data_size / T::size() as usize == 1 {
            data_size *= 4 / T::size() as usize
        }
        let mut bytes = vec![0u8; area.width as usize * area.height as usize * data_size];
        unsafe {
            self.context.read_pixels(
                area.x as i32,
                area.y as i32,
                area.width as i32,
                area.height as i32,
                format_from_data_type::<T>(),
                T::data_type(),
                crate::context::PixelPackData::Slice(&mut bytes),
            );
            self.context.error_check()?;
        }
        let mut pixels = from_byte_slice(&bytes).to_vec();
        flip_y(&mut pixels, area.width as usize, area.height as usize);
        Ok(pixels)
    }

    ///
    /// Returns the depth values from the screen as a list of 32-bit floats.
    /// Only available on desktop.
    ///
    #[cfg(not(target_arch = "wasm32"))]
    pub fn read_depth_area(&self, area: Rectangle) -> ThreeDResult<Vec<f32>> {
        if let Target::Color { .. } = self.target {
            Err(CoreError::RenderTargetRead("depth".to_string()))?;
        }
        self.bind(crate::context::DRAW_FRAMEBUFFER)?;
        self.bind(crate::context::READ_FRAMEBUFFER)?;
        let mut pixels = vec![0u8; area.width as usize * area.height as usize * 4];
        unsafe {
            self.context.read_pixels(
                area.x as i32,
                area.y as i32,
                area.width as i32,
                area.height as i32,
                crate::context::DEPTH_COMPONENT,
                crate::context::FLOAT,
                crate::context::PixelPackData::Slice(&mut pixels),
            );
        }
        self.context.error_check()?;
        Ok(from_byte_slice(&pixels).to_vec())
    }

    ///
    /// Copies the content of the color and depth texture to the specified viewport of this render target.
    /// Only copies the channels given by the write mask.
    ///
    pub fn copy_from(
        &self,
        color_texture: Option<&Texture2D>,
        depth_texture: Option<&DepthTargetTexture2D>,
        viewport: Viewport,
        write_mask: WriteMask,
    ) -> ThreeDResult<&Self> {
        self.write(|| {
            copy_from(
                &self.context,
                color_texture,
                depth_texture,
                viewport,
                write_mask,
            )
        })
    }

    ///
    /// Copies the content of the given layers of the color and depth array textures to the specified viewport of this render target.
    /// Only copies the channels given by the write mask.
    ///
    pub fn copy_from_array(
        &self,
        color_texture: Option<(&Texture2DArray, u32)>,
        depth_texture: Option<(&DepthTargetTexture2DArray, u32)>,
        viewport: Viewport,
        write_mask: WriteMask,
    ) -> ThreeDResult<&Self> {
        self.write(|| {
            copy_from_array(
                &self.context,
                color_texture,
                depth_texture,
                viewport,
                write_mask,
            )
        })
    }

    pub fn area(&self) -> Rectangle {
        Rectangle::new_at_origo(self.target.width(), self.target.height())
    }

    pub(in crate::core) fn bind(&self, target: u32) -> ThreeDResult<()> {
        unsafe {
            self.context.bind_framebuffer(target, self.id);
        }
        self.target.bind(&self.context);
        self.context.framebuffer_check()?;
        self.context.error_check()
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
