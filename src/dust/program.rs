use gl;
use std;

use utility;
use shader;

#[derive(Debug)]
pub enum Error {
    Shader(shader::Error),
    FailedToLinkProgram {message: String}
}

impl From<shader::Error> for Error {
    fn from(other: shader::Error) -> Self {
        Error::Shader(other)
    }
}

pub struct Program {
    gl: gl::Gl,
    id: gl::types::GLuint,
}

impl Program
{
    pub fn from_resource(gl: &gl::Gl, name: &str) -> Result<Program, Error>
    {
        const POSSIBLE_EXT: [&str; 2] = [
            ".vert",
            ".frag",
        ];

        let shaders = POSSIBLE_EXT.iter()
            .map(|file_extension| {
                shader::Shader::from_resource(gl, &format!("{}{}", name, file_extension))
            })
            .collect::<Result<Vec<shader::Shader>, shader::Error>>()?;

        Program::from_shaders(gl, &shaders[..])
    }

    pub fn from_source(gl: &gl::Gl, vertex_shader_source: &str, fragment_shader_source: &str) -> Result<Program, Error>
    {
        let vert_shader = shader::Shader::from_vert_source(gl, vertex_shader_source)?;
        let frag_shader = shader::Shader::from_frag_source(gl, fragment_shader_source)?;
        return Program::from_shaders( gl, &[vert_shader, frag_shader] );
    }

    pub fn from_shaders(gl: &gl::Gl, shaders: &[shader::Shader]) -> Result<Program, Error>
    {
        let program_id = unsafe { gl.CreateProgram() };

        for shader in shaders {
            unsafe { gl.AttachShader(program_id, shader.id()); }
        }

        unsafe { gl.LinkProgram(program_id); }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl.GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl.GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = utility::create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl.GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar
                );
            }

            return Err(Error::FailedToLinkProgram {message: error.to_string_lossy().into_owned() });;
        }

        for shader in shaders {
            unsafe { gl.DetachShader(program_id, shader.id()); }
        }

        Ok(Program { gl: gl.clone(), id: program_id })
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn set_used(&self) {
        unsafe {
            self.gl.UseProgram(self.id);
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteProgram(self.id);
        }
    }
}
