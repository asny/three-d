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
    pub fn new_(context: &Context, cpu_texture: &CPUTexture) -> Result<Texture2D, Error>
    {
        let id = generate(context)?;
        let number_of_mip_maps = calculate_number_of_mip_maps(cpu_texture.mip_map_filter, cpu_texture.width, cpu_texture.height, 1);
        set_parameters(context, &id,consts::TEXTURE_2D,
                       cpu_texture.min_filter, cpu_texture.mag_filter,
                       if number_of_mip_maps == 1 {None} else {cpu_texture.mip_map_filter},
                       cpu_texture.wrap_s, cpu_texture.wrap_t, None);
        context.tex_storage_2d(consts::TEXTURE_2D,
                               number_of_mip_maps,
                               cpu_texture.format as u32,
                               cpu_texture.width as u32,
                               cpu_texture.height as u32);
        let mut texture = Self { context: context.clone(), id, width: cpu_texture.width, height: cpu_texture.height, format: cpu_texture.format, number_of_mip_maps };
        if let Some(ref bytes) = cpu_texture.bytes {
            texture.fill_with_u8(bytes)?;
        }
        Ok(texture)
    }

    pub fn new(context: &Context, width: usize, height: usize, min_filter: Interpolation, mag_filter: Interpolation, mip_map_filter: Option<Interpolation>,
               wrap_s: Wrapping, wrap_t: Wrapping, format: Format) -> Result<Texture2D, Error>
    {
        let id = generate(context)?;
        let number_of_mip_maps = calculate_number_of_mip_maps(mip_map_filter, width, height, 1);
        set_parameters(context, &id,consts::TEXTURE_2D, min_filter, mag_filter, if number_of_mip_maps == 1 {None} else {mip_map_filter}, wrap_s, wrap_t, None);
        context.tex_storage_2d(consts::TEXTURE_2D,
                        number_of_mip_maps,
                        format as u32,
                        width as u32,
                        height as u32);
        Ok(Self { context: context.clone(), id, width, height, format, number_of_mip_maps })
    }

    pub fn fill_with_u8(&mut self, data: &[u8]) -> Result<(), Error>
    {
        let format =
            match self.format {
                Format::R8 => Ok(consts::RED),
                Format::RGB8 => Ok(consts::RGB),
                Format::RGBA8 => Ok(consts::RGBA),
                _ => Err(Error::FailedToCreateTexture {message: "Wrong texture format".to_string()})
            }?;

        let mut desired_length = self.width * self.height;
        if format == consts::RGB { desired_length *= 3 };
        if format == consts::RGBA { desired_length *= 4 };

        if data.len() != desired_length {
            Err(Error::FailedToCreateTexture {message: format!("Wrong size of data for the texture ({} != {})", data.len(), desired_length)})?
        }
        self.context.bind_texture(consts::TEXTURE_2D, &self.id);
        self.context.tex_sub_image_2d_with_u8_data(consts::TEXTURE_2D, 0, 0, 0,
                                              self.width as u32, self.height as u32,
                                              format, consts::UNSIGNED_BYTE, data);
        self.generate_mip_maps();
        Ok(())
    }

    pub fn fill_with_f32(&mut self, data: &[f32]) -> Result<(), Error>
    {
        let format =
            match self.format {
                Format::R32F => Ok(consts::RED),
                Format::RGB32F => Ok(consts::RGB),
                Format::RGBA32F => Ok(consts::RGBA),
                _ => Err(Error::FailedToCreateTexture {message: "Wrong texture format".to_string()})
            }?;

        let mut desired_length = self.width * self.height;
        if format == consts::RGB { desired_length *= 3 };
        if format == consts::RGBA { desired_length *= 4 };

        if data.len() != desired_length {
            Err(Error::FailedToCreateTexture {message: format!("Wrong size of data for the texture ({} != {})", data.len(), desired_length)})?
        }
        self.context.bind_texture(consts::TEXTURE_2D, &self.id);
        self.context.tex_sub_image_2d_with_f32_data(consts::TEXTURE_2D,0,0,0,
                                           self.width as u32,self.height as u32,
                                               format,consts::FLOAT,data);
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
                    format as u32,
                    width as u32,
                    height as u32);
        Ok(Self { context: context.clone(), id, width, height, format, number_of_mip_maps })
    }

    pub fn new_with_u8(context: &Context, right: &CPUTexture, left: &CPUTexture, top: &CPUTexture, bottom: &CPUTexture, front: &CPUTexture, back: &CPUTexture) -> Result<Self, Error>
    {
        let error = || {Error::FailedToCreateTexture { message:"".to_owned() }};
        let mut texture = Self::new(context, right.width as usize, right.height as usize,
                                    right.min_filter, right.mag_filter, right.mip_map_filter, right.wrap_s, right.wrap_t, right.wrap_r, right.format)?;

        texture.fill_with_u8([&right.bytes.as_ref().ok_or_else(error)?,
            &left.bytes.as_ref().ok_or_else(error)?,
            &top.bytes.as_ref().ok_or_else(error)?,
            &bottom.bytes.as_ref().ok_or_else(error)?,
            &front.bytes.as_ref().ok_or_else(error)?,
            &back.bytes.as_ref().ok_or_else(error)?])?;
        Ok(texture)
    }

    pub fn fill_with_u8(&mut self, data: [&[u8]; 6]) -> Result<(), Error>
    {
        let format =
            match self.format {
                Format::R8 => Ok(consts::RED),
                Format::RGB8 => Ok(consts::RGB),
                Format::RGBA8 => Ok(consts::RGBA),
                _ => Err(Error::FailedToCreateTexture {message: "Wrong texture format".to_string()})
            }?;

        let mut desired_length = self.width * self.height;
        if format == consts::RGB { desired_length *= 3 };
        if format == consts::RGBA { desired_length *= 4 };

        if data[0].len() != desired_length {
            Err(Error::FailedToCreateTexture {message: format!("Wrong size of data for the texture ({} != {})", data[0].len(), desired_length)})?
        }
        self.context.bind_texture(consts::TEXTURE_CUBE_MAP, &self.id);
        for i in 0..6 {
            self.context.tex_sub_image_2d_with_u8_data(consts::TEXTURE_CUBE_MAP_POSITIVE_X + i as u32, 0, 0, 0,
                                                  self.width as u32, self.height as u32,
                                                  format, consts::UNSIGNED_BYTE, data[i]);
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
                        format as u32,
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