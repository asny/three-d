use crate::core::texture::*;

///
/// A 2D texture, basically an image that is transferred to the GPU.
///
pub struct Texture2D {
    context: Context,
    id: crate::context::Texture,
    width: u32,
    height: u32,
    number_of_mip_maps: u32,
}

impl Texture2D {
    ///
    /// Construcs a new texture with the given data.
    ///
    pub fn new(context: &Context, cpu_texture: &CpuTexture) -> ThreeDResult<Self> {
        match cpu_texture.data {
            TextureData::RU8(ref data) => Self::new_with_data(context, cpu_texture, data),
            TextureData::RgU8(ref data) => Self::new_with_data(context, cpu_texture, data),
            TextureData::RgbU8(ref data) => Self::new_with_data(context, cpu_texture, data),
            TextureData::RgbaU8(ref data) => Self::new_with_data(context, cpu_texture, data),
            TextureData::RF16(ref data) => Self::new_with_data(context, cpu_texture, data),
            TextureData::RgF16(ref data) => Self::new_with_data(context, cpu_texture, data),
            TextureData::RgbF16(ref data) => Self::new_with_data(context, cpu_texture, data),
            TextureData::RgbaF16(ref data) => Self::new_with_data(context, cpu_texture, data),
            TextureData::RF32(ref data) => Self::new_with_data(context, cpu_texture, data),
            TextureData::RgF32(ref data) => Self::new_with_data(context, cpu_texture, data),
            TextureData::RgbF32(ref data) => Self::new_with_data(context, cpu_texture, data),
            TextureData::RgbaF32(ref data) => Self::new_with_data(context, cpu_texture, data),
        }
    }

    fn new_with_data<T: TextureDataType>(
        context: &Context,
        cpu_texture: &CpuTexture,
        data: &[T],
    ) -> ThreeDResult<Self> {
        let mut texture = Self::new_empty::<T>(
            context,
            cpu_texture.width,
            cpu_texture.height,
            cpu_texture.min_filter,
            cpu_texture.mag_filter,
            cpu_texture.mip_map_filter,
            cpu_texture.wrap_s,
            cpu_texture.wrap_t,
        )?;
        texture.fill(data)?;
        Ok(texture)
    }

    ///
    /// Constructs a new empty 2D texture.
    ///
    pub fn new_empty<T: TextureDataType>(
        context: &Context,
        width: u32,
        height: u32,
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
            number_of_mip_maps,
        };
        texture.bind();
        set_parameters(
            context,
            crate::context::TEXTURE_2D,
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
            context.tex_storage_2d(
                crate::context::TEXTURE_2D,
                number_of_mip_maps as i32,
                T::internal_format(),
                width as i32,
                height as i32,
            );
        }
        texture.generate_mip_maps();
        context.error_check()?;
        Ok(texture)
    }

    ///
    /// Fills this texture with the given data.
    ///
    /// # Errors
    /// Return an error if the length of the data array is smaller or bigger than the necessary number of bytes to fill the entire texture.
    ///
    pub fn fill<T: TextureDataType>(&mut self, data: &[T]) -> ThreeDResult<()> {
        check_data_length(self.width, self.height, 1, data.len())?;
        self.bind();
        unsafe {
            self.context.tex_sub_image_2d(
                crate::context::TEXTURE_2D,
                0,
                0,
                0,
                self.width as i32,
                self.height as i32,
                format::<T>(),
                T::data_type(),
                crate::context::PixelUnpackData::Slice(crate::core::internal::to_byte_slice(data)),
            );
        }
        self.generate_mip_maps();
        self.context.error_check()
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
        RenderTarget::new_color(&self.context.clone(), self)?.write(clear_state, render)
    }

    ///
    /// Returns the color values of the pixels in this color texture inside the given viewport.
    ///
    /// **Note:** Currently only works for the RGBA format.
    ///
    pub fn read<T: TextureDataType + crate::core::internal::PrimitiveDataType>(
        &self,
        viewport: Viewport,
    ) -> ThreeDResult<Vec<Vector4<T>>> {
        let id = crate::core::render_target::new_framebuffer(&self.context)?;
        unsafe {
            self.context
                .bind_framebuffer(crate::context::DRAW_FRAMEBUFFER, Some(id));
            self.context
                .draw_buffers(&[crate::context::COLOR_ATTACHMENT0]);
            self.bind_as_color_target(0);

            self.context
                .bind_framebuffer(crate::context::READ_FRAMEBUFFER, Some(id));
            self.context
                .draw_buffers(&[crate::context::COLOR_ATTACHMENT0]);
            self.bind_as_color_target(0);

            self.context.framebuffer_check()?;

            let mut pixels = vec![
                vec4(T::default(), T::default(), T::default(), T::default());
                viewport.width as usize * viewport.height as usize
            ];
            self.context.read_pixels(
                viewport.x as i32,
                viewport.y as i32,
                viewport.width as i32,
                viewport.height as i32,
                format::<Vector4<T>>(),
                T::data_type(),
                crate::context::PixelPackData::Slice(crate::core::internal::to_mut_byte_slice(
                    &mut pixels,
                )),
            );
            Ok(pixels)
        }
    }

    /// The width of this texture.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// The height of this texture.
    pub fn height(&self) -> u32 {
        self.height
    }

    pub(crate) fn generate_mip_maps(&self) {
        if self.number_of_mip_maps > 1 {
            self.bind();
            unsafe {
                self.context.generate_mipmap(crate::context::TEXTURE_2D);
            }
        }
    }

    pub(in crate::core) fn bind_as_color_target(&self, channel: u32) {
        unsafe {
            self.context.framebuffer_texture_2d(
                crate::context::FRAMEBUFFER,
                crate::context::COLOR_ATTACHMENT0 + channel,
                crate::context::TEXTURE_2D,
                Some(self.id),
                0,
            );
        }
    }
    fn bind(&self) {
        unsafe {
            self.context
                .bind_texture(crate::context::TEXTURE_2D, Some(self.id));
        }
    }
}

impl internal::TextureExtensions for Texture2D {
    fn bind(&self) {
        self.bind();
    }
}

impl Texture for Texture2D {}

impl Drop for Texture2D {
    fn drop(&mut self) {
        unsafe {
            self.context.delete_texture(self.id);
        }
    }
}

///
/// A 2D color texture that can be rendered into and read from.
///
/// **Note:** [DepthTest] is disabled if not also writing to a depth texture.
/// Use a [RenderTarget] to write to both color and depth.
///
#[deprecated = "Use Texture2D instead"]
pub struct ColorTargetTexture2D {
    tex: Texture2D,
}

#[allow(deprecated)]
impl ColorTargetTexture2D {
    ///
    /// Constructs a new 2D color target texture.
    ///
    pub fn new<T: TextureDataType>(
        context: &Context,
        width: u32,
        height: u32,
        min_filter: Interpolation,
        mag_filter: Interpolation,
        mip_map_filter: Option<Interpolation>,
        wrap_s: Wrapping,
        wrap_t: Wrapping,
        _format: Format,
    ) -> ThreeDResult<Self> {
        Ok(Self {
            tex: Texture2D::new_empty::<T>(
                context,
                width,
                height,
                min_filter,
                mag_filter,
                mip_map_filter,
                wrap_s,
                wrap_t,
            )?,
        })
    }
}

#[allow(deprecated)]
impl std::ops::Deref for ColorTargetTexture2D {
    type Target = Texture2D;
    fn deref(&self) -> &Self::Target {
        &self.tex
    }
}

#[allow(deprecated)]
impl std::ops::DerefMut for ColorTargetTexture2D {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tex
    }
}
