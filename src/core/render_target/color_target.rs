use super::*;

///
/// Adds additional functionality to clear, read from and write to a texture.
/// Use the `as_color_target` function directly on the texture structs (for example [Texture2D]) to construct a color target.
/// Combine this together with a [DepthTarget] with [RenderTarget::new] to be able to write to both a depth and color target at the same time.
/// A color target purely adds functionality, so it can be created each time it is needed, the actual data is saved in the texture.
///
/// **Note:** [DepthTest] is disabled if not also writing to a [DepthTarget].
///
#[derive(Clone)]
pub struct ColorTarget<'a> {
    pub(crate) context: Context,
    mip_level: Option<u32>,
    target: Option<ColorTexture<'a>>,
    multisample_target: Option<&'a Texture2DMultisample>,
}

impl<'a> ColorTarget<'a> {
    pub(in crate::core) fn new_texture2d(
        context: &Context,
        texture: &'a Texture2D,
        mip_level: Option<u32>,
    ) -> Self {
        ColorTarget {
            context: context.clone(),
            mip_level,
            target: Some(ColorTexture::Single(texture)),
            multisample_target: None,
        }
    }

    pub(in crate::core) fn new_texture_cube_map(
        context: &Context,
        texture: &'a TextureCubeMap,
        sides: &'a [CubeMapSide],
        mip_level: Option<u32>,
    ) -> Self {
        ColorTarget {
            context: context.clone(),
            mip_level,
            target: Some(ColorTexture::CubeMap { texture, sides }),
            multisample_target: None,
        }
    }

    pub(in crate::core) fn new_texture_2d_array(
        context: &Context,
        texture: &'a Texture2DArray,
        layers: &'a [u32],
        mip_level: Option<u32>,
    ) -> Self {
        ColorTarget {
            context: context.clone(),
            mip_level,
            target: Some(ColorTexture::Array { texture, layers }),
            multisample_target: None,
        }
    }

    pub(in crate::core) fn new_texture_2d_multisample(
        context: &Context,
        texture: &'a Texture2DMultisample,
    ) -> Self {
        ColorTarget {
            context: context.clone(),
            mip_level: None,
            target: None,
            multisample_target: Some(texture),
        }
    }

    ///
    /// Clears the color of this color target as defined by the given clear state.
    ///
    pub fn clear(&self, clear_state: ClearState) -> &Self {
        self.clear_partially(self.scissor_box(), clear_state)
    }

    ///
    /// Clears the color of the part of this color target that is inside the given scissor box.
    ///
    pub fn clear_partially(&self, scissor_box: ScissorBox, clear_state: ClearState) -> &Self {
        self.as_render_target().clear_partially(
            scissor_box,
            ClearState {
                depth: None,
                ..clear_state
            },
        );
        self
    }

    ///
    /// Writes whatever rendered in the `render` closure into this color target.
    ///
    pub fn write<E: std::error::Error>(
        &self,
        render: impl FnOnce() -> Result<(), E>,
    ) -> Result<&Self, E> {
        self.write_partially(self.scissor_box(), render)
    }

    ///
    /// Writes whatever rendered in the `render` closure into the part of this color target defined by the scissor box.
    ///
    pub fn write_partially<E: std::error::Error>(
        &self,
        scissor_box: ScissorBox,
        render: impl FnOnce() -> Result<(), E>,
    ) -> Result<&Self, E> {
        self.as_render_target()
            .write_partially(scissor_box, render)?;
        Ok(self)
    }

    ///
    /// Returns the colors of the pixels in this color target.
    /// The number of channels per pixel and the data format for each channel returned from this function is specified by the generic parameter `T`.
    ///
    /// **Note:**
    /// The base type of the generic parameter `T` must match the base type of the color target, for example if the color targets base type is `u8`, the base type of `T` must also be `u8`.
    ///
    /// **Web:**
    /// The generic parameter `T` is limited to:
    /// - Unsigned byte RGBA (Specify `T` as either `Vec4<u8>` or `[u8; 4]`) which works with any color target using `u8` as its base type.
    /// - 32-bit float RGBA (Specify `T` as either `Vec4<f32>` or `[f32; 4]`) which works with any color target using `f16` or `f32` as its base type.
    ///
    pub fn read<T: TextureDataType>(&self) -> Vec<T> {
        self.read_partially(self.scissor_box())
    }

