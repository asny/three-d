use gl;

#[derive(Debug)]
pub enum Error {
    UnknownShaderType {message: String},
    FailedToCompileShader {shader_type: String, message: String}
}

pub struct Shader {
    gl: gl::Gl,
    id: gl::Shader,
}

impl Shader
{
    pub fn from_source(gl: &gl::Gl, src: &str, kind: u32) -> Result<Shader, Error>
    {
        match gl::shader_from_source(gl, src, kind) {
            Ok(shader) => Ok(Shader {gl: gl.clone(), id: shader}),
            Err(message) => Err(Error::FailedToCompileShader {
                shader_type: if kind == gl::consts::VERTEX_SHADER {"Vertex shader".to_string()} else {"Fragment shader".to_string()}, message})
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