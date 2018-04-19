use gl;
use std;
use glm;

use dust::utility;
use dust::shader;

use std::collections::HashMap;
use std::ffi::{CString};

#[derive(Debug)]
pub enum Error {
    Shader(shader::Error),
    FailedToLinkProgram {message: String},
    FailedToCreateCString(std::ffi::NulError),
    FailedToFindPositions {message: String}
}

impl From<shader::Error> for Error {
    fn from(other: shader::Error) -> Self {
        Error::Shader(other)
    }
}

impl From<std::ffi::NulError> for Error {
    fn from(other: std::ffi::NulError) -> Self {
        Error::FailedToCreateCString(other)
    }
}

#[derive(Clone)]
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

    pub fn add_uniform_int(&self, name: &str, data: &i32) -> Result<(), Error>
    {
        let location= self.get_uniform_location(name)?;
        unsafe {
            self.gl.Uniform1iv(location, 1, data);
        }
        Ok(())
    }

    pub fn add_uniform_float(&self, name: &str, data: &f32) -> Result<(), Error>
    {
        let location= self.get_uniform_location(name)?;
        unsafe {
            self.gl.Uniform1fv(location, 1, data);
        }
        Ok(())
    }

    pub fn add_uniform_vec2(&self, name: &str, data: &glm::Vec2) -> Result<(), Error>
    {
        let location= self.get_uniform_location(name)?;
        unsafe {
            self.gl.Uniform2fv(location, 1, &data[0]);
        }
        Ok(())
    }

    pub fn add_uniform_vec3(&self, name: &str, data: &glm::Vec3) -> Result<(), Error>
    {
        let location= self.get_uniform_location(name)?;
        unsafe {
            self.gl.Uniform3fv(location, 1, &data[0]);
        }
        Ok(())
    }


    pub fn add_uniform_vec4(&self, name: &str, data: &glm::Vec4) -> Result<(), Error>
    {
        let location= self.get_uniform_location(name)?;
        unsafe {
            self.gl.Uniform4fv(location, 1, &data[0]);
        }
        Ok(())
    }

    pub fn add_uniform_mat4(&self, name: &str, data: &glm::Matrix4<f32>) -> Result<(), Error>
    {
        let location= self.get_uniform_location(name)?;
        unsafe {
            self.gl.UniformMatrix4fv(location, 1, gl::FALSE, &data[0][0]);
        }
        Ok(())
    }

    fn get_uniform_location(&self, name: &str) -> Result<i32, Error>
    {
        self.set_used();
        let location: i32;
        let c_str = CString::new(name)?;
        unsafe {
            location = self.gl.GetUniformLocation(self.id, c_str.as_ptr());
        }
        Ok(location)
    }

    pub fn setup_attributes(&self, attributes: &HashMap<String, Vec<f32>>) -> Result<(), Error>
    {
        let no_attributes = attributes.len();
        let no_vertices = (attributes.get("Position").ok_or(Error::FailedToFindPositions {message: format!("All drawables must have an attribute called Position")})?).len() / 3;
        let stride = 3 * no_attributes;

        let mut data: Vec<f32> = Vec::with_capacity(no_attributes * no_vertices * 3);
        unsafe { data.set_len(no_attributes * no_vertices * 3); }
        let mut offset = 0;
        for (_key, value) in attributes
        {
            for vertex_id in 0..no_vertices
            {
                for d in 0..3
                {
                    data[offset + vertex_id * stride + d] = value[vertex_id * 3 + d];
                }
            }
            offset = offset + 3;
        }

        let mut vbo: gl::types::GLuint = 0;
        unsafe {
            self.gl.GenBuffers(1, &mut vbo);
            self.gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
            self.gl.BufferData(
                gl::ARRAY_BUFFER, // target
                (stride * no_vertices * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
                data.as_ptr() as *const gl::types::GLvoid, // pointer to data
                gl::STATIC_DRAW, // usage
            );
        }

        let mut offset: usize = 0;
        for (key, _value) in attributes {
            let location = self.get_attribute_location(key)? as gl::types::GLuint;
            unsafe {
                self.gl.EnableVertexAttribArray(location);
                self.gl.VertexAttribPointer(
                    location, // index of the generic vertex attribute
                    3, // the number of components per generic vertex attribute
                    gl::FLOAT, // data type
                    gl::FALSE, // normalized (int-to-float conversion)
                    (stride * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
                    (offset * std::mem::size_of::<f32>()) as *const std::os::raw::c_void // offset of the first component
                );
            }
            offset = offset + 3;
        }

        unsafe {
            self.gl.BindBuffer(gl::ARRAY_BUFFER, 0);
        }
        Ok(())
    }

    fn get_attribute_location(&self, name: &str) -> Result<i32, Error>
    {
        self.set_used();
        let location: i32;
        let c_str = CString::new(name)?;
        unsafe {
            location = self.gl.GetAttribLocation(self.id, c_str.as_ptr());
        }
        Ok(location)
    }

    pub fn set_used(&self) {
        unsafe {
            static mut CURRENTLY_USED: gl::types::GLuint = 1000000 as u32;
            if self.id != CURRENTLY_USED
            {
                self.gl.UseProgram(self.id);
                CURRENTLY_USED = self.id;
            }
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
