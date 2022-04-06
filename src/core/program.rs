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
    attributes: HashMap<String, u32>,
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
        unsafe {
            let vert_shader = context
                .create_shader(crate::context::VERTEX_SHADER)
                .map_err(|e| CoreError::ShaderCreation(e))?;
            let frag_shader = context
                .create_shader(crate::context::FRAGMENT_SHADER)
                .map_err(|e| CoreError::ShaderCreation(e))?;

            #[cfg(target_arch = "wasm32")]
            const HEADER: &str = "#version 300 es
                    #ifdef GL_FRAGMENT_PRECISION_HIGH
                        precision highp float;
                        precision highp int;
                        precision highp sampler2DArray;
                        precision highp sampler3D;
                    #else
                        precision mediump float;
                        precision mediump int;
                        precision mediump sampler2DArray;
                        precision mediump sampler3D;
                    #endif\n";
            #[cfg(not(target_arch = "wasm32"))]
            const HEADER: &str = "#version 330 core\n";

            context.shader_source(vert_shader, &format!("{}{}", HEADER, vertex_shader_source));
            context.shader_source(
                frag_shader,
                &format!("{}{}", HEADER, fragment_shader_source),
            );
            context.compile_shader(vert_shader);
            context.compile_shader(frag_shader);

            let id = context
                .create_program()
                .map_err(|e| CoreError::ProgramCreation(e))?;
            context.attach_shader(id, vert_shader);
            context.attach_shader(id, frag_shader);
            context.link_program(id);

            if !context.get_program_link_status(id) {
                let log = context.get_shader_info_log(vert_shader);
                if log.len() > 0 {
                    Err(CoreError::ShaderCompilation("vertex".to_string(), log))?;
                }
                let log = context.get_shader_info_log(frag_shader);
                if log.len() > 0 {
                    Err(CoreError::ShaderCompilation("fragment".to_string(), log))?;
                }
                let log = context.get_program_info_log(id);
                if log.len() > 0 {
                    Err(CoreError::ShaderLink(log))?;
                }
                unreachable!();
            }

            context.detach_shader(id, vert_shader);
            context.detach_shader(id, frag_shader);
            context.delete_shader(vert_shader);
            context.delete_shader(frag_shader);

            // Init vertex attributes
            let num_attribs = context.get_active_attributes(id);
            let mut attributes = HashMap::new();
            for i in 0..num_attribs {
                if let Some(crate::context::ActiveAttribute { name, .. }) =
                    context.get_active_attribute(id, i)
                {
                    let location = context.get_attrib_location(id, &name).unwrap();
                    /*println!(
                        "Attribute location: {}, name: {}, type: {}, size: {}",
                        location, name, atype, size
                    );*/
                    attributes.insert(name, location);
                }
            }

            // Init uniforms
            let num_uniforms = context.get_active_uniforms(id);
            let mut uniforms = HashMap::new();
            for i in 0..num_uniforms {
                if let Some(crate::context::ActiveUniform { name, .. }) =
                    context.get_active_uniform(id, i)
                {
                    if let Some(location) = context.get_uniform_location(id, &name) {
                        let name = name.split('[').collect::<Vec<_>>()[0].to_string();
                        /*println!(
                            "Uniform location: {:?}, name: {}, type: {}, size: {}",
                            location, name, utype, size
                        );*/
                        uniforms.insert(name, location);
                    }
                }
            }

            context.error_check()?;
            Ok(Program {
                context: context.clone(),
                id,
                attributes,
                uniforms,
                uniform_blocks: RefCell::new(HashMap::new()),
                textures: RefCell::new(HashMap::new()),
            })
        }
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
        T::send_uniform(&self.context, location, &[data]);
        self.unuse_program();
        self.context.error_check()
    }

    ///
    /// Calls [Self::use_uniform] if [Self::requires_uniform] returns true.
    ///
    pub fn use_uniform_if_required<T: UniformDataType>(
        &self,
        name: &str,
        data: T,
    ) -> ThreeDResult<()> {
        if self.requires_uniform(name) {
            self.use_uniform(name, data)?;
        }
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
        T::send_uniform(&self.context, location, data);
        self.unuse_program();
        self.context.error_check()
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
        self.use_program();
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
    pub fn use_texture(&self, name: &str, texture: &Texture2D) -> ThreeDResult<()> {
        self.use_texture_internal(name)?;
        texture.bind();
        self.context.error_check()
    }

    ///
    /// Use the given [DepthTargetTexture2D] in this shader program and associate it with the given named variable.
    /// The glsl shader variable must be of type `uniform sampler2D` and can only be accessed in the fragment shader.
    ///
    /// # Errors
    /// Will return an error if the texture is not defined in the shader code or not used.
    /// In the latter case the variable is removed by the shader compiler.
    ///
    pub fn use_depth_texture(
        &self,
        name: &str,
        texture: &DepthTargetTexture2D,
    ) -> ThreeDResult<()> {
        self.use_texture_internal(name)?;
        texture.bind();
        self.context.error_check()
    }

    ///
    /// Use the given texture array in this shader program and associate it with the given named variable.
    /// The glsl shader variable must be of type `uniform sampler2DArray` and can only be accessed in the fragment shader.
    ///
    /// # Errors
    /// Will return an error if the texture is not defined in the shader code or not used.
    /// In the latter case the variable is removed by the shader compiler.
    ///
    pub fn use_texture_array(&self, name: &str, texture: &Texture2DArray) -> ThreeDResult<()> {
        self.use_texture_internal(name)?;
        texture.bind();
        self.context.error_check()
    }

    ///
    /// Use the given texture array in this shader program and associate it with the given named variable.
    /// The glsl shader variable must be of type `uniform sampler2DArray` and can only be accessed in the fragment shader.
    ///
    /// # Errors
    /// Will return an error if the texture is not defined in the shader code or not used.
    /// In the latter case the variable is removed by the shader compiler.
    ///
    pub fn use_depth_texture_array(
        &self,
        name: &str,
        texture: &DepthTargetTexture2DArray,
    ) -> ThreeDResult<()> {
        self.use_texture_internal(name)?;
        texture.bind();
        self.context.error_check()
    }

    ///
    /// Use the given texture cube map in this shader program and associate it with the given named variable.
    /// The glsl shader variable must be of type `uniform samplerCube` and can only be accessed in the fragment shader.
    ///
    /// # Errors
    /// Will return an error if the texture is not defined in the shader code or not used.
    /// In the latter case the variable is removed by the shader compiler.
    ///
    pub fn use_texture_cube(&self, name: &str, texture: &TextureCubeMap) -> ThreeDResult<()> {
        self.use_texture_internal(name)?;
        texture.bind();
        self.context.error_check()
    }

    ///
    /// Use the given texture cube map in this shader program and associate it with the given named variable.
    /// The glsl shader variable must be of type `uniform samplerCube` and can only be accessed in the fragment shader.
    ///
    /// # Errors
    /// Will return an error if the texture is not defined in the shader code or not used.
    /// In the latter case the variable is removed by the shader compiler.
    ///
    pub fn use_depth_texture_cube(
        &self,
        name: &str,
        texture: &DepthTargetTextureCubeMap,
    ) -> ThreeDResult<()> {
        self.use_texture_internal(name)?;
        texture.bind();
        self.context.error_check()
    }

    ///
    /// Use the given 3D texture in this shader program and associate it with the given named variable.
    /// The glsl shader variable must be of type `uniform sampler3D` and can only be accessed in the fragment shader.
    ///
    /// # Errors
    /// Will return an error if the texture is not defined in the shader code or not used.
    /// In the latter case the variable is removed by the shader compiler.
    ///
    pub fn use_texture_3d(&self, name: &str, texture: &Texture3D) -> ThreeDResult<()> {
        self.use_texture_internal(name)?;
        texture.bind();
        self.context.error_check()
    }

    fn use_texture_internal(&self, name: &str) -> ThreeDResult<u32> {
        if !self.textures.borrow().contains_key(name) {
            let mut map = self.textures.borrow_mut();
            let index = map.len() as u32;
            map.insert(name.to_owned(), index);
        };
        let index = self.textures.borrow().get(name).unwrap().clone();
        self.use_uniform(name, index as i32)?;
        unsafe {
            self.context
                .active_texture(crate::context::TEXTURE0 + index);
        }
        Ok(index)
    }

    ///
    /// Use the given [UniformBuffer] in this shader program and associate it with the given named variable.
    ///
    pub fn use_uniform_block(&self, name: &str, buffer: &UniformBuffer) -> ThreeDResult<()> {
        if !self.uniform_blocks.borrow().contains_key(name) {
            let mut map = self.uniform_blocks.borrow_mut();
            let location = unsafe {
                self.context
                    .get_uniform_block_index(self.id, name)
                    .ok_or(CoreError::UnusedUniform(name.to_string()))?
            };
            let index = map.len() as u32;
            map.insert(name.to_owned(), (location, index));
        };
        let (location, index) = self.uniform_blocks.borrow().get(name).unwrap().clone();
        unsafe {
            self.context.uniform_block_binding(self.id, location, index);
            buffer.bind(index);
            self.context
                .bind_buffer(crate::context::UNIFORM_BUFFER, None);
        }
        self.context.error_check()
    }

    ///
    /// Uses the given [VertexBuffer] data in this shader program and associates it with the given named variable.
    /// Each value in the buffer is used when rendering one vertex using the [Program::draw_arrays] or [Program::draw_elements] methods.
    /// Therefore the buffer must contain the same number of values as the number of vertices specified in those draw calls.
    ///
    /// # Errors
    /// Will return an error if the attribute is not defined in the shader code or not used.
    /// In the latter case the variable is removed by the shader compiler.
    ///
    pub fn use_vertex_attribute(&self, name: &str, buffer: &VertexBuffer) -> ThreeDResult<()> {
        if buffer.count() > 0 {
            buffer.bind();
            let loc = self.location(name)?;
            unsafe {
                self.context.bind_vertex_array(Some(self.context.vao));
                self.context.enable_vertex_attrib_array(loc);
                self.context.vertex_attrib_pointer_f32(
                    loc,
                    buffer.data_size() as i32,
                    buffer.data_type(),
                    false,
                    0,
                    0,
                );
                self.context.vertex_attrib_divisor(loc, 0);
                self.context.bind_buffer(crate::context::ARRAY_BUFFER, None);
            }
            self.unuse_program();
        }
        self.context.error_check()
    }

    ///
    /// Uses the given [InstanceBuffer] data in this shader program and associates it with the given named variable.
    /// Each value in the buffer is used when rendering one instance using the [Program::draw_arrays_instanced] or [Program::draw_elements_instanced] methods.
    /// Therefore the buffer must contain the same number of values as the number of instances specified in those draw calls.
    ///
    /// # Errors
    /// Will return an error if the attribute is not defined in the shader code or not used.
    /// In the latter case the variable is removed by the shader compiler.
    ///
    pub fn use_instance_attribute(&self, name: &str, buffer: &InstanceBuffer) -> ThreeDResult<()> {
        if buffer.count() > 0 {
            buffer.bind();
            let loc = self.location(name)?;
            unsafe {
                self.context.bind_vertex_array(Some(self.context.vao));
                self.context.enable_vertex_attrib_array(loc);
                self.context.vertex_attrib_pointer_f32(
                    loc,
                    buffer.data_size() as i32,
                    buffer.data_type(),
                    false,
                    0,
                    0,
                );
                self.context.vertex_attrib_divisor(loc, 1);
                self.context.bind_buffer(crate::context::ARRAY_BUFFER, None);
            }
            self.unuse_program();
        }
        self.context.error_check()
    }

    ///
    /// Uses the given [VertexBuffer] in this shader program and associates it with the given named variable.
    /// Each value in the buffer is used when rendering one vertex using the [Program::draw_arrays] or [Program::draw_elements] methods.
    /// Therefore the buffer must contain the same number of values as the number of vertices specified in those draw calls.
    ///
    /// # Errors
    /// Will return an error if the attribute is not defined in the shader code or not used.
    /// In the latter case the variable is removed by the shader compiler.
    ///
    pub fn use_attribute(&self, name: &str, buffer: &VertexBuffer) -> ThreeDResult<()> {
        if buffer.count() > 0 {
            buffer.bind();
            let loc = self.location(name)?;
            unsafe {
                self.context.bind_vertex_array(Some(self.context.vao));
                self.context.enable_vertex_attrib_array(loc);
                self.context
                    .vertex_attrib_pointer_f32(loc, 1, buffer.data_type(), false, 0, 0);
                self.context.vertex_attrib_divisor(loc, 0);
                self.context.bind_buffer(crate::context::ARRAY_BUFFER, None);
            }
            self.unuse_program();
        }
        self.context.error_check()
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
    pub fn use_attribute_instanced(&self, name: &str, buffer: &InstanceBuffer) -> ThreeDResult<()> {
        if buffer.count() > 0 {
            buffer.bind();
            let loc = self.location(name)?;
            unsafe {
                self.context.bind_vertex_array(Some(self.context.vao));
                self.context.enable_vertex_attrib_array(loc);
                self.context
                    .vertex_attrib_pointer_f32(loc, 1, buffer.data_type(), false, 0, 0);
                self.context.vertex_attrib_divisor(loc, 1);
                self.context.bind_buffer(crate::context::ARRAY_BUFFER, None);
            }
            self.unuse_program();
        }
        self.context.error_check()
    }

    ///
    /// Uses the given [VertexBuffer] in this shader program and associates it with the given named variable.
    /// Each contiguous 2 values in the buffer are used when rendering one vertex using the [Program::draw_arrays] or [Program::draw_elements] methods.
    /// Therefore the buffer must contain 2 times the number of values as the number of vertices specified in those draw calls.
    ///
    /// # Errors
    /// Will return an error if the attribute is not defined in the shader code or not used.
    /// In the latter case the variable is removed by the shader compiler.
    ///
    pub fn use_attribute_vec2(&self, name: &str, buffer: &VertexBuffer) -> ThreeDResult<()> {
        if buffer.count() > 0 {
            buffer.bind();
            let loc = self.location(name)?;
            unsafe {
                self.context.bind_vertex_array(Some(self.context.vao));
                self.context.enable_vertex_attrib_array(loc);
                self.context
                    .vertex_attrib_pointer_f32(loc, 2, buffer.data_type(), false, 0, 0);
                self.context.vertex_attrib_divisor(loc, 0);
                self.context.bind_buffer(crate::context::ARRAY_BUFFER, None);
            }
            self.unuse_program();
        }
        self.context.error_check()
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
    pub fn use_attribute_vec2_instanced(
        &self,
        name: &str,
        buffer: &InstanceBuffer,
    ) -> ThreeDResult<()> {
        if buffer.count() > 0 {
            buffer.bind();
            let loc = self.location(name)?;
            unsafe {
                self.context.bind_vertex_array(Some(self.context.vao));
                self.context.enable_vertex_attrib_array(loc);
                self.context
                    .vertex_attrib_pointer_f32(loc, 2, buffer.data_type(), false, 0, 0);
                self.context.vertex_attrib_divisor(loc, 1);
                self.context.bind_buffer(crate::context::ARRAY_BUFFER, None);
            }
            self.unuse_program();
        }
        self.context.error_check()
    }

    ///
    /// Uses the given [VertexBuffer] in this shader program and associates it with the given named variable.
    /// Each contiguous 3 values in the buffer are used when rendering one instance using the [Program::draw_arrays_instanced] or [Program::draw_elements_instanced] methods.
    /// Therefore the buffer must contain 3 times the number of values as the number of instances specified in those draw calls.
    ///
    /// # Errors
    /// Will return an error if the attribute is not defined in the shader code or not used.
    /// In the latter case the variable is removed by the shader compiler.
    ///
    pub fn use_attribute_vec3(&self, name: &str, buffer: &VertexBuffer) -> ThreeDResult<()> {
        if buffer.count() > 0 {
            buffer.bind();
            let loc = self.location(&name)?;
            unsafe {
                self.context.bind_vertex_array(Some(self.context.vao));
                self.context.enable_vertex_attrib_array(loc);
                self.context
                    .vertex_attrib_pointer_f32(loc, 3, buffer.data_type(), false, 0, 0);
                self.context.vertex_attrib_divisor(loc, 0);
                self.context.bind_buffer(crate::context::ARRAY_BUFFER, None);
            }
            self.unuse_program();
        }
        self.context.error_check()
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
    pub fn use_attribute_vec3_instanced(
        &self,
        name: &str,
        buffer: &InstanceBuffer,
    ) -> ThreeDResult<()> {
        if buffer.count() > 0 {
            buffer.bind();
            let loc = self.location(&name)?;
            unsafe {
                self.context.bind_vertex_array(Some(self.context.vao));
                self.context.enable_vertex_attrib_array(loc);
                self.context
                    .vertex_attrib_pointer_f32(loc, 3, buffer.data_type(), false, 0, 0);
                self.context.vertex_attrib_divisor(loc, 1);
                self.context.bind_buffer(crate::context::ARRAY_BUFFER, None);
            }
            self.unuse_program();
        }
        self.context.error_check()
    }

    ///
    /// Uses the given [VertexBuffer] in this shader program and associates it with the given named variable.
    /// Each contiguous 4 values in the buffer are used when rendering one instance using the [Program::draw_arrays_instanced] or [Program::draw_elements_instanced] methods.
    /// Therefore the buffer must contain 4 times the number of values as the number of instances specified in those draw calls.
    ///
    /// # Errors
    /// Will return an error if the attribute is not defined in the shader code or not used.
    /// In the latter case the variable is removed by the shader compiler.
    ///
    pub fn use_attribute_vec4(&self, name: &str, buffer: &VertexBuffer) -> ThreeDResult<()> {
        if buffer.count() > 0 {
            buffer.bind();
            let loc = self.location(name)?;
            unsafe {
                self.context.bind_vertex_array(Some(self.context.vao));
                self.context.enable_vertex_attrib_array(loc);
                self.context
                    .vertex_attrib_pointer_f32(loc, 4, buffer.data_type(), false, 0, 0);
                self.context.vertex_attrib_divisor(loc, 0);
                self.context.bind_buffer(crate::context::ARRAY_BUFFER, None);
            }
            self.unuse_program();
        }
        self.context.error_check()
    }

    ///
    /// Uses the given buffer data in this shader program and associates it with the given named variable.
    /// Each contiguous 4 values in the buffer are used when rendering one instance using the [Program::draw_arrays_instanced] or [Program::draw_elements_instanced] methods.
    /// Therefore the buffer must contain 4 times the number of values as the number of instances specified in those draw calls.
    ///
    pub fn use_attribute_vec4_instanced(
        &self,
        name: &str,
        buffer: &InstanceBuffer,
    ) -> ThreeDResult<()> {
        if buffer.count() > 0 {
            buffer.bind();
            let loc = self.location(name)?;
            unsafe {
                self.context.bind_vertex_array(Some(self.context.vao));
                self.context.enable_vertex_attrib_array(loc);
                self.context
                    .vertex_attrib_pointer_f32(loc, 4, buffer.data_type(), false, 0, 0);
                self.context.vertex_attrib_divisor(loc, 1);
                self.context.bind_buffer(crate::context::ARRAY_BUFFER, None);
            }
            self.unuse_program();
        }
        self.context.error_check()
    }

    ///
    /// Draws `count` number of triangles with the given render states and viewport using this shader program.
    /// Requires that all attributes and uniforms have been defined using the use_attribute and use_uniform methods.
    /// Assumes that the data for the three vertices in a triangle is defined contiguous in each vertex buffer.
    /// If you want to use an [ElementBuffer], see [Program::draw_elements].
    ///
    pub fn draw_arrays(
        &self,
        render_states: RenderStates,
        viewport: Viewport,
        count: u32,
    ) -> ThreeDResult<()> {
        Self::set_viewport(&self.context, viewport);
        Self::set_states(&self.context, render_states)?;
        self.use_program();
        unsafe {
            self.context
                .draw_arrays(crate::context::TRIANGLES, 0, count as i32);
            for location in self.attributes.values() {
                self.context.disable_vertex_attrib_array(*location);
            }
            self.context.bind_vertex_array(None);
        }
        self.unuse_program();
        self.context.error_check()
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
    ) -> ThreeDResult<()> {
        Self::set_viewport(&self.context, viewport);
        Self::set_states(&self.context, render_states)?;
        self.use_program();
        unsafe {
            self.context.draw_arrays_instanced(
                crate::context::TRIANGLES,
                0,
                count as i32,
                instance_count as i32,
            );
            self.context
                .bind_buffer(crate::context::ELEMENT_ARRAY_BUFFER, None);
            for location in self.attributes.values() {
                self.context.disable_vertex_attrib_array(*location);
            }
            self.context.bind_vertex_array(None);
        }
        self.unuse_program();
        self.context.error_check()
    }

    ///
    /// Draws the triangles defined by the given [ElementBuffer] with the given render states and viewport using this shader program.
    /// Requires that all attributes and uniforms have been defined using the use_attribute and use_uniform methods.
    /// If you do not want to use an [ElementBuffer], see [Program::draw_arrays]. If you only want to draw a subset of the triangles in the given [ElementBuffer], see [Program::draw_subset_of_elements].
    ///
    pub fn draw_elements(
        &self,
        render_states: RenderStates,
        viewport: Viewport,
        element_buffer: &ElementBuffer,
    ) -> ThreeDResult<()> {
        self.draw_subset_of_elements(
            render_states,
            viewport,
            element_buffer,
            0,
            element_buffer.count() as u32,
        )
    }

    ///
    /// Draws a subset of the triangles defined by the given [ElementBuffer] with the given render states and viewport using this shader program.
    /// Requires that all attributes and uniforms have been defined using the use_attribute and use_uniform methods.
    /// If you do not want to use an [ElementBuffer], see [Program::draw_arrays].
    ///
    pub fn draw_subset_of_elements(
        &self,
        render_states: RenderStates,
        viewport: Viewport,
        element_buffer: &ElementBuffer,
        first: u32,
        count: u32,
    ) -> ThreeDResult<()> {
        Self::set_viewport(&self.context, viewport);
        Self::set_states(&self.context, render_states)?;
        self.use_program();
        element_buffer.bind();
        unsafe {
            self.context.draw_elements(
                crate::context::TRIANGLES,
                count as i32,
                element_buffer.data_type(),
                first as i32,
            );
            self.context
                .bind_buffer(crate::context::ELEMENT_ARRAY_BUFFER, None);

            for location in self.attributes.values() {
                self.context.disable_vertex_attrib_array(*location);
            }
            self.context.bind_vertex_array(None);
        }
        self.unuse_program();
        self.context.error_check()
    }

    ///
    /// Same as [Program::draw_elements] except it renders 'instance_count' instances of the same set of triangles.
    /// Use the [Program::use_attribute_instanced], [Program::use_attribute_vec2_instanced], [Program::use_attribute_vec3_instanced] and [Program::use_attribute_vec4_instanced] methods to send unique data for each instance to the shader.
    ///
    pub fn draw_elements_instanced(
        &self,
        render_states: RenderStates,
        viewport: Viewport,
        element_buffer: &ElementBuffer,
        instance_count: u32,
    ) -> ThreeDResult<()> {
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
    pub fn draw_subset_of_elements_instanced(
        &self,
        render_states: RenderStates,
        viewport: Viewport,
        element_buffer: &ElementBuffer,
        first: u32,
        count: u32,
        instance_count: u32,
    ) -> ThreeDResult<()> {
        Self::set_viewport(&self.context, viewport);
        Self::set_states(&self.context, render_states)?;
        self.use_program();
        element_buffer.bind();
        unsafe {
            self.context.draw_elements_instanced(
                crate::context::TRIANGLES,
                count as i32,
                element_buffer.data_type(),
                first as i32,
                instance_count as i32,
            );
            self.context
                .bind_buffer(crate::context::ELEMENT_ARRAY_BUFFER, None);
            for location in self.attributes.values() {
                self.context.disable_vertex_attrib_array(*location);
            }
            self.context.bind_vertex_array(None);
        }
        self.unuse_program();
        self.context.error_check()
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
        self.attributes.contains_key(name)
    }

    fn location(&self, name: &str) -> ThreeDResult<u32> {
        self.use_program();
        let location = self
            .attributes
            .get(name)
            .ok_or_else(|| CoreError::UnusedAttribute(name.to_string()))?;
        Ok(*location)
    }

    fn use_program(&self) {
        unsafe {
            self.context.use_program(Some(self.id));
        }
    }

    fn unuse_program(&self) {
        unsafe {
            self.context.use_program(None);
        }
    }

    fn set_states(context: &Context, render_states: RenderStates) -> ThreeDResult<()> {
        Self::set_cull(context, render_states.cull);
        Self::set_write_mask(context, render_states.write_mask);
        Self::set_clip(context, render_states.clip);
        Self::set_depth(
            context,
            Some(render_states.depth_test),
            render_states.write_mask.depth,
        );
        Self::set_blend(context, render_states.blend);
        context.error_check()
    }

    fn set_clip(context: &Context, clip: Clip) {
        unsafe {
            if let Clip::Enabled {
                x,
                y,
                width,
                height,
            } = clip
            {
                context.enable(crate::context::SCISSOR_TEST);
                context.scissor(x as i32, y as i32, width as i32, height as i32);
            } else {
                context.disable(crate::context::SCISSOR_TEST);
            }
        }
    }

    fn set_viewport(context: &Context, viewport: Viewport) {
        unsafe {
            context.viewport(
                viewport.x,
                viewport.y,
                viewport.width as i32,
                viewport.height as i32,
            );
        }
    }

    fn set_cull(context: &Context, cull: Cull) {
        unsafe {
            match cull {
                Cull::None => {
                    context.disable(crate::context::CULL_FACE);
                }
                Cull::Back => {
                    context.enable(crate::context::CULL_FACE);
                    context.cull_face(crate::context::BACK);
                }
                Cull::Front => {
                    context.enable(crate::context::CULL_FACE);
                    context.cull_face(crate::context::FRONT);
                }
                Cull::FrontAndBack => {
                    context.enable(crate::context::CULL_FACE);
                    context.cull_face(crate::context::FRONT_AND_BACK);
                }
            }
        }
    }

    fn set_blend(context: &Context, blend: Blend) {
        unsafe {
            if let Blend::Enabled {
                source_rgb_multiplier,
                source_alpha_multiplier,
                destination_rgb_multiplier,
                destination_alpha_multiplier,
                rgb_equation,
                alpha_equation,
            } = blend
            {
                context.enable(crate::context::BLEND);
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
                context.disable(crate::context::BLEND);
            }
        }
    }

    fn blend_const_from_multiplier(multiplier: BlendMultiplierType) -> u32 {
        match multiplier {
            BlendMultiplierType::Zero => crate::context::ZERO,
            BlendMultiplierType::One => crate::context::ONE,
            BlendMultiplierType::SrcColor => crate::context::SRC_COLOR,
            BlendMultiplierType::OneMinusSrcColor => crate::context::ONE_MINUS_SRC_COLOR,
            BlendMultiplierType::DstColor => crate::context::DST_COLOR,
            BlendMultiplierType::OneMinusDstColor => crate::context::ONE_MINUS_DST_COLOR,
            BlendMultiplierType::SrcAlpha => crate::context::SRC_ALPHA,
            BlendMultiplierType::OneMinusSrcAlpha => crate::context::ONE_MINUS_SRC_ALPHA,
            BlendMultiplierType::DstAlpha => crate::context::DST_ALPHA,
            BlendMultiplierType::OneMinusDstAlpha => crate::context::ONE_MINUS_DST_ALPHA,
            BlendMultiplierType::SrcAlphaSaturate => crate::context::SRC_ALPHA_SATURATE,
        }
    }

    fn blend_const_from_equation(equation: BlendEquationType) -> u32 {
        match equation {
            BlendEquationType::Add => crate::context::FUNC_ADD,
            BlendEquationType::Subtract => crate::context::FUNC_SUBTRACT,
            BlendEquationType::ReverseSubtract => crate::context::FUNC_REVERSE_SUBTRACT,
            BlendEquationType::Min => crate::context::MIN,
            BlendEquationType::Max => crate::context::MAX,
        }
    }

    pub(crate) fn set_write_mask(context: &Context, write_mask: WriteMask) {
        unsafe {
            context.color_mask(
                write_mask.red,
                write_mask.green,
                write_mask.blue,
                write_mask.alpha,
            );
            Self::set_depth(context, None, write_mask.depth);
        }
    }

    fn set_depth(context: &Context, depth_test: Option<DepthTest>, depth_mask: bool) {
        unsafe {
            if depth_mask == false && depth_test == Some(DepthTest::Always) {
                context.disable(crate::context::DEPTH_TEST);
            } else {
                context.enable(crate::context::DEPTH_TEST);
                context.depth_mask(depth_mask);
                if let Some(depth_test) = depth_test {
                    match depth_test {
                        DepthTest::Never => {
                            context.depth_func(crate::context::NEVER);
                        }
                        DepthTest::Less => {
                            context.depth_func(crate::context::LESS);
                        }
                        DepthTest::Equal => {
                            context.depth_func(crate::context::EQUAL);
                        }
                        DepthTest::LessOrEqual => {
                            context.depth_func(crate::context::LEQUAL);
                        }
                        DepthTest::Greater => {
                            context.depth_func(crate::context::GREATER);
                        }
                        DepthTest::NotEqual => {
                            context.depth_func(crate::context::NOTEQUAL);
                        }
                        DepthTest::GreaterOrEqual => {
                            context.depth_func(crate::context::GEQUAL);
                        }
                        DepthTest::Always => {
                            context.depth_func(crate::context::ALWAYS);
                        }
                    }
                }
            }
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            self.context.delete_program(self.id);
        }
    }
}
