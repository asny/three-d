use crate::context::consts;
use crate::core::texture::*;

///
/// A texture that covers all 6 sides of a cube.
///
pub struct TextureCubeMap {
    context: Context,
    id: crate::context::Texture,
    width: u32,
    height: u32,
    format: Format,
    number_of_mip_maps: u32,
    is_hdr: bool,
}

impl TextureCubeMap {
    ///
    /// Creates a new texture cube map from the given cpu texture.
    /// The cpu texture data must contain 6 images all with the width and height specified in the cpu texture.
    /// The images are used in the following order; right, left, top, bottom, front, back.
    ///
    pub fn new<T: TextureDataType>(
        context: &Context,
        cpu_texture: &CPUTexture<T>,
    ) -> ThreeDResult<TextureCubeMap> {
        let id = generate(context)?;
        let number_of_mip_maps = calculate_number_of_mip_maps(
            cpu_texture.mip_map_filter,
            cpu_texture.width,
            cpu_texture.height,
        );
        set_parameters(
            context,
            &id,
            consts::TEXTURE_CUBE_MAP,
            cpu_texture.min_filter,
            cpu_texture.mag_filter,
            if number_of_mip_maps == 1 {
                None
            } else {
                cpu_texture.mip_map_filter
            },
            cpu_texture.wrap_s,
            cpu_texture.wrap_t,
            Some(cpu_texture.wrap_r),
        );
        context.bind_texture(consts::TEXTURE_CUBE_MAP, &id);
        context.tex_storage_2d(
            consts::TEXTURE_CUBE_MAP,
            number_of_mip_maps,
            T::internal_format(cpu_texture.format)?,
            cpu_texture.width,
            cpu_texture.height,
        );
        let mut texture = Self {
            context: context.clone(),
            id,
            width: cpu_texture.width,
            height: cpu_texture.height,
            format: cpu_texture.format,
            number_of_mip_maps,
            is_hdr: T::bits_per_channel() > 8,
        };
        texture.fill(&cpu_texture.data)?;
        Ok(texture)
    }

    ///
    /// Fills the cube map texture with the given data which should contain pixel data for 6 images in the following order; right, left, top, bottom, front, back.
    ///
    /// # Errors
    /// Returns an error if the length of the data does not correspond to 6 images with the width, height and format specified at construction.
    ///
    pub fn fill<T: TextureDataType>(&mut self, data: &[T]) -> ThreeDResult<()> {
        let offset = data.len() / 6;
        check_data_length(self.width, self.height, 1, self.format, offset)?;
        self.context
            .bind_texture(consts::TEXTURE_CUBE_MAP, &self.id);
        for i in 0..6 {
            T::fill(
                &self.context,
                consts::TEXTURE_CUBE_MAP_POSITIVE_X + i as u32,
                self.width,
                self.height,
                self.format,
                &data[i * offset..(i + 1) * offset],
            );
        }
        self.generate_mip_maps();
        Ok(())
    }

    pub(in crate::core) fn generate_mip_maps(&self) {
        if self.number_of_mip_maps > 1 {
            self.context
                .bind_texture(consts::TEXTURE_CUBE_MAP, &self.id);
            self.context.generate_mipmap(consts::TEXTURE_CUBE_MAP);
        }
    }
}

impl TextureCube for TextureCubeMap {
    fn bind(&self, location: u32) {
        bind_at(&self.context, &self.id, consts::TEXTURE_CUBE_MAP, location);
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
    fn is_hdr(&self) -> bool {
        self.is_hdr
    }
}

impl Drop for TextureCubeMap {
    fn drop(&mut self) {
        self.context.delete_texture(&self.id);
    }
}
