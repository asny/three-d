use crate::context::{consts, Context};
use crate::core::*;
use crate::definition::*;
use crate::math::*;

///
/// A array of 2D color textures that can be rendered into.
///
/// **Note:** [Depth test](crate::DepthTestType) is disabled if not also writing to a depth texture array.
/// Use a [RenderTargetArray](crate::RenderTargetArray) to write to both color and depth.
///
pub struct ColorTargetTexture2DArray<T: TextureDataType> {
    context: Context,
    id: crate::context::Texture,
    width: u32,
    height: u32,
    depth: u32,
    number_of_mip_maps: u32,
    _dummy: T,
}

impl<T: TextureDataType> ColorTargetTexture2DArray<T> {
    pub fn new(
        context: &Context,
        width: u32,
        height: u32,
        depth: u32,
        min_filter: Interpolation,
        mag_filter: Interpolation,
        mip_map_filter: Option<Interpolation>,
        wrap_s: Wrapping,
        wrap_t: Wrapping,
        format: Format,
    ) -> Result<Self, Error> {
        let id = generate(context)?;
        let number_of_mip_maps = calculate_number_of_mip_maps(mip_map_filter, width, height, depth);
        set_parameters(
            context,
            &id,
            consts::TEXTURE_2D_ARRAY,
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
        context.bind_texture(consts::TEXTURE_2D_ARRAY, &id);
        context.tex_storage_3d(
            consts::TEXTURE_2D_ARRAY,
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
            _dummy: T::default(),
        })
    }

    ///
    /// Renders whatever rendered in the `render` closure into the textures defined by the input parameters `color_layers`.
    /// Output at location *i* defined in the fragment shader is written to the color texture layer at the *ith* index in `color_layers`.
    /// Before writing, the textures are cleared based on the given clear state.
    ///
    /// **Note:** [Depth test](crate::DepthTestType) is disabled if not also writing to a depth texture array.
    /// Use a [RenderTargetArray](crate::RenderTargetArray) to write to both color and depth.
    ///
    pub fn write<F: FnOnce() -> Result<(), Error>>(
        &self,
        color_layers: &[u32],
        clear_state: ClearState,
        render: F,
    ) -> Result<(), Error> {
        RenderTargetArray::new_color(&self.context, &self)?.write(
            color_layers,
            0,
            clear_state,
            render,
        )
    }

    ///
    /// Copies the content of the color texture at the given layer to the specified [destination](crate::CopyDestination) at the given viewport.
    /// Will only copy the channels specified by the write mask.
    ///
    /// # Errors
    /// Will return an error if the destination is a depth texture.
    ///
    pub fn copy_to(
        &self,
        color_layer: u32,
        destination: CopyDestination<T>,
        viewport: Viewport,
        write_mask: WriteMask,
    ) -> Result<(), Error> {
        RenderTargetArray::<T>::new_color(&self.context, &self)?.copy_to(
            color_layer,
            0,
            destination,
            viewport,
            write_mask,
        )
    }

    pub(crate) fn generate_mip_maps(&self) {
        if self.number_of_mip_maps > 1 {
            self.context
                .bind_texture(consts::TEXTURE_2D_ARRAY, &self.id);
            self.context.generate_mipmap(consts::TEXTURE_2D_ARRAY);
        }
    }

    pub(crate) fn bind_as_color_target(&self, layer: u32, channel: u32) {
        self.context.framebuffer_texture_layer(
            consts::DRAW_FRAMEBUFFER,
            consts::COLOR_ATTACHMENT0 + channel,
            &self.id,
            0,
            layer,
        );
    }
}

impl<T: TextureDataType> TextureArray for ColorTargetTexture2DArray<T> {
    fn bind(&self, location: u32) {
        bind_at(&self.context, &self.id, consts::TEXTURE_2D_ARRAY, location);
    }
    fn width(&self) -> u32 {
        self.width
    }
    fn height(&self) -> u32 {
        self.height
    }
    fn depth(&self) -> u32 {
        self.depth
    }
}

impl<T: TextureDataType> Drop for ColorTargetTexture2DArray<T> {
    fn drop(&mut self) {
        self.context.delete_texture(&self.id);
    }
}
