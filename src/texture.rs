use gl;
use std;

#[derive(Debug)]
pub enum Error {

}


pub trait Texture {
    fn bind(&self, location: u32);
}

pub struct Texture2D {
    gl: gl::Gl,
    id: u32,
    target: u32,
    width: usize,
    height: usize
}

// TEXTURE 2D
impl Texture2D
{
    pub fn create(gl: &gl::Gl, width: usize, height: usize) -> Result<Texture2D, Error>
    {
        let id = generate(gl)?;
        let texture = Texture2D { gl: gl.clone(), id, target: gl::TEXTURE_2D, width, height };

        bind(&texture.gl, texture.id, texture.target);
        unsafe {
            gl.TexParameteri(texture.target, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl.TexParameteri(texture.target, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl.TexParameteri(texture.target, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl.TexParameteri(texture.target, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        }

        Ok(texture)
    }

    pub fn fill_with(&mut self, data: &Vec<f32>)
    {
        let no_elements = 1;
        let d = extend_data(data, self.width * self.height, 0.0);
        bind(&self.gl, self.id, self.target);
        unsafe {
            let format = if no_elements == 1 {gl::RED} else {gl::RGB};
            let internal_format = if no_elements == 1 {gl::R32F} else {gl::RGB32F};
            self.gl.TexImage2D(self.target,
                             0,
                             internal_format as i32,
                             self.width as i32,
                             self.height as i32,
                             0,
                             format,
                             gl::FLOAT,
                             d.as_ptr() as *const gl::types::GLvoid);
        }
    }
}

impl Texture for Texture2D
{
    fn bind(&self, location: u32)
    {
        bind_at(&self.gl, self.id, self.target, location);
    }
}

impl Drop for Texture2D
{
    fn drop(&mut self)
    {
        drop(&self.gl, &self.id);
    }
}

// COMMON FUNCTIONS
fn generate(gl: &gl::Gl) -> Result<u32, Error>
{
    let mut id: u32 = 0;
    unsafe {
        gl.GenTextures(1, &mut id);
    }
    Ok(id)
}

fn bind_at(gl: &gl::Gl, id: u32, target: u32, location: u32)
{
    unsafe {
        gl.ActiveTexture(gl::TEXTURE0 + location);
    }
    bind(gl, id, target);
}

fn bind(gl: &gl::Gl, id: u32, target: u32)
{
    unsafe {
        gl.BindTexture(target, id);
    }
}

fn drop(gl: &gl::Gl, id: &u32)
{
    unsafe {
        gl.DeleteTextures(1, id);
    }
}

fn extend_data<T>(data: &Vec<T>, desired_length: usize, value: T) -> Vec<T> where T: std::clone::Clone
{
    let mut d = data.clone();
    if d.len() < desired_length
    {
        use std::iter;
        let mut fill = Vec::new();
        fill.extend(iter::repeat(value).take(desired_length - data.len()));
        d.append(&mut fill);
    }
    d
}