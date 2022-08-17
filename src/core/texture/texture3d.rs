use crate::core::texture::*;
///
/// A 3D color texture.
///
pub struct Texture3D {
    context: Context,
    id: crate::context::Texture,
    width: u32,
    height: u32,
    depth: u32,
    number_of_mip_maps: u32,
    data_byte_size: usize,
}

impl Texture3D {
    ///
    /// Construcs a new 3D texture with the given data.
    ///
    pub fn new(context: &Context, cpu_texture: &CpuTexture3D) -> Self {
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
        cpu_texture: &CpuTexture3D,
        data: &[T],
    ) -> Self {
        let mut texture = Self::new_empty::<T>(
            context,
            cpu_texture.width,
            cpu_texture.height,
            cpu_texture.depth,
            cpu_texture.min_filter,
            cpu_texture.mag_filter,
            cpu_texture.mip_map_filter,
            cpu_texture.wrap_s,
            cpu_texture.wrap_t,
            cpu_texture.wrap_r,
        );
        texture.fill(data);
        texture
    }

    ///
    /// Creates a new empty 3D color texture.
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
        wrap_r: Wrapping,
    ) -> Self {
        let id = generate(context);
        let number_of_mip_maps =
            calculate_number_of_mip_maps(mip_map_filter, width, height, Some(depth));
        let texture = Self {
            context: context.clone(),
            id,
            width,
            height,
            depth,
            number_of_mip_maps,
            data_byte_size: std::mem::size_of::<T>(),
        };
        texture.bind();
        set_parameters(
            context,
            crate::context::TEXTURE_3D,
            min_filter,
            mag_filter,
            if number_of_mip_maps == 1 {
                None
            } else {
                mip_map_filter
            },
            wrap_s,
            wrap_t,
            Some(wrap_r),
        );
        unsafe {
            context.tex_storage_3d(
                crate::context::TEXTURE_3D,
                number_of_mip_maps as i32,
                T::internal_format(),
                width as i32,
                height as i32,
                depth as i32,
            );
        }
        texture.generate_mip_maps();
        texture
    }

    ///
    /// Fills this texture with the given data.
    ///
    /// # Panic
    /// Will panic if the length of the data does not correspond to the width, height, depth and format specified at construction.
    /// It is therefore necessary to create a new texture if the texture size or format has changed.
    ///
    pub fn fill<T: TextureDataType>(&mut self, data: &[T]) {
        check_data_length::<T>(
            self.width,
            self.height,
            self.depth,
            self.data_byte_size,
            data.len(),
        );
        self.bind();
        unsafe {
            self.context.tex_sub_image_3d(
                crate::context::TEXTURE_3D,
                0,
                0,
                0,
                0,
                self.width as i32,
                self.height as i32,
                self.depth as i32,
                format_from_data_type::<T>(),
                T::data_type(),
                crate::context::PixelUnpackData::Slice(to_byte_slice(data)),
            );
        }
        self.generate_mip_maps();
    }

    /// The width of this texture.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// The height of this texture.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// The depth of this texture.
    pub fn depth(&self) -> u32 {
        self.depth
    }

    fn generate_mip_maps(&self) {
        if self.number_of_mip_maps > 1 {
            self.bind();
            unsafe {
                self.context.generate_mipmap(crate::context::TEXTURE_3D);
            }
        }
    }
    pub(in crate::core) fn bind(&self) {
        unsafe {
            self.context
                .bind_texture(crate::context::TEXTURE_3D, Some(self.id));
        }
    }
}

impl Drop for Texture3D {
    fn drop(&mut self) {
        unsafe {
            self.context.delete_texture(self.id);
        }
    }
}
