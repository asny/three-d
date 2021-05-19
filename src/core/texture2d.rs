use crate::context::{consts, Context};
use crate::core::*;
use crate::definition::*;
use crate::math::*;

///
/// A 2D texture, basically an image that is transferred to the GPU.
/// For a texture that can be rendered into, see [ColorTargetTexture2D](crate::ColorTargetTexture2D).
///
pub struct Texture2D {
    context: Context,
    id: crate::context::Texture,
    width: u32,
    height: u32,
    format: Format,
    number_of_mip_maps: u32,
}

impl Texture2D {
    ///
    /// Construcs a new texture with the given data.
    ///
    pub fn new<T: TextureDataType>(
        context: &Context,
        cpu_texture: &CPUTexture<T>,
    ) -> Result<Texture2D, Error> {
        let id = generate(context)?;
        let number_of_mip_maps = calculate_number_of_mip_maps(
            cpu_texture.mip_map_filter,
            cpu_texture.width,
            cpu_texture.height,
            1,
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
        };
        tex.fill(&cpu_texture.data)?;
        Ok(tex)
    }

    ///
    /// Fills this texture with the given data.
    ///
    /// # Errors
    /// Return an error if the length of the data array is smaller or bigger than the necessary number of bytes to fill the entire texture.
    ///
    pub fn fill<T: TextureDataType>(&mut self, data: &[T]) -> Result<(), Error> {
        check_data_length(self.width, self.height, 1, self.format, data.len())?;
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

    pub(crate) fn generate_mip_maps(&self) {
        if self.number_of_mip_maps > 1 {
            self.context.bind_texture(consts::TEXTURE_2D, &self.id);
            self.context.generate_mipmap(consts::TEXTURE_2D);
        }
    }
}

impl Texture for Texture2D {
    fn bind(&self, location: u32) {
        bind_at(&self.context, &self.id, consts::TEXTURE_2D, location);
    }
    fn width(&self) -> u32 {
        self.width
    }
    fn height(&self) -> u32 {
        self.height
    }
}

impl Drop for Texture2D {
    fn drop(&mut self) {
        self.context.delete_texture(&self.id);
    }
}
