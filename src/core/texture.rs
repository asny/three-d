use crate::core::Error;
use crate::context::{Context, consts};
use crate::cpu_texture::*;

pub trait Texture {
    fn bind(&self, location: u32);
}

pub struct Texture2D {
    context: Context,
    id: crate::context::Texture,
    pub width: usize,
    pub height: usize,
    format: Format,
    number_of_mip_maps: u32
}

impl Texture2D
{
    pub fn new_with_u8(context: &Context, cpu_texture: &CPUTexture<u8>) -> Result<Texture2D, Error>
    {
        let mut texture = Self::new(context, cpu_texture.width, cpu_texture.height,
                                    cpu_texture.min_filter, cpu_texture.mag_filter, cpu_texture.mip_map_filter,
                                    cpu_texture.wrap_s, cpu_texture.wrap_t, cpu_texture.format)?;
        texture.fill_with_u8(&cpu_texture.data)?;
        Ok(texture)
    }

    pub fn new_with_f32(context: &Context, cpu_texture: &CPUTexture<f32>) -> Result<Texture2D, Error>
    {
        let mut texture = Self::new(context, cpu_texture.width, cpu_texture.height,
                                    cpu_texture.min_filter, cpu_texture.mag_filter, cpu_texture.mip_map_filter,
                                    cpu_texture.wrap_s, cpu_texture.wrap_t, cpu_texture.format)?;
        texture.fill_with_f32(&cpu_texture.data)?;
        Ok(texture)
    }

    pub fn new(context: &Context, width: usize, height: usize, min_filter: Interpolation, mag_filter: Interpolation, mip_map_filter: Option<Interpolation>,
               wrap_s: Wrapping, wrap_t: Wrapping, format: Format) -> Result<Texture2D, Error>
    {
        let id = generate(context)?;
        let number_of_mip_maps = calculate_number_of_mip_maps(mip_map_filter, width, height, 1);
        set_parameters(context, &id,consts::TEXTURE_2D, min_filter, mag_filter, if number_of_mip_maps == 1 {None} else {mip_map_filter}, wrap_s, wrap_t, None);
        context.tex_storage_2d(consts::TEXTURE_2D, number_of_mip_maps,
                               internal_format_from(format), width as u32, height as u32);
        Ok(Self { context: context.clone(), id, width, height, format, number_of_mip_maps })
    }

    pub fn fill_with_u8(&mut self, data: &[u8]) -> Result<(), Error>
    {
        check_data_length(self.width, self.height, 1, self.format, data.len())?;
        self.context.bind_texture(consts::TEXTURE_2D, &self.id);
        self.context.tex_sub_image_2d_with_u8_data(consts::TEXTURE_2D, 0, 0, 0,
                                              self.width as u32, self.height as u32,
                                                   format_from(self.format), consts::UNSIGNED_BYTE, data);
        self.generate_mip_maps();
        Ok(())
    }

    pub fn fill_with_f32(&mut self, data: &[f32]) -> Result<(), Error>
    {
        check_data_length(self.width, self.height, 1, self.format, data.len())?;
        self.context.bind_texture(consts::TEXTURE_2D, &self.id);
        self.context.tex_sub_image_2d_with_f32_data(consts::TEXTURE_2D, 0, 0, 0,
                                                    self.width as u32, self.height as u32,
                                                    format_from(self.format), consts::FLOAT, data);
        self.generate_mip_maps();
        Ok(())
    }

    pub(crate) fn generate_mip_maps(&self) {
        if self.number_of_mip_maps > 1 {
            self.context.bind_texture(consts::TEXTURE_2D, &self.id);
            self.context.generate_mipmap(consts::TEXTURE_2D);
        }
    }

    pub(crate) fn bind_as_color_target(&self, channel: usize)
    {
        self.context.framebuffer_texture_2d(consts::FRAMEBUFFER,
                       consts::COLOR_ATTACHMENT0 + channel as u32, consts::TEXTURE_2D, &self.id, 0);
    }

    pub(crate) fn bind_as_depth_target(&self)
    {
        self.context.framebuffer_texture_2d(consts::FRAMEBUFFER,
                       consts::DEPTH_ATTACHMENT, consts::TEXTURE_2D, &self.id, 0);
    }
}

impl Texture for Texture2D
{
    fn bind(&self, location: u32)
    {
        bind_at(&self.context, &self.id, consts::TEXTURE_2D, location);
    }
}

impl Drop for Texture2D
{
    fn drop(&mut self)
    {
        self.context.delete_texture(&self.id);
    }
}

pub struct TextureCubeMap {
    context: Context,
    id: crate::context::Texture,
    pub width: usize,
    pub height: usize,
    format: Format,
    number_of_mip_maps: u32
}

