use gl;
use std;
use utility;
use loader;

#[derive(Debug)]
pub enum Error {
    Loader(loader::Error),
    FailedToConvertToCString(std::ffi::NulError),
    UnknownShaderType {message: String},
    FailedToCompileShader {name: String, message: String}
}

impl From<loader::Error> for Error {
    fn from(other: loader::Error) -> Self {
        Error::Loader(other)
    }
}

impl From<std::ffi::NulError> for Error {
    fn from(other: std::ffi::NulError) -> Self {
        Error::FailedToConvertToCString(other)
    }
}

pub struct Shader {
    gl: gl::Gl,
    id: gl::types::GLuint,
}

impl Shader
{
    pub fn from_resource(gl: &gl::Gl, name: &str) -> Result<Shader, Error>
    {
        const POSSIBLE_EXT: [(&str, gl::types::GLenum); 2] = [
            (".vert", gl::VERTEX_SHADER),
            (".frag", gl::FRAGMENT_SHADER),
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

    pub fn from_source(gl: &gl::Gl, source: &str, kind: gl::types::GLenum, name: &str) -> Result<Shader, Error>
    {
        #[cfg(not(target_os = "emscripten"))]
        let header = "#version 330 core\nprecision mediump float;\n";
        #[cfg(target_os = "emscripten")]
        let header = "#version 300 es\nprecision mediump float;\n";

        let s: &str = &[header, source].concat();

        let id = shader_from_source(gl, s, kind, name)?;
        Ok(Shader { gl: gl.clone(), id })
    }

    pub fn from_vert_source(gl: &gl::Gl, source: &str) -> Result<Shader, Error> {
        Shader::from_source(gl, source, gl::VERTEX_SHADER, "")
    }

    pub fn from_frag_source(gl: &gl::Gl, source: &str) -> Result<Shader, Error> {
        Shader::from_source(gl, source, gl::FRAGMENT_SHADER, "")
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteShader(self.id);
        }
    }
}

fn shader_from_source(
    gl: &gl::Gl,
    source: &str,
    kind: gl::types::GLenum,
    name: &str
) -> Result<gl::types::GLuint, Error>
{
    use std::ffi::{CStr, CString};
    let c_str: &CStr = &CString::new(source)?;

    let id = unsafe { gl.CreateShader(kind) };
    unsafe {
        gl.ShaderSource(id, 1, &c_str.as_ptr(), std::ptr::null());
        gl.CompileShader(id);
    }

    let mut success: gl::types::GLint = 1;
    unsafe {
        gl.GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl.GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let error = utility::create_whitespace_cstring_with_len(len as usize);

        unsafe {
            gl.GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar
            );
        }

        return Err(Error::FailedToCompileShader{
            name: name.to_string(),
            message: format!("Failed to compile shader due to error: {}", error.to_string_lossy().into_owned())
        });
    }

    Ok(id)
}
