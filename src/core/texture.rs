use crate::context::{consts, Context};
use crate::core::*;
use crate::definition::*;
use crate::math::*;

pub use crate::{Format, Interpolation, Wrapping};

///
/// A texture that can be sampled in a fragment shader (see [use_texture](crate::Program::use_texture)).
///
pub trait Texture {
    /// Binds this texture to the current shader program.
    fn bind(&self, location: u32);
    /// The width of this texture.
    fn width(&self) -> usize;
    /// The height of this texture.
    fn height(&self) -> usize;
}

///
/// A texture array that can be sampled in a fragment shader (see [use_texture_array](crate::Program::use_texture_array)).
///
pub trait TextureArray {
    /// Binds this texture array to the current shader program.
    fn bind(&self, location: u32);
    /// The width of this texture.
    fn width(&self) -> usize;
    /// The height of this texture.
    fn height(&self) -> usize;
    /// The depth of this texture, ie. the number of layers.
    fn depth(&self) -> usize;
}

///
/// A 2D texture, basically an image that is transferred to the GPU.
/// For a texture that can be rendered into, see [ColorTargetTexture2D](crate::ColorTargetTexture2D).
///
pub struct Texture2D {
    context: Context,
    id: crate::context::Texture,
    width: usize,
    height: usize,
    format: Format,
    number_of_mip_maps: u32,
}

impl Texture2D {
    ///
    /// Construcs a new texture with the given data.
    ///
    pub fn new<T: TextureValueType>(
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
    pub fn fill<T: TextureValueType>(&mut self, data: &[T]) -> Result<(), Error> {
        check_data_length(self.width, self.height, 1, self.format, data.len())?;
        self.context.bind_texture(consts::TEXTURE_2D, &self.id);
        T::fill(
            &self.context,
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
    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        self.height
    }
}

impl Drop for Texture2D {
    fn drop(&mut self) {
        self.context.delete_texture(&self.id);
    }
}

///
/// A 2D color texture that can be rendered into and read from.
///
/// **Note:** [Depth test](crate::DepthTestType) is disabled if not also writing to a depth texture.
/// Use a [RenderTarget](crate::RenderTarget) to write to both color and depth.
///
pub struct ColorTargetTexture2D<T: TextureValueType> {
    context: Context,
    id: crate::context::Texture,
    width: usize,
    height: usize,
    number_of_mip_maps: u32,
    format: Format,
    _dummy: T,
}

impl<T: TextureValueType> ColorTargetTexture2D<T> {
    ///
    /// Constructs a new 2D color target texture.
    ///
    pub fn new(
        context: &Context,
        width: usize,
        height: usize,
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
            width as u32,
            height as u32,
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

    ///
    /// Renders whatever rendered in the `render` closure into the texture.
    /// Before writing, the texture is cleared based on the given clear state.
    ///
    /// **Note:** [Depth test](crate::DepthTestType) is disabled if not also writing to a depth texture.
    /// Use a [RenderTarget](crate::RenderTarget) to write to both color and depth.
    ///
    pub fn write<F: FnOnce() -> Result<(), Error>>(
        &self,
        clear_state: ClearState,
        render: F,
    ) -> Result<(), Error> {
        RenderTarget::<T>::new_color(&self.context, &self)?.write(clear_state, render)
    }

    ///
    /// Copies the content of the color texture to the specified [destination](crate::CopyDestination) at the given viewport.
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
            viewport.width * viewport.height * self.format.color_channel_count()
        ];
        let render_target = RenderTarget::new_color(&self.context, &self)?;
        render_target.bind(consts::DRAW_FRAMEBUFFER)?;
        render_target.bind(consts::READ_FRAMEBUFFER)?;
        T::read(&self.context, viewport, self.format, &mut pixels);
        Ok(pixels)
    }

    pub(super) fn generate_mip_maps(&self) {
        if self.number_of_mip_maps > 1 {
            self.context.bind_texture(consts::TEXTURE_2D, &self.id);
            self.context.generate_mipmap(consts::TEXTURE_2D);
        }
    }

    pub(super) fn bind_as_color_target(&self, channel: usize) {
        self.context.framebuffer_texture_2d(
            consts::FRAMEBUFFER,
            consts::COLOR_ATTACHMENT0 + channel as u32,
            consts::TEXTURE_2D,
            &self.id,
            0,
        );
    }
}

impl<T: TextureValueType> Texture for ColorTargetTexture2D<T> {
    fn bind(&self, location: u32) {
        bind_at(&self.context, &self.id, consts::TEXTURE_2D, location);
    }
    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        self.height
    }
}

impl<T: TextureValueType> Drop for ColorTargetTexture2D<T> {
    fn drop(&mut self) {
        self.context.delete_texture(&self.id);
    }
}

