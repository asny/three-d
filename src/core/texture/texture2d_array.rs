use crate::core::texture::*;

///
/// A array of 2D color textures that can be rendered into.
///
/// **Note:** [DepthTest] is disabled if not also writing to a [DepthTarget].
/// Use a [RenderTarget] to write to both color and depth.
///
pub struct Texture2DArray {
    pub(in crate::core) context: Context,
    id: crate::context::Texture,
    width: u32,
    height: u32,
    depth: u32,
    number_of_mip_maps: u32,
}

impl Texture2DArray {
    ///
    /// Creates a new array of 2D textures.
    ///
    pub fn new_empty<T: TextureDataType>(
        context: &Context,
        width: u32,
        height: u32,
        depth: u32,
        min_filter: Interpolation,
        mag_filter: Interpolation,
        mip_map_filter: Option<Interpolation>,
        wrap_s: Wrapping,
        wrap_t: Wrapping,
    ) -> ThreeDResult<Self> {
        let id = generate(context)?;
        let number_of_mip_maps = calculate_number_of_mip_maps(mip_map_filter, width, height, None);
        let texture = Self {
            context: context.clone(),
            id,
            width,
            height,
            depth,
            number_of_mip_maps,
        };
        texture.bind();
        set_parameters(
            context,
            crate::context::TEXTURE_2D_ARRAY,
            min_filter,
            mag_filter,
            if number_of_mip_maps == 1 {
                None
            } else {
                mip_map_filter
            },
            wrap_s,
            wrap_t,
            None,
        )?;
        unsafe {
            context.tex_storage_3d(
                crate::context::TEXTURE_2D_ARRAY,
                number_of_mip_maps as i32,
                T::internal_format(),
                width as i32,
                height as i32,
                depth as i32,
            );
        }
        texture.generate_mip_maps();
        context.error_check()?;
        Ok(texture)
    }

    pub fn as_color_target<'a>(
        &'a mut self,
        layers: &'a [u32],
        mip_level: Option<u32>,
    ) -> ColorTarget<'a> {
        ColorTarget::new_texture_2d_array(self, layers, mip_level)
    }

    ///
    /// Renders whatever rendered in the `render` closure into the textures defined by the input parameters `layers`.
    /// Output at location *i* defined in the fragment shader is written to the color texture layer at the *ith* index in `layers`.
    /// Before writing, the textures are cleared based on the given clear state.
    ///
    /// **Note:** [DepthTest] is disabled if not also writing to a [DepthTarget].
    /// Use a [RenderTarget] to write to both color and depth.
    ///
    #[deprecated = "use as_color_target followed by clear and write"]
    pub fn write<'a, F: FnOnce() -> ThreeDResult<()>>(
        &'a mut self,
        layers: &'a [u32],
        clear_state: ClearState,
        render: F,
    ) -> ThreeDResult<()> {
        self.as_color_target(layers, None)
            .as_render_target()?
            .clear_deprecated(clear_state)?
            .write(render)?;
        Ok(())
    }

    /// The width of this texture.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// The height of this texture.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// The number of layers.
    pub fn depth(&self) -> u32 {
        self.depth
    }

    pub(in crate::core) fn generate_mip_maps(&self) {
        if self.number_of_mip_maps > 1 {
            self.bind();
            unsafe {
                self.context
                    .generate_mipmap(crate::context::TEXTURE_2D_ARRAY);
            }
        }
    }

    pub(in crate::core) fn bind_as_color_target(&self, layer: u32, channel: u32, mip_level: u32) {
        unsafe {
            self.context.framebuffer_texture_layer(
                crate::context::DRAW_FRAMEBUFFER,
                crate::context::COLOR_ATTACHMENT0 + channel,
                Some(self.id),
                mip_level as i32,
                layer as i32,
            );
        }
    }

    pub(in crate::core) fn bind(&self) {
        unsafe {
            self.context
                .bind_texture(crate::context::TEXTURE_2D_ARRAY, Some(self.id));
        }
    }
}

impl Drop for Texture2DArray {
    fn drop(&mut self) {
        unsafe {
            self.context.delete_texture(self.id);
        }
    }
}
