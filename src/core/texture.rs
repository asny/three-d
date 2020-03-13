use crate::core::Gl;
use crate::core::Error;

pub enum Interpolation {
    Nearest = gl::consts::NEAREST as isize,
    Linear = gl::consts::LINEAR as isize,
    NearestMipmapNearest = gl::consts::NEAREST_MIPMAP_NEAREST as isize,
    LinearMipmapNearest = gl::consts::LINEAR_MIPMAP_NEAREST as isize,
    NearestMipmapLinear = gl::consts::NEAREST_MIPMAP_LINEAR as isize,
    LinearMipmapLinear = gl::consts::LINEAR_MIPMAP_LINEAR as isize
}

pub enum Wrapping {
    Repeat = gl::consts::REPEAT as isize,
    MirroredRepeat = gl::consts::MIRRORED_REPEAT as isize,
    ClampToEdge = gl::consts::CLAMP_TO_EDGE as isize
}

pub enum Format {
    RGBA8 = gl::consts::RGBA8 as isize,
    Depth32F = gl::consts::DEPTH_COMPONENT32F as isize
}

pub trait Texture {
    fn bind(&self, location: u32);
}

pub struct Texture2D {
    gl: Gl,
    id: gl::Texture,
    pub width: usize,
    pub height: usize
}

// TEXTURE 2D
impl Texture2D
{
    pub fn new(gl: &Gl, width: usize, height: usize, min_filter: Interpolation, mag_filter: Interpolation,
           wrap_s: Wrapping, wrap_t: Wrapping) -> Result<Texture2D, Error>
    {
        let id = generate(gl)?;
        let texture = Texture2D { gl: gl.clone(), id, width, height };
        texture.set_interpolation(min_filter, mag_filter);
        texture.set_wrapping(wrap_s, wrap_t);
        Ok(texture)
    }

    pub fn new_empty(gl: &Gl, width: usize, height: usize, min_filter: Interpolation, mag_filter: Interpolation,
           wrap_s: Wrapping, wrap_t: Wrapping, format: Format) -> Result<Texture2D, Error>
    {
        let texture = Texture2D::new(gl, width, height, min_filter, mag_filter, wrap_s, wrap_t)?;
        gl.tex_storage_2d(gl::consts::TEXTURE_2D,
                        1,
                        format as u32,
                        width as u32,
                        height as u32);
        Ok(texture)
    }

    #[cfg(feature = "image-io")]
    pub fn new_from_bytes(gl: &Gl, bytes: &[u8]) -> Result<Texture2D, Error>
    {
        use image::GenericImageView;
        let img = image::load_from_memory(bytes)?;
        let mut texture = Texture2D::new(gl, img.dimensions().0 as usize, img.dimensions().1 as usize,
            Interpolation::Linear, Interpolation::Linear, Wrapping::ClampToEdge, Wrapping::ClampToEdge)?;
        texture.fill_with_u8(texture.width, texture.height, &mut img.raw_pixels());
        Ok(texture)
    }

    #[cfg(all(not(target_arch = "wasm32"), feature = "image-io"))]
    pub fn new_from_file(gl: &Gl, path: &str) -> Result<Texture2D, Error>
    {
        use image::GenericImageView;
        let img = image::open(path)?;
        let mut texture = Texture2D::new(gl, img.dimensions().0 as usize, img.dimensions().1 as usize,
        Interpolation::Linear, Interpolation::Linear, Wrapping::ClampToEdge, Wrapping::ClampToEdge)?;
        texture.fill_with_u8(texture.width, texture.height, &mut img.raw_pixels());
        Ok(texture)
    }
    pub fn set_interpolation(&self, min_filter: Interpolation, mag_filter: Interpolation)
    {
        bind(&self.gl, &self.id, gl::consts::TEXTURE_2D);
        self.gl.tex_parameteri(gl::consts::TEXTURE_2D, gl::consts::TEXTURE_MIN_FILTER, min_filter as i32);
        self.gl.tex_parameteri(gl::consts::TEXTURE_2D, gl::consts::TEXTURE_MAG_FILTER, mag_filter as i32);
    }

