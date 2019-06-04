
use std::collections::HashMap;
use crate::*;

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
    gl: Gl,
    id: gl::Program,
    vertex_attributes: HashMap<String, u32>,
    uniforms: HashMap<String, u32>
}

impl Program
{
    pub fn from_source(gl: &Gl, vertex_shader_source: &str, fragment_shader_source: &str) -> Result<Program, Error>
    {
        let vert_shader = shader::Shader::from_source(gl, vertex_shader_source, gl::consts::VERTEX_SHADER)?;
        let frag_shader = shader::Shader::from_source(gl, fragment_shader_source, gl::consts::FRAGMENT_SHADER)?;
        return Program::from_shaders( gl, &[vert_shader, frag_shader] );
    }

    pub fn from_shaders(gl: &Gl, shaders: &[shader::Shader]) -> Result<Program, Error>
    {
        let id = gl.create_program();

        for shader in shaders {
            shader.attach_shader(&id);
        }

        gl.link_program(&id).map_err(|message| Error::FailedToLinkProgram {message})?;

        for shader in shaders {
            shader.detach_shader(&id);
        }

        // Init vertex attributes
        let num_attribs = gl.get_program_parameter(&id, gl::consts::ACTIVE_ATTRIBUTES);
        let mut vertex_attributes = HashMap::new();
        for i in 0..num_attribs {
            let info = gl.get_active_attrib(&id, i);
            let location = gl.get_attrib_location(&id, &info.name).unwrap();
            println!("Attribute location: {}, name: {}, type: {}, size: {}", location, info.name, info._type, info.size);
            gl.enable_vertex_attrib_array(location);
            vertex_attributes.insert(info.name, location);
        }

        // Init uniforms
        let num_uniforms = gl.get_program_parameter(&id, gl::consts::ACTIVE_UNIFORMS);
        let mut uniforms = HashMap::new();
        for i in 0..num_uniforms {
            let info = gl.get_active_uniform(&id, i);
            let location = gl.get_uniform_location(&id, &info.name).unwrap();
            println!("Uniform location: {}, name: {}, type: {}, size: {}", location, info.name, info._type, info.size);
            uniforms.insert(info.name, location);
        }

        Ok(Program { gl: gl.clone(), id, vertex_attributes, uniforms })
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
        let loc = self.uniforms.get(name).ok_or_else(|| Error::FailedToFindUniform {message: format!("Failed to find uniform {}", name)})?;
        Ok(*loc)
    }

    pub fn use_uniform_block(&self, buffer: &buffer::UniformBuffer, block_name: &str)
    {
        //TODO: use new index
        let location = self.gl.get_uniform_block_index(&self.id, block_name);
        self.gl.uniform_block_binding(&self.id, location, 0);
        buffer.bind(0);
    }

    pub fn use_attribute_vec2_float(&self, buffer: &buffer::VertexBuffer, attribute_name: &str, index: usize) -> Result<(), Error>
    {
        self.use_attribute_vec2_float_divisor(buffer, attribute_name, index, 0)?;
        Ok(())
    }

    pub fn use_attribute_vec2_float_divisor(&self, buffer: &buffer::VertexBuffer, attribute_name: &str, index: usize, divisor: usize) -> Result<(), Error>
    {
        buffer.bind();
        let stride = buffer.stride();
        let offset = buffer.offset_from(index);
        let loc = self.location(&attribute_name)?;
        self.gl.vertex_attrib_pointer(loc, 2, gl::consts::FLOAT, false, stride as u32, offset as u32);
        self.gl.vertex_attrib_divisor(loc, divisor as u32);
        self.gl.unbind_buffer(gl::consts::ARRAY_BUFFER);
        Ok(())
    }

    pub fn use_attribute_vec3_float(&self, buffer: &buffer::VertexBuffer, attribute_name: &str, index: usize) -> Result<(), Error>
    {
        self.use_attribute_vec3_float_divisor(buffer, attribute_name, index, 0)?;
        Ok(())
    }

    pub fn use_attribute_vec3_float_divisor(&self, buffer: &buffer::VertexBuffer, attribute_name: &str, index: usize, divisor: usize) -> Result<(), Error>
    {
        buffer.bind();
        let stride = buffer.stride();
        let offset = buffer.offset_from(index);
        let loc = self.location(&attribute_name)?;
        self.gl.vertex_attrib_pointer(loc, 3, gl::consts::FLOAT, false, stride as u32, offset as u32);
        self.gl.vertex_attrib_divisor(loc, divisor as u32);
        self.gl.unbind_buffer(gl::consts::ARRAY_BUFFER);
        Ok(())
    }

    pub fn draw_arrays(&self, count: u32)
    {
        self.set_used();
        self.gl.draw_arrays(gl::consts::TRIANGLES, 0, count);
        self.gl.unuse_program();
    }

    pub fn draw_elements(&self, element_buffer: &buffer::ElementBuffer)
    {
        self.draw_subset_of_elements(element_buffer, 0,element_buffer.count() as u32);
    }

    pub fn draw_subset_of_elements(&self, element_buffer: &buffer::ElementBuffer, first: u32, count: u32)
    {
        self.set_used();
        element_buffer.bind();
        self.gl.draw_elements(gl::consts::TRIANGLES, count, gl::consts::UNSIGNED_INT, first);
        self.gl.unbind_buffer(gl::consts::ELEMENT_ARRAY_BUFFER);
        self.gl.unuse_program();
    }

    pub fn draw_elements_instanced(&self, element_buffer: &buffer::ElementBuffer, count: u32)
    {
        self.set_used();
        element_buffer.bind();
        self.gl.draw_elements_instanced(gl::consts::TRIANGLES, element_buffer.count() as u32, gl::consts::UNSIGNED_INT, 0, count);
        self.gl.unbind_buffer(gl::consts::ELEMENT_ARRAY_BUFFER);
        self.gl.unuse_program();
    }

    fn location(&self, name: &str) -> Result<u32, Error>
    {
        self.set_used();
        let location = self.vertex_attributes.get(name).ok_or_else(
            || Error::FailedToFindAttribute {message: format!("The attribute {} is sent to the shader but never used.", name)})?;
        Ok(*location)
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

    fn set_used(&self) {
        self.gl.use_program(&self.id);
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        self.gl.delete_program(&self.id);
    }
}