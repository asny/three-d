use gl;
use image;
use image::GenericImage;

#[derive(Debug)]
pub enum Error {
    Image(image::ImageError),
    IO(std::io::Error),
    FailedToCreateTexture {message: String}
}

impl From<image::ImageError> for Error {
    fn from(other: image::ImageError) -> Self {
        Error::Image(other)
    }
}

impl From<std::io::Error> for Error {
    fn from(other: std::io::Error) -> Self {
        Error::IO(other)
    }
}

pub trait Texture {
    fn bind(&self, location: u32);
}

pub struct Texture2D {
    gl: gl::Gl,
    id: gl::Texture,
    target: u32
}

// TEXTURE 2D
impl Texture2D
{
    pub fn new(gl: &gl::Gl) -> Result<Texture2D, Error>
    {
        let id = generate(gl)?;
        let texture = Texture2D { gl: gl.clone(), id, target: gl::bindings::TEXTURE_2D };

        bind(&texture.gl, &texture.id, texture.target);
        gl.tex_parameteri(texture.target, gl::bindings::TEXTURE_MIN_FILTER, gl::bindings::LINEAR as i32);
        gl.tex_parameteri(texture.target, gl::bindings::TEXTURE_MAG_FILTER, gl::bindings::LINEAR as i32);
        gl.tex_parameteri(texture.target, gl::bindings::TEXTURE_WRAP_S, gl::bindings::REPEAT as i32);
        gl.tex_parameteri(texture.target, gl::bindings::TEXTURE_WRAP_T, gl::bindings::REPEAT as i32);

        Ok(texture)
    }

    pub fn new_from_file(gl: &gl::Gl, path: &str) -> Result<Texture2D, Error>
    {
        let img = image::open(path)?;
        let mut texture = Texture2D::new(gl)?;
        texture.fill_with_u8(img.dimensions().0 as usize, img.dimensions().1 as usize, &mut img.raw_pixels());
        Ok(texture)
    }

    pub fn new_as_color_target(gl: &gl::Gl, width: usize, height: usize, channel: u32) -> Result<Texture2D, Error>
    {
        let id = generate(gl)?;
        let texture = Texture2D { gl: gl.clone(), id, target: gl::bindings::TEXTURE_2D };

        bind(&texture.gl, &texture.id, texture.target);
        gl.tex_parameteri(texture.target, gl::bindings::TEXTURE_MIN_FILTER, gl::bindings::NEAREST as i32);
        gl.tex_parameteri(texture.target, gl::bindings::TEXTURE_MAG_FILTER, gl::bindings::NEAREST as i32);
        gl.tex_parameteri(texture.target, gl::bindings::TEXTURE_WRAP_S, gl::bindings::CLAMP_TO_EDGE as i32);
        gl.tex_parameteri(texture.target, gl::bindings::TEXTURE_WRAP_T, gl::bindings::CLAMP_TO_EDGE as i32);

        gl.tex_image_2d(texture.target,
                             0,
                             gl::bindings::RGBA32F,
                             width as u32,
                             height as u32,
                             0,
                             gl::bindings::RGBA,
                             gl::bindings::FLOAT);
        gl.framebuffer_texture_2d(gl::bindings::FRAMEBUFFER, gl::bindings::COLOR_ATTACHMENT0 + channel, gl::bindings::TEXTURE_2D, &texture.id, 0);

        Ok(texture)
    }

    pub fn new_as_depth_target(gl: &gl::Gl, width: usize, height: usize) -> Result<Texture2D, Error>
    {
        let id = generate(gl)?;
        let texture = Texture2D { gl: gl.clone(), id, target: gl::bindings::TEXTURE_2D };

        bind(&texture.gl, &texture.id, texture.target);
        gl.tex_parameteri(texture.target, gl::bindings::TEXTURE_MIN_FILTER, gl::bindings::NEAREST as i32);
        gl.tex_parameteri(texture.target, gl::bindings::TEXTURE_MAG_FILTER, gl::bindings::NEAREST as i32);
        gl.tex_parameteri(texture.target, gl::bindings::TEXTURE_WRAP_S, gl::bindings::CLAMP_TO_EDGE as i32);
        gl.tex_parameteri(texture.target, gl::bindings::TEXTURE_WRAP_T, gl::bindings::CLAMP_TO_EDGE as i32);

        gl.tex_image_2d(texture.target,
                          0,
                          gl::bindings::DEPTH_COMPONENT32F,
                          width as u32,
                          height as u32,
                          0,
                          gl::bindings::DEPTH_COMPONENT,
                          gl::bindings::FLOAT);

        gl.framebuffer_texture_2d(gl::bindings::FRAMEBUFFER, gl::bindings::DEPTH_ATTACHMENT, gl::bindings::TEXTURE_2D, &texture.id, 0);

        Ok(texture)
    }