///
/// Type of formats for depth render targets ([DepthTargetTexture2D](crate::DepthTargetTexture2D) and
/// [DepthTargetTexture2DArray](crate::DepthTargetTexture2DArray)).
///
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum DepthFormat {
    Depth16,
    Depth24,
    Depth32F,
}

///
/// A 2D depth texture that can be rendered into and read from. See also [RenderTarget](crate::RenderTarget).
///
pub struct DepthTargetTexture2D {
    context: Context,
    id: crate::context::Texture,
    width: usize,
    height: usize,
}

impl DepthTargetTexture2D {
    ///
    /// Constructs a new 2D depth target texture.
    ///
    pub fn new(
        context: &Context,
        width: usize,
        height: usize,
        wrap_s: Wrapping,
        wrap_t: Wrapping,
        format: DepthFormat,
    ) -> Result<Self, Error> {
        let id = generate(context)?;
        set_parameters(
            context,
            &id,
            consts::TEXTURE_2D,
            Interpolation::Nearest,
            Interpolation::Nearest,
            None,
            wrap_s,
            wrap_t,
            None,
        );
        context.tex_storage_2d(
            consts::TEXTURE_2D,
            1,
            internal_format_from_depth(format),
            width as u32,
            height as u32,
        );
        Ok(Self {
            context: context.clone(),
            id,
            width,
            height,
        })
    }

    ///
    /// Write the depth of whatever rendered in the `render` closure into the texture.
    /// Before writing, the texture is cleared based on the given clear state.
    ///
    pub fn write<F: FnOnce() -> Result<(), Error>>(
        &self,
        clear_state: Option<f32>,
        render: F,
    ) -> Result<(), Error> {
        RenderTarget::<f32>::new_depth(&self.context, &self)?.write(
            ClearState {
                depth: clear_state,
                ..ClearState::none()
            },
            render,
        )
    }

    ///
    /// Copies the content of the depth texture to the specified [destination](crate::CopyDestination) at the given viewport.
    ///
    /// # Errors
    /// Will return an error if the destination is a color texture.
    ///
    pub fn copy_to<T: TextureValueType>(
        &self,
        destination: CopyDestination<T>,
        viewport: Viewport,
    ) -> Result<(), Error> {
        RenderTarget::new_depth(&self.context, &self)?.copy_to(
            destination,
            viewport,
            WriteMask::DEPTH,
        )
    }

    pub(super) fn bind_as_depth_target(&self) {
        self.context.framebuffer_texture_2d(
            consts::FRAMEBUFFER,
            consts::DEPTH_ATTACHMENT,
            consts::TEXTURE_2D,
            &self.id,
            0,
        );
    }
}

impl Texture for DepthTargetTexture2D {
    fn bind(&self, location: u32) {
        bind_at(&self.context, &self.id, consts::TEXTURE_2D, location);
    }
    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        self.height
    }
}

impl Drop for DepthTargetTexture2D {
    fn drop(&mut self) {
        self.context.delete_texture(&self.id);
    }
}

///
/// A texture that covers all 6 sides of a cube.
///
pub struct TextureCubeMap {
    context: Context,
    id: crate::context::Texture,
    width: usize,
    height: usize,
    format: Format,
    number_of_mip_maps: u32,
}

impl TextureCubeMap {
    pub fn new_with_u8(context: &Context, cpu_texture: &CPUTexture<u8>) -> Result<Self, Error> {
        let mut texture = Self::new(context, cpu_texture)?;
        texture.fill_with_u8(&cpu_texture.data)?;
        Ok(texture)
    }

    // data contains 6 images in the following order; right, left, top, bottom, front, back
    pub fn fill_with_u8(&mut self, data: &[u8]) -> Result<(), Error> {
        let offset = data.len() / 6;
        check_data_length(self.width, self.height, 1, self.format, offset)?;
        self.context
            .bind_texture(consts::TEXTURE_CUBE_MAP, &self.id);
        for i in 0..6 {
            self.context.tex_sub_image_2d_with_u8_data(
                consts::TEXTURE_CUBE_MAP_POSITIVE_X + i as u32,
                0,
                0,
                0,
                self.width as u32,
                self.height as u32,
                internal::format_from(self.format),
                consts::UNSIGNED_BYTE,
                &data[i * offset..(i + 1) * offset],
            );
        }
        self.generate_mip_maps();
        Ok(())
    }

    fn new<T: TextureValueType>(
        context: &Context,
        cpu_texture: &CPUTexture<T>,
    ) -> Result<TextureCubeMap, Error> {
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
            cpu_texture.width as u32,
            cpu_texture.height as u32,
        );
        Ok(Self {
            context: context.clone(),
            id,
            width: cpu_texture.width,
            height: cpu_texture.height,
            format: cpu_texture.format,
            number_of_mip_maps,
        })
    }

    pub fn bind(&self, location: u32) {
        bind_at(&self.context, &self.id, consts::TEXTURE_CUBE_MAP, location);
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub(crate) fn generate_mip_maps(&self) {
        if self.number_of_mip_maps > 1 {
            self.context
                .bind_texture(consts::TEXTURE_CUBE_MAP, &self.id);
            self.context.generate_mipmap(consts::TEXTURE_CUBE_MAP);
        }
    }
}

