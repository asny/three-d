use gl;

#[derive(Debug)]
pub enum Error {
    UnknownShaderType {message: String},
    FailedToCompileShader {name: String, message: String}
}

pub struct Shader {
    gl: gl::Gl,
    id: gl::Shader,
}

impl Shader
{
    pub fn from_source(gl: &gl::Gl, source: &str, kind: u32, name: &str) -> Result<Shader, Error>
    {
        match gl::shader_from_source(gl, source, kind) {
            Ok(shader) => Ok(Shader {gl: gl.clone(), id: shader}),
            Err(message) => Err(Error::FailedToCompileShader {name: name.to_string(), message})
        }
    }

    pub fn attach_shader(&self, program: &gl::Program)
    {
        self.gl.attach_shader(program, &self.id);
    }

    pub fn detach_shader(&self, program: &gl::Program)
    {
        self.gl.detach_shader(program, &self.id);
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        self.gl.delete_shader(Some(&self.id));
    }
}