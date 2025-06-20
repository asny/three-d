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
    data_byte_size: usize,
}

impl Texture2D {
    ///
    /// Construcs a new texture with the given data.
    ///
    /// **Note:** Mip maps will not be generated for RGB16F and RGB32F format, even if `mip_map_filter` is specified.
    ///
    pub fn new(context: &Context, cpu_texture: &CpuTexture) -> Self {
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
    ) -> Self {
        let texture = Self::new_empty::<T>(
            context,
            cpu_texture.width,
            cpu_texture.height,
            cpu_texture.min_filter,
            cpu_texture.mag_filter,
            cpu_texture.mipmap,
            cpu_texture.wrap_s,
            cpu_texture.wrap_t,
        );
        texture.fill(data);
        texture
    }

    ///
    /// Constructs a new empty 2D texture with the given parameters.
    /// The format is determined by the generic [TextureDataType] parameter
    /// (for example, if [u8; 4] is specified, the format is RGBA and the data type is byte).
    ///
    /// **Note:** Mip maps will not be generated for RGB16F and RGB32F format, even if `mip_map_filter` is specified.
    ///
    pub fn new_empty<T: TextureDataType>(
        context: &Context,
        width: u32,
        height: u32,
        min_filter: Interpolation,
        mag_filter: Interpolation,
        mipmap: Option<Mipmap>,
        wrap_s: Wrapping,
        wrap_t: Wrapping,
    ) -> Self {
        unsafe {
            Self::new_unchecked::<T>(
                context,
                width,
                height,
                min_filter,
                mag_filter,
                mipmap,
                wrap_s,
                wrap_t,
                |texture| {
                    context.tex_storage_2d(
                        crate::context::TEXTURE_2D,
                        texture.number_of_mip_maps() as i32,
                        T::internal_format(),
                        width as i32,
                        height as i32,
                    );
                },
            )
        }
    }

    ///
    /// Fills this texture with the given data and generate mip maps if specified at construction.
    ///
    /// # Panic
    /// Will panic if the length of the data does not correspond to the width, height and format specified at construction.
    /// It is therefore necessary to create a new texture if the texture size or format has changed.
    ///
    pub fn fill<T: TextureDataType>(&self, data: &[T]) {
        check_data_length::<T>(self.width, self.height, 1, self.data_byte_size, data.len());
        self.bind();
        let mut data = data.to_owned();
        flip_y(&mut data, self.width as usize, self.height as usize);
        unsafe {
            self.context.tex_sub_image_2d(
                crate::context::TEXTURE_2D,
                0,
                0,
                0,
                self.width as i32,
                self.height as i32,
                format_from_data_type::<T>(),
                T::data_type(),
                crate::context::PixelUnpackData::Slice(Some(to_byte_slice(&data))),
            );
        }
        self.generate_mip_maps();
    }

    ///
    /// Returns a [ColorTarget] which can be used to clear, write to and read from the given mip level of this texture.
    /// Combine this together with a [DepthTarget] with [RenderTarget::new] to be able to write to both a depth and color target at the same time.
    /// If `None` is specified as the mip level, the 0 level mip level is used and mip maps are generated after a write operation if a mip map filter is specified.
    /// Otherwise, the given mip level is used and no mip maps are generated.
    ///
    /// **Note:** [DepthTest] is disabled if not also writing to a depth texture.
    ///
    pub fn as_color_target(&self, mip_level: Option<u32>) -> ColorTarget<'_> {
        ColorTarget::new_texture2d(&self.context, self, mip_level)
    }

    /// The width of this texture.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// The height of this texture.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// The number of mip maps of this texture.
    pub fn number_of_mip_maps(&self) -> u32 {
        self.number_of_mip_maps
    }

    pub(crate) fn generate_mip_maps(&self) {
        if self.number_of_mip_maps > 1 {
            self.bind();
            unsafe {
                self.context.generate_mipmap(crate::context::TEXTURE_2D);
            }
        }
    }

    pub(in crate::core) fn bind_as_color_target(&self, channel: u32, mip_level: u32) {
        unsafe {
            self.context.framebuffer_texture_2d(
                crate::context::FRAMEBUFFER,
                crate::context::COLOR_ATTACHMENT0 + channel,
                crate::context::TEXTURE_2D,
                Some(self.id),
                mip_level as i32,
            );
        }
    }
    pub(in crate::core) fn bind(&self) {
        unsafe {
            self.context
                .bind_texture(crate::context::TEXTURE_2D, Some(self.id));
        }
    }

    ///
    /// Creates a new texture where it is up to the caller to allocate and transfer data to the GPU
    /// using low-level context calls inside the callback.
    /// This function binds the texture and sets the parameters before calling the callback and generates mip maps afterwards.
    ///
    /// # Safety
    ///
    /// This function is unsafe and should only be used in special cases,
    /// for example when you have an uncommon source of data or the data is in a special format like sRGB.
    ///
    pub unsafe fn new_unchecked<T: TextureDataType>(
        context: &Context,
        width: u32,
        height: u32,
        min_filter: Interpolation,
        mag_filter: Interpolation,
        mipmap: Option<Mipmap>,
        wrap_s: Wrapping,
        wrap_t: Wrapping,
        callback: impl FnOnce(&Self),
    ) -> Self {
        let id = generate(context);
        let number_of_mip_maps = calculate_number_of_mip_maps::<T>(mipmap, width, height, None);
        let texture = Self {
            context: context.clone(),
            id,
            width,
            height,
            number_of_mip_maps,
            data_byte_size: std::mem::size_of::<T>(),
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
                mipmap
            },
            wrap_s,
            wrap_t,
            None,
        );
        callback(&texture);
        texture.generate_mip_maps();
        texture
    }
}

impl Drop for Texture2D {
    fn drop(&mut self) {
        unsafe {
            self.context.delete_texture(self.id);
        }
    }
}
