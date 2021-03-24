use crate::context::{consts, Context};
use crate::core::Error;
use crate::cpu_texture::*;

pub use crate::cpu_texture::{Format,Wrapping,Interpolation};

///
/// A texture that can be sampled in a fragment shader (see [use_texture](crate::Program::use_texture)).
///
pub trait Texture {
    fn bind(&self, location: u32);
    fn width(&self) -> usize;
    fn height(&self) -> usize;
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
    pub fn new_with_u8(
        context: &Context,
        cpu_texture: &CPUTexture<u8>,
    ) -> Result<Texture2D, Error> {
        let mut texture = Self::new(context, cpu_texture)?;
        texture.fill_with_u8(&cpu_texture.data)?;
        Ok(texture)
    }

    pub fn new_with_f32(
        context: &Context,
        cpu_texture: &CPUTexture<f32>,
    ) -> Result<Texture2D, Error> {
        let mut texture = Self::new(context, cpu_texture)?;
        texture.fill_with_f32(&cpu_texture.data)?;
        Ok(texture)
    }

    pub fn fill_with_u8(&mut self, data: &[u8]) -> Result<(), Error> {
        check_u8_format(self.format)?;
        check_data_length(self.width, self.height, 1, self.format, data.len())?;
        self.context.bind_texture(consts::TEXTURE_2D, &self.id);
        self.context.tex_sub_image_2d_with_u8_data(
            consts::TEXTURE_2D,
            0,
            0,
            0,
            self.width as u32,
            self.height as u32,
            format_from(self.format),
            consts::UNSIGNED_BYTE,
            data,
        );
        self.generate_mip_maps();
        Ok(())
    }

    pub fn fill_with_f32(&mut self, data: &[f32]) -> Result<(), Error> {
        check_f32_format(self.format)?;
        check_data_length(self.width, self.height, 1, self.format, data.len())?;
        self.context.bind_texture(consts::TEXTURE_2D, &self.id);
        self.context.tex_sub_image_2d_with_f32_data(
            consts::TEXTURE_2D,
            0,
            0,
            0,
            self.width as u32,
            self.height as u32,
            format_from(self.format),
            consts::FLOAT,
            data,
        );
        self.generate_mip_maps();
        Ok(())
    }

    fn new<T>(context: &Context, cpu_texture: &CPUTexture<T>) -> Result<Texture2D, Error> {
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
            internal_format_from(cpu_texture.format),
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
    fn depth(&self) -> usize {
        1
    }
}

impl Drop for Texture2D {
    fn drop(&mut self) {
        self.context.delete_texture(&self.id);
    }
}

///
/// A 2D texture that can be rendered into using a [RenderTarget](crate::RenderTarget).
///
pub struct ColorTargetTexture2D {
    context: Context,
    id: crate::context::Texture,
    width: usize,
    height: usize,
    number_of_mip_maps: u32,
}

impl ColorTargetTexture2D {
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
            internal_format_from(format),
            width as u32,
            height as u32,
        );
        Ok(Self {
            context: context.clone(),
            id,
            width,
            height,
            number_of_mip_maps,
        })
    }

    pub(crate) fn generate_mip_maps(&self) {
        if self.number_of_mip_maps > 1 {
            self.context.bind_texture(consts::TEXTURE_2D, &self.id);
            self.context.generate_mipmap(consts::TEXTURE_2D);
        }
    }

    pub(crate) fn bind_as_color_target(&self, channel: usize) {
        self.context.framebuffer_texture_2d(
            consts::FRAMEBUFFER,
            consts::COLOR_ATTACHMENT0 + channel as u32,
            consts::TEXTURE_2D,
            &self.id,
            0,
        );
    }
}

impl Texture for ColorTargetTexture2D {
    fn bind(&self, location: u32) {
        bind_at(&self.context, &self.id, consts::TEXTURE_2D, location);
    }
    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        self.height
    }
    fn depth(&self) -> usize {
        1
    }
}

impl Drop for ColorTargetTexture2D {
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
/// A 2D texture that can be rendered into using a [RenderTarget](crate::RenderTarget).
///
pub struct DepthTargetTexture2D {
    context: Context,
    id: crate::context::Texture,
    width: usize,
    height: usize,
}

impl DepthTargetTexture2D {
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