    ///
    /// Returns the colors of the pixels in this color target inside the given scissor box.
    /// The number of channels per pixel and the data format for each channel returned from this function is specified by the generic parameter `T`.
    ///
    /// **Note:**
    /// The base type of the generic parameter `T` must match the base type of the color target, for example if the color targets base type is `u8`, the base type of `T` must also be `u8`.
    ///
    /// **Web:**
    /// The generic parameter `T` is limited to:
    /// - Unsigned byte RGBA (Specify `T` as either `Vec4<u8>` or `[u8; 4]`) which works with any color target using `u8` as its base type.
    /// - 32-bit float RGBA (Specify `T` as either `Vec4<f32>` or `[f32; 4]`) which works with any color target using `f16` or `f32` as its base type.
    ///
    pub fn read_partially<T: TextureDataType>(&self, scissor_box: ScissorBox) -> Vec<T> {
        self.as_render_target().read_color_partially(scissor_box)
    }

    ///
    /// Returns the width of the color target in texels.
    /// If using the zero mip level of the underlying texture, then this is simply the width of that texture, otherwise it is the width of the given mip level.
    ///
    pub fn width(&self) -> u32 {
        if let Some(target) = self.target {
            match target {
                ColorTexture::Single(texture) => size_with_mip(texture.width(), self.mip_level),
                ColorTexture::Array { texture, .. } => {
                    size_with_mip(texture.width(), self.mip_level)
                }
                ColorTexture::CubeMap { texture, .. } => {
                    size_with_mip(texture.width(), self.mip_level)
                }
            }
        } else {
            self.multisample_target.as_ref().unwrap().width()
        }
    }

    ///
    /// Returns the height of the color target in texels.
    /// If using the zero mip level of the underlying texture, then this is simply the height of that texture, otherwise it is the height of the given mip level.
    ///
    pub fn height(&self) -> u32 {
        if let Some(target) = self.target {
            match target {
                ColorTexture::Single(texture) => size_with_mip(texture.height(), self.mip_level),
                ColorTexture::Array { texture, .. } => {
                    size_with_mip(texture.height(), self.mip_level)
                }
                ColorTexture::CubeMap { texture, .. } => {
                    size_with_mip(texture.height(), self.mip_level)
                }
            }
        } else {
            self.multisample_target.as_ref().unwrap().height()
        }
    }

    pub(super) fn as_render_target(&self) -> RenderTarget<'a> {
        RenderTarget::new_color(self.clone())
    }

    pub(super) fn generate_mip_maps(&self) {
        if let Some(target) = self.target {
            match target {
                ColorTexture::Single(texture) => {
                    if self.mip_level.is_none() {
                        texture.generate_mip_maps()
                    }
                }
                ColorTexture::Array { texture, .. } => {
                    if self.mip_level.is_none() {
                        texture.generate_mip_maps()
                    }
                }
                ColorTexture::CubeMap { texture, .. } => {
                    if self.mip_level.is_none() {
                        texture.generate_mip_maps()
                    }
                }
            }
        }
    }

    pub(super) fn bind(&self, context: &Context) {
        if let Some(target) = self.target {
            match target {
                ColorTexture::Single(texture) => unsafe {
                    context.draw_buffers(&[crate::context::COLOR_ATTACHMENT0]);
                    texture.bind_as_color_target(0, self.mip_level.unwrap_or(0));
                },
                ColorTexture::Array { texture, layers } => unsafe {
                    context.draw_buffers(
                        &(0..layers.len())
                            .map(|i| crate::context::COLOR_ATTACHMENT0 + i as u32)
                            .collect::<Vec<u32>>(),
                    );
                    (0..layers.len()).for_each(|channel| {
                        texture.bind_as_color_target(
                            layers[channel],
                            channel as u32,
                            self.mip_level.unwrap_or(0),
                        );
                    });
                },
                ColorTexture::CubeMap { texture, sides } => unsafe {
                    context.draw_buffers(
                        &(0..sides.len())
                            .map(|i| crate::context::COLOR_ATTACHMENT0 + i as u32)
                            .collect::<Vec<u32>>(),
                    );
                    (0..sides.len()).for_each(|channel| {
                        texture.bind_as_color_target(
                            sides[channel],
                            channel as u32,
                            self.mip_level.unwrap_or(0),
                        );
                    });
                },
            }
        } else {
            unsafe {
                context.draw_buffers(&[crate::context::COLOR_ATTACHMENT0]);
                self.multisample_target
                    .as_ref()
                    .unwrap()
                    .bind_as_color_target(0);
            }
        }
    }
}
