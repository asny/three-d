use crate::core::texture::*;
use crate::core::*;

///
/// A color texture cube map that can be rendered into.
///
/// **Note:** [DepthTest] is disabled if not also writing to a [DepthTargetTextureCubeMap].
/// Use a [RenderTargetCubeMap] to write to both color and depth.
///
pub struct ColorTargetTextureCubeMap<T: TextureDataType> {
    context: Context,
    id: crate::context::Texture,
    width: u32,
    height: u32,
    number_of_mip_maps: u32,
    format: Format,
    _dummy: T,
}

impl<T: TextureDataType> ColorTargetTextureCubeMap<T> {
    ///
    /// Creates a new color target cube map.
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
        let id = generate(context)?;
        let number_of_mip_maps = calculate_number_of_mip_maps(mip_map_filter, width, height);
        set_parameters(
            context,
            &id,
            consts::TEXTURE_CUBE_MAP,
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
        context.bind_texture(consts::TEXTURE_CUBE_MAP, &id);
        context.tex_storage_2d(
            consts::TEXTURE_CUBE_MAP,
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
            _dummy: T::default(),
        })
    }

    pub fn write<F: FnOnce() -> ThreeDResult<()>>(
        &self,
        color_layers: &[u32],
        clear_state: ClearState,
        render: F,
    ) -> ThreeDResult<()> {
        RenderTargetCubeMap::new_color(&self.context, &self)?.write(
            color_layers,
            0,
            clear_state,
            render,
        )
    }

    pub(in crate::core) fn generate_mip_maps(&self) {
        if self.number_of_mip_maps > 1 {
            self.context
                .bind_texture(consts::TEXTURE_CUBE_MAP, &self.id);
            self.context.generate_mipmap(consts::TEXTURE_CUBE_MAP);
        }
    }

    pub(in crate::core) fn bind_as_color_target(&self, layer: u32, channel: u32) {
        self.context.framebuffer_texture_2d(
            consts::DRAW_FRAMEBUFFER,
            consts::COLOR_ATTACHMENT0 + channel,
            consts::TEXTURE_CUBE_MAP_POSITIVE_X + layer,
            &self.id,
            0,
        );
    }
}

impl<T: TextureDataType> TextureCube for ColorTargetTextureCubeMap<T> {
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
}

impl<T: TextureDataType> Drop for ColorTargetTextureCubeMap<T> {
    fn drop(&mut self) {
        self.context.delete_texture(&self.id);
    }
}
