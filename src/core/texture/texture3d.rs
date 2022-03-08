use crate::core::texture::*;
use crate::core::*;

///
/// A 3D color texture.
///
pub struct Texture3D<T: TextureDataType> {
    context: Context,
    id: crate::context::Texture,
    width: u32,
    height: u32,
    depth: u32,
    number_of_mip_maps: u32,
    format: Format,
    _dummy: T,
}

impl<T: TextureDataType> Texture3D<T> {
    ///
    /// Construcs a new 3D texture with the given data.
    ///
    pub fn new(context: &Context, cpu_texture: &CpuTexture3D<T>) -> ThreeDResult<Self> {
        let mut texture = Self::new_empty(
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
            cpu_texture.format,
        )?;
        texture.fill(&cpu_texture.data)?;
        Ok(texture)
    }

    ///
    /// Creates a new empty 3D color texture.
    ///
    pub fn new_empty(
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
        format: Format,
    ) -> ThreeDResult<Self> {
        let id = generate(context)?;
        let number_of_mip_maps =
            calculate_number_of_mip_maps(mip_map_filter, width, height, Some(depth));
        set_parameters(
            context,
            &id,
            consts::TEXTURE_3D,
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
        context.bind_texture(consts::TEXTURE_3D, &id);
        context.tex_storage_3d(
            consts::TEXTURE_3D,
            number_of_mip_maps,
            T::internal_format(format)?,
            width,
            height,
            depth,
        );
        Ok(Self {
            context: context.clone(),
            id,
            width,
            height,
            depth,
            number_of_mip_maps,
            format,
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
        check_data_length(self.width, self.height, self.depth, self.format, data.len())?;
        self.context.bind_texture(consts::TEXTURE_3D, &self.id);
        T::fill(
            &self.context,
            consts::TEXTURE_3D,
            self.width,
            self.height,
            Some(self.depth),
            self.format,
            data,
        );
        self.generate_mip_maps();
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

    /// The depth of this texture.
    pub fn depth(&self) -> u32 {
        self.depth
    }

    /// The format of this texture.
    pub fn format(&self) -> Format {
        self.format
    }

    fn generate_mip_maps(&self) {
        if self.number_of_mip_maps > 1 {
            self.context.bind_texture(consts::TEXTURE_3D, &self.id);
            self.context.generate_mipmap(consts::TEXTURE_3D);
        }
    }
}

impl<T: TextureDataType> super::internal::TextureExtensions for Texture3D<T> {
    fn bind(&self) {
        self.context.bind_texture(consts::TEXTURE_3D, &self.id);
    }
}

impl<T: TextureDataType> Texture for Texture3D<T> {}

impl<T: TextureDataType> Drop for Texture3D<T> {
    fn drop(&mut self) {
        self.context.delete_texture(&self.id);
    }
}