impl TextureCubeMap
{
    pub fn new(context: &Context, width: usize, height: usize, min_filter: Interpolation, mag_filter: Interpolation, mip_map_filter: Option<Interpolation>,
               wrap_s: Wrapping, wrap_t: Wrapping, wrap_r: Wrapping, format: Format) -> Result<TextureCubeMap, Error>
    {
        let id = generate(context)?;
        let number_of_mip_maps = calculate_number_of_mip_maps(mip_map_filter, width, height, 1);
        set_parameters(context, &id,consts::TEXTURE_CUBE_MAP, min_filter, mag_filter,
                       if number_of_mip_maps == 1 {None} else {mip_map_filter}, wrap_s, wrap_t, Some(wrap_r));
        context.bind_texture(consts::TEXTURE_CUBE_MAP, &id);
        context.tex_storage_2d(consts::TEXTURE_CUBE_MAP,
                               number_of_mip_maps,
                               internal_format_from(format),
                               width as u32,
                               height as u32);
        Ok(Self { context: context.clone(), id, width, height, format, number_of_mip_maps })
    }

    pub fn new_with_u8(context: &Context, right: &CPUTexture<u8>, left: &CPUTexture<u8>, top: &CPUTexture<u8>, bottom: &CPUTexture<u8>, front: &CPUTexture<u8>, back: &CPUTexture<u8>) -> Result<Self, Error>
    {
        let mut texture = Self::new(context, right.width as usize, right.height as usize,
                                    right.min_filter, right.mag_filter, right.mip_map_filter, right.wrap_s, right.wrap_t, right.wrap_r, right.format)?;

        texture.fill_with_u8([&right.data,
            &left.data,
            &top.data,
            &bottom.data,
            &front.data,
            &back.data])?;
        Ok(texture)
    }

    pub fn fill_with_u8(&mut self, data: [&[u8]; 6]) -> Result<(), Error>
    {
        check_data_length(self.width, self.height, 1, self.format, data[0].len())?;
        self.context.bind_texture(consts::TEXTURE_CUBE_MAP, &self.id);
        for i in 0..6 {
            self.context.tex_sub_image_2d_with_u8_data(consts::TEXTURE_CUBE_MAP_POSITIVE_X + i as u32, 0, 0, 0,
                                                       self.width as u32, self.height as u32,
                                                       format_from(self.format), consts::UNSIGNED_BYTE, data[i]);
        }
        self.generate_mip_maps();
        Ok(())
    }

    pub(crate) fn generate_mip_maps(&self) {
        if self.number_of_mip_maps > 1 {
            self.context.bind_texture(consts::TEXTURE_CUBE_MAP, &self.id);
            self.context.generate_mipmap(consts::TEXTURE_CUBE_MAP);
        }
    }
}

impl Texture for TextureCubeMap
{
    fn bind(&self, location: u32)
    {
        bind_at(&self.context, &self.id, consts::TEXTURE_CUBE_MAP, location);
    }
}

impl Drop for TextureCubeMap
{
    fn drop(&mut self)
    {
        self.context.delete_texture(&self.id);
    }
}

pub struct Texture2DArray {
    context: Context,
    id: crate::context::Texture,
    pub width: usize,
    pub height: usize,
    pub depth: usize,
    number_of_mip_maps: u32
}

impl Texture2DArray
{
    pub fn new(context: &Context, width: usize, height: usize, depth: usize, min_filter: Interpolation, mag_filter: Interpolation, mip_map_filter: Option<Interpolation>,
               wrap_s: Wrapping, wrap_t: Wrapping, format: Format) -> Result<Self, Error>
    {
        let id = generate(context)?;
        let number_of_mip_maps = calculate_number_of_mip_maps(mip_map_filter, width, height, depth);
        set_parameters(context, &id,consts::TEXTURE_2D_ARRAY, min_filter, mag_filter, if number_of_mip_maps == 1 {None} else {mip_map_filter}, wrap_s, wrap_t, None);
        context.bind_texture(consts::TEXTURE_2D_ARRAY, &id);
        context.tex_storage_3d(consts::TEXTURE_2D_ARRAY,
                        number_of_mip_maps,
                        internal_format_from(format),
                        width as u32,
                        height as u32,
                        depth as u32);
        Ok(Self { context: context.clone(), id, width, height, depth, number_of_mip_maps })
    }

    pub(crate) fn generate_mip_maps(&self) {
        if self.number_of_mip_maps > 1 {
            self.context.bind_texture(consts::TEXTURE_2D_ARRAY, &self.id);
            self.context.generate_mipmap(consts::TEXTURE_2D_ARRAY);
        }
    }