impl Drop for TextureCubeMap {
    fn drop(&mut self) {
        self.context.delete_texture(&self.id);
    }
}

///
/// A array of 2D color textures that can be rendered into.
///
/// **Note:** [Depth test](crate::DepthTestType) is disabled if not also writing to a depth texture array.
/// Use a [RenderTargetArray](crate::RenderTargetArray) to write to both color and depth.
///
pub struct ColorTargetTexture2DArray<T: TextureValueType> {
    context: Context,
    id: crate::context::Texture,
    width: usize,
    height: usize,
    depth: usize,
    number_of_mip_maps: u32,
    _dummy: T,
}

impl<T: TextureValueType> ColorTargetTexture2DArray<T> {
    pub fn new(
        context: &Context,
        width: usize,
        height: usize,
        depth: usize,
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
            width as u32,
            height as u32,
            depth as u32,
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
        color_layers: &[usize],
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
        color_layer: usize,
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

    pub(crate) fn bind_as_color_target(&self, layer: usize, channel: usize) {
        self.context.framebuffer_texture_layer(
            consts::DRAW_FRAMEBUFFER,
            consts::COLOR_ATTACHMENT0 + channel as u32,
            &self.id,
            0,
            layer as u32,
        );
    }
}

impl<T: TextureValueType> TextureArray for ColorTargetTexture2DArray<T> {
    fn bind(&self, location: u32) {
        bind_at(&self.context, &self.id, consts::TEXTURE_2D_ARRAY, location);
    }
    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        self.height
    }
    fn depth(&self) -> usize {
        self.depth
    }
}

impl<T: TextureValueType> Drop for ColorTargetTexture2DArray<T> {
    fn drop(&mut self) {
        self.context.delete_texture(&self.id);
    }
}

///
/// An array of 2D depth textures that can be rendered into and read from. See also [RenderTargetArray](crate::RenderTargetArray).
///
pub struct DepthTargetTexture2DArray {
    context: Context,
    id: crate::context::Texture,
    width: usize,
    height: usize,
    depth: usize,
}

impl DepthTargetTexture2DArray {
    pub fn new(
        context: &Context,
        width: usize,
        height: usize,
        depth: usize,
        wrap_s: Wrapping,
        wrap_t: Wrapping,
        format: DepthFormat,
    ) -> Result<Self, Error> {
        let id = generate(context)?;
        set_parameters(
            context,
            &id,
            consts::TEXTURE_2D_ARRAY,
            Interpolation::Nearest,
            Interpolation::Nearest,
            None,
            wrap_s,
            wrap_t,
            None,
        );
        context.bind_texture(consts::TEXTURE_2D_ARRAY, &id);
        context.tex_storage_3d(
            consts::TEXTURE_2D_ARRAY,
            1,
            internal_format_from_depth(format),
            width as u32,
            height as u32,
            depth as u32,
        );
        Ok(Self {
            context: context.clone(),
            id,
            width,
            height,
            depth,
        })
    }

    ///
    /// Writes the depth of whatever rendered in the `render` closure into the depth texture defined by the input parameter `depth_layer`.
    /// Before writing, the texture is cleared based on the given clear state.
    ///
    pub fn write<F: FnOnce() -> Result<(), Error>>(
        &self,
        depth_layer: usize,
        clear_state: Option<f32>,
        render: F,
    ) -> Result<(), Error> {
        RenderTargetArray::<u8>::new_depth(&self.context, &self)?.write(
            &[],
            depth_layer,
            ClearState {
                depth: clear_state,
                ..ClearState::none()
            },
            render,
        )
    }

    ///
    /// Copies the content of the depth texture at the given layer to the specified [destination](crate::CopyDestination) at the given viewport.
    ///
    /// # Errors
    /// Will return an error if the destination is a color texture.
    ///
    pub fn copy_to<T: TextureValueType>(
        &self,
        depth_layer: usize,
        destination: CopyDestination<T>,
        viewport: Viewport,
    ) -> Result<(), Error> {
        RenderTargetArray::new_depth(&self.context, &self)?.copy_to(
            0,
            depth_layer,
            destination,
            viewport,
            WriteMask::DEPTH,
        )
    }

    pub(crate) fn bind_as_depth_target(&self, layer: usize) {
        self.context.framebuffer_texture_layer(
            consts::DRAW_FRAMEBUFFER,
            consts::DEPTH_ATTACHMENT,
            &self.id,
            0,
            layer as u32,
        );
    }
}

