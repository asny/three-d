use gl;

#[derive(Debug)]
pub enum Error {
    Loader(io::Error),
    UnknownShaderType {message: String},
    FailedToCompileShader {name: String, message: String}
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::Loader(other)
    }
}

pub struct Shader {
    gl: gl::Gl,
    id: gl::Shader,
}

impl Shader
{
    pub fn from_resource(gl: &gl::Gl, name: &str) -> Result<Shader, Error>
    {
        let splitted: Vec<&str> = name.split('.').collect();
        let shader_kind = match splitted.last() {
            Some(&"vert") => {Ok(gl::consts::VERTEX_SHADER)},
            Some(&"frag") => {Ok(gl::consts::FRAGMENT_SHADER)},
            _ => {Err(Error::UnknownShaderType {message: format!("Can not determine shader type for resource {:?}", name)})}
        }?;

        let source = io::load_string(name)?;

        Shader::from_source(gl, &source, shader_kind, name)
    }

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