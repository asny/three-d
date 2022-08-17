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
        let mut texture = Self::new_empty::<T>(
            context,
            cpu_texture.width,
            cpu_texture.height,
            cpu_texture.min_filter,
            cpu_texture.mag_filter,
            cpu_texture.mip_map_filter,
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
    pub fn new_empty<T: TextureDataType>(
        context: &Context,
        width: u32,
        height: u32,
        min_filter: Interpolation,
        mag_filter: Interpolation,
        mip_map_filter: Option<Interpolation>,
        wrap_s: Wrapping,
        wrap_t: Wrapping,
    ) -> Self {
        let id = generate(context);
        let number_of_mip_maps = calculate_number_of_mip_maps(mip_map_filter, width, height, None);
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
                mip_map_filter
            },
            wrap_s,
            wrap_t,
            None,
        );
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
        texture
    }

    ///
    /// Fills this texture with the given data.
    ///
    /// # Panic
    /// Will panic if the length of the data does not correspond to the width, height and format specified at construction.
    /// It is therefore necessary to create a new texture if the texture size or format has changed.
    ///
    pub fn fill<T: TextureDataType>(&mut self, data: &[T]) {
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
                crate::context::PixelUnpackData::Slice(to_byte_slice(&data)),
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
    pub fn as_color_target<'a>(&'a mut self, mip_level: Option<u32>) -> ColorTarget<'a> {
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
}

impl Drop for Texture2D {
    fn drop(&mut self) {
        unsafe {
            self.context.delete_texture(self.id);
        }
    }
}
