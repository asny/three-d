use crate::context::{consts, AttributeLocation, ShaderType};
use crate::core::*;
use std::cell::RefCell;
use std::collections::HashMap;

///
/// A shader program consisting of a programmable vertex shader followed by a programmable fragment shader.
/// Functionality includes transferring per vertex data to the vertex shader (see the use_attribute functionality)
/// and transferring uniform data to both shader stages (see the use_uniform and use_texture functionality)
/// and execute the shader program (see the draw functionality).
///
pub struct Program {
    context: Context,
    id: crate::context::Program,
    vertex_attributes: HashMap<String, AttributeLocation>,
    textures: RefCell<HashMap<String, u32>>,
    uniforms: HashMap<String, crate::context::UniformLocation>,
    uniform_blocks: RefCell<HashMap<String, (u32, u32)>>,
}

impl Program {
    ///
    /// Creates a new shader program from the given vertex and fragment glsl shader source.
    ///
    pub fn from_source(
        context: &Context,
        vertex_shader_source: &str,
        fragment_shader_source: &str,
    ) -> ThreeDResult<Program> {
        let vert_shader = context
            .create_shader(ShaderType::Vertex)
            .ok_or(CoreError::ShaderCreation)?;
        let frag_shader = context
            .create_shader(ShaderType::Fragment)
            .ok_or(CoreError::ShaderCreation)?;
        context.compile_shader(vertex_shader_source, &vert_shader);
        context.compile_shader(fragment_shader_source, &frag_shader);

        let id = context.create_program();
        context.attach_shader(&id, &vert_shader);
        context.attach_shader(&id, &frag_shader);
        let success = context.link_program(&id);

        if !success {
            if let Some(log) = context.get_shader_info_log(&vert_shader) {
                Err(CoreError::ShaderCompilation("vertex".to_string(), log))?;
            }
            if let Some(log) = context.get_shader_info_log(&frag_shader) {
                Err(CoreError::ShaderCompilation("fragment".to_string(), log))?;
            }
            if let Some(log) = context.get_program_info_log(&id) {
                Err(CoreError::ShaderLink(log))?;
            }
            unreachable!();
        }

        context.detach_shader(&id, &vert_shader);
        context.detach_shader(&id, &frag_shader);
        context.delete_shader(Some(&vert_shader));
        context.delete_shader(Some(&frag_shader));

        // Init vertex attributes
        let num_attribs = context.get_program_parameter(&id, consts::ACTIVE_ATTRIBUTES);
        let mut vertex_attributes = HashMap::new();
        for i in 0..num_attribs {
            let info = context.get_active_attrib(&id, i);
            let location = context.get_attrib_location(&id, &info.name()).unwrap();
            //println!("Attribute location: {}, name: {}, type: {}, size: {}", location, info.name(), info.type_(), info.size());
            vertex_attributes.insert(info.name(), location);
        }

        // Init uniforms
        let num_uniforms = context.get_program_parameter(&id, consts::ACTIVE_UNIFORMS);
        let mut uniforms = HashMap::new();
        for i in 0..num_uniforms {
            let info = context.get_active_uniform(&id, i);
            let location = context.get_uniform_location(&id, &info.name());
            /*println!(
                "Uniform location: {:?}, name: {}, type: {}, size: {}",
                location,
                info.name().split('[').collect::<Vec<_>>()[0].to_string(),
                info.type_(),
                info.size()
            );*/
            if let Some(loc) = location {
                uniforms.insert(
                    info.name().split('[').collect::<Vec<_>>()[0].to_string(),
                    loc,
                );
            }
        }

        Ok(Program {
            context: context.clone(),
            id,
            vertex_attributes,
            uniforms,
            uniform_blocks: RefCell::new(HashMap::new()),
            textures: RefCell::new(HashMap::new()),
        })
    }

    ///
    /// Send the given uniform data to this shader program and associate it with the given named variable.
    /// The glsl shader variable must be of type `uniform int` if the data is an integer, `uniform vec2` if it is of type [Vec2] etc.
    /// The uniform variable is uniformly available across all processing of vertices and fragments.
    ///
    /// # Errors
    /// Will return an error if the uniform is not defined in the shader code or not used.
    /// In the latter case the variable is removed by the shader compiler.
    ///
    pub fn use_uniform<T: UniformDataType>(&self, name: &str, data: T) -> ThreeDResult<()> {
        let location = self.get_uniform_location(name)?;
        data.send(&self.context, location);
        self.context.unuse_program();
        Ok(())
    }

    ///
    /// Send the given array of uniform data to this shader program and associate it with the given named variable.
    /// The glsl shader variable must be of same type and length as the data, so if the data is an array of three [Vec2], the variable must be `uniform vec2[3]`.
    /// The uniform variable is uniformly available across all processing of vertices and fragments.
    ///
    /// # Errors
    /// Will return an error if the uniform is not defined in the shader code or not used.
    /// In the latter case the variable is removed by the shader compiler.
    ///
    pub fn use_uniform_array<T: UniformDataType>(
        &self,
        name: &str,
        data: &[T],
    ) -> ThreeDResult<()> {
        let location = self.get_uniform_location(name)?;
        T::send_array(data, &self.context, location);
        self.context.unuse_program();
        Ok(())
    }