    pub fn fill_with_u8(&mut self, width: usize, height: usize, data: &[u8])
    {
        let mut d = extend_data(data, width * height, 0);
        bind(&self.gl, &self.id, self.target);
        self.gl.tex_image_2d_with_data(self.target,
                             0,
                             gl::bindings::RGB8,
                             width as u32,
                             height as u32,
                             0,
                             gl::bindings::RGB,
                             gl::bindings::UNSIGNED_BYTE,
                             &mut d);
    }

    pub fn fill_with_f32(&mut self, width: usize, height: usize, data: &[f32])
    {
        let no_elements = 1;
        let mut d = extend_data(data, width * height, 0.0);
        bind(&self.gl, &self.id, self.target);
        let format = if no_elements == 1 {gl::bindings::RED} else {gl::bindings::RGB};
        let internal_format = if no_elements == 1 {gl::bindings::R32F} else {gl::bindings::RGB32F};
        self.gl.tex_image_2d_with_data(self.target,
                             0,
                             internal_format,
                             width as u32,
                             height as u32,
                             0,
                             format,
                             gl::bindings::FLOAT,
                             &mut d);
    }

    pub fn get_pixels(&self, width: usize, height: usize) -> Vec<u8>
    {
        let pixels = vec![0.0 as f32; width * height * 3];
        bind(&self.gl, &self.id, self.target);
        unsafe {
            self.gl.GetTexImage(self.target, 0, gl::bindings::RGB, gl::bindings::FLOAT, pixels.as_ptr() as *mut gl::bindings::types::GLvoid);
        }
        pixels.iter().map(|x| (*x * 255.0) as u8).collect()
    }

    pub fn save_as_file(&self, path: &str, width: usize, height: usize) -> Result<(), Error>
    {
        let pixels = self.get_pixels(width, height);
        image::save_buffer(&std::path::Path::new(path), &pixels, width as u32, height as u32, image::RGB(8))?;
        Ok(())
    }

}

impl Texture for Texture2D
{
    fn bind(&self, location: u32)
    {
        bind_at(&self.gl, &self.id, self.target, location);
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
    gl: gl::Gl,
    id: gl::Texture,
    target: u32
}

// TEXTURE 3D
impl Texture3D
{
    pub fn new(gl: &gl::Gl) -> Result<Texture3D, Error>
    {
        let id = generate(gl)?;
        let texture = Texture3D { gl: gl.clone(), id, target: gl::bindings::TEXTURE_CUBE_MAP };

        bind(&texture.gl, &texture.id, texture.target);
        gl.tex_parameteri(texture.target, gl::bindings::TEXTURE_MIN_FILTER, gl::bindings::LINEAR as i32);
        gl.tex_parameteri(texture.target, gl::bindings::TEXTURE_MAG_FILTER, gl::bindings::LINEAR as i32);
        gl.tex_parameteri(texture.target, gl::bindings::TEXTURE_WRAP_S, gl::bindings::CLAMP_TO_EDGE as i32);
        gl.tex_parameteri(texture.target, gl::bindings::TEXTURE_WRAP_T, gl::bindings::CLAMP_TO_EDGE as i32);
        gl.tex_parameteri(texture.target, gl::bindings::TEXTURE_WRAP_R, gl::bindings::CLAMP_TO_EDGE as i32);

        Ok(texture)
    }

    pub fn new_from_files(gl: &gl::Gl, path: &str, back_name: &str, front_name: &str, top_name: &str, left_name: &str, right_name: &str) -> Result<Texture3D, Error>
    {
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
            let format = gl::bindings::RGB;
            let internal_format = gl::bindings::RGB8;
            self.gl.tex_image_2d_with_data(gl::bindings::TEXTURE_CUBE_MAP_POSITIVE_X + i as u32,
                                           0,
                                           internal_format,
                                           width as u32,
                                           height as u32,
                                           0,
                                           format,
                                           gl::bindings::UNSIGNED_BYTE,
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

// COMMON FUNCTIONS
fn generate(gl: &gl::Gl) -> Result<gl::Texture, Error>
{
    gl.create_texture().ok_or_else(|| Error::FailedToCreateTexture {message: "Failed to create texture".to_string()} )
}

fn bind_at(gl: &gl::Gl, id: &gl::Texture, target: u32, location: u32)
{
    gl.active_texture(gl::bindings::TEXTURE0 + location);
    bind(gl, id, target);
}

fn bind(gl: &gl::Gl, id: &gl::Texture, target: u32)
{
    gl.bind_texture(target, id)
}

fn drop(gl: &gl::Gl, id: &gl::Texture)
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