impl TextureArray for DepthTargetTexture2DArray {
    fn bind(&self, location: u32) {
        bind_at(&self.context, &self.id, consts::TEXTURE_2D_ARRAY, location);
    }
    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        self.height
    }
    fn depth(&self) -> usize {
        self.depth
    }
}

impl Drop for DepthTargetTexture2DArray {
    fn drop(&mut self) {
        self.context.delete_texture(&self.id);
    }
}

// COMMON FUNCTIONS
fn generate(context: &Context) -> Result<crate::context::Texture, Error> {
    context.create_texture().ok_or_else(|| Error::TextureError {
        message: "Failed to create texture".to_string(),
    })
}

fn bind_at(context: &Context, id: &crate::context::Texture, target: u32, location: u32) {
    context.active_texture(consts::TEXTURE0 + location);
    context.bind_texture(target, id);
}

fn set_parameters(
    context: &Context,
    id: &crate::context::Texture,
    target: u32,
    min_filter: Interpolation,
    mag_filter: Interpolation,
    mip_map_filter: Option<Interpolation>,
    wrap_s: Wrapping,
    wrap_t: Wrapping,
    wrap_r: Option<Wrapping>,
) {
    context.bind_texture(target, id);
    match mip_map_filter {
        None => context.tex_parameteri(
            target,
            consts::TEXTURE_MIN_FILTER,
            interpolation_from(min_filter),
        ),
        Some(Interpolation::Nearest) => {
            if min_filter == Interpolation::Nearest {
                context.tex_parameteri(
                    target,
                    consts::TEXTURE_MIN_FILTER,
                    consts::NEAREST_MIPMAP_NEAREST as i32,
                );
            } else {
                context.tex_parameteri(
                    target,
                    consts::TEXTURE_MIN_FILTER,
                    consts::LINEAR_MIPMAP_NEAREST as i32,
                )
            }
        }
        Some(Interpolation::Linear) => {
            if min_filter == Interpolation::Nearest {
                context.tex_parameteri(
                    target,
                    consts::TEXTURE_MIN_FILTER,
                    consts::NEAREST_MIPMAP_LINEAR as i32,
                );
            } else {
                context.tex_parameteri(
                    target,
                    consts::TEXTURE_MIN_FILTER,
                    consts::LINEAR_MIPMAP_LINEAR as i32,
                )
            }
        }
    }
    context.tex_parameteri(
        target,
        consts::TEXTURE_MAG_FILTER,
        interpolation_from(mag_filter),
    );
    context.tex_parameteri(target, consts::TEXTURE_WRAP_S, wrapping_from(wrap_s));
    context.tex_parameteri(target, consts::TEXTURE_WRAP_T, wrapping_from(wrap_t));
    if let Some(r) = wrap_r {
        context.tex_parameteri(target, consts::TEXTURE_WRAP_R, wrapping_from(r));
    }
}

fn calculate_number_of_mip_maps(
    mip_map_filter: Option<Interpolation>,
    width: usize,
    height: usize,
    depth: usize,
) -> u32 {
    if mip_map_filter.is_some() {
        let w = (width as f64).log2().ceil();
        let h = (height as f64).log2().ceil();
        let d = (depth as f64).log2().ceil();
        w.max(h).max(d).floor() as u32 + 1
    } else {
        1
    }
}

fn check_data_length(
    width: usize,
    height: usize,
    depth: usize,
    format: Format,
    length: usize,
) -> Result<(), Error> {
    let expected_pixels = width * height * depth;
    let actual_pixels = length / format.color_channel_count();

    if expected_pixels != actual_pixels {
        Err(Error::TextureError {
            message: format!(
                "Wrong size of data for the texture (got {} pixels but expected {} pixels)",
                actual_pixels, expected_pixels
            ),
        })?;
    }
    Ok(())
}

fn internal_format_from_depth(format: DepthFormat) -> u32 {
    match format {
        DepthFormat::Depth16 => consts::DEPTH_COMPONENT16,
        DepthFormat::Depth24 => consts::DEPTH_COMPONENT24,
        DepthFormat::Depth32F => consts::DEPTH_COMPONENT32F,
    }
}

fn wrapping_from(wrapping: Wrapping) -> i32 {
    (match wrapping {
        Wrapping::Repeat => consts::REPEAT,
        Wrapping::MirroredRepeat => consts::MIRRORED_REPEAT,
        Wrapping::ClampToEdge => consts::CLAMP_TO_EDGE,
    }) as i32
}

fn interpolation_from(interpolation: Interpolation) -> i32 {
    (match interpolation {
        Interpolation::Nearest => consts::NEAREST,
        Interpolation::Linear => consts::LINEAR,
    }) as i32
}
