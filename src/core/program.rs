use crate::core::*;
use std::collections::HashMap;
use std::sync::RwLock;

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
    textures: RwLock<HashMap<String, u32>>,
    uniforms: HashMap<String, crate::context::UniformLocation>,
    uniform_blocks: RwLock<HashMap<String, (u32, u32)>>,
}

impl Program {
    ///
    /// Creates a new shader program from the given vertex and fragment glsl shader source.
    ///
    pub fn from_source(
        context: &Context,
        vertex_shader_source: &str,
        fragment_shader_source: &str,
    ) -> Result<Self, CoreError> {
        unsafe {
            let vert_shader = context
                .create_shader(crate::context::VERTEX_SHADER)
                .expect("Failed creating vertex shader");
            let frag_shader = context
                .create_shader(crate::context::FRAGMENT_SHADER)
                .expect("Failed creating fragment shader");

            let header: &str = if context.version().is_embedded {
                "#version 300 es
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
                    #endif\n"
            } else {
                "#version 330 core\n"
            };
            let vertex_shader_source = format!("{}{}", header, vertex_shader_source);
            let fragment_shader_source = format!("{}{}", header, fragment_shader_source);

            context.shader_source(vert_shader, &vertex_shader_source);
            context.shader_source(frag_shader, &fragment_shader_source);
            context.compile_shader(vert_shader);
            context.compile_shader(frag_shader);

            let id = context.create_program().expect("Failed creating program");
            context.attach_shader(id, vert_shader);
            context.attach_shader(id, frag_shader);
            context.link_program(id);

            if !context.get_program_link_status(id) {
                let log = context.get_shader_info_log(vert_shader);
                if log.len() > 0 {
                    Err(CoreError::ShaderCompilation(
                        "vertex".to_string(),
                        log,
                        vertex_shader_source,
                    ))?;
                }
                let log = context.get_shader_info_log(frag_shader);
                if log.len() > 0 {
                    Err(CoreError::ShaderCompilation(
                        "fragment".to_string(),
                        log,
                        fragment_shader_source,
                    ))?;
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
                    let location = context
                        .get_attrib_location(id, &name)
                        .expect(&format!("Could not get the location of uniform {}", name));
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

            Ok(Program {
                context: context.clone(),
                id,
                attributes,
                uniforms,
                uniform_blocks: RwLock::new(HashMap::new()),
                textures: RwLock::new(HashMap::new()),
            })
        }
    }

    ///
    /// Send the given uniform data to this shader program and associate it with the given named variable.
    /// The glsl shader variable must be of type `uniform int` if the data is an integer, `uniform vec2` if it is of type [Vec2] etc.
    /// The uniform variable is uniformly available across all processing of vertices and fragments.
    ///
    /// # Panic
    /// Will panic if the uniform is not defined or not used in the shader code.
    /// In the latter case the variable is removed by the shader compiler.
    ///
    pub fn use_uniform<T: UniformDataType>(&self, name: &str, data: T) {
        let location = self.get_uniform_location(name);
        T::send_uniform(&self.context, location, &[data]);
        self.unuse_program();
    }

    ///
    /// Calls [Self::use_uniform] if [Self::requires_uniform] returns true.
    ///
    pub fn use_uniform_if_required<T: UniformDataType>(&self, name: &str, data: T) {
        if self.requires_uniform(name) {
            self.use_uniform(name, data);
        }
    }

    ///
    /// Send the given array of uniform data to this shader program and associate it with the given named variable.
    /// The glsl shader variable must be of same type and length as the data, so if the data is an array of three [Vec2], the variable must be `uniform vec2[3]`.
    /// The uniform variable is uniformly available across all processing of vertices and fragments.
    ///
    /// # Panic
    /// Will panic if the uniform is not defined in the shader code or not used.
    /// In the latter case the variable is removed by the shader compiler.
    ///
    pub fn use_uniform_array<T: UniformDataType>(&self, name: &str, data: &[T]) {
        let location = self.get_uniform_location(name);
        T::send_uniform(&self.context, location, data);
        self.unuse_program();
    }

    fn get_uniform_location(&self, name: &str) -> &crate::context::UniformLocation {
        self.use_program();
        self.uniforms.get(name).expect(&format!(
            "the uniform {} is sent to the shader but not defined or never used",
            name
        ))
    }

    ///
    /// Use the given [Texture2D] in this shader program and associate it with the given named variable.
    /// The glsl shader variable must be of type `uniform sampler2D` and can only be accessed in the fragment shader.
    ///
    /// # Panic
    /// Will panic if the texture is not defined in the shader code or not used.
    /// In the latter case the variable is removed by the shader compiler.
    ///
    pub fn use_texture(&self, name: &str, texture: &Texture2D) {
        self.use_texture_internal(name);
        texture.bind();
    }

    ///
    /// Use the given [DepthTexture2D] in this shader program and associate it with the given named variable.
    /// The glsl shader variable must be of type `uniform sampler2D` and can only be accessed in the fragment shader.
    ///
    /// # Panic
    /// Will panic if the texture is not defined in the shader code or not used.
    /// In the latter case the variable is removed by the shader compiler.
    ///
    pub fn use_depth_texture(&self, name: &str, texture: &DepthTexture2D) {
        self.use_texture_internal(name);
        texture.bind();
    }

    ///
    /// Use the given texture array in this shader program and associate it with the given named variable.
    /// The glsl shader variable must be of type `uniform sampler2DArray` and can only be accessed in the fragment shader.
    ///
    /// # Panic
    /// Will panic if the texture is not defined in the shader code or not used.
    /// In the latter case the variable is removed by the shader compiler.
    ///
    pub fn use_texture_array(&self, name: &str, texture: &Texture2DArray) {
        self.use_texture_internal(name);
        texture.bind();
    }

    ///
    /// Use the given texture array in this shader program and associate it with the given named variable.
    /// The glsl shader variable must be of type `uniform sampler2DArray` and can only be accessed in the fragment shader.
    ///
    /// # Panic
    /// Will panic if the texture is not defined in the shader code or not used.
    /// In the latter case the variable is removed by the shader compiler.
    ///
    pub fn use_depth_texture_array(&self, name: &str, texture: &DepthTexture2DArray) {
        self.use_texture_internal(name);
        texture.bind();
    }

    ///
    /// Use the given texture cube map in this shader program and associate it with the given named variable.
    /// The glsl shader variable must be of type `uniform samplerCube` and can only be accessed in the fragment shader.
    ///
    /// # Panic
    /// Will panic if the texture is not defined in the shader code or not used.
    /// In the latter case the variable is removed by the shader compiler.
    ///
    pub fn use_texture_cube(&self, name: &str, texture: &TextureCubeMap) {
        self.use_texture_internal(name);
        texture.bind();
    }

    ///
    /// Use the given texture cube map in this shader program and associate it with the given named variable.
    /// The glsl shader variable must be of type `uniform samplerCube` and can only be accessed in the fragment shader.
    ///
    /// # Panic
    /// Will panic if the texture is not defined in the shader code or not used.
    /// In the latter case the variable is removed by the shader compiler.
    ///
    pub fn use_depth_texture_cube(&self, name: &str, texture: &DepthTextureCubeMap) {
        self.use_texture_internal(name);
        texture.bind();
    }

    ///
    /// Use the given 3D texture in this shader program and associate it with the given named variable.
    /// The glsl shader variable must be of type `uniform sampler3D` and can only be accessed in the fragment shader.
    ///
    /// # Panic
    /// Will panic if the texture is not defined in the shader code or not used.
    /// In the latter case the variable is removed by the shader compiler.
    ///
    pub fn use_texture_3d(&self, name: &str, texture: &Texture3D) {
        self.use_texture_internal(name);
        texture.bind();
    }

    fn use_texture_internal(&self, name: &str) -> u32 {
        if !self.textures.read().unwrap().contains_key(name) {
            let mut map = self.textures.write().unwrap();
            let index = map.len() as u32;
            map.insert(name.to_owned(), index);
        };
        let index = self.textures.read().unwrap().get(name).unwrap().clone();
        self.use_uniform(name, index as i32);
        unsafe {
            self.context
                .active_texture(crate::context::TEXTURE0 + index);
        }
        index
    }

    ///
    /// Use the given [UniformBuffer] in this shader program and associate it with the given named variable.
    ///
    pub fn use_uniform_block(&self, name: &str, buffer: &UniformBuffer) {
        if !self.uniform_blocks.read().unwrap().contains_key(name) {
            let mut map = self.uniform_blocks.write().unwrap();
            let location = unsafe {
                self.context
                    .get_uniform_block_index(self.id, name)
                    .expect(&format!(
                        "the uniform block {} is sent to the shader but not defined or never used",
                        name
                    ))
            };
            let index = map.len() as u32;
            map.insert(name.to_owned(), (location, index));
        };
        let (location, index) = self
            .uniform_blocks
            .read()
            .unwrap()
            .get(name)
            .unwrap()
            .clone();
        unsafe {
            self.context.uniform_block_binding(self.id, location, index);
            buffer.bind(index);
            self.context
                .bind_buffer(crate::context::UNIFORM_BUFFER, None);
        }
    }

    ///
    /// Uses the given [VertexBuffer] data in this shader program and associates it with the given named variable.
    /// Each value in the buffer is used when rendering one vertex using the [Program::draw_arrays] or [Program::draw_elements] methods.
    /// Therefore the buffer must contain the same number of values as the number of vertices specified in those draw calls.
    ///
    /// # Panic
    /// Will panic if the attribute is not defined in the shader code or not used.
    /// In the latter case the variable is removed by the shader compiler.
    ///
    pub fn use_vertex_attribute(&self, name: &str, buffer: &VertexBuffer) {
        if buffer.count() > 0 {
            buffer.bind();
            let loc = self.location(name);
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
    }

    ///
    /// Uses the given [InstanceBuffer] data in this shader program and associates it with the given named variable.
    /// Each value in the buffer is used when rendering one instance using the [Program::draw_arrays_instanced] or [Program::draw_elements_instanced] methods.
    /// Therefore the buffer must contain the same number of values as the number of instances specified in those draw calls.
    ///
    /// # Panic
    /// Will panic if the attribute is not defined in the shader code or not used.
    /// In the latter case the variable is removed by the shader compiler.
    ///
    pub fn use_instance_attribute(&self, name: &str, buffer: &InstanceBuffer) {
        if buffer.count() > 0 {
            buffer.bind();
            let loc = self.location(name);
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
    }

    ///
    /// Draws `count` number of triangles with the given render states and viewport using this shader program.
    /// Requires that all attributes and uniforms have been defined using the use_attribute and use_uniform methods.
    /// Assumes that the data for the three vertices in a triangle is defined contiguous in each vertex buffer.
    /// If you want to use an [ElementBuffer], see [Program::draw_elements].
    ///
    pub fn draw_arrays(&self, render_states: RenderStates, viewport: Viewport, count: u32) {
        self.context.set_viewport(viewport);
        self.context.set_render_states(render_states);
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

        #[cfg(debug_assertions)]
        self.context
            .error_check()
            .expect("Unexpected rendering error occured")
    }

    ///
    /// Same as [Program::draw_arrays] except it renders 'instance_count' instances of the same set of triangles.
    /// Use the [Program::use_instance_attribute], method to send unique data for each instance to the shader.
    ///
    pub fn draw_arrays_instanced(
        &self,
        render_states: RenderStates,
        viewport: Viewport,
        count: u32,
        instance_count: u32,
    ) {
        self.context.set_viewport(viewport);
        self.context.set_render_states(render_states);
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

        #[cfg(debug_assertions)]
        self.context
            .error_check()
            .expect("Unexpected rendering error occured")
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
    ) {
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
    ) {
        self.context.set_viewport(viewport);
        self.context.set_render_states(render_states);
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

        #[cfg(debug_assertions)]
        self.context
            .error_check()
            .expect("Unexpected rendering error occured")
    }

    ///
    /// Same as [Program::draw_elements] except it renders 'instance_count' instances of the same set of triangles.
    /// Use the [Program::use_instance_attribute] method to send unique data for each instance to the shader.
    ///
    pub fn draw_elements_instanced(
        &self,
        render_states: RenderStates,
        viewport: Viewport,
        element_buffer: &ElementBuffer,
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
    /// Use the [Program::use_instance_attribute] method to send unique data for each instance to the shader.
    ///
    pub fn draw_subset_of_elements_instanced(
        &self,
        render_states: RenderStates,
        viewport: Viewport,
        element_buffer: &ElementBuffer,
        first: u32,
        count: u32,
        instance_count: u32,
    ) {
        self.context.set_viewport(viewport);
        self.context.set_render_states(render_states);
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

        #[cfg(debug_assertions)]
        self.context
            .error_check()
            .expect("Unexpected rendering error occured")
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

    fn location(&self, name: &str) -> u32 {
        self.use_program();
        *self.attributes.get(name).expect(&format!(
            "the attribute {} is sent to the shader but not defined or never used",
            name
        ))
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
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            self.context.delete_program(self.id);
        }
    }
}