    ///
    /// Send the given integer value to this shader program and associate it with the given named variable.
    /// The glsl shader variable must be of type `uniform int`, meaning it is uniformly available across all processing of vertices and fragments.
    ///
    /// # Errors
    /// Will return an error if the uniform is not defined in the shader code or not used.
    /// In the latter case the variable is removed by the shader compiler.
    ///
    pub fn use_uniform_int(&self, name: &str, data: &i32) -> ThreeDResult<()> {
        self.use_uniform(name, data)
    }

    ///
    /// Send the given float value to this shader program and associate it with the given named variable.
    /// The glsl shader variable must be of type `uniform float`, meaning it is uniformly available across all processing of vertices and fragments.
    ///
    /// # Errors
    /// Will return an error if the uniform is not defined in the shader code or not used.
    /// In the latter case the variable is removed by the shader compiler.
    ///
    pub fn use_uniform_float(&self, name: &str, data: &f32) -> ThreeDResult<()> {
        self.use_uniform(name, data)
    }

    ///
    /// Send the given [Vec2](crate::Vec2) value to this shader program and associate it with the given named variable.
    /// The glsl shader variable must be of type `uniform vec2`, meaning it is uniformly available across all processing of vertices and fragments.
    ///
    /// # Errors
    /// Will return an error if the uniform is not defined in the shader code or not used.
    /// In the latter case the variable is removed by the shader compiler.
    ///
    pub fn use_uniform_vec2(&self, name: &str, data: &Vec2) -> ThreeDResult<()> {
        self.use_uniform(name, data)
    }

    ///
    /// Send the given [Vec3](crate::Vec3) value to this shader program and associate it with the given named variable.
    /// The glsl shader variable must be of type `uniform vec3`, meaning it is uniformly available across all processing of vertices and fragments.
    ///
    /// # Errors
    /// Will return an error if the uniform is not defined in the shader code or not used.
    /// In the latter case the variable is removed by the shader compiler.
    ///
    pub fn use_uniform_vec3(&self, name: &str, data: &Vec3) -> ThreeDResult<()> {
        self.use_uniform(name, data)
    }

    ///
    /// Send the given [Vec4](crate::Vec4) value to this shader program and associate it with the given named variable.
    /// The glsl shader variable must be of type `uniform vec4`, meaning it is uniformly available across all processing of vertices and fragments.
    ///
    /// # Errors
    /// Will return an error if the uniform is not defined in the shader code or not used.
    /// In the latter case the variable is removed by the shader compiler.
    ///
    pub fn use_uniform_vec4(&self, name: &str, data: &Vec4) -> ThreeDResult<()> {
        self.use_uniform(name, data)
    }

    ///
    /// Send the given [Quat](crate::Quat) value to this shader program and associate it with the given named variable.
    /// The glsl shader variable must be of type `uniform vec4`, meaning it is uniformly available across all processing of vertices and fragments.
    ///
    /// # Errors
    /// Will return an error if the uniform is not defined in the shader code or not used.
    /// In the latter case the variable is removed by the shader compiler.
    ///
    pub fn use_uniform_quat(&self, name: &str, data: &Quat) -> ThreeDResult<()> {
        self.use_uniform(name, data)
    }

    ///
    /// Send the given [Mat2](crate::Mat2) value to this shader program and associate it with the given named variable.
    /// The glsl shader variable must be of type `uniform mat2`, meaning it is uniformly available across all processing of vertices and fragments.
    ///
    /// # Errors
    /// Will return an error if the uniform is not defined in the shader code or not used.
    /// In the latter case the variable is removed by the shader compiler.
    ///
    pub fn use_uniform_mat2(&self, name: &str, data: &Mat2) -> ThreeDResult<()> {
        self.use_uniform(name, data)
    }

    ///
    /// Send the given [Mat3](crate::Mat3) value to this shader program and associate it with the given named variable.
    /// The glsl shader variable must be of type `uniform mat3`, meaning it is uniformly available across all processing of vertices and fragments.
    ///
    /// # Errors
    /// Will return an error if the uniform is not defined in the shader code or not used.
    /// In the latter case the variable is removed by the shader compiler.
    ///
    pub fn use_uniform_mat3(&self, name: &str, data: &Mat3) -> ThreeDResult<()> {
        self.use_uniform(name, data)
    }

    ///
    /// Send the given [Mat4](crate::Mat4) value to this shader program and associate it with the given named variable.
    /// The glsl shader variable must be of type `uniform mat4`, meaning it is uniformly available across all processing of vertices and fragments.
    ///
    /// # Errors
    /// Will return an error if the uniform is not defined in the shader code or not used.
    /// In the latter case the variable is removed by the shader compiler.
    ///
    pub fn use_uniform_mat4(&self, name: &str, data: &Mat4) -> ThreeDResult<()> {
        self.use_uniform(name, data)
    }

