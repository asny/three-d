use gl;

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

    pub fn fill_with(&mut self, data: &Vec<f32>, width: u32, height: u32)
    {
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
                             data.as_ptr() as *const gl::types::GLvoid);

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
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteTextures(1, &self.id);
        }
    }
}