    pub fn set_wrapping(&self, wrap_s: Wrapping, wrap_t: Wrapping)
    {
        bind(&self.gl, &self.id, gl::consts::TEXTURE_2D);
        self.gl.tex_parameteri(gl::consts::TEXTURE_2D, gl::consts::TEXTURE_WRAP_S, wrap_s as i32);
        self.gl.tex_parameteri(gl::consts::TEXTURE_2D, gl::consts::TEXTURE_WRAP_T, wrap_t as i32);
    }

    pub fn fill_with_u8(&mut self, width: usize, height: usize, data: &[u8])
    {
        let mut d = extend_data(data, width * height, 0);
        bind(&self.gl, &self.id, gl::consts::TEXTURE_2D);
        self.gl.tex_image_2d_with_u8_data(gl::consts::TEXTURE_2D,
                                          0,
                                          gl::consts::RGB8,
                                          width as u32,
                                          height as u32,
                                          0,
                                          gl::consts::RGB,
                                          gl::consts::UNSIGNED_BYTE,
                                          &mut d);
    }

    pub fn fill_with_f32(&mut self, width: usize, height: usize, data: &[f32])
    {
        let no_elements = 1;
        let mut d = extend_data(data, width * height, 0.0);
        bind(&self.gl, &self.id, gl::consts::TEXTURE_2D);
        let format = if no_elements == 1 {gl::consts::RED} else {gl::consts::RGB};
        let internal_format = if no_elements == 1 {gl::consts::R16F} else {gl::consts::RGB16F};
        self.gl.tex_image_2d_with_f32_data(gl::consts::TEXTURE_2D,
                                           0,
                                           internal_format,
                                           width as u32,
                                           height as u32,
                                           0,
                                           format,
                                           gl::consts::FLOAT,
                                           &mut d);
    }

    pub fn bind_as_color_target(&self, channel: usize)
    {
        self.gl.framebuffer_texture_2d(gl::consts::FRAMEBUFFER,
                       gl::consts::COLOR_ATTACHMENT0 + channel as u32, gl::consts::TEXTURE_2D, &self.id, 0);
    }

    pub fn bind_as_depth_target(&self)
    {
        self.gl.framebuffer_texture_2d(gl::consts::FRAMEBUFFER,
                       gl::consts::DEPTH_ATTACHMENT, gl::consts::TEXTURE_2D, &self.id, 0);
    }
}

impl Texture for Texture2D
{
    fn bind(&self, location: u32)
    {
        bind_at(&self.gl, &self.id, gl::consts::TEXTURE_2D, location);
    }
}

impl Drop for Texture2D
{
    fn drop(&mut self)
    {
        drop(&self.gl, &self.id);
    }
}

pub struct Texture3D {
    gl: Gl,
    id: gl::Texture,
    target: u32
}

// TEXTURE 3D
impl Texture3D
{
    pub fn new(gl: &Gl) -> Result<Texture3D, Error>
    {
        let id = generate(gl)?;
        let texture = Texture3D { gl: gl.clone(), id, target: gl::consts::TEXTURE_CUBE_MAP };

        bind(&texture.gl, &texture.id, texture.target);
        gl.tex_parameteri(texture.target, gl::consts::TEXTURE_MIN_FILTER, gl::consts::LINEAR as i32);
        gl.tex_parameteri(texture.target, gl::consts::TEXTURE_MAG_FILTER, gl::consts::LINEAR as i32);
        gl.tex_parameteri(texture.target, gl::consts::TEXTURE_WRAP_S, gl::consts::CLAMP_TO_EDGE as i32);
        gl.tex_parameteri(texture.target, gl::consts::TEXTURE_WRAP_T, gl::consts::CLAMP_TO_EDGE as i32);
        gl.tex_parameteri(texture.target, gl::consts::TEXTURE_WRAP_R, gl::consts::CLAMP_TO_EDGE as i32);

        Ok(texture)
    }