    fn get_uniform_location(&self, name: &str) -> ThreeDResult<&crate::context::UniformLocation> {
        self.set_used();
        let loc = self
            .uniforms
            .get(name)
            .ok_or_else(|| CoreError::UnusedUniform(name.to_string()))?;
        Ok(loc)
    }

    ///
    /// Use the given [Texture2D] in this shader program and associate it with the given named variable.
    /// The glsl shader variable must be of type `uniform sampler2D` and can only be accessed in the fragment shader.
    ///
    /// # Errors
    /// Will return an error if the texture is not defined in the shader code or not used.
    /// In the latter case the variable is removed by the shader compiler.
    ///
    pub fn use_texture(&self, name: &str, texture: &impl Texture) -> ThreeDResult<()> {
        let index = self.get_texture_index(name);
        texture.bind(index);
        self.use_uniform(name, index as i32)?;
        Ok(())
    }

    ///
    /// Use the given [TextureArray] in this shader program and associate it with the given named variable.
    /// The glsl shader variable must be of type `uniform sampler2DArray` and can only be accessed in the fragment shader.
    ///
    /// # Errors
    /// Will return an error if the texture is not defined in the shader code or not used.
    /// In the latter case the variable is removed by the shader compiler.
    ///
    pub fn use_texture_array(&self, name: &str, texture: &impl TextureArray) -> ThreeDResult<()> {
        let index = self.get_texture_index(name);
        texture.bind(index);
        self.use_uniform(name, index as i32)?;
        Ok(())
    }

    ///
    /// Use the given [TextureCube] in this shader program and associate it with the given named variable.
    /// The glsl shader variable must be of type `uniform samplerCube` and can only be accessed in the fragment shader.
    ///
    /// # Errors
    /// Will return an error if the texture is not defined in the shader code or not used.
    /// In the latter case the variable is removed by the shader compiler.
    ///
    pub fn use_texture_cube(&self, name: &str, texture: &impl TextureCube) -> ThreeDResult<()> {
        let index = self.get_texture_index(name);
        texture.bind(index);
        self.use_uniform(name, index as i32)?;
        Ok(())
    }

    fn get_texture_index(&self, name: &str) -> u32 {
        if !self.textures.borrow().contains_key(name) {
            let mut map = self.textures.borrow_mut();
            let index = map.len() as u32;
            map.insert(name.to_owned(), index);
        };
        self.textures.borrow().get(name).unwrap().clone()
    }

    ///
    /// Use the given [UniformBuffer] in this shader program and associate it with the given named variable.
    ///
    pub fn use_uniform_block(&self, name: &str, buffer: &UniformBuffer) {
        if !self.uniform_blocks.borrow().contains_key(name) {
            let mut map = self.uniform_blocks.borrow_mut();
            let location = self.context.get_uniform_block_index(&self.id, name);
            let index = map.len() as u32;
            map.insert(name.to_owned(), (location, index));
        };
        let (location, index) = self.uniform_blocks.borrow().get(name).unwrap().clone();
        self.context
            .uniform_block_binding(&self.id, location, index);
        buffer.bind(index);
        self.context.unbind_buffer(consts::UNIFORM_BUFFER);
    }

    ///
    /// Uses the given [Buffer] in this shader program and associates it with the given named variable.
    /// Each value in the buffer is used when rendering one vertex using the [Program::draw_arrays] or [Program::draw_elements] methods.
    /// Therefore the buffer must contain the same number of values as the number of vertices specified in those draw calls.
    ///
    /// # Errors
    /// Will return an error if the attribute is not defined in the shader code or not used.
    /// In the latter case the variable is removed by the shader compiler.
    ///
    pub fn use_vertex_attribute<T: BufferDataType>(
        &self,
        name: &str,
        buffer: &Buffer<T>,
    ) -> ThreeDResult<()> {
        if buffer.count() > 0 {
            buffer.bind();
            let loc = self.location(name)?;
            self.context.enable_vertex_attrib_array(loc);
            self.context.vertex_attrib_pointer(
                loc,
                buffer.attribute_size(),
                T::data_type(),
                false,
                0,
                0,
            );
            self.context.vertex_attrib_divisor(loc, 0);
            self.context.unbind_buffer(consts::ARRAY_BUFFER);
            self.context.unuse_program();
        }
        Ok(())
    }

    ///
    /// Uses the given buffer data in this shader program and associates it with the given named variable.
    /// Each value in the buffer is used when rendering one instance using the [Program::draw_arrays_instanced] or [Program::draw_elements_instanced] methods.
    /// Therefore the buffer must contain the same number of values as the number of instances specified in those draw calls.
    ///
    /// # Errors
    /// Will return an error if the attribute is not defined in the shader code or not used.
    /// In the latter case the variable is removed by the shader compiler.
    ///
    pub fn use_instance_attribute<T: BufferDataType>(
        &self,
        name: &str,
        buffer: &InstanceBuffer<T>,
    ) -> ThreeDResult<()> {
        if buffer.count() > 0 {
            buffer.bind();
            let loc = self.location(name)?;
            self.context.enable_vertex_attrib_array(loc);
            self.context.vertex_attrib_pointer(
                loc,
                buffer.attribute_size(),
                T::data_type(),
                false,
                0,
                0,
            );
            self.context.vertex_attrib_divisor(loc, 1);
            self.context.unbind_buffer(consts::ARRAY_BUFFER);
            self.context.unuse_program();
        }
        Ok(())
    }

