use gl;
use crate::loader;

#[derive(Debug)]
pub enum Error {
    Loader(loader::Error),
    UnknownShaderType {message: String},
    FailedToCompileShader {name: String, message: String}
}

impl From<loader::Error> for Error {
    fn from(other: loader::Error) -> Self {
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
        const POSSIBLE_EXT: [(&str, u32); 2] = [
            (".vert", gl::consts::VERTEX_SHADER),
            (".frag", gl::consts::FRAGMENT_SHADER),
        ];

        let shader_kind = POSSIBLE_EXT.iter()
            .find(|&&(file_extension, _)| {
                name.ends_with(file_extension)
            })
            .map(|&(_, kind)| kind)
            .ok_or_else(|| Error::UnknownShaderType {message: format!("Can not determine shader type for resource {:?}", name) })?;

        let source = loader::load_string(name)?;

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