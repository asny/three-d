use crate::core::render_target::*;

///
/// Adds additional functionality to write to and copy from both a [Texture2DArray]and
/// a [DepthTargetTexture2DArray] at the same time.
/// It purely adds functionality, so it can be created each time it is needed, the data is saved in the textures.
///
pub struct RenderTargetArray<'a, 'b, T: TextureDataType> {
    context: Context,
    id: crate::context::Framebuffer,
    color_texture: Option<&'a Texture2DArray<T>>,
    depth_texture: Option<&'b DepthTargetTexture2DArray>,
}
impl<'a, 'b> RenderTargetArray<'a, 'b, u8> {
    ///
    /// Constructs a new render target that enables rendering into the given
    /// [DepthTargetTexture2DArray].
    ///
    pub fn new_depth(
        context: &Context,
        depth_texture: &'b DepthTargetTexture2DArray,
    ) -> ThreeDResult<Self> {
        Ok(Self {
            context: context.clone(),
            id: new_framebuffer(context)?,
            color_texture: None,
            depth_texture: Some(depth_texture),
        })
    }
}

impl<'a, 'b, T: TextureDataType> RenderTargetArray<'a, 'b, T> {
    ///
    /// Constructs a new render target array that enables rendering into the given
    /// [Texture2DArray] and [DepthTargetTexture2DArray] array textures.
    ///
    pub fn new(
        context: &Context,
        color_texture: &'a Texture2DArray<T>,
        depth_texture: &'b DepthTargetTexture2DArray,
    ) -> ThreeDResult<Self> {
        Ok(Self {
            context: context.clone(),
            id: new_framebuffer(context)?,
            color_texture: Some(color_texture),
            depth_texture: Some(depth_texture),
        })
    }

    ///
    /// Constructs a new render target array that enables rendering into the given
    /// [Texture2DArray].
    ///
    pub fn new_color(
        context: &Context,
        color_texture: &'a Texture2DArray<T>,
    ) -> ThreeDResult<Self> {
        Ok(Self {
            context: context.clone(),
            id: new_framebuffer(context)?,
            color_texture: Some(color_texture),
            depth_texture: None,
        })
    }

    pub(crate) fn new_depth_internal(
        context: &Context,
        depth_texture: &'b DepthTargetTexture2DArray,
    ) -> ThreeDResult<Self> {
        Ok(Self {
            context: context.clone(),
            id: new_framebuffer(context)?,
            color_texture: None,
            depth_texture: Some(depth_texture),
        })
    }

    ///
    /// Renders whatever rendered in the `render` closure into the textures defined at construction
    /// and defined by the input parameters `color_layers` and `depth_layer`.
    /// Output at location *i* defined in the fragment shader is written to the color texture layer at the *ith* index in `color_layers`.
    /// The depth is written to the depth texture defined by `depth_layer`.
    /// Before writing, the textures are cleared based on the given clear state.
    ///
    pub fn write(
        &self,
        color_layers: &[u32],
        depth_layer: u32,
        clear_state: ClearState,
        render: impl FnOnce() -> ThreeDResult<()>,
    ) -> ThreeDResult<()> {
        self.bind(Some(color_layers), Some(depth_layer))?;
        clear(
            &self.context,
            &ClearState {
                red: self.color_texture.and(clear_state.red),
                green: self.color_texture.and(clear_state.green),
                blue: self.color_texture.and(clear_state.blue),
                alpha: self.color_texture.and(clear_state.alpha),
                depth: self.depth_texture.and(clear_state.depth),
            },
        );
        render()?;
        if let Some(color_texture) = self.color_texture {
            color_texture.generate_mip_maps();
        }
        Ok(())
    }

    ///
    /// Copies the content of the specified color and depth layers in this render target to the given viewport of the given destination.
    /// Only copies the channels specified by the write mask.
    ///
    #[deprecated = "Use RenderTarget::copy_from_array or Screen::copy_from_array instead"]
    pub fn copy_to(
        &self,
        color_layer: u32,
        depth_layer: u32,
        destination: CopyDestination<T>,
        viewport: Viewport,
        write_mask: WriteMask,
    ) -> ThreeDResult<()> {
        let copy = || {
            let fragment_shader_source = "
            uniform sampler2DArray colorMap;
            uniform sampler2DArray depthMap;
            uniform int colorLayer;
            uniform int depthLayer;
            in vec2 uv;
            layout (location = 0) out vec4 color;
            void main()
            {
                color = texture(colorMap, vec3(uv, colorLayer));
                gl_FragDepth = texture(depthMap, vec3(uv, depthLayer)).r;
            }";
            self.context.effect(fragment_shader_source, |effect| {
                if let Some(tex) = self.color_texture {
                    effect.use_texture_array("colorMap", tex)?;
                    effect.use_uniform("colorLayer", color_layer as i32)?;
                }
                if let Some(tex) = self.depth_texture {
                    effect.use_texture_array("depthMap", tex)?;
                    effect.use_uniform("depthLayer", depth_layer as i32)?;
                }
                effect.apply(
                    RenderStates {
                        depth_test: DepthTest::Always,
                        write_mask,
                        ..Default::default()
                    },
                    viewport,
                )
            })
        };
        match destination {
            CopyDestination::RenderTarget(other) => {
                other.write(ClearState::none(), copy)?;
            }
            CopyDestination::Screen => {
                Screen::write(&self.context, ClearState::none(), copy)?;
            }
            CopyDestination::ColorTexture(tex) => {
                if self.color_texture.is_none() {
                    Err(CoreError::RenderTargetCopy(
                        "color".to_string(),
                        "depth".to_string(),
                    ))?;
                }
                tex.write(ClearState::none(), copy)?;
            }
            CopyDestination::DepthTexture(tex) => {
                if self.depth_texture.is_none() {
                    Err(CoreError::RenderTargetCopy(
                        "depth".to_string(),
                        "color".to_string(),
                    ))?;
                }
                tex.write(None, copy)?;
            }
        }
        Ok(())
    }

    fn bind(&self, color_layers: Option<&[u32]>, depth_layer: Option<u32>) -> ThreeDResult<()> {
        self.context
            .bind_framebuffer(consts::DRAW_FRAMEBUFFER, Some(&self.id));
        if let Some(color_texture) = self.color_texture {
            if let Some(color_layers) = color_layers {
                self.context.draw_buffers(
                    &(0..color_layers.len())
                        .map(|i| consts::COLOR_ATTACHMENT0 + i as u32)
                        .collect::<Vec<u32>>(),
                );
                for channel in 0..color_layers.len() {
                    color_texture.bind_as_color_target(color_layers[channel], channel as u32);
                }
            }
        }
        if let Some(depth_texture) = self.depth_texture {
            if let Some(depth_layer) = depth_layer {
                depth_texture.bind_as_depth_target(depth_layer);
            }
        }
        #[cfg(feature = "debug")]
        check(&self.context)?;
        Ok(())
    }
}

impl<T: TextureDataType> Drop for RenderTargetArray<'_, '_, T> {
    fn drop(&mut self) {
        self.context.delete_framebuffer(Some(&self.id));
    }
}