    pub(crate) fn bind_as_color_target(&self, layer: usize, channel: usize)
    {
        self.context.framebuffer_texture_layer(consts::DRAW_FRAMEBUFFER,
                      consts::COLOR_ATTACHMENT0 + channel as u32, &self.id, 0, layer as u32);
    }

    pub(crate) fn bind_as_depth_target(&self, layer: usize)
    {
        self.context.framebuffer_texture_layer(consts::DRAW_FRAMEBUFFER,
                       consts::DEPTH_ATTACHMENT, &self.id, 0, layer as u32);
    }
}

impl Texture for Texture2DArray
{
    fn bind(&self, location: u32)
    {
        bind_at(&self.context, &self.id, consts::TEXTURE_2D_ARRAY, location);
    }
}

impl Drop for Texture2DArray
{
    fn drop(&mut self)
    {
        self.context.delete_texture(&self.id);
    }
}


// COMMON FUNCTIONS
fn generate(context: &Context) -> Result<crate::context::Texture, Error>
{
    context.create_texture().ok_or_else(|| Error::FailedToCreateTexture {message: "Failed to create texture".to_string()} )
}

fn bind_at(context: &Context, id: &crate::context::Texture, target: u32, location: u32)
{
    context.active_texture(consts::TEXTURE0 + location);
    context.bind_texture(target, id);
}

fn set_parameters(context: &Context, id: &crate::context::Texture, target: u32, min_filter: Interpolation, mag_filter: Interpolation, mip_map_filter: Option<Interpolation>, wrap_s: Wrapping, wrap_t: Wrapping, wrap_r: Option<Wrapping>)
{
    context.bind_texture(target, id);
    match mip_map_filter {
        None => context.tex_parameteri(target, consts::TEXTURE_MIN_FILTER, min_filter as i32),
        Some(Interpolation::Nearest) =>
            if min_filter == Interpolation::Nearest {
                context.tex_parameteri(target, consts::TEXTURE_MIN_FILTER, consts::NEAREST_MIPMAP_NEAREST as i32);
            } else {
                context.tex_parameteri(target, consts::TEXTURE_MIN_FILTER, consts::LINEAR_MIPMAP_NEAREST as i32)
            },
        Some(Interpolation::Linear) =>
            if min_filter == Interpolation::Nearest {
                context.tex_parameteri(target, consts::TEXTURE_MIN_FILTER, consts::NEAREST_MIPMAP_LINEAR as i32);
            } else {
                context.tex_parameteri(target, consts::TEXTURE_MIN_FILTER, consts::LINEAR_MIPMAP_LINEAR as i32)
            }
    }
    context.tex_parameteri(target, consts::TEXTURE_MAG_FILTER, mag_filter as i32);
    context.tex_parameteri(target, consts::TEXTURE_WRAP_S, wrap_s as i32);
    context.tex_parameteri(target, consts::TEXTURE_WRAP_T, wrap_t as i32);
    if let Some(r) = wrap_r {
        context.tex_parameteri(target, consts::TEXTURE_WRAP_R, r as i32);
    }
}

fn calculate_number_of_mip_maps(mip_map_filter: Option<Interpolation>, width: usize, height: usize, depth: usize) -> u32 {
    if mip_map_filter.is_some() {
            let w = (width as f64).log2().ceil();
            let h = (height as f64).log2().ceil();
            let d = (depth as f64).log2().ceil();
            w.max(h).max(d).floor() as u32 + 1
        } else {1}
}

fn check_data_length(width: usize, height: usize, depth: usize, format: Format, length: usize) -> Result<(), Error> {
    let desired_length = width * height * depth *
        match format_from(format) {
            consts::RED => 1,
            consts::RGB => 3,
            consts::RGBA => 4,
            _ => unreachable!()
        };

    if length != desired_length {
        Err(Error::FailedToCreateTexture {message: format!("Wrong size of data for the texture ({} != {})", length, desired_length)})?;
    }
    Ok(())
}

fn internal_format_from(format: Format) -> u32 {
    match format {
        Format::R8 => consts::R8,
        Format::RGB8 => consts::RGB8,
        Format::RGBA8 => consts::RGBA8,
        _ => format as u32
    }
}

fn format_from(format: Format) -> u32 {
    match format {
        Format::R8 => consts::RED,
        Format::RGB8 => consts::RGB,
        Format::RGBA8 => consts::RGBA,
        Format::R32F => consts::RED,
        Format::RGB32F => consts::RGB,
        Format::RGBA32F => consts::RGBA,
        _ => unreachable!()
    }
}