    ///
    /// Uses the given [Buffer] in this shader program and associates it with the given named variable.
    /// Each value in the buffer is used when rendering one vertex using the [Program::draw_arrays] or [Program::draw_elements] methods.
    /// Therefore the buffer must contain the same number of values as the number of vertices specified in those draw calls.
    ///
    /// # Errors
    /// Will return an error if the attribute is not defined in the shader code or not used.
    /// In the latter case the variable is removed by the shader compiler.
    ///
    #[deprecated = "use use_vertex_attribute"]
    pub fn use_attribute<T: BufferDataType>(
        &self,
        name: &str,
        buffer: &Buffer<T>,
    ) -> ThreeDResult<()> {
        if buffer.count() > 0 {
            buffer.bind();
            let loc = self.location(name)?;
            self.context.enable_vertex_attrib_array(loc);
            self.context
                .vertex_attrib_pointer(loc, 1, T::data_type(), false, 0, 0);
            self.context.vertex_attrib_divisor(loc, 0);
            self.context.unbind_buffer(consts::ARRAY_BUFFER);
            self.context.unuse_program();
        }
        Ok(())
    }

    ///
    /// Uses the given buffer data in this shader program and associates it with the given named variable.
    /// Each value in the buffer is used when rendering one instance using the [Program::draw_arrays_instanced] or [Program::draw_elements_instanced] methods.
    /// Therefore the buffer must contain the same number of values as the number of instances specified in those draw calls.
    ///
    /// # Errors
    /// Will return an error if the attribute is not defined in the shader code or not used.
    /// In the latter case the variable is removed by the shader compiler.
    ///
    #[deprecated = "use use_instance_attribute"]
    pub fn use_attribute_instanced<T: BufferDataType>(
        &self,
        name: &str,
        buffer: &InstanceBuffer<T>,
    ) -> ThreeDResult<()> {
        if buffer.count() > 0 {
            buffer.bind();
            let loc = self.location(name)?;
            self.context.enable_vertex_attrib_array(loc);
            self.context
                .vertex_attrib_pointer(loc, 1, T::data_type(), false, 0, 0);
            self.context.vertex_attrib_divisor(loc, 1);
            self.context.unbind_buffer(consts::ARRAY_BUFFER);
            self.context.unuse_program();
        }
        Ok(())
    }

    ///
    /// Uses the given [Buffer] in this shader program and associates it with the given named variable.
    /// Each contiguous 2 values in the buffer are used when rendering one vertex using the [Program::draw_arrays] or [Program::draw_elements] methods.
    /// Therefore the buffer must contain 2 times the number of values as the number of vertices specified in those draw calls.
    ///
    /// # Errors
    /// Will return an error if the attribute is not defined in the shader code or not used.
    /// In the latter case the variable is removed by the shader compiler.
    ///
    #[deprecated = "use use_vertex_attribute"]
    pub fn use_attribute_vec2<T: BufferDataType>(
        &self,
        name: &str,
        buffer: &Buffer<T>,
    ) -> ThreeDResult<()> {
        if buffer.count() > 0 {
            buffer.bind();
            let loc = self.location(name)?;
            self.context.enable_vertex_attrib_array(loc);
            self.context
                .vertex_attrib_pointer(loc, 2, T::data_type(), false, 0, 0);
            self.context.vertex_attrib_divisor(loc, 0);
            self.context.unbind_buffer(consts::ARRAY_BUFFER);
            self.context.unuse_program();
        }
        Ok(())
    }

    ///
    /// Uses the given buffer data in this shader program and associates it with the given named variable.
    /// Each contiguous 2 values in the buffer are used when rendering one instance using the [Program::draw_arrays_instanced] or [Program::draw_elements_instanced] methods.
    /// Therefore the buffer must contain 2 times the number of values as the number of instances specified in those draw calls.
    ///
    /// # Errors
    /// Will return an error if the attribute is not defined in the shader code or not used.
    /// In the latter case the variable is removed by the shader compiler.
    ///
    #[deprecated = "use use_instance_attribute"]
    pub fn use_attribute_vec2_instanced<T: BufferDataType>(
        &self,
        name: &str,
        buffer: &InstanceBuffer<T>,
    ) -> ThreeDResult<()> {
        if buffer.count() > 0 {
            buffer.bind();
            let loc = self.location(name)?;
            self.context.enable_vertex_attrib_array(loc);
            self.context
                .vertex_attrib_pointer(loc, 2, T::data_type(), false, 0, 0);
            self.context.vertex_attrib_divisor(loc, 1);
            self.context.unbind_buffer(consts::ARRAY_BUFFER);
            self.context.unuse_program();
        }
        Ok(())
    }

