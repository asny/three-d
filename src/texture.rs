use gl;
use std;

#[derive(Debug)]
pub enum Error {

}

pub struct Texture {
    gl: gl::Gl,
    id: u32,
    target: u32
}

impl Texture
{
    pub fn create(gl: &gl::Gl) -> Result<Texture, Error>
    {
        let mut id: u32 = 0;
        unsafe {
            gl.GenTextures(1, &mut id);
        }
        let texture = Texture{ gl: gl.clone(), id, target: gl::TEXTURE_2D };
        Ok(texture)
    }

    fn bind(&self)
    {
        unsafe {
            self.gl.BindTexture(self.target, self.id);
        }
    }

    pub fn fill_with(&mut self, data: &Vec<f32>, width: usize, height: usize, no_elements: usize)
    {
        let d = Texture::extend_data(data, width * height, 0.0);
        self.bind();
        unsafe {
            let format = if no_elements == 1 {gl::RED} else {gl::RGB};
            let internal_format = if no_elements == 1 {gl::R32F} else {gl::RGB32F};
            self.gl.TexImage2D(self.target,
                             0,
                             internal_format as i32,
                             width as i32,
                             height as i32,
                             0,
                             format,
                             gl::FLOAT,
                             d.as_ptr() as *const gl::types::GLvoid);

            self.gl.TexParameteri(self.target, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            self.gl.TexParameteri(self.target, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            self.gl.TexParameteri(self.target, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            self.gl.TexParameteri(self.target, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        }
    }

    pub fn fill_with_int(&mut self, data: &Vec<u32>, width: usize, height: usize)
    {
        let d = Texture::extend_data(data, width * height, 0);
        self.bind();
        unsafe {
            self.gl.TexImage2D(self.target,
                             0,
                             gl::R32F as i32,
                             width as i32,
                             height as i32,
                             0,
                             gl::RED,
                             gl::FLOAT,
                             d.as_ptr() as *const gl::types::GLvoid);

            self.gl.TexParameteri(self.target, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            self.gl.TexParameteri(self.target, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            self.gl.TexParameteri(self.target, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            self.gl.TexParameteri(self.target, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        }
    }

    pub fn bind_at(&self, location: u32)
    {
        unsafe {
            self.gl.ActiveTexture(gl::TEXTURE0 + location);
        }
        self.bind();
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
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteTextures(1, &self.id);
        }
    }
}