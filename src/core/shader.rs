use gl;

#[derive(Debug)]
pub enum Error {
    UnknownShaderType {message: String},
    FailedToCreateShader {shader_type: String, message: String},
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
        let shader_type = if kind == gl::consts::VERTEX_SHADER {"Vertex shader".to_string()} else {"Fragment shader".to_string()};
        let shader = gl.create_shader(kind).ok_or(Error::FailedToCreateShader{ shader_type: shader_type.clone(), message:"Unable to create shader object".to_string() })?;
        gl.compile_shader(src, &shader).map_err(|e| Error::FailedToCompileShader { shader_type, message: e })?;
        Ok(Shader {gl: gl.clone(), id: shader})
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