    ///
    /// Uses the given [Buffer] in this shader program and associates it with the given named variable.
    /// Each contiguous 3 values in the buffer are used when rendering one instance using the [Program::draw_arrays_instanced] or [Program::draw_elements_instanced] methods.
    /// Therefore the buffer must contain 3 times the number of values as the number of instances specified in those draw calls.
    ///
    /// # Errors
    /// Will return an error if the attribute is not defined in the shader code or not used.
    /// In the latter case the variable is removed by the shader compiler.
    ///
    #[deprecated = "use use_vertex_attribute"]
    pub fn use_attribute_vec3<T: BufferDataType>(
        &self,
        name: &str,
        buffer: &Buffer<T>,
    ) -> ThreeDResult<()> {
        if buffer.count() > 0 {
            buffer.bind();
            let loc = self.location(&name)?;
            self.context.enable_vertex_attrib_array(loc);
            self.context
                .vertex_attrib_pointer(loc, 3, T::data_type(), false, 0, 0);
            self.context.vertex_attrib_divisor(loc, 0);
            self.context.unbind_buffer(consts::ARRAY_BUFFER);
            self.context.unuse_program();
        }
        Ok(())
    }

    ///
    /// Uses the given buffer data in this shader program and associates it with the given named variable.
    /// Each contiguous 3 values in the buffer are used when rendering one instance using the [Program::draw_arrays_instanced] or [Program::draw_elements_instanced] methods.
    /// Therefore the buffer must contain 3 times the number of values as the number of instances specified in those draw calls.
    ///
    /// # Errors
    /// Will return an error if the attribute is not defined in the shader code or not used.
    /// In the latter case the variable is removed by the shader compiler.
    ///
    #[deprecated = "use use_instance_attribute"]
    pub fn use_attribute_vec3_instanced<T: BufferDataType>(
        &self,
        name: &str,
        buffer: &InstanceBuffer<T>,
    ) -> ThreeDResult<()> {
        if buffer.count() > 0 {
            buffer.bind();
            let loc = self.location(&name)?;
            self.context.enable_vertex_attrib_array(loc);
            self.context
                .vertex_attrib_pointer(loc, 3, T::data_type(), false, 0, 0);
            self.context.vertex_attrib_divisor(loc, 1);
            self.context.unbind_buffer(consts::ARRAY_BUFFER);
            self.context.unuse_program();
        }
        Ok(())
    }

    ///
    /// Uses the given [Buffer] in this shader program and associates it with the given named variable.
    /// Each contiguous 4 values in the buffer are used when rendering one instance using the [Program::draw_arrays_instanced] or [Program::draw_elements_instanced] methods.
    /// Therefore the buffer must contain 4 times the number of values as the number of instances specified in those draw calls.
    ///
    /// # Errors
    /// Will return an error if the attribute is not defined in the shader code or not used.
    /// In the latter case the variable is removed by the shader compiler.
    ///
    #[deprecated = "use use_vertex_attribute"]
    pub fn use_attribute_vec4<T: BufferDataType>(
        &self,
        name: &str,
        buffer: &Buffer<T>,
    ) -> ThreeDResult<()> {
        if buffer.count() > 0 {
            buffer.bind();
            let loc = self.location(name)?;
            self.context.enable_vertex_attrib_array(loc);
            self.context
                .vertex_attrib_pointer(loc, 4, T::data_type(), false, 0, 0);
            self.context.vertex_attrib_divisor(loc, 0);
            self.context.unbind_buffer(consts::ARRAY_BUFFER);
            self.context.unuse_program();
        }
        Ok(())
    }

    ///
    /// Uses the given buffer data in this shader program and associates it with the given named variable.
    /// Each contiguous 4 values in the buffer are used when rendering one instance using the [Program::draw_arrays_instanced] or [Program::draw_elements_instanced] methods.
    /// Therefore the buffer must contain 4 times the number of values as the number of instances specified in those draw calls.
    ///
    #[deprecated = "use use_instance_attribute"]
    pub fn use_attribute_vec4_instanced<T: BufferDataType>(
        &self,
        name: &str,
        buffer: &InstanceBuffer<T>,
    ) -> ThreeDResult<()> {
        if buffer.count() > 0 {
            buffer.bind();
            let loc = self.location(name)?;
            self.context.enable_vertex_attrib_array(loc);
            self.context
                .vertex_attrib_pointer(loc, 4, T::data_type(), false, 0, 0);
            self.context.vertex_attrib_divisor(loc, 1);
            self.context.unbind_buffer(consts::ARRAY_BUFFER);
            self.context.unuse_program();
        }
        Ok(())
    }

