use std::collections::HashMap;
use std::cell::RefCell;
use crate::core::*;

pub struct Program {
    gl: Gl,
    id: crate::context::Program,
    vertex_attributes: HashMap<String, u32>,
    textures: RefCell<HashMap<String, u32>>,
    uniforms: HashMap<String, crate::context::UniformLocation>,
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

    fn get_uniform_location(&self, name: &str) -> Result<&crate::context::UniformLocation, Error>
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

    pub fn use_attribute_vec4_float(&self, buffer: &buffer::VertexBuffer, attribute_name: &str) -> Result<(), Error>
    {
        self.use_attribute_vec3_float_divisor(buffer, attribute_name, 0)?;
        Ok(())
    }

    pub fn use_attribute_vec4_float_divisor(&self, buffer: &buffer::VertexBuffer, attribute_name: &str, divisor: usize) -> Result<(), Error>
    {
        if buffer.count() > 0 {
            buffer.bind();
            let loc = self.location(&attribute_name)?;
            self.gl.enable_vertex_attrib_array(loc);
            self.gl.vertex_attrib_pointer(loc, 4, consts::FLOAT, false, 0, 0);
            self.gl.vertex_attrib_divisor(loc, divisor as u32);
            self.gl.unbind_buffer(consts::ARRAY_BUFFER);
            self.gl.unuse_program();
        }
        Ok(())
    }

    pub fn draw_arrays(&self, render_states: RenderStates, viewport: Viewport, count: u32)
    {
        Self::set_viewport(&self.gl, viewport);
        Self::set_states(&self.gl, render_states);
        self.set_used();
        self.gl.draw_arrays(consts::TRIANGLES, 0, count);
        for location in self.vertex_attributes.values() {
            self.gl.disable_vertex_attrib_array(*location);
        }
        self.gl.unuse_program();
    }

    pub fn draw_arrays_instanced(&self, render_states: RenderStates, viewport: Viewport, count: u32, instance_count: u32)
    {
        Self::set_viewport(&self.gl, viewport);
        Self::set_states(&self.gl, render_states);
        self.set_used();
        self.gl.draw_arrays_instanced(consts::TRIANGLES, 0, count, instance_count);
        self.gl.unbind_buffer(consts::ELEMENT_ARRAY_BUFFER);
        for location in self.vertex_attributes.values() {
            self.gl.disable_vertex_attrib_array(*location);
        }
        self.gl.unuse_program();
    }

    pub fn draw_elements(&self, render_states: RenderStates, viewport: Viewport, element_buffer: &buffer::ElementBuffer)
    {
        self.draw_subset_of_elements(render_states, viewport, element_buffer, 0,element_buffer.count() as u32);
    }

    pub fn draw_subset_of_elements(&self, render_states: RenderStates, viewport: Viewport, element_buffer: &buffer::ElementBuffer, first: u32, count: u32)
    {
        Self::set_viewport(&self.gl, viewport);
        Self::set_states(&self.gl, render_states);
        self.set_used();
        element_buffer.bind();
        self.gl.draw_elements(consts::TRIANGLES, count, consts::UNSIGNED_INT, first);
        self.gl.unbind_buffer(consts::ELEMENT_ARRAY_BUFFER);

        for location in self.vertex_attributes.values() {
            self.gl.disable_vertex_attrib_array(*location);
        }
        self.gl.unuse_program();
    }

