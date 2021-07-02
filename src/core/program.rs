use crate::context::{consts, AttributeLocation, Context, ShaderType};
use crate::core::{Error::ProgramError, *};
use crate::math::*;
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
    ) -> Result<Program, Error> {
        let vert_shader = context
            .create_shader(ShaderType::Vertex)
            .ok_or(ProgramError {
                message: "Unable to create Vertex shader object".to_string(),
            })?;
        let frag_shader = context
            .create_shader(ShaderType::Fragment)
            .ok_or(ProgramError {
                message: "Unable to create Fragment shader object".to_string(),
            })?;
        context.compile_shader(vertex_shader_source, &vert_shader);
        context.compile_shader(fragment_shader_source, &frag_shader);

        let id = context.create_program();
        context.attach_shader(&id, &vert_shader);
        context.attach_shader(&id, &frag_shader);
        let success = context.link_program(&id);

        if !success {
            let mut message = "Failed to compile shader program:\n".to_string();
            if let Some(log) = context.get_program_info_log(&id) {
                message = format!("{}\nLink error: {}", message, log);
            }
            if let Some(log) = context.get_shader_info_log(&vert_shader) {
                message = format!("{}\nVertex shader error: {}", message, log);
            }
            if let Some(log) = context.get_shader_info_log(&frag_shader) {
                message = format!("{}\nFragment shader error: {}", message, log);
            }
            return Err(Error::ProgramError { message });
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
            //println!("Uniform location: {:?}, name: {}, type: {}, size: {}", location, info.name(), info.type_(), info.size());
            if let Some(loc) = location {
                uniforms.insert(info.name(), loc);
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
    /// Send the given integer value to this shader program and associate it with the given named variable.
    /// The glsl shader variable must be of type `uniform int`, meaning it is uniformly available across all processing of vertices and fragments.
    ///
    pub fn use_uniform_int(&self, name: &str, data: &i32) -> Result<(), Error> {
        let location = self.get_uniform_location(name)?;
        self.context.uniform1i(location, *data);
        self.context.unuse_program();
        Ok(())
    }

    ///
    /// Send the given float value to this shader program and associate it with the given named variable.
    /// The glsl shader variable must be of type `uniform float`, meaning it is uniformly available across all processing of vertices and fragments.
    ///
    pub fn use_uniform_float(&self, name: &str, data: &f32) -> Result<(), Error> {
        let location = self.get_uniform_location(name)?;
        self.context.uniform1f(location, *data);
        self.context.unuse_program();
        Ok(())
    }

    ///
    /// Send the given [Vec2](crate::Vec2) value to this shader program and associate it with the given named variable.
    /// The glsl shader variable must be of type `uniform vec2`, meaning it is uniformly available across all processing of vertices and fragments.
    ///
    pub fn use_uniform_vec2(&self, name: &str, data: &Vec2) -> Result<(), Error> {
        let location = self.get_uniform_location(name)?;
        self.context.uniform2fv(location, &mut [data.x, data.y]);
        self.context.unuse_program();
        Ok(())
    }

    ///
    /// Send the given [Vec3](crate::Vec3) value to this shader program and associate it with the given named variable.
    /// The glsl shader variable must be of type `uniform vec3`, meaning it is uniformly available across all processing of vertices and fragments.
    ///
    pub fn use_uniform_vec3(&self, name: &str, data: &Vec3) -> Result<(), Error> {
        let location = self.get_uniform_location(name)?;
        self.context
            .uniform3fv(location, &mut [data.x, data.y, data.z]);
        self.context.unuse_program();
        Ok(())
    }

    ///
    /// Send the given [Vec4](crate::Vec4) value to this shader program and associate it with the given named variable.
    /// The glsl shader variable must be of type `uniform vec4`, meaning it is uniformly available across all processing of vertices and fragments.
    ///
    pub fn use_uniform_vec4(&self, name: &str, data: &Vec4) -> Result<(), Error> {
        let location = self.get_uniform_location(name)?;
        self.context
            .uniform4fv(location, &mut [data.x, data.y, data.z, data.w]);
        self.context.unuse_program();
        Ok(())
    }

    ///
    /// Send the given [Mat2](crate::Mat2) value to this shader program and associate it with the given named variable.
    /// The glsl shader variable must be of type `uniform mat2`, meaning it is uniformly available across all processing of vertices and fragments.
    ///
    pub fn use_uniform_mat2(&self, name: &str, data: &Mat2) -> Result<(), Error> {
        let location = self.get_uniform_location(name)?;
        self.context
            .uniform_matrix2fv(location, &mut data.to_slice());
        self.context.unuse_program();
        Ok(())
    }

    ///
    /// Send the given [Mat3](crate::Mat3) value to this shader program and associate it with the given named variable.
    /// The glsl shader variable must be of type `uniform mat3`, meaning it is uniformly available across all processing of vertices and fragments.
    ///
    pub fn use_uniform_mat3(&self, name: &str, data: &Mat3) -> Result<(), Error> {
        let location = self.get_uniform_location(name)?;
        self.context
            .uniform_matrix3fv(location, &mut data.to_slice());
        self.context.unuse_program();
        Ok(())
    }

    ///
    /// Send the given [Mat4](crate::Mat4) value to this shader program and associate it with the given named variable.
    /// The glsl shader variable must be of type `uniform mat4`, meaning it is uniformly available across all processing of vertices and fragments.
    ///
    pub fn use_uniform_mat4(&self, name: &str, data: &Mat4) -> Result<(), Error> {
        let location = self.get_uniform_location(name)?;
        self.context
            .uniform_matrix4fv(location, &mut data.to_slice());
        self.context.unuse_program();
        Ok(())
    }

    fn get_uniform_location(&self, name: &str) -> Result<&crate::context::UniformLocation, Error> {
        self.set_used();
        let loc = self.uniforms.get(name).ok_or_else(|| ProgramError {
            message: format!(
                "The uniform {} is sent to the shader but it is never used.",
                name
            ),
        })?;
        Ok(loc)
    }

    ///
    /// Use the given [Texture2D](crate::Texture2D) in this shader program and associate it with the given named variable.
    /// The glsl shader variable must be of type `uniform sampler2D` and can only be accessed in the fragment shader.
    ///
    pub fn use_texture(&self, name: &str, texture: &impl Texture) -> Result<(), Error> {
        let index = self.get_texture_index(name);
        texture.bind(index);
        self.use_uniform_int(name, &(index as i32))?;
        Ok(())
    }

    ///
    /// Use the given [TextureArray](crate::TextureArray) in this shader program and associate it with the given named variable.
    /// The glsl shader variable must be of type `uniform sampler2DArray` and can only be accessed in the fragment shader.
    ///
    pub fn use_texture_array(&self, name: &str, texture: &impl TextureArray) -> Result<(), Error> {
        let index = self.get_texture_index(name);
        texture.bind(index);
        self.use_uniform_int(name, &(index as i32))?;
        Ok(())
    }

    ///
    /// Use the given [TextureCube](crate::TextureCube) in this shader program and associate it with the given named variable.
    /// The glsl shader variable must be of type `uniform samplerCube` and can only be accessed in the fragment shader.
    ///
    pub fn use_texture_cube(&self, name: &str, texture: &impl TextureCube) -> Result<(), Error> {
        let index = self.get_texture_index(name);
        texture.bind(index);
        self.use_uniform_int(name, &(index as i32))?;
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

    pub fn use_attribute(&self, name: &str, buffer: &VertexBuffer) -> Result<(), Error> {
        self.use_attribute_divisor(name, buffer, 0)?;
        Ok(())
    }

    pub fn use_attribute_divisor(
        &self,
        name: &str,
        buffer: &VertexBuffer,
        divisor: u32,
    ) -> Result<(), Error> {
        if buffer.count() > 0 {
            buffer.bind();
            let loc = self.location(name)?;
            self.context.enable_vertex_attrib_array(loc);
            self.context
                .vertex_attrib_pointer(loc, 1, buffer.data_type(), false, 0, 0);
            self.context.vertex_attrib_divisor(loc, divisor);
            self.context.unbind_buffer(consts::ARRAY_BUFFER);
            self.context.unuse_program();
        }
        Ok(())
    }

    pub fn use_attribute_vec2(&self, name: &str, buffer: &VertexBuffer) -> Result<(), Error> {
        self.use_attribute_vec2_divisor(name, buffer, 0)?;
        Ok(())
    }

    pub fn use_attribute_vec2_divisor(
        &self,
        name: &str,
        buffer: &VertexBuffer,
        divisor: u32,
    ) -> Result<(), Error> {
        if buffer.count() > 0 {
            buffer.bind();
            let loc = self.location(name)?;
            self.context.enable_vertex_attrib_array(loc);
            self.context
                .vertex_attrib_pointer(loc, 2, buffer.data_type(), false, 0, 0);
            self.context.vertex_attrib_divisor(loc, divisor);
            self.context.unbind_buffer(consts::ARRAY_BUFFER);
            self.context.unuse_program();
        }
        Ok(())
    }

    pub fn use_attribute_vec3(&self, name: &str, buffer: &VertexBuffer) -> Result<(), Error> {
        self.use_attribute_vec3_divisor(name, buffer, 0)?;
        Ok(())
    }

    pub fn use_attribute_vec3_divisor(
        &self,
        name: &str,
        buffer: &VertexBuffer,
        divisor: u32,
    ) -> Result<(), Error> {
        if buffer.count() > 0 {
            buffer.bind();
            let loc = self.location(&name)?;
            self.context.enable_vertex_attrib_array(loc);
            self.context
                .vertex_attrib_pointer(loc, 3, buffer.data_type(), false, 0, 0);
            self.context.vertex_attrib_divisor(loc, divisor);
            self.context.unbind_buffer(consts::ARRAY_BUFFER);
            self.context.unuse_program();
        }
        Ok(())
    }

    pub fn use_attribute_vec4(&self, name: &str, buffer: &VertexBuffer) -> Result<(), Error> {
        self.use_attribute_vec4_divisor(name, buffer, 0)?;
        Ok(())
    }

    pub fn use_attribute_vec4_divisor(
        &self,
        name: &str,
        buffer: &VertexBuffer,
        divisor: u32,
    ) -> Result<(), Error> {
        if buffer.count() > 0 {
            buffer.bind();
            let loc = self.location(name)?;
            self.context.enable_vertex_attrib_array(loc);
            self.context
                .vertex_attrib_pointer(loc, 4, buffer.data_type(), false, 0, 0);
            self.context.vertex_attrib_divisor(loc, divisor);
            self.context.unbind_buffer(consts::ARRAY_BUFFER);
            self.context.unuse_program();
        }
        Ok(())
    }

    pub fn draw_arrays(
        &self,
        render_states: RenderStates,
        cull: CullType,
        viewport: Viewport,
        count: u32,
    ) {
        Self::set_viewport(&self.context, viewport);
        Self::set_cull(&self.context, cull);
        Self::set_states(&self.context, render_states);
        self.set_used();
        self.context.draw_arrays(consts::TRIANGLES, 0, count);
        for location in self.vertex_attributes.values() {
            self.context.disable_vertex_attrib_array(*location);
        }
        self.context.unuse_program();
    }

    pub fn draw_arrays_instanced(
        &self,
        render_states: RenderStates,
        cull: CullType,
        viewport: Viewport,
        count: u32,
        instance_count: u32,
    ) {
        Self::set_viewport(&self.context, viewport);
        Self::set_cull(&self.context, cull);
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

    pub fn draw_elements(
        &self,
        render_states: RenderStates,
        cull: CullType,
        viewport: Viewport,
        element_buffer: &ElementBuffer,
    ) {
        self.draw_subset_of_elements(
            render_states,
            cull,
            viewport,
            element_buffer,
            0,
            element_buffer.count() as u32,
        );
    }

    pub fn draw_subset_of_elements(
        &self,
        render_states: RenderStates,
        cull: CullType,
        viewport: Viewport,
        element_buffer: &ElementBuffer,
        first: u32,
        count: u32,
    ) {
        Self::set_viewport(&self.context, viewport);
        Self::set_cull(&self.context, cull);
        Self::set_states(&self.context, render_states);
        self.set_used();
        element_buffer.bind();
        self.context
            .draw_elements(consts::TRIANGLES, count, element_buffer.data_type(), first);
        self.context.unbind_buffer(consts::ELEMENT_ARRAY_BUFFER);

        for location in self.vertex_attributes.values() {
            self.context.disable_vertex_attrib_array(*location);
        }
        self.context.unuse_program();
    }

    pub fn draw_elements_instanced(
        &self,
        render_states: RenderStates,
        cull: CullType,
        viewport: Viewport,
        element_buffer: &ElementBuffer,
        count: u32,
    ) {
        Self::set_viewport(&self.context, viewport);
        Self::set_cull(&self.context, cull);
        Self::set_states(&self.context, render_states);
        self.set_used();
        element_buffer.bind();
        self.context.draw_elements_instanced(
            consts::TRIANGLES,
            element_buffer.count() as u32,
            element_buffer.data_type(),
            0,
            count,
        );
        self.context.unbind_buffer(consts::ELEMENT_ARRAY_BUFFER);
        for location in self.vertex_attributes.values() {
            self.context.disable_vertex_attrib_array(*location);
        }
        self.context.unuse_program();
    }

    fn location(&self, name: &str) -> Result<AttributeLocation, Error> {
        self.set_used();
        let location = self
            .vertex_attributes
            .get(name)
            .ok_or_else(|| ProgramError {
                message: format!(
                    "The attribute {} is sent to the shader but it is never used.",
                    name
                ),
            })?;
        Ok(*location)
    }

    fn set_used(&self) {
        self.context.use_program(&self.id);
    }

    fn set_states(context: &Context, render_states: RenderStates) {
        Self::set_write_mask(context, render_states.write_mask);
        Self::set_clip(context, render_states.clip);
        Self::set_depth(
            context,
            Some(render_states.depth_test),
            render_states.write_mask.depth,
        );
        Self::set_blend(context, render_states.blend);
    }

    fn set_clip(context: &Context, clip: Option<ClipParameters>) {
        unsafe {
            static mut CURRENT: Option<ClipParameters> = None;
            if clip != CURRENT {
                if let Some(clip) = clip {
                    context.enable(consts::SCISSOR_TEST);
                    context.scissor(
                        clip.x as i32,
                        clip.y as i32,
                        clip.width as i32,
                        clip.height as i32,
                    );
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

    fn set_cull(context: &Context, cull: CullType) {
        unsafe {
            static mut CURRENT_CULL: CullType = CullType::None;
            if cull != CURRENT_CULL {
                match cull {
                    CullType::None => {
                        context.disable(consts::CULL_FACE);
                    }
                    CullType::Back => {
                        context.enable(consts::CULL_FACE);
                        context.cull_face(consts::BACK);
                    }
                    CullType::Front => {
                        context.enable(consts::CULL_FACE);
                        context.cull_face(consts::FRONT);
                    }
                    CullType::FrontAndBack => {
                        context.enable(consts::CULL_FACE);
                        context.cull_face(consts::FRONT_AND_BACK);
                    }
                }
                CURRENT_CULL = cull;
            }
        }
    }

    fn set_blend(context: &Context, blend: Option<BlendParameters>) {
        unsafe {
            static mut CURRENT: Option<BlendParameters> = None;
            if blend != CURRENT {
                if let Some(blend_parameters) = blend {
                    context.enable(consts::BLEND);
                    context.blend_func_separate(
                        Self::blend_const_from_multiplier(blend_parameters.source_rgb_multiplier),
                        Self::blend_const_from_multiplier(
                            blend_parameters.destination_rgb_multiplier,
                        ),
                        Self::blend_const_from_multiplier(blend_parameters.source_alpha_multiplier),
                        Self::blend_const_from_multiplier(
                            blend_parameters.destination_alpha_multiplier,
                        ),
                    );
                    context.blend_equation_separate(
                        Self::blend_const_from_equation(blend_parameters.rgb_equation),
                        Self::blend_const_from_equation(blend_parameters.alpha_equation),
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

    fn set_depth(context: &Context, depth_test: Option<DepthTestType>, depth_mask: bool) {
        unsafe {
            static mut CURRENT_DEPTH_ENABLE: bool = false;
            static mut CURRENT_DEPTH_MASK: bool = true;
            static mut CURRENT_DEPTH_TEST: DepthTestType = DepthTestType::Less;

            if depth_mask == false && depth_test == Some(DepthTestType::Always) {
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
                    DepthTestType::Never => {
                        context.depth_func(consts::NEVER);
                    }
                    DepthTestType::Less => {
                        context.depth_func(consts::LESS);
                    }
                    DepthTestType::Equal => {
                        context.depth_func(consts::EQUAL);
                    }
                    DepthTestType::LessOrEqual => {
                        context.depth_func(consts::LEQUAL);
                    }
                    DepthTestType::Greater => {
                        context.depth_func(consts::GREATER);
                    }
                    DepthTestType::NotEqual => {
                        context.depth_func(consts::NOTEQUAL);
                    }
                    DepthTestType::GreaterOrEqual => {
                        context.depth_func(consts::GEQUAL);
                    }
                    DepthTestType::Always => {
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