    ///
    /// Draws `count` number of triangles with the given render states and viewport using this shader program.
    /// Requires that all attributes and uniforms have been defined using the use_attribute and use_uniform methods.
    /// Assumes that the data for the three vertices in a triangle is defined contiguous in each vertex buffer.
    /// If you want to use an [ElementBuffer], see [Program::draw_elements].
    ///
    pub fn draw_arrays(&self, render_states: RenderStates, viewport: Viewport, count: u32) {
        Self::set_viewport(&self.context, viewport);
        Self::set_states(&self.context, render_states);
        self.set_used();
        self.context.draw_arrays(consts::TRIANGLES, 0, count);
        for location in self.vertex_attributes.values() {
            self.context.disable_vertex_attrib_array(*location);
        }
        self.context.unuse_program();
    }

    ///
    /// Same as [Program::draw_arrays] except it renders 'instance_count' instances of the same set of triangles.
    /// Use the [Program::use_attribute_instanced], [Program::use_attribute_vec2_instanced], [Program::use_attribute_vec3_instanced] and [Program::use_attribute_vec4_instanced] methods to send unique data for each instance to the shader.
    ///
    pub fn draw_arrays_instanced(
        &self,
        render_states: RenderStates,
        viewport: Viewport,
        count: u32,
        instance_count: u32,
    ) {
        Self::set_viewport(&self.context, viewport);
        Self::set_states(&self.context, render_states);
        self.set_used();
        self.context
            .draw_arrays_instanced(consts::TRIANGLES, 0, count, instance_count);
        self.context.unbind_buffer(consts::ELEMENT_ARRAY_BUFFER);
        for location in self.vertex_attributes.values() {
            self.context.disable_vertex_attrib_array(*location);
        }
        self.context.unuse_program();
    }

    ///
    /// Draws the triangles defined by the given [ElementBuffer] with the given render states and viewport using this shader program.
    /// Requires that all attributes and uniforms have been defined using the use_attribute and use_uniform methods.
    /// If you do not want to use an [ElementBuffer], see [Program::draw_arrays]. If you only want to draw a subset of the triangles in the given [ElementBuffer], see [Program::draw_subset_of_elements].
    ///
    pub fn draw_with_indices(
        &self,
        render_states: RenderStates,
        viewport: Viewport,
        index_buffer: &IndexBuffer,
    ) {
        match index_buffer {
            IndexBuffer::U8(ref buffer) => self.draw_elements(render_states, viewport, buffer),
            IndexBuffer::U16(ref buffer) => self.draw_elements(render_states, viewport, buffer),
            IndexBuffer::U32(ref buffer) => self.draw_elements(render_states, viewport, buffer),
        }
    }

    ///
    /// Draws the triangles defined by the given [ElementBuffer] with the given render states and viewport using this shader program.
    /// Requires that all attributes and uniforms have been defined using the use_attribute and use_uniform methods.
    /// If you do not want to use an [ElementBuffer], see [Program::draw_arrays]. If you only want to draw a subset of the triangles in the given [ElementBuffer], see [Program::draw_subset_of_elements].
    ///
    pub fn draw_elements<T: ElementBufferDataType>(
        &self,
        render_states: RenderStates,
        viewport: Viewport,
        element_buffer: &ElementBuffer<T>,
    ) {
        self.draw_subset_of_elements(
            render_states,
            viewport,
            element_buffer,
            0,
            element_buffer.count() as u32,
        );
    }

    ///
    /// Draws a subset of the triangles defined by the given [ElementBuffer] with the given render states and viewport using this shader program.
    /// Requires that all attributes and uniforms have been defined using the use_attribute and use_uniform methods.
    /// If you do not want to use an [ElementBuffer], see [Program::draw_arrays].
    ///
    pub fn draw_subset_of_elements<T: ElementBufferDataType>(
        &self,
        render_states: RenderStates,
        viewport: Viewport,
        element_buffer: &ElementBuffer<T>,
        first: u32,
        count: u32,
    ) {
        Self::set_viewport(&self.context, viewport);
        Self::set_states(&self.context, render_states);
        self.set_used();
        element_buffer.bind();
        self.context
            .draw_elements(consts::TRIANGLES, count, T::data_type(), first);
        self.context.unbind_buffer(consts::ELEMENT_ARRAY_BUFFER);

        for location in self.vertex_attributes.values() {
            self.context.disable_vertex_attrib_array(*location);
        }
        self.context.unuse_program();
    }

    ///
    /// Same as [Program::draw_elements] except it renders 'instance_count' instances of the same set of triangles.
    /// Use the [Program::use_attribute_instanced], [Program::use_attribute_vec2_instanced], [Program::use_attribute_vec3_instanced] and [Program::use_attribute_vec4_instanced] methods to send unique data for each instance to the shader.
    ///
    pub fn draw_instances_with_indices(
        &self,
        render_states: RenderStates,
        viewport: Viewport,
        index_buffer: &IndexBuffer,
        instance_count: u32,
    ) {
        match index_buffer {
            IndexBuffer::U8(ref buffer) => {
                self.draw_elements_instanced(render_states, viewport, buffer, instance_count)
            }
            IndexBuffer::U16(ref buffer) => {
                self.draw_elements_instanced(render_states, viewport, buffer, instance_count)
            }
            IndexBuffer::U32(ref buffer) => {
                self.draw_elements_instanced(render_states, viewport, buffer, instance_count)
            }
        }
    }

