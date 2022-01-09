use crate::core::texture::*;

///
/// A 2D texture, basically an image that is transferred to the GPU.
///
pub struct Texture2D<T: TextureDataType> {
    context: Context,
    id: crate::context::Texture,
    width: u32,
    height: u32,
    format: Format,
    number_of_mip_maps: u32,
    transparent: bool,
    _dummy: T,
}

impl<T: TextureDataType> Texture2D<T> {
    ///
    /// Construcs a new texture with the given data.
    ///
    pub fn new(context: &Context, cpu_texture: &CPUTexture<T>) -> ThreeDResult<Texture2D<T>> {
        let id = generate(context)?;
        let number_of_mip_maps = calculate_number_of_mip_maps(
            cpu_texture.mip_map_filter,
            cpu_texture.width,
            cpu_texture.height,
        );
        set_parameters(
            context,
            &id,
            consts::TEXTURE_2D,
            cpu_texture.min_filter,
            cpu_texture.mag_filter,
            if number_of_mip_maps == 1 {
                None
            } else {
                cpu_texture.mip_map_filter
            },
            cpu_texture.wrap_s,
            cpu_texture.wrap_t,
            None,
        );
        context.tex_storage_2d(
            consts::TEXTURE_2D,
            number_of_mip_maps,
            T::internal_format(cpu_texture.format)?,
            cpu_texture.width as u32,
            cpu_texture.height as u32,
        );
        let mut tex = Self {
            context: context.clone(),
            id,
            width: cpu_texture.width,
            height: cpu_texture.height,
            format: cpu_texture.format,
            number_of_mip_maps,
            transparent: false,
            _dummy: T::default(),
        };
        tex.fill(&cpu_texture.data)?;
        Ok(tex)
    }

    ///
    /// Constructs a new empty 2D texture.
    ///
    pub fn new_empty(
        context: &Context,
        width: u32,
        height: u32,
        min_filter: Interpolation,
        mag_filter: Interpolation,
        mip_map_filter: Option<Interpolation>,
        wrap_s: Wrapping,
        wrap_t: Wrapping,
        format: Format,
    ) -> ThreeDResult<Self> {
        let id = generate(context)?;
        let number_of_mip_maps = calculate_number_of_mip_maps(mip_map_filter, width, height);
        set_parameters(
            context,
            &id,
            consts::TEXTURE_2D,
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
        );
        context.tex_storage_2d(
            consts::TEXTURE_2D,
            number_of_mip_maps,
            T::internal_format(format)?,
            width,
            height,
        );
        Ok(Self {
            context: context.clone(),
            id,
            width,
            height,
            number_of_mip_maps,
            format,
            transparent: format == Format::RGBA,
            _dummy: T::default(),
        })
    }

    ///
    /// Fills this texture with the given data.
    ///
    /// # Errors
    /// Return an error if the length of the data array is smaller or bigger than the necessary number of bytes to fill the entire texture.
    ///
    pub fn fill(&mut self, data: &[T]) -> ThreeDResult<()> {
        check_data_length(self.width, self.height, 1, self.format, data.len())?;
        self.transparent = if self.format == Format::RGBA {
            let mut transparent = false;
            for i in 0..self.width as usize * self.height as usize {
                if !T::is_max(data[i * 4 + 3]) {
                    transparent = true;
                    break;
                }
            }
            transparent
        } else {
            false
        };
        self.context.bind_texture(consts::TEXTURE_2D, &self.id);
        T::fill(
            &self.context,
            consts::TEXTURE_2D,
            self.width(),
            self.height(),
            self.format,
            data,
        );
        self.generate_mip_maps();
        Ok(())
    }

    ///
    /// Renders whatever rendered in the `render` closure into the texture.
    /// Before writing, the texture is cleared based on the given clear state.
    ///
    /// **Note:** [DepthTest] is disabled if not also writing to a depth texture.
    /// Use a [RenderTarget] to write to both color and depth.
    ///
    pub fn write<F: FnOnce() -> ThreeDResult<()>>(
        &mut self,
        clear_state: ClearState,
        render: F,
    ) -> ThreeDResult<()> {
        RenderTarget::<T>::new_color(&self.context.clone(), self)?.write(clear_state, render)
    }

