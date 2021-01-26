use std::collections::HashMap;
use std::cell::RefCell;
use crate::core::*;

pub struct Program {
    gl: Gl,
    id: crate::gl::Program,
    vertex_attributes: HashMap<String, u32>,
    textures: RefCell<HashMap<String, u32>>,
    uniforms: HashMap<String, crate::gl::UniformLocation>,
    uniform_blocks: RefCell<HashMap<String, (u32, u32)>>
}

impl Program
{
    pub fn from_source(gl: &Gl, vertex_shader_source: &str, fragment_shader_source: &str) -> Result<Program, Error>
    {
        let vert_shader = gl.create_shader(consts::VERTEX_SHADER)
            .ok_or(Error::FailedToCreateShader{ shader_type: "Vertex shader".to_string(), message:"Unable to create shader object".to_string() })?;
        let frag_shader = gl.create_shader(consts::FRAGMENT_SHADER)
            .ok_or(Error::FailedToCreateShader{ shader_type: "Fragment shader".to_string(), message:"Unable to create shader object".to_string() })?;
        gl.compile_shader(vertex_shader_source, &vert_shader);
        gl.compile_shader(fragment_shader_source, &frag_shader);

        let id = gl.create_program();
        gl.attach_shader(&id, &vert_shader);
        gl.attach_shader(&id, &frag_shader);
        let success = gl.link_program(&id);

        if !success {
            let mut message = "Failed to compile shader program:\n".to_string();
            if let Some(log) = gl.get_program_info_log(&id) {
                message = format!("{}\nLink error: {}", message, log);
            }
            if let Some(log) = gl.get_shader_info_log(&vert_shader) {
                message = format!("{}\nVertex shader error: {}", message, log);
            }
            if let Some(log) = gl.get_shader_info_log(&frag_shader) {
                message = format!("{}\nFragment shader error: {}", message, log);
            }
            return Err(Error::FailedToLinkProgram { message });
        }

        gl.detach_shader(&id, &vert_shader);
        gl.detach_shader(&id, &frag_shader);
        gl.delete_shader(Some(&vert_shader));
        gl.delete_shader(Some(&frag_shader));

        // Init vertex attributes
        let num_attribs = gl.get_program_parameter(&id, consts::ACTIVE_ATTRIBUTES);
        let mut vertex_attributes = HashMap::new();
        for i in 0..num_attribs {
            let info = gl.get_active_attrib(&id, i);
            let location = gl.get_attrib_location(&id, &info.name()).unwrap();
            //println!("Attribute location: {}, name: {}, type: {}, size: {}", location, info.name(), info.type_(), info.size());
            vertex_attributes.insert(info.name(), location);
        }

        // Init uniforms
        let num_uniforms = gl.get_program_parameter(&id, consts::ACTIVE_UNIFORMS);
        let mut uniforms = HashMap::new();
        for i in 0..num_uniforms {
            let info = gl.get_active_uniform(&id, i);
            let location = gl.get_uniform_location(&id, &info.name());
            //println!("Uniform location: {:?}, name: {}, type: {}, size: {}", location, info.name(), info.type_(), info.size());
            if let Some(loc) = location {
                uniforms.insert(info.name(), loc);
            }
        }

        Ok(Program { gl: gl.clone(), id, vertex_attributes, uniforms, uniform_blocks: RefCell::new(HashMap::new()),
            textures: RefCell::new(HashMap::new())})
    }

    pub fn add_uniform_int(&self, name: &str, data: &i32) -> Result<(), Error>
    {
        let location = self.get_uniform_location(name)?;
        self.gl.uniform1i(location, *data);
        self.gl.unuse_program();
        Ok(())
    }

    pub fn add_uniform_float(&self, name: &str, data: &f32) -> Result<(), Error>
    {
        let location = self.get_uniform_location(name)?;
        self.gl.uniform1f(location, *data);
        self.gl.unuse_program();
        Ok(())
    }