    #[cfg(feature = "image-io")]
    pub fn new_from_bytes(gl: &Gl, back_bytes: &[u8], front_bytes: &[u8], top_bytes: &[u8], left_bytes: &[u8], right_bytes: &[u8]) -> Result<Texture3D, Error>
    {
        use image::GenericImageView;
        let back = image::load_from_memory(back_bytes)?;
        let front = image::load_from_memory(front_bytes)?;
        let top = image::load_from_memory(top_bytes)?;
        let left = image::load_from_memory(left_bytes)?;
        let right = image::load_from_memory(right_bytes)?;

        let mut texture = Texture3D::new(gl)?;
        texture.fill_with_u8(back.dimensions().0 as usize, back.dimensions().1 as usize,
                             [&mut right.raw_pixels(), &mut left.raw_pixels(), &mut top.raw_pixels(),
                              &mut top.raw_pixels(), &mut front.raw_pixels(), &mut back.raw_pixels()]);
        Ok(texture)
    }

    #[cfg(all(not(target_arch = "wasm32"), feature = "image-io"))]
    pub fn new_from_files(gl: &Gl, path: &str, back_name: &str, front_name: &str, top_name: &str, left_name: &str, right_name: &str) -> Result<Texture3D, Error>
    {
        use image::GenericImageView;
        let back = image::open(format!("{}{}", path, back_name))?;
        let front = image::open(format!("{}{}", path, front_name))?;
        let top = image::open(format!("{}{}", path, top_name))?;
        let left = image::open(format!("{}{}", path, left_name))?;
        let right = image::open(format!("{}{}", path, right_name))?;

        let mut texture = Texture3D::new(gl)?;
        texture.fill_with_u8(back.dimensions().0 as usize, back.dimensions().1 as usize,
                             [&mut right.raw_pixels(), &mut left.raw_pixels(), &mut top.raw_pixels(),
                              &mut top.raw_pixels(), &mut front.raw_pixels(), &mut back.raw_pixels()]);
        Ok(texture)
    }

    pub fn fill_with_u8(&mut self, width: usize, height: usize, data: [&mut [u8]; 6])
    {
        bind(&self.gl, &self.id, self.target);
        for i in 0..6 {
            let format = gl::consts::RGB;
            let internal_format = gl::consts::RGB8;
            self.gl.tex_image_2d_with_u8_data(gl::consts::TEXTURE_CUBE_MAP_POSITIVE_X + i as u32,
                                              0,
                                              internal_format,
                                              width as u32,
                                              height as u32,
                                              0,
                                              format,
                                              gl::consts::UNSIGNED_BYTE,
                                              data[i]);
        }
    }
}

impl Texture for Texture3D
{
    fn bind(&self, location: u32)
    {
        bind_at(&self.gl, &self.id, self.target, location);
    }
}

impl Drop for Texture3D
{
    fn drop(&mut self)
    {
        drop(&self.gl, &self.id);
    }
}

pub struct Texture2DArray {
    gl: Gl,
    id: gl::Texture,
    target: u32,
    attachment: u32,
    pub width: usize,
    pub height: usize,
    pub depth: usize,
}

