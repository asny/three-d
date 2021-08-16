use crate::context::{consts, Context};
use crate::core::texture::*;
use crate::core::*;

///
/// A 2D color texture that can be rendered into and read from.
///
/// **Note:** [DepthTest] is disabled if not also writing to a depth texture.
/// Use a [RenderTarget] to write to both color and depth.
///
pub struct ColorTargetTexture2D<T: TextureDataType> {
    context: Context,
    id: crate::context::Texture,
    width: u32,
    height: u32,
    number_of_mip_maps: u32,
    format: Format,
    /// Set this to true if you want to render some geometry with this texture and the texture contain alpha values and those alpha values are below maximum.
    pub transparent: bool,
    _dummy: T,
}

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
    ) -> Result<Self, Error> {
        let id = generate(context)?;
        let number_of_mip_maps = calculate_number_of_mip_maps(mip_map_filter, width, height, 1);
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
    /// Renders whatever rendered in the `render` closure into the texture.
    /// Before writing, the texture is cleared based on the given clear state.
    ///
    /// **Note:** [DepthTest] is disabled if not also writing to a depth texture.
    /// Use a [RenderTarget] to write to both color and depth.
    ///
    pub fn write<F: FnOnce() -> Result<(), Error>>(
        &self,
        clear_state: ClearState,
        render: F,
    ) -> Result<(), Error> {
        RenderTarget::<T>::new_color(&self.context, &self)?.write(clear_state, render)
    }

    ///
    /// Copies the content of the color texture to the specified destination at the given viewport.
    /// Will only copy the channels specified by the write mask.
    ///
    /// # Errors
    /// Will return an error if the destination is a depth texture.
    ///
    pub fn copy_to(
        &self,
        destination: CopyDestination<T>,
        viewport: Viewport,
        write_mask: WriteMask,
    ) -> Result<(), Error> {
        RenderTarget::new_color(&self.context, &self)?.copy_to(destination, viewport, write_mask)
    }

    ///
    /// Returns the color values of the pixels in this color texture inside the given viewport.
    ///
    /// **Note:** Only works for the RGBA format.
    ///
    /// # Errors
    /// Will return an error if the color texture is not RGBA format.
    ///
    pub fn read(&self, viewport: Viewport) -> Result<Vec<T>, Error> {
        if self.format != Format::RGBA {
            Err(Error::TextureError {
                message: "Cannot read color from anything else but an RGBA texture.".to_owned(),
            })?;
        }

        let mut pixels = vec![
            T::default();
            viewport.width as usize
                * viewport.height as usize
                * self.format.color_channel_count() as usize
        ];
        let render_target = RenderTarget::new_color(&self.context, &self)?;
        render_target.bind(consts::DRAW_FRAMEBUFFER)?;
        render_target.bind(consts::READ_FRAMEBUFFER)?;
        T::read(&self.context, viewport, self.format, &mut pixels);
        Ok(pixels)
    }

    pub(in crate::core) fn generate_mip_maps(&self) {
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

impl<T: TextureDataType> Texture for ColorTargetTexture2D<T> {
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

impl<T: TextureDataType> Drop for ColorTargetTexture2D<T> {
    fn drop(&mut self) {
        self.context.delete_texture(&self.id);
    }
}