    ///
    /// Same as [Program::draw_elements] except it renders 'instance_count' instances of the same set of triangles.
    /// Use the [Program::use_attribute_instanced], [Program::use_attribute_vec2_instanced], [Program::use_attribute_vec3_instanced] and [Program::use_attribute_vec4_instanced] methods to send unique data for each instance to the shader.
    ///
    pub fn draw_elements_instanced<T: ElementBufferDataType>(
        &self,
        render_states: RenderStates,
        viewport: Viewport,
        element_buffer: &ElementBuffer<T>,
        instance_count: u32,
    ) {
        self.draw_subset_of_elements_instanced(
            render_states,
            viewport,
            element_buffer,
            0,
            element_buffer.count() as u32,
            instance_count,
        )
    }

    ///
    /// Same as [Program::draw_subset_of_elements] except it renders 'instance_count' instances of the same set of triangles.
    /// Use the [Program::use_attribute_instanced], [Program::use_attribute_vec2_instanced], [Program::use_attribute_vec3_instanced] and [Program::use_attribute_vec4_instanced] methods to send unique data for each instance to the shader.
    ///
    pub fn draw_subset_of_elements_instanced<T: ElementBufferDataType>(
        &self,
        render_states: RenderStates,
        viewport: Viewport,
        element_buffer: &ElementBuffer<T>,
        first: u32,
        count: u32,
        instance_count: u32,
    ) {
        Self::set_viewport(&self.context, viewport);
        Self::set_states(&self.context, render_states);
        self.set_used();
        element_buffer.bind();
        self.context.draw_elements_instanced(
            consts::TRIANGLES,
            count,
            T::data_type(),
            first,
            instance_count,
        );
        self.context.unbind_buffer(consts::ELEMENT_ARRAY_BUFFER);
        for location in self.vertex_attributes.values() {
            self.context.disable_vertex_attrib_array(*location);
        }
        self.context.unuse_program();
    }

    ///
    /// Returns true if this program uses the uniform with the given name.
    ///
    pub fn requires_uniform(&self, name: &str) -> bool {
        self.uniforms.contains_key(name)
    }

    ///
    /// Returns true if this program uses the attribute with the given name.
    ///
    pub fn requires_attribute(&self, name: &str) -> bool {
        self.vertex_attributes.contains_key(name)
    }

    fn location(&self, name: &str) -> ThreeDResult<AttributeLocation> {
        self.set_used();
        let location = self
            .vertex_attributes
            .get(name)
            .ok_or_else(|| CoreError::UnusedAttribute(name.to_string()))?;
        Ok(*location)
    }

    fn set_used(&self) {
        self.context.use_program(&self.id);
    }

    fn set_states(context: &Context, render_states: RenderStates) {
        Self::set_cull(context, render_states.cull);
        Self::set_write_mask(context, render_states.write_mask);
        Self::set_clip(context, render_states.clip);
        Self::set_depth(
            context,
            Some(render_states.depth_test),
            render_states.write_mask.depth,
        );
        Self::set_blend(context, render_states.blend);
    }

    fn set_clip(context: &Context, clip: Clip) {
        unsafe {
            static mut CURRENT: Clip = Clip::Disabled;
            if clip != CURRENT {
                if let Clip::Enabled {
                    x,
                    y,
                    width,
                    height,
                } = clip
                {
                    context.enable(consts::SCISSOR_TEST);
                    context.scissor(x as i32, y as i32, width as i32, height as i32);
                } else {
                    context.disable(consts::SCISSOR_TEST);
                }
                CURRENT = clip;
            }
        }
    }

    fn set_viewport(context: &Context, viewport: Viewport) {
        unsafe {
            static mut CURRENT_VIEWPORT: Viewport = Viewport {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            };
            if viewport != CURRENT_VIEWPORT {
                context.viewport(
                    viewport.x,
                    viewport.y,
                    viewport.width as i32,
                    viewport.height as i32,
                );
                CURRENT_VIEWPORT = viewport;
            }
        }
    }

    fn set_cull(context: &Context, cull: Cull) {
        unsafe {
            static mut CURRENT_CULL: Cull = Cull::None;
            if cull != CURRENT_CULL {
                match cull {
                    Cull::None => {
                        context.disable(consts::CULL_FACE);
                    }
                    Cull::Back => {
                        context.enable(consts::CULL_FACE);
                        context.cull_face(consts::BACK);
                    }
                    Cull::Front => {
                        context.enable(consts::CULL_FACE);
                        context.cull_face(consts::FRONT);
                    }
                    Cull::FrontAndBack => {
                        context.enable(consts::CULL_FACE);
                        context.cull_face(consts::FRONT_AND_BACK);
                    }
                }
                CURRENT_CULL = cull;
            }
        }
    }