    ///
    /// Copies the content of the color texture to the specified destination at the given viewport.
    /// Will only copy the channels specified by the write mask.
    ///
    /// # Errors
    /// Will return an error if the destination is a depth texture.
    ///
    pub fn copy_to(
        &mut self,
        destination: CopyDestination<T>,
        viewport: Viewport,
        write_mask: WriteMask,
    ) -> ThreeDResult<()> {
        RenderTarget::new_color(&self.context.clone(), self)?.copy_to(
            destination,
            viewport,
            write_mask,
        )
    }

    ///
    /// Returns the color values of the pixels in this color texture inside the given viewport.
    ///
    /// **Note:** Only works for the RGBA format.
    ///
    /// # Errors
    /// Will return an error if the color texture is not RGBA format.
    ///
    pub fn read(&self, viewport: Viewport) -> ThreeDResult<Vec<T>> {
        if self.format != Format::RGBA {
            Err(CoreError::ReadWrongFormat)?;
        }
        let id = crate::core::render_target::new_framebuffer(&self.context)?;

        self.context
            .bind_framebuffer(consts::DRAW_FRAMEBUFFER, Some(&id));
        self.context.draw_buffers(&[consts::COLOR_ATTACHMENT0]);
        self.bind_as_color_target(0);

        self.context
            .bind_framebuffer(consts::READ_FRAMEBUFFER, Some(&id));
        self.context.draw_buffers(&[consts::COLOR_ATTACHMENT0]);
        self.bind_as_color_target(0);

        #[cfg(feature = "debug")]
        check(&self.context)?;

        let mut pixels = vec![
            T::default();
            viewport.width as usize
                * viewport.height as usize
                * self.format.color_channel_count() as usize
        ];
        T::read(&self.context, viewport, self.format, &mut pixels);
        Ok(pixels)
    }

    pub(crate) fn generate_mip_maps(&self) {
        if self.number_of_mip_maps > 1 {
            self.context.bind_texture(consts::TEXTURE_2D, &self.id);
            self.context.generate_mipmap(consts::TEXTURE_2D);
        }
    }

    pub(in crate::core) fn bind_as_color_target(&self, channel: u32) {
        self.context.framebuffer_texture_2d(
            consts::FRAMEBUFFER,
            consts::COLOR_ATTACHMENT0 + channel,
            consts::TEXTURE_2D,
            &self.id,
            0,
        );
    }
}

impl<T: TextureDataType> Texture for Texture2D<T> {
    fn bind(&self, location: u32) {
        bind_at(&self.context, &self.id, consts::TEXTURE_2D, location);
    }
    fn width(&self) -> u32 {
        self.width
    }
    fn height(&self) -> u32 {
        self.height
    }
    fn format(&self) -> Format {
        self.format
    }
    fn is_transparent(&self) -> bool {
        self.transparent
    }
}

impl<T: TextureDataType> Drop for Texture2D<T> {
    fn drop(&mut self) {
        self.context.delete_texture(&self.id);
    }
}

///
/// A 2D color texture that can be rendered into and read from.
///
/// **Note:** [DepthTest] is disabled if not also writing to a depth texture.
/// Use a [RenderTarget] to write to both color and depth.
///
#[deprecated = "Use Texture2D instead"]
pub struct ColorTargetTexture2D<T: TextureDataType> {
    tex: Texture2D<T>,
}

#[allow(deprecated)]
impl<T: TextureDataType> ColorTargetTexture2D<T> {
    ///
    /// Constructs a new 2D color target texture.
    ///
    pub fn new(
        context: &Context,
        width: u32,
        height: u32,
        min_filter: Interpolation,
        mag_filter: Interpolation,
        mip_map_filter: Option<Interpolation>,
        wrap_s: Wrapping,
        wrap_t: Wrapping,
        format: Format,
    ) -> ThreeDResult<Self> {
        Ok(Self {
            tex: Texture2D::new_empty(
                context,
                width,
                height,
                min_filter,
                mag_filter,
                mip_map_filter,
                wrap_s,
                wrap_t,
                format,
            )?,
        })
    }
}

#[allow(deprecated)]
impl<T: TextureDataType> std::ops::Deref for ColorTargetTexture2D<T> {
    type Target = Texture2D<T>;
    fn deref(&self) -> &Self::Target {
        &self.tex
    }
}

#[allow(deprecated)]
impl<T: TextureDataType> std::ops::DerefMut for ColorTargetTexture2D<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tex
    }
}
