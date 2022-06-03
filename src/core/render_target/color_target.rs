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
    target: CT<'a>,
}

#[derive(Clone)]
enum CT<'a> {
    Texture2D {
        texture: &'a Texture2D,
        mip_level: Option<u32>,
    },
    Texture2DArray {
        texture: &'a Texture2DArray,
        layers: &'a [u32],
        mip_level: Option<u32>,
    },
    TextureCubeMap {
        texture: &'a TextureCubeMap,
        side: CubeMapSide,
        mip_level: Option<u32>,
    },
}

impl<'a> ColorTarget<'a> {
    pub(in crate::core) fn new_texture2d(
        context: &Context,
        texture: &'a Texture2D,
        mip_level: Option<u32>,
    ) -> Self {
        ColorTarget {
            context: context.clone(),
            target: CT::Texture2D { texture, mip_level },
        }
    }

    pub(in crate::core) fn new_texture_cube_map(
        context: &Context,
        texture: &'a TextureCubeMap,
        side: CubeMapSide,
        mip_level: Option<u32>,
    ) -> Self {
        ColorTarget {
            context: context.clone(),
            target: CT::TextureCubeMap {
                texture,
                side,
                mip_level,
            },
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
            target: CT::Texture2DArray {
                texture,
                layers,
                mip_level,
            },
        }
    }

    ///
    /// Clears the color of this color target as defined by the given clear state.
    ///
    pub fn clear(&self, clear_state: ClearState) -> ThreeDResult<&Self> {
        self.clear_partially(self.scissor_box(), clear_state)
    }

    ///
    /// Clears the color of the part of this color target that is inside the given scissor box.
    ///
    pub fn clear_partially(
        &self,
        scissor_box: ScissorBox,
        clear_state: ClearState,
    ) -> ThreeDResult<&Self> {
        self.as_render_target()?.clear_partially(
            scissor_box,
            ClearState {
                depth: None,
                ..clear_state
            },
        )?;
        Ok(self)
    }

    ///
    /// Writes whatever rendered in the `render` closure into this color target.
    ///
    pub fn write(&self, render: impl FnOnce() -> ThreeDResult<()>) -> ThreeDResult<&Self> {
        self.write_partially(self.scissor_box(), render)
    }

    ///
    /// Writes whatever rendered in the `render` closure into the part of this color target defined by the scissor box.
    ///
    pub fn write_partially(
        &self,
        scissor_box: ScissorBox,
        render: impl FnOnce() -> ThreeDResult<()>,
    ) -> ThreeDResult<&Self> {
        self.as_render_target()?
            .write_partially(scissor_box, render)?;
        Ok(self)
    }

    ///
    /// Returns the colors of the pixels in this color target.
    /// The number of channels per pixel and the data format for each channel is specified by the generic parameter.
    ///
    /// **Note:** On web, the data format needs to match the data format of the color texture.
    ///
    pub fn read<T: TextureDataType>(&self) -> ThreeDResult<Vec<T>> {
        self.read_partially(self.scissor_box())
    }

    ///
    /// Returns the colors of the pixels in this color target inside the given scissor box.
    /// The number of channels per pixel and the data format for each channel is specified by the generic parameter.
    ///
    /// **Note:** On web, the data format needs to match the data format of the color texture.
    ///
    pub fn read_partially<T: TextureDataType>(
        &self,
        scissor_box: ScissorBox,
    ) -> ThreeDResult<Vec<T>> {
        self.as_render_target()?.read_color_partially(scissor_box)
    }

    ///
    /// Returns the width of the color target in texels.
    /// If using the zero mip level of the underlying texture, then this is simply the width of that texture, otherwise it is the width of the given mip level.
    ///
    pub fn width(&self) -> u32 {
        match self.target {
            CT::Texture2D { texture, mip_level } => size_with_mip(texture.width(), mip_level),
            CT::Texture2DArray {
                texture, mip_level, ..
            } => size_with_mip(texture.width(), mip_level),
            CT::TextureCubeMap {
                texture, mip_level, ..
            } => size_with_mip(texture.width(), mip_level),
        }
    }

    ///
    /// Returns the height of the color target in texels.
    /// If using the zero mip level of the underlying texture, then this is simply the height of that texture, otherwise it is the height of the given mip level.
    ///
    pub fn height(&self) -> u32 {
        match self.target {
            CT::Texture2D { texture, mip_level } => size_with_mip(texture.height(), mip_level),
            CT::Texture2DArray {
                texture, mip_level, ..
            } => size_with_mip(texture.height(), mip_level),
            CT::TextureCubeMap {
                texture, mip_level, ..
            } => size_with_mip(texture.height(), mip_level),
        }
    }

    ///
    /// Returns the scissor box that encloses the entire target.
    ///
    pub fn scissor_box(&self) -> ScissorBox {
        ScissorBox::new_at_origo(self.width(), self.height())
    }

    pub(super) fn as_render_target(&self) -> ThreeDResult<RenderTarget<'a>> {
        RenderTarget::new_color(self.clone())
    }

    pub(super) fn generate_mip_maps(&self) {
        match self.target {
            CT::Texture2D { texture, mip_level } => {
                if mip_level.is_none() {
                    texture.generate_mip_maps()
                }
            }
            CT::Texture2DArray {
                texture, mip_level, ..
            } => {
                if mip_level.is_none() {
                    texture.generate_mip_maps()
                }
            }
            CT::TextureCubeMap {
                texture, mip_level, ..
            } => {
                if mip_level.is_none() {
                    texture.generate_mip_maps()
                }
            }
        }
    }

    pub(super) fn bind(&self, context: &Context) {
        match self.target {
            CT::Texture2D { texture, mip_level } => unsafe {
                context.draw_buffers(&[crate::context::COLOR_ATTACHMENT0]);
                texture.bind_as_color_target(0, mip_level.unwrap_or(0));
            },
            CT::Texture2DArray {
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
            CT::TextureCubeMap {
                texture,
                side,
                mip_level,
            } => unsafe {
                context.draw_buffers(&[crate::context::COLOR_ATTACHMENT0]);
                texture.bind_as_color_target(side, 0, mip_level.unwrap_or(0));
            },
        }
    }
}