    pub(crate) fn bind_as_depth_target(&self) {
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
    fn depth(&self) -> usize {
        1
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
        check_u8_format(self.format)?;
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
                format_from(self.format),
                consts::UNSIGNED_BYTE,
                &data[i * offset..(i + 1) * offset],
            );
        }
        self.generate_mip_maps();
        Ok(())
    }

    fn new<T>(context: &Context, cpu_texture: &CPUTexture<T>) -> Result<TextureCubeMap, Error> {
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
            internal_format_from(cpu_texture.format),
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

    pub(crate) fn generate_mip_maps(&self) {
        if self.number_of_mip_maps > 1 {
            self.context
                .bind_texture(consts::TEXTURE_CUBE_MAP, &self.id);
            self.context.generate_mipmap(consts::TEXTURE_CUBE_MAP);
        }
    }
}

impl Texture for TextureCubeMap {
    fn bind(&self, location: u32) {
        bind_at(&self.context, &self.id, consts::TEXTURE_CUBE_MAP, location);
    }
    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        self.height
    }
    fn depth(&self) -> usize {
        1
    }
}

impl Drop for TextureCubeMap {
    fn drop(&mut self) {
        self.context.delete_texture(&self.id);
    }
}

///
/// A 2D texture array that can be rendered into using a [RenderTargetArray](crate::RenderTargetArray).
///
pub struct ColorTargetTexture2DArray {
    context: Context,
    id: crate::context::Texture,
    width: usize,
    height: usize,
    depth: usize,
    number_of_mip_maps: u32,
}

impl ColorTargetTexture2DArray {
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
            internal_format_from(format),
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
        })
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

impl Texture for ColorTargetTexture2DArray {
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

impl Drop for ColorTargetTexture2DArray {
    fn drop(&mut self) {
        self.context.delete_texture(&self.id);
    }
}

///
/// A 2D texture array that can be rendered into using a [RenderTargetArray](crate::RenderTargetArray).
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

impl Texture for DepthTargetTexture2DArray {
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
    context
        .create_texture()
        .ok_or_else(|| Error::FailedToCreateTexture {
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

fn check_u8_format(format: Format) -> Result<(), Error> {
    if format == Format::R8
        || format == Format::RGB8
        || format == Format::RGBA8
        || format == Format::SRGB8
        || format == Format::SRGBA8
    {
        Ok(())
    } else {
        Err(Error::FailedToCreateTexture {
            message: format!("Failed filling texture with format {:?} with u8.", format),
        })
    }
}

fn check_f32_format(format: Format) -> Result<(), Error> {
    if format == Format::R32F || format == Format::RGB32F || format == Format::RGBA32F {
        Ok(())
    } else {
        Err(Error::FailedToCreateTexture {
            message: format!("Failed filling texture with format {:?} with f32.", format),
        })
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
    let actual_pixels = length
        / match format_from(format) {
            consts::RED => 1,
            consts::RGB => 3,
            consts::RGBA => 4,
            _ => unreachable!(),
        };

    if expected_pixels != actual_pixels {
        Err(Error::FailedToCreateTexture {
            message: format!(
                "Wrong size of data for the texture (got {} pixels but expected {} pixels)",
                actual_pixels, expected_pixels
            ),
        })?;
    }
    Ok(())
}

fn internal_format_from(format: Format) -> u32 {
    match format {
        Format::R8 => consts::R8,
        Format::RGB8 => consts::RGB8,
        Format::RGBA8 => consts::RGBA8,
        Format::SRGB8 => consts::SRGB8,
        Format::SRGBA8 => consts::SRGB8_ALPHA8,
        Format::R32F => consts::R32F,
        Format::RGB32F => consts::RGB32F,
        Format::RGBA32F => consts::RGBA32F,
    }
}

fn internal_format_from_depth(format: DepthFormat) -> u32 {
    match format {
        DepthFormat::Depth16 => consts::DEPTH_COMPONENT16,
        DepthFormat::Depth24 => consts::DEPTH_COMPONENT24,
        DepthFormat::Depth32F => consts::DEPTH_COMPONENT32F,
    }
}

fn format_from(format: Format) -> u32 {
    match format {
        Format::R8 => consts::RED,
        Format::R32F => consts::RED,
        Format::RGB8 => consts::RGB,
        Format::RGB32F => consts::RGB,
        Format::SRGB8 => consts::RGB,
        Format::RGBA8 => consts::RGBA,
        Format::RGBA32F => consts::RGBA,
        Format::SRGBA8 => consts::RGBA,
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