// TEXTURE 3D
impl Texture2DArray
{
    pub fn new_as_color_targets(gl: &Gl, width: usize, height: usize, depth: usize) -> Result<Texture2DArray, Error>
    {
        let id = generate(gl)?;
        let texture = Texture2DArray { gl: gl.clone(), id, target: gl::consts::TEXTURE_2D_ARRAY, attachment: gl::consts::COLOR_ATTACHMENT0,
            width, height, depth};

        bind(&texture.gl, &texture.id, texture.target);
        gl.tex_parameteri(texture.target, gl::consts::TEXTURE_MIN_FILTER, gl::consts::NEAREST as i32);
        gl.tex_parameteri(texture.target, gl::consts::TEXTURE_MAG_FILTER, gl::consts::NEAREST as i32);
        gl.tex_parameteri(texture.target, gl::consts::TEXTURE_WRAP_S, gl::consts::CLAMP_TO_EDGE as i32);
        gl.tex_parameteri(texture.target, gl::consts::TEXTURE_WRAP_T, gl::consts::CLAMP_TO_EDGE as i32);

        gl.tex_storage_3d(texture.target,
                        1,
                        gl::consts::RGBA8,
                        width as u32,
                        height as u32,
                        depth as u32);

        Ok(texture)
    }

    pub fn new_as_depth_targets(gl: &Gl, width: usize, height: usize, depth: usize) -> Result<Texture2DArray, Error>
    {
        let id = generate(gl)?;
        let texture = Texture2DArray { gl: gl.clone(), id, target: gl::consts::TEXTURE_2D_ARRAY, attachment: gl::consts::DEPTH_ATTACHMENT,
            width, height, depth };

        bind(&texture.gl, &texture.id, texture.target);
        gl.tex_parameteri(texture.target, gl::consts::TEXTURE_MIN_FILTER, gl::consts::NEAREST as i32);
        gl.tex_parameteri(texture.target, gl::consts::TEXTURE_MAG_FILTER, gl::consts::NEAREST as i32);
        gl.tex_parameteri(texture.target, gl::consts::TEXTURE_WRAP_S, gl::consts::CLAMP_TO_EDGE as i32);
        gl.tex_parameteri(texture.target, gl::consts::TEXTURE_WRAP_T, gl::consts::CLAMP_TO_EDGE as i32);

        gl.tex_storage_3d(texture.target,
                        1,
                        gl::consts::DEPTH_COMPONENT32F,
                        width as u32,
                        height as u32,
                        depth as u32);

        Ok(texture)
    }

    pub fn bind_as_color_target(&self, layer: usize, channel: usize)
    {
        self.gl.framebuffer_texture_layer(gl::consts::DRAW_FRAMEBUFFER,
                      self.attachment + channel as u32, &self.id, 0, layer as u32);
    }

    pub fn bind_as_depth_target(&self, layer: usize)
    {
        self.gl.framebuffer_texture_layer(gl::consts::DRAW_FRAMEBUFFER,
                       gl::consts::DEPTH_ATTACHMENT, &self.id, 0, layer as u32);
    }
}

impl Texture for Texture2DArray
{
    fn bind(&self, location: u32)
    {
        bind_at(&self.gl, &self.id, self.target, location);
    }
}

impl Drop for Texture2DArray
{
    fn drop(&mut self)
    {
        drop(&self.gl, &self.id);
    }
}


// COMMON FUNCTIONS
fn generate(gl: &Gl) -> Result<gl::Texture, Error>
{
    gl.create_texture().ok_or_else(|| Error::FailedToCreateTexture {message: "Failed to create texture".to_string()} )
}

fn bind_at(gl: &Gl, id: &gl::Texture, target: u32, location: u32)
{
    gl.active_texture(gl::consts::TEXTURE0 + location);
    bind(gl, id, target);
}

fn bind(gl: &Gl, id: &gl::Texture, target: u32)
{
    gl.bind_texture(target, id)
}

fn drop(gl: &Gl, id: &gl::Texture)
{
    gl.delete_texture(id);
}

fn extend_data<T>(data: &[T], desired_length: usize, value: T) -> Vec<T> where T: std::clone::Clone
{
    let mut result = Vec::new();
    result.extend_from_slice(data);
    if data.len() < desired_length
    {
        result.extend(std::iter::repeat(value).take(desired_length - data.len()));
    }
    result
}