    pub fn add_uniform_vec2(&self, name: &str, data: &Vec2) -> Result<(), Error>
    {
        let location = self.get_uniform_location(name)?;
        self.gl.uniform2fv(location, &mut [data.x, data.y]);
        self.gl.unuse_program();
        Ok(())
    }

    pub fn add_uniform_vec3(&self, name: &str, data: &Vec3) -> Result<(), Error>
    {
        let location = self.get_uniform_location(name)?;
        self.gl.uniform3fv(location, &mut [data.x, data.y, data.z]);
        self.gl.unuse_program();
        Ok(())
    }

    pub fn add_uniform_vec4(&self, name: &str, data: &Vec4) -> Result<(), Error>
    {
        let location= self.get_uniform_location(name)?;
        self.gl.uniform4fv(location, &mut [data.x, data.y, data.z, data.w]);
        self.gl.unuse_program();
        Ok(())
    }

    pub fn add_uniform_mat2(&self, name: &str, data: &Mat2) -> Result<(), Error>
    {
        let location = self.get_uniform_location(name)?;
        self.gl.uniform_matrix2fv(location, &mut data.to_slice());
        self.gl.unuse_program();
        Ok(())
    }

    pub fn add_uniform_mat3(&self, name: &str, data: &Mat3) -> Result<(), Error>
    {
        let location = self.get_uniform_location(name)?;
        self.gl.uniform_matrix3fv(location, &mut data.to_slice());
        self.gl.unuse_program();
        Ok(())
    }

    pub fn add_uniform_mat4(&self, name: &str, data: &Mat4) -> Result<(), Error>
    {
        let location = self.get_uniform_location(name)?;
        self.gl.uniform_matrix4fv(location, &mut data.to_slice());
        self.gl.unuse_program();
        Ok(())
    }

    fn get_uniform_location(&self, name: &str) -> Result<&crate::gl::UniformLocation, Error>
    {
        self.set_used();
        let loc = self.uniforms.get(name).ok_or_else(|| Error::FailedToFindUniform {message: format!("Failed to find uniform {}", name)})?;
        Ok(loc)
    }

    pub fn use_texture(&self, texture: &dyn Texture, texture_name: &str) -> Result<(), Error>
    {
        if !self.textures.borrow().contains_key(texture_name) {
            let mut map = self.textures.borrow_mut();
            let index = map.len() as u32;
            map.insert(texture_name.to_owned(), index);
        };
        let index = self.textures.borrow().get(texture_name).unwrap().clone();
        texture.bind(index);
        self.add_uniform_int(texture_name, &(index as i32))?;
        Ok(())
    }

    pub fn use_uniform_block(&self, buffer: &buffer::UniformBuffer, block_name: &str)
    {
        if !self.uniform_blocks.borrow().contains_key(block_name) {
            let mut map = self.uniform_blocks.borrow_mut();
            let location = self.gl.get_uniform_block_index(&self.id, block_name);
            let index = map.len() as u32;
            map.insert(block_name.to_owned(), (location, index));
        };
        let (location, index) = self.uniform_blocks.borrow().get(block_name).unwrap().clone();
        self.gl.uniform_block_binding(&self.id, location, index);
        buffer.bind(index);
        self.gl.unbind_buffer(consts::UNIFORM_BUFFER);
    }

    pub fn use_attribute_float(&self, buffer: &buffer::VertexBuffer, attribute_name: &str) -> Result<(), Error>
    {
        self.use_attribute_float_divisor(buffer, attribute_name, 0)?;
        Ok(())
    }

    pub fn use_attribute_float_divisor(&self, buffer: &buffer::VertexBuffer, attribute_name: &str,  divisor: usize) -> Result<(), Error>
    {
        if buffer.count() > 0 {
            buffer.bind();
            let loc = self.location(&attribute_name)?;
            self.gl.enable_vertex_attrib_array(loc);
            self.gl.vertex_attrib_pointer(loc, 1, consts::FLOAT, false, 0, 0);
            self.gl.vertex_attrib_divisor(loc, divisor as u32);
            self.gl.unbind_buffer(consts::ARRAY_BUFFER);
            self.gl.unuse_program();
        }
        Ok(())
    }