    pub fn draw_elements_instanced(&self, render_states: RenderStates, viewport: Viewport, element_buffer: &buffer::ElementBuffer, count: u32)
    {
        Self::set_viewport(&self.gl, viewport);
        Self::set_states(&self.gl, render_states);
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

    fn set_states(gl: &Gl, render_states: RenderStates) {
        Self::set_cull(gl, render_states.cull);
        Self::set_color_mask(gl, render_states.color_mask);
        Self::set_depth(gl, Some(render_states.depth_test), render_states.depth_mask);
        Self::set_blend(gl, render_states.blend);
    }

    fn set_viewport(gl: &Gl, viewport: Viewport) {
        unsafe {
            static mut CURRENT_VIEWPORT: Viewport = Viewport {x: 0, y: 0, width: 0, height: 0};
            if viewport != CURRENT_VIEWPORT
            {
                gl.viewport(viewport.x, viewport.y, viewport.width, viewport.height);
                CURRENT_VIEWPORT = viewport;
            }
        }
    }

    fn set_cull(gl: &Gl, cull: CullType) {
        unsafe {
            static mut CURRENT_CULL: CullType = CullType::None;
            if cull != CURRENT_CULL
            {
                match cull {
                    CullType::None => {
                        gl.disable(consts::CULL_FACE);
                    },
                    CullType::Back => {
                        gl.enable(consts::CULL_FACE);
                        gl.cull_face(consts::BACK);
                    },
                    CullType::Front => {
                        gl.enable(consts::CULL_FACE);
                        gl.cull_face(consts::FRONT);
                    },
                    CullType::FrontAndBack => {
                        gl.enable(consts::CULL_FACE);
                        gl.cull_face(consts::FRONT_AND_BACK);
                    }
                }
                CURRENT_CULL = cull;
            }
        }
    }

    fn set_blend(gl: &Gl, blend: Option<BlendParameters>)
    {
        unsafe {
            static mut CURRENT: Option<BlendParameters> = None;
            if blend != CURRENT
            {
                if let Some(blend_parameters) = blend {
                    gl.enable(consts::BLEND);
                    gl.blend_func_separate(Self::blend_const_from_multiplier(blend_parameters.source_rgb_multiplier),
                                           Self::blend_const_from_multiplier(blend_parameters.destination_rgb_multiplier),
                                           Self::blend_const_from_multiplier(blend_parameters.source_alpha_multiplier),
                                           Self::blend_const_from_multiplier(blend_parameters.destination_alpha_multiplier));
                    gl.blend_equation_separate(Self::blend_const_from_equation(blend_parameters.rgb_equation),
                                               Self::blend_const_from_equation(blend_parameters.alpha_equation));
                } else {
                    gl.disable(consts::BLEND);
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
            BlendMultiplierType::SrcAlphaSaturate => consts::SRC_ALPHA_SATURATE
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

    pub(crate) fn set_color_mask(gl: &Gl, color_mask: ColorMask)
    {
        unsafe {
            static mut CURRENT_COLOR_MASK: ColorMask = ColorMask {red: true, green: true, blue: true, alpha: true};
            if color_mask != CURRENT_COLOR_MASK
            {
                gl.color_mask(color_mask.red, color_mask.green, color_mask.blue, color_mask.alpha);
                CURRENT_COLOR_MASK = color_mask;
            }
        }
    }

    pub(crate) fn set_depth(gl: &Gl, depth_test: Option<DepthTestType>, depth_mask: bool) {
        unsafe {
            static mut CURRENT_DEPTH_ENABLE: bool = false;
            static mut CURRENT_DEPTH_MASK: bool = true;
            static mut CURRENT_DEPTH_TEST: DepthTestType = DepthTestType::Less;

            if depth_mask == false && depth_test == Some(DepthTestType::Always) {
                if CURRENT_DEPTH_ENABLE {
                    gl.disable(consts::DEPTH_TEST);
                    CURRENT_DEPTH_ENABLE = false;
                    return;
                }
            }
            else {
                if !CURRENT_DEPTH_ENABLE {
                    gl.enable(consts::DEPTH_TEST);
                    CURRENT_DEPTH_ENABLE = true;
                }
            }

            if depth_mask != CURRENT_DEPTH_MASK
            {
                gl.depth_mask(depth_mask);
                CURRENT_DEPTH_MASK = depth_mask;
            }

            if depth_test.is_some() && depth_test.unwrap() != CURRENT_DEPTH_TEST
            {
                match depth_test.unwrap() {
                    DepthTestType::Never => {
                        gl.depth_func(consts::NEVER);
                    },
                    DepthTestType::Less => {
                        gl.depth_func(consts::LESS);
                    },
                    DepthTestType::Equal => {
                        gl.depth_func(consts::EQUAL);
                    },
                    DepthTestType::LessOrEqual => {
                        gl.depth_func(consts::LEQUAL);
                    },
                    DepthTestType::Greater => {
                        gl.depth_func(consts::GREATER);
                    },
                    DepthTestType::NotEqual => {
                        gl.depth_func(consts::NOTEQUAL);
                    },
                    DepthTestType::GreaterOrEqual => {
                        gl.depth_func(consts::GEQUAL);
                    },
                    DepthTestType::Always => {
                        gl.depth_func(consts::ALWAYS);
                    }
                }
                CURRENT_DEPTH_TEST = depth_test.unwrap();
            }
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        self.gl.delete_program(&self.id);
    }
}