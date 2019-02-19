use gl;

use crate::core::shader;
use crate::core::state;
use crate::core::buffer;

use crate::types::*;

#[derive(Debug)]
pub enum Error {
    Shader(shader::Error),
    FailedToLinkProgram {message: String},
    FailedToCreateCString(std::ffi::NulError),
    FailedToFindPositions {message: String},
    FailedToFindAttribute {message: String},
    FailedToFindUniform {message: String}
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

pub struct Program {
    gl: gl::Gl,
    id: gl::Program,
}

impl Program
{
    pub fn from_source(gl: &gl::Gl, vertex_shader_source: &str, fragment_shader_source: &str) -> Result<Program, Error>
    {
        let vert_shader = shader::Shader::from_source(gl, vertex_shader_source, gl::consts::VERTEX_SHADER)?;
        let frag_shader = shader::Shader::from_source(gl, fragment_shader_source, gl::consts::FRAGMENT_SHADER)?;
        return Program::from_shaders( gl, &[vert_shader, frag_shader] );
    }

    pub fn from_shaders(gl: &gl::Gl, shaders: &[shader::Shader]) -> Result<Program, Error>
    {
        let program = gl.create_program();

        for shader in shaders {
            shader.attach_shader(&program);
        }

        gl.link_program(&program).map_err(|message| Error::FailedToLinkProgram {message})?;

        for shader in shaders {
            shader.detach_shader(&program);
        }

        Ok(Program { gl: gl.clone(), id: program })
    }

    pub fn add_uniform_int(&self, name: &str, data: &i32) -> Result<(), Error>
    {
        let location = self.get_uniform_location(name)?;
        self.gl.uniform1i(location, *data);
        Ok(())
    }

    pub fn add_uniform_float(&self, name: &str, data: &f32) -> Result<(), Error>
    {
        let location = self.get_uniform_location(name)?;
        self.gl.uniform1f(location, *data);
        Ok(())
    }

    pub fn add_uniform_vec2(&self, name: &str, data: &Vec2) -> Result<(), Error>
    {
        let location = self.get_uniform_location(name)?;
        self.gl.uniform2fv(location, &mut [data.x, data.y]);
        Ok(())
    }

    pub fn add_uniform_vec3(&self, name: &str, data: &Vec3) -> Result<(), Error>
    {
        let location = self.get_uniform_location(name)?;
        self.gl.uniform3fv(location, &mut [data.x, data.y, data.z]);
        Ok(())
    }

    pub fn add_uniform_vec4(&self, name: &str, data: &Vec4) -> Result<(), Error>
    {
        let location= self.get_uniform_location(name)?;
        self.gl.uniform4fv(location, &mut [data.x, data.y, data.z, data.w]);
        Ok(())
    }

    pub fn add_uniform_mat2(&self, name: &str, data: &Mat2) -> Result<(), Error>
    {
        let location = self.get_uniform_location(name)?;
        self.gl.uniform_matrix2fv(location, &mut [data.x.x, data.x.y, data.y.x, data.y.y]);
        Ok(())
    }

    pub fn add_uniform_mat3(&self, name: &str, data: &Mat3) -> Result<(), Error>
    {
        let location = self.get_uniform_location(name)?;
        self.gl.uniform_matrix3fv(location, &mut [data.x.x, data.x.y, data.x.z, data.y.x, data.y.y, data.y.z, data.z.x, data.z.y, data.z.z]);
        Ok(())
    }

    pub fn add_uniform_mat4(&self, name: &str, data: &Mat4) -> Result<(), Error>
    {
        let location = self.get_uniform_location(name)?;
        self.gl.uniform_matrix4fv(location, &mut [data.x.x, data.x.y, data.x.z, data.x.w, data.y.x, data.y.y, data.y.z, data.y.w, data.z.x, data.z.y, data.z.z, data.z.w, data.w.x, data.w.y, data.w.z, data.w.w]);
        Ok(())
    }

    fn get_uniform_location(&self, name: &str) -> Result<gl::UniformLocation, Error>
    {
        self.set_used();
        self.gl.get_uniform_location(&self.id, name).ok_or_else(|| Error::FailedToFindUniform {message: format!("Failed to find {}", name)})
    }

    pub fn setup_attributes(&self, buffer: &buffer::VertexBuffer) -> Result<(), Error>
    {
        let mut offset = 0;
        for att in buffer.attributes_iter() {
            self.setup_attribute(buffer,att.name.as_ref(), att.no_components, buffer.stride(), offset, 0)?;
            offset = offset + att.no_components;
        }

        Ok(())
    }

    pub fn setup_attribute(&self, buffer: &buffer::VertexBuffer, name: &str, no_components: usize, stride: usize, offset: usize, divisor: usize) -> Result<(), Error>
    {
        self.set_used();
        buffer.bind();
        let location = self.gl.get_attrib_location(&self.id, name).ok_or_else(
            || Error::FailedToFindAttribute {message: format!("The attribute {} is sent to the shader but never used.", name)})?;
        self.gl.enable_vertex_attrib_array(location);
        self.gl.vertex_attrib_pointer(location, no_components as u32, gl::consts::FLOAT, false, stride as u32, offset as u32);
        self.gl.vertex_attrib_divisor(location, divisor as u32);
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

    pub fn set_used(&self) {
        self.gl.use_program(&self.id);
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        self.gl.delete_program(&self.id);
    }
}