    fn set_blend(context: &Context, blend: Blend) {
        unsafe {
            static mut CURRENT: Blend = Blend::Disabled;
            if blend != CURRENT {
                if let Blend::Enabled {
                    source_rgb_multiplier,
                    source_alpha_multiplier,
                    destination_rgb_multiplier,
                    destination_alpha_multiplier,
                    rgb_equation,
                    alpha_equation,
                } = blend
                {
                    context.enable(consts::BLEND);
                    context.blend_func_separate(
                        Self::blend_const_from_multiplier(source_rgb_multiplier),
                        Self::blend_const_from_multiplier(destination_rgb_multiplier),
                        Self::blend_const_from_multiplier(source_alpha_multiplier),
                        Self::blend_const_from_multiplier(destination_alpha_multiplier),
                    );
                    context.blend_equation_separate(
                        Self::blend_const_from_equation(rgb_equation),
                        Self::blend_const_from_equation(alpha_equation),
                    );
                } else {
                    context.disable(consts::BLEND);
                }
                CURRENT = blend;
            }
        }
    }

    fn blend_const_from_multiplier(multiplier: BlendMultiplierType) -> u32 {
        match multiplier {
            BlendMultiplierType::Zero => consts::ZERO,
            BlendMultiplierType::One => consts::ONE,
            BlendMultiplierType::SrcColor => consts::SRC_COLOR,
            BlendMultiplierType::OneMinusSrcColor => consts::ONE_MINUS_SRC_COLOR,
            BlendMultiplierType::DstColor => consts::DST_COLOR,
            BlendMultiplierType::OneMinusDstColor => consts::ONE_MINUS_DST_COLOR,
            BlendMultiplierType::SrcAlpha => consts::SRC_ALPHA,
            BlendMultiplierType::OneMinusSrcAlpha => consts::ONE_MINUS_SRC_ALPHA,
            BlendMultiplierType::DstAlpha => consts::DST_ALPHA,
            BlendMultiplierType::OneMinusDstAlpha => consts::ONE_MINUS_DST_ALPHA,
            BlendMultiplierType::SrcAlphaSaturate => consts::SRC_ALPHA_SATURATE,
        }
    }

    fn blend_const_from_equation(equation: BlendEquationType) -> u32 {
        match equation {
            BlendEquationType::Add => consts::FUNC_ADD,
            BlendEquationType::Subtract => consts::FUNC_SUBTRACT,
            BlendEquationType::ReverseSubtract => consts::FUNC_REVERSE_SUBTRACT,
            BlendEquationType::Min => consts::MIN,
            BlendEquationType::Max => consts::MAX,
        }
    }

    pub(crate) fn set_write_mask(context: &Context, write_mask: WriteMask) {
        unsafe {
            static mut CURRENT_COLOR_MASK: WriteMask = WriteMask::COLOR_AND_DEPTH;
            if write_mask != CURRENT_COLOR_MASK {
                context.color_mask(
                    write_mask.red,
                    write_mask.green,
                    write_mask.blue,
                    write_mask.alpha,
                );
                Self::set_depth(context, None, write_mask.depth);
                CURRENT_COLOR_MASK = write_mask;
            }
        }
    }

    fn set_depth(context: &Context, depth_test: Option<DepthTest>, depth_mask: bool) {
        unsafe {
            static mut CURRENT_DEPTH_ENABLE: bool = false;
            static mut CURRENT_DEPTH_MASK: bool = true;
            static mut CURRENT_DEPTH_TEST: DepthTest = DepthTest::Less;

            if depth_mask == false && depth_test == Some(DepthTest::Always) {
                if CURRENT_DEPTH_ENABLE {
                    context.disable(consts::DEPTH_TEST);
                    CURRENT_DEPTH_ENABLE = false;
                    return;
                }
            } else {
                if !CURRENT_DEPTH_ENABLE {
                    context.enable(consts::DEPTH_TEST);
                    CURRENT_DEPTH_ENABLE = true;
                }
            }

            if depth_mask != CURRENT_DEPTH_MASK {
                context.depth_mask(depth_mask);
                CURRENT_DEPTH_MASK = depth_mask;
            }

            if depth_test.is_some() && depth_test.unwrap() != CURRENT_DEPTH_TEST {
                match depth_test.unwrap() {
                    DepthTest::Never => {
                        context.depth_func(consts::NEVER);
                    }
                    DepthTest::Less => {
                        context.depth_func(consts::LESS);
                    }
                    DepthTest::Equal => {
                        context.depth_func(consts::EQUAL);
                    }
                    DepthTest::LessOrEqual => {
                        context.depth_func(consts::LEQUAL);
                    }
                    DepthTest::Greater => {
                        context.depth_func(consts::GREATER);
                    }
                    DepthTest::NotEqual => {
                        context.depth_func(consts::NOTEQUAL);
                    }
                    DepthTest::GreaterOrEqual => {
                        context.depth_func(consts::GEQUAL);
                    }
                    DepthTest::Always => {
                        context.depth_func(consts::ALWAYS);
                    }
                }
                CURRENT_DEPTH_TEST = depth_test.unwrap();
            }
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        self.context.delete_program(&self.id);
    }
}
