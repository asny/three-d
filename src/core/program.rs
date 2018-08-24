use gl;
use std;
use glm;

use core::shader;
use core::state;
use core::buffer;

use std::ffi::{CString};

#[derive(Debug)]
pub enum Error {
    Shader(shader::Error),
    FailedToLinkProgram {message: String},
    FailedToCreateCString(std::ffi::NulError),
    FailedToFindPositions {message: String},
    FailedToFindAttribute {message: String}
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

            let error = create_whitespace_cstring_with_len(len as usize);

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

    pub fn setup_attributes(&self, buffer: &buffer::VertexBuffer) -> Result<(), Error>
    {
        self.set_used();
        buffer.bind();

        for att in buffer.attributes_iter() {
            self.setup_attribute(att.name.as_ref(), att.no_components, buffer.stride(), att.offset)?;
        }

        Ok(())
    }

    fn setup_attribute(&self, name: &str, no_components: usize, stride: usize, offset: usize) -> Result<(), Error>
    {
        let c_str = CString::new(name)?;
        unsafe {
            let location = self.gl.GetAttribLocation(self.id, c_str.as_ptr());
            if location == -1
            {
                return Err(Error::FailedToFindAttribute {message: format!("The attribute {} is sent to the shader but never used.", name)});
            }

            self.gl.EnableVertexAttribArray(location as gl::types::GLuint);
            self.gl.VertexAttribPointer(
                location as gl::types::GLuint, // index of the generic vertex attribute
                no_components as gl::types::GLint, // the number of components per generic vertex attribute
                gl::FLOAT, // data type
                gl::FALSE, // normalized (int-to-float conversion)
                (stride * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
                (offset * std::mem::size_of::<f32>()) as *const std::os::raw::c_void // offset of the first component
            );
        }
        Ok(())
    }

    // STATES
    pub fn blend(&self, blend_type: state::BlendType)
    {
        state::blend(&self.gl, blend_type);
    }

    pub fn cull(&self, cull_type: state::CullType)
    {
        state::cull(&self.gl, cull_type);
    }

    pub fn depth_test(&self, depth_test_type: state::DepthTestType)
    {
        state::depth_test(&self.gl, depth_test_type);
    }

    pub fn depth_write(&self, enable: bool)
    {
        state::depth_write(&self.gl, enable);
    }

    pub fn polygon_mode(&self, polygon_type: state::PolygonType)
    {
        state::polygon_mode(&self.gl, polygon_type);
    }

    pub fn set_used(&self) {
        unsafe {
            static mut CURRENTLY_USED: gl::types::GLuint = std::u32::MAX;
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

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}