use crate::core::Error;
use crate::gl::Gl;
use crate::gl::consts;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Interpolation {
    Nearest = consts::NEAREST as isize,
    Linear = consts::LINEAR as isize
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Wrapping {
    Repeat = consts::REPEAT as isize,
    MirroredRepeat = consts::MIRRORED_REPEAT as isize,
    ClampToEdge = consts::CLAMP_TO_EDGE as isize
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Format {
    R8 = consts::R8 as isize,
    R32F = consts::R32F as isize,
    RGB8 = consts::RGB8 as isize,
    RGB32F = consts::RGB32F as isize,
    RGBA4 = consts::RGBA4 as isize,
    RGBA8 = consts::RGBA8 as isize,
    RGBA32F = consts::RGBA32F as isize,
    Depth16 = consts::DEPTH_COMPONENT16 as isize,
    Depth24 = consts::DEPTH_COMPONENT24 as isize,
    Depth32F = consts::DEPTH_COMPONENT32F as isize
}

pub trait Texture {
    fn bind(&self, location: u32);
}

pub struct Texture2D {
    gl: Gl,
    id: crate::gl::Texture,
    pub width: usize,
    pub height: usize,
    format: Format,
    number_of_mip_maps: u32
}

impl Texture2D
{
    pub fn new(gl: &Gl, width: usize, height: usize, min_filter: Interpolation, mag_filter: Interpolation, mip_map_filter: Option<Interpolation>,
           wrap_s: Wrapping, wrap_t: Wrapping, format: Format) -> Result<Texture2D, Error>
    {
        let id = generate(gl)?;
        let number_of_mip_maps = calculate_number_of_mip_maps(mip_map_filter, width, height, 1);
        set_parameters(gl, &id,consts::TEXTURE_2D, min_filter, mag_filter, if number_of_mip_maps == 1 {None} else {mip_map_filter}, wrap_s, wrap_t, None);
        gl.tex_storage_2d(consts::TEXTURE_2D,
                        number_of_mip_maps,
                        format as u32,
                        width as u32,
                        height as u32);
        Ok(Self { gl: gl.clone(), id, width, height, format, number_of_mip_maps })
    }

    #[cfg(feature = "image-io")]
    pub fn new_from_bytes(gl: &Gl, min_filter: Interpolation, mag_filter: Interpolation, mip_map_filter: Option<Interpolation>,
           wrap_s: Wrapping, wrap_t: Wrapping, bytes: &[u8]) -> Result<Texture2D, Error>
    {
        use image::GenericImageView;
        let img = image::load_from_memory(bytes)?;
        let (width, height) = img.dimensions();
        Self::new_with_u8(gl, min_filter, mag_filter, mip_map_filter, wrap_s, wrap_t, width, height, &img.raw_pixels())
    }

    #[cfg(all(not(target_arch = "wasm32"), feature = "image-io"))]
    pub fn new_from_file(gl: &Gl, min_filter: Interpolation, mag_filter: Interpolation, mip_map_filter: Option<Interpolation>,
           wrap_s: Wrapping, wrap_t: Wrapping, path: &str) -> Result<Texture2D, Error>
    {
        use image::GenericImageView;
        let img = image::open(path)?;
        let (width, height) = img.dimensions();
        Self::new_with_u8(gl, min_filter, mag_filter, mip_map_filter, wrap_s, wrap_t, width, height, &img.raw_pixels())
    }

    pub fn new_with_u8(gl: &Gl, min_filter: Interpolation, mag_filter: Interpolation, mip_map_filter: Option<Interpolation>,
           wrap_s: Wrapping, wrap_t: Wrapping, width: u32, height: u32, data: &[u8]) -> Result<Texture2D, Error>
    {
        let number_of_channels = data.len() as i32 / (width as i32 * height as i32);
        let format = match number_of_channels {
            1 => Ok(Format::R8),
            3 => Ok(Format::RGB8),
            4 => Ok(Format::RGBA8),
            _ => Err(Error::FailedToCreateTexture {message: format!("Unsupported texture format")})
        }?;

        let mut texture = Texture2D::new(gl, width as usize, height as usize,
            min_filter, mag_filter, mip_map_filter, wrap_s, wrap_t, format)?;
        texture.fill_with_u8(data)?;
        Ok(texture)
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
        self.gl.bind_texture(consts::TEXTURE_2D, &self.id);
        self.gl.tex_sub_image_2d_with_u8_data(consts::TEXTURE_2D, 0, 0, 0,
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
        self.gl.bind_texture(consts::TEXTURE_2D, &self.id);
        self.gl.tex_sub_image_2d_with_f32_data(consts::TEXTURE_2D,0,0,0,
                                           self.width as u32,self.height as u32,
                                               format,consts::FLOAT,data);
        self.generate_mip_maps();
        Ok(())
    }

    pub(crate) fn generate_mip_maps(&self) {
        if self.number_of_mip_maps > 1 {
            self.gl.bind_texture(consts::TEXTURE_2D, &self.id);
            self.gl.generate_mipmap(consts::TEXTURE_2D);
        }
    }

    pub(crate) fn bind_as_color_target(&self, channel: usize)
    {
        self.gl.framebuffer_texture_2d(consts::FRAMEBUFFER,
                       consts::COLOR_ATTACHMENT0 + channel as u32, consts::TEXTURE_2D, &self.id, 0);
    }

    pub(crate) fn bind_as_depth_target(&self)
    {
        self.gl.framebuffer_texture_2d(consts::FRAMEBUFFER,
                       consts::DEPTH_ATTACHMENT, consts::TEXTURE_2D, &self.id, 0);
    }
}

impl Texture for Texture2D
{
    fn bind(&self, location: u32)
    {
        bind_at(&self.gl, &self.id, consts::TEXTURE_2D, location);
    }
}

impl Drop for Texture2D
{
    fn drop(&mut self)
    {
        self.gl.delete_texture(&self.id);
    }
}

pub struct TextureCubeMap {
    gl: Gl,
    id: crate::gl::Texture,
    pub width: usize,
    pub height: usize,
    format: Format,
    number_of_mip_maps: u32
}

impl TextureCubeMap
{
    pub fn new(gl: &Gl, width: usize, height: usize, min_filter: Interpolation, mag_filter: Interpolation, mip_map_filter: Option<Interpolation>,
           wrap_s: Wrapping, wrap_t: Wrapping, wrap_r: Wrapping, format: Format) -> Result<TextureCubeMap, Error>
    {
        let id = generate(gl)?;
        let number_of_mip_maps = calculate_number_of_mip_maps(mip_map_filter, width, height, 1);
        set_parameters(gl, &id,consts::TEXTURE_CUBE_MAP, min_filter, mag_filter,
                       if number_of_mip_maps == 1 {None} else {mip_map_filter}, wrap_s, wrap_t, Some(wrap_r));
        gl.bind_texture(consts::TEXTURE_CUBE_MAP, &id);
        gl.tex_storage_2d(consts::TEXTURE_CUBE_MAP,
                    number_of_mip_maps,
                    format as u32,
                    width as u32,
                    height as u32);
        Ok(Self { gl: gl.clone(), id, width, height, format, number_of_mip_maps })
    }

    #[cfg(feature = "image-io")]
    pub fn new_from_bytes(gl: &Gl, min_filter: Interpolation, mag_filter: Interpolation, mip_map_filter: Option<Interpolation>,
           wrap_s: Wrapping, wrap_t: Wrapping, wrap_r: Wrapping,
                      back_bytes: &[u8], front_bytes: &[u8], top_bytes: &[u8], left_bytes: &[u8], right_bytes: &[u8]) -> Result<TextureCubeMap, Error>
    {
        use image::GenericImageView;
        let back = image::load_from_memory(back_bytes)?;
        let front = image::load_from_memory(front_bytes)?;
        let top = image::load_from_memory(top_bytes)?;
        let left = image::load_from_memory(left_bytes)?;
        let right = image::load_from_memory(right_bytes)?;
        let (width, height) = back.dimensions();
        Self::new_with_u8(gl, min_filter, mag_filter, mip_map_filter, wrap_s, wrap_t, wrap_r, width, height, [&right.raw_pixels(), &left.raw_pixels(), &top.raw_pixels(),
                              &top.raw_pixels(), &front.raw_pixels(), &back.raw_pixels()])
    }

    #[cfg(all(not(target_arch = "wasm32"), feature = "image-io"))]
    pub fn new_from_files(gl: &Gl, min_filter: Interpolation, mag_filter: Interpolation, mip_map_filter: Option<Interpolation>,
           wrap_s: Wrapping, wrap_t: Wrapping, wrap_r: Wrapping, path: &str, back_name: &str, front_name: &str, top_name: &str, left_name: &str, right_name: &str) -> Result<TextureCubeMap, Error>
    {
        use image::GenericImageView;
        let back = image::open(format!("{}{}", path, back_name))?;
        let front = image::open(format!("{}{}", path, front_name))?;
        let top = image::open(format!("{}{}", path, top_name))?;
        let left = image::open(format!("{}{}", path, left_name))?;
        let right = image::open(format!("{}{}", path, right_name))?;
        let (width, height) = back.dimensions();
        Self::new_with_u8(gl, min_filter, mag_filter, mip_map_filter, wrap_s, wrap_t, wrap_r, width, height, [&right.raw_pixels(), &left.raw_pixels(), &top.raw_pixels(),
                              &top.raw_pixels(), &front.raw_pixels(), &back.raw_pixels()])
    }

    pub fn new_with_u8(gl: &Gl, min_filter: Interpolation, mag_filter: Interpolation, mip_map_filter: Option<Interpolation>,
           wrap_s: Wrapping, wrap_t: Wrapping, wrap_r: Wrapping, width: u32, height: u32, data: [&[u8]; 6]) -> Result<Self, Error>
    {
        let number_of_channels = data[0].len() as i32 / (width as i32 * height as i32);
        let format = match number_of_channels {
            1 => Ok(Format::R8),
            3 => Ok(Format::RGB8),
            4 => Ok(Format::RGBA8),
            _ => Err(Error::FailedToCreateTexture {message: format!("Unsupported texture format")})
        }?;

        let mut texture = Self::new(gl, width as usize, height as usize,
            min_filter, mag_filter, mip_map_filter, wrap_s, wrap_t, wrap_r, format)?;
        texture.fill_with_u8(data)?;
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
        self.gl.bind_texture(consts::TEXTURE_CUBE_MAP, &self.id);
        for i in 0..6 {
            self.gl.tex_sub_image_2d_with_u8_data(consts::TEXTURE_CUBE_MAP_POSITIVE_X + i as u32, 0, 0, 0,
                                                  self.width as u32, self.height as u32,
                                                  format, consts::UNSIGNED_BYTE, data[i]);
        }
        self.generate_mip_maps();
        Ok(())
    }

    pub(crate) fn generate_mip_maps(&self) {
        if self.number_of_mip_maps > 1 {
            self.gl.bind_texture(consts::TEXTURE_CUBE_MAP, &self.id);
            self.gl.generate_mipmap(consts::TEXTURE_CUBE_MAP);
        }
    }
}

impl Texture for TextureCubeMap
{
    fn bind(&self, location: u32)
    {
        bind_at(&self.gl, &self.id, consts::TEXTURE_CUBE_MAP, location);
    }
}

impl Drop for TextureCubeMap
{
    fn drop(&mut self)
    {
        self.gl.delete_texture(&self.id);
    }
}

pub struct Texture2DArray {
    gl: Gl,
    id: crate::gl::Texture,
    pub width: usize,
    pub height: usize,
    pub depth: usize,
    number_of_mip_maps: u32
}

impl Texture2DArray
{
    pub fn new(gl: &Gl, width: usize, height: usize, depth: usize, min_filter: Interpolation, mag_filter: Interpolation, mip_map_filter: Option<Interpolation>,
           wrap_s: Wrapping, wrap_t: Wrapping, format: Format) -> Result<Self, Error>
    {
        let id = generate(gl)?;
        let number_of_mip_maps = calculate_number_of_mip_maps(mip_map_filter, width, height, depth);
        set_parameters(gl, &id,consts::TEXTURE_2D_ARRAY, min_filter, mag_filter, if number_of_mip_maps == 1 {None} else {mip_map_filter}, wrap_s, wrap_t, None);
        gl.bind_texture(consts::TEXTURE_2D_ARRAY, &id);
        gl.tex_storage_3d(consts::TEXTURE_2D_ARRAY,
                        number_of_mip_maps,
                        format as u32,
                        width as u32,
                        height as u32,
                        depth as u32);
        Ok(Self { gl: gl.clone(), id, width, height, depth, number_of_mip_maps })
    }

    pub(crate) fn generate_mip_maps(&self) {
        if self.number_of_mip_maps > 1 {
            self.gl.bind_texture(consts::TEXTURE_2D_ARRAY, &self.id);
            self.gl.generate_mipmap(consts::TEXTURE_2D_ARRAY);
        }
    }

    pub(crate) fn bind_as_color_target(&self, layer: usize, channel: usize)
    {
        self.gl.framebuffer_texture_layer(consts::DRAW_FRAMEBUFFER,
                      consts::COLOR_ATTACHMENT0 + channel as u32, &self.id, 0, layer as u32);
    }

    pub(crate) fn bind_as_depth_target(&self, layer: usize)
    {
        self.gl.framebuffer_texture_layer(consts::DRAW_FRAMEBUFFER,
                       consts::DEPTH_ATTACHMENT, &self.id, 0, layer as u32);
    }
}

impl Texture for Texture2DArray
{
    fn bind(&self, location: u32)
    {
        bind_at(&self.gl, &self.id, consts::TEXTURE_2D_ARRAY, location);
    }
}

impl Drop for Texture2DArray
{
    fn drop(&mut self)
    {
        self.gl.delete_texture(&self.id);
    }
}


// COMMON FUNCTIONS
fn generate(gl: &Gl) -> Result<crate::gl::Texture, Error>
{
    gl.create_texture().ok_or_else(|| Error::FailedToCreateTexture {message: "Failed to create texture".to_string()} )
}

fn bind_at(gl: &Gl, id: &crate::gl::Texture, target: u32, location: u32)
{
    gl.active_texture(consts::TEXTURE0 + location);
    gl.bind_texture(target, id);
}

fn set_parameters(gl: &Gl, id: &crate::gl::Texture, target: u32, min_filter: Interpolation, mag_filter: Interpolation, mip_map_filter: Option<Interpolation>, wrap_s: Wrapping, wrap_t: Wrapping, wrap_r: Option<Wrapping>)
{
    gl.bind_texture(target, id);
    match mip_map_filter {
        None => gl.tex_parameteri(target, consts::TEXTURE_MIN_FILTER, min_filter as i32),
        Some(Interpolation::Nearest) =>
            if min_filter == Interpolation::Nearest {
                gl.tex_parameteri(target, consts::TEXTURE_MIN_FILTER, consts::NEAREST_MIPMAP_NEAREST as i32);
            } else {
                gl.tex_parameteri(target, consts::TEXTURE_MIN_FILTER, consts::LINEAR_MIPMAP_NEAREST as i32)
            },
        Some(Interpolation::Linear) =>
            if min_filter == Interpolation::Nearest {
                gl.tex_parameteri(target, consts::TEXTURE_MIN_FILTER, consts::NEAREST_MIPMAP_LINEAR as i32);
            } else {
                gl.tex_parameteri(target, consts::TEXTURE_MIN_FILTER, consts::LINEAR_MIPMAP_LINEAR as i32)
            }
    }
    gl.tex_parameteri(target, consts::TEXTURE_MAG_FILTER, mag_filter as i32);
    gl.tex_parameteri(target, consts::TEXTURE_WRAP_S, wrap_s as i32);
    gl.tex_parameteri(target, consts::TEXTURE_WRAP_T, wrap_t as i32);
    if let Some(r) = wrap_r {
        gl.tex_parameteri(target, consts::TEXTURE_WRAP_R, r as i32);
    }
}

fn calculate_number_of_mip_maps(mip_map_filter: Option<Interpolation>, width: usize, height: usize, depth: usize) -> u32 {
    if mip_map_filter.is_some() {
            let w = (width as f64).log2().ceil();
            let h = (height as f64).log2().ceil();
            let d = (depth as f64).log2().ceil();
            w.max(h).max(d) as u32 + 1
        } else {1}
}