    pub fn use_attribute_vec2_float(&self, buffer: &buffer::VertexBuffer, attribute_name: &str) -> Result<(), Error>
    {
        self.use_attribute_vec2_float_divisor(buffer, attribute_name, 0)?;
        Ok(())
    }

    pub fn use_attribute_vec2_float_divisor(&self, buffer: &buffer::VertexBuffer, attribute_name: &str, divisor: usize) -> Result<(), Error>
    {
        if buffer.count() > 0 {
            buffer.bind();
            let loc = self.location(&attribute_name)?;
            self.gl.enable_vertex_attrib_array(loc);
            self.gl.vertex_attrib_pointer(loc, 2, consts::FLOAT, false, 0, 0);
            self.gl.vertex_attrib_divisor(loc, divisor as u32);
            self.gl.unbind_buffer(consts::ARRAY_BUFFER);
            self.gl.unuse_program();
        }
        Ok(())
    }

    pub fn use_attribute_vec3_float(&self, buffer: &buffer::VertexBuffer, attribute_name: &str) -> Result<(), Error>
    {
        self.use_attribute_vec3_float_divisor(buffer, attribute_name, 0)?;
        Ok(())
    }

    pub fn use_attribute_vec3_float_divisor(&self, buffer: &buffer::VertexBuffer, attribute_name: &str, divisor: usize) -> Result<(), Error>
    {
        if buffer.count() > 0 {
            buffer.bind();
            let loc = self.location(&attribute_name)?;
            self.gl.enable_vertex_attrib_array(loc);
            self.gl.vertex_attrib_pointer(loc, 3, consts::FLOAT, false, 0, 0);
            self.gl.vertex_attrib_divisor(loc, divisor as u32);
            self.gl.unbind_buffer(consts::ARRAY_BUFFER);
            self.gl.unuse_program();
        }
        Ok(())
    }

    pub fn draw_arrays(&self, count: u32)
    {
        self.set_used();
        self.gl.draw_arrays(consts::TRIANGLES, 0, count);
        for location in self.vertex_attributes.values() {
            self.gl.disable_vertex_attrib_array(*location);
        }
        self.gl.unuse_program();
    }

    pub fn draw_arrays_instanced(&self, count: u32, instance_count: u32)
    {
        self.set_used();
        self.gl.draw_arrays_instanced(consts::TRIANGLES, 0, count, instance_count);
        self.gl.unbind_buffer(consts::ELEMENT_ARRAY_BUFFER);
        for location in self.vertex_attributes.values() {
            self.gl.disable_vertex_attrib_array(*location);
        }
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
        self.gl.draw_elements(consts::TRIANGLES, count, consts::UNSIGNED_INT, first);
        self.gl.unbind_buffer(consts::ELEMENT_ARRAY_BUFFER);

        for location in self.vertex_attributes.values() {
            self.gl.disable_vertex_attrib_array(*location);
        }
        self.gl.unuse_program();
    }

    pub fn draw_elements_instanced(&self, element_buffer: &buffer::ElementBuffer, count: u32)
    {
        self.set_used();
        element_buffer.bind();
        self.gl.draw_elements_instanced(consts::TRIANGLES, element_buffer.count() as u32, consts::UNSIGNED_INT, 0, count);
        self.gl.unbind_buffer(consts::ELEMENT_ARRAY_BUFFER);
        for location in self.vertex_attributes.values() {
            self.gl.disable_vertex_attrib_array(*location);
        }
        self.gl.unuse_program();
    }

    fn location(&self, name: &str) -> Result<u32, Error>
    {
        self.set_used();
        let location = self.vertex_attributes.get(name).ok_or_else(
            || Error::FailedToFindAttribute {message: format!("The attribute {} is sent to the shader but never used.", name)})?;
        Ok(*location)
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