
use web_sys::WebGl2RenderingContext as InnerGl;

#[allow(non_camel_case_types)]
pub type consts = InnerGl;

pub mod defines
{
    pub type AttributeLocation = u32;
    pub use web_sys::WebGlUniformLocation as UniformLocation;
    pub use web_sys::WebGlShader as Shader;
    pub use web_sys::WebGlProgram as Program;
    pub use web_sys::WebGlBuffer as Buffer;
    pub use web_sys::WebGlFramebuffer as Framebuffer;
    pub use web_sys::WebGlTexture as Texture;
    pub use web_sys::WebGlVertexArrayObject as VertexArrayObject;
}
pub use crate::wgl2::defines::*;

#[derive(Clone)]
pub struct Gl {
    inner: std::rc::Rc<InnerGl>,
}

impl Gl {
    pub fn new(webgl_context: InnerGl) -> Gl
    {
        Gl {
            inner: std::rc::Rc::new(webgl_context)
        }
    }

    pub fn buffer_data_u32(&self, target: u32, data: &[u32], usage: u32)
    {
        use wasm_bindgen::JsCast;
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<js_sys::WebAssembly::Memory>().unwrap()
            .buffer();
        let data_location = data.as_ptr() as u32 / 4;
        let array = js_sys::Uint32Array::new(&memory_buffer)
            .subarray(data_location, data_location + data.len() as u32);

        self.inner.buffer_data_with_array_buffer_view(
            target,
            &array,
            usage
        );
    }

    pub fn buffer_data_f32(&self, target: u32, data: &[f32], usage: u32)
    {
        use wasm_bindgen::JsCast;
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<js_sys::WebAssembly::Memory>().unwrap()
            .buffer();
        let data_location = data.as_ptr() as u32 / 4;
        let array = js_sys::Float32Array::new(&memory_buffer)
            .subarray(data_location, data_location + data.len() as u32);

        self.inner.buffer_data_with_array_buffer_view(
            target,
            &array,
            usage
        );
    }

    pub fn compile_shader(&self, source: &str, shader: &Shader) -> Result<(), String>
    {
        let header = "#version 300 es\nprecision highp float;\n";
        let s: &str = &[header, source].concat();

        self.inner.shader_source(shader, s);
        self.inner.compile_shader(shader);

        if self.inner.get_shader_parameter(shader, consts::COMPILE_STATUS).as_bool().unwrap_or(false)
        {
            Ok(())
        } else {
            Err(self.inner.get_shader_info_log(shader).unwrap_or_else(|| "Unknown error creating shader".into()))
        }
    }

    pub fn create_program(&self) -> Program
    {
        self.inner.create_program().unwrap()
    }

    pub fn link_program(&self, program: &Program) -> Result<(), String>
    {
        self.inner.link_program(program);
        if self.inner.get_program_parameter(program, consts::LINK_STATUS).as_bool().unwrap_or(false)
        {
            Ok(())
        } else {
            Err(self.inner.get_program_info_log(program).unwrap_or_else(|| "Unknown error creating program object".into()))
        }
    }

    pub fn bind_vertex_array(&self, array: &VertexArrayObject)
    {
        self.inner.bind_vertex_array(Some(array));
    }

    pub fn delete_texture(&self, texture: &Texture)
    {
        self.inner.delete_texture(Some(texture));
    }

    pub fn bind_texture(&self, target: u32, texture: &Texture)
    {
        self.inner.bind_texture(target, Some(texture));
    }

    pub fn tex_storage_2d(&self, target: u32, level: u32, internalformat: u32, width: u32, height: u32)
    {
        self.inner.tex_storage_2d(target, level as i32, internalformat, width as i32, height as i32);
    }

    pub fn tex_image_2d(&self, target: u32, level: u32, internalformat: u32, width: u32, height: u32, border: u32, format: u32, data_type: u32)
    {
        self.inner.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(target,
                                                                                              level as i32,
                                                                                              internalformat as i32,
                                                                                              width as i32,
                                                                                              height as i32,
                                                                                              border as i32,
                                                                                              format,
                                                                                              data_type,
                                                                                              None).unwrap();
    }

    pub fn tex_image_2d_with_u8_data(&self, target: u32, level: u32, internalformat: u32, width: u32, height: u32, border: u32, format: u32, data_type: u32, pixels: &mut [u8])
    {
        self.inner.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(target,
                                                                                              level as i32,
                                                                                              internalformat as i32,
                                                                                              width as i32,
                                                                                              height as i32,
                                                                                              border as i32,
                                                                                              format,
                                                                                              data_type,
                                                                                              Some(pixels)).unwrap();
    }

    pub fn tex_image_2d_with_f32_data(&self, target: u32, level: u32, internalformat: u32, width: u32, height: u32, border: u32, format: u32, data_type: u32, pixels: &mut [f32])
    {
        use wasm_bindgen::JsCast;
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<js_sys::WebAssembly::Memory>().unwrap()
            .buffer();
        let data_location = pixels.as_ptr() as u32 / 4;
        let array = js_sys::Float32Array::new(&memory_buffer)
            .subarray(data_location, data_location + pixels.len() as u32);

        self.inner.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(target,
                                                                                              level as i32,
                                                                                              internalformat as i32,
                                                                                              width as i32,
                                                                                              height as i32,
                                                                                              border as i32,
                                                                                              format,
                                                                                              data_type,
                                                                                              Some(&array)).unwrap();

    }

    pub fn framebuffer_texture_2d(&self, target: u32, attachment: u32, textarget: u32, texture: &Texture, level: u32)
    {
        self.inner.framebuffer_texture_2d(target, attachment, textarget, Some(texture), level as i32);
    }

    pub fn get_attrib_location(&self, program: &Program, name: &str) -> Option<AttributeLocation>
    {
        Some(self.inner.get_attrib_location(program, name) as u32)
    }

    pub fn use_program(&self, program: &Program)
    {
        self.inner.use_program(Some(program));
    }

    pub fn delete_program(&self, program: &Program)
    {
        self.inner.delete_program(Some(program));
    }

    pub fn draw_elements(&self, mode: u32, count: u32, data_type: u32, offset: u32)
    {
        self.inner.draw_elements_with_i32(mode, count as i32, data_type, offset as i32);
    }

    pub fn draw_elements_instanced(&self, mode: u32, count: u32, data_type: u32, offset: u32, instance_count: u32)
    {
        self.inner.draw_elements_instanced_with_i32(mode, count as i32, data_type, offset as i32, instance_count as i32);
    }

    pub fn draw_buffers(&self, draw_buffers: &[u32])
    {
        use wasm_bindgen::JsCast;
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<js_sys::WebAssembly::Memory>().unwrap()
            .buffer();
        let data_location = draw_buffers.as_ptr() as u32 / 4;
        let array = js_sys::Uint32Array::new(&memory_buffer)
            .subarray(data_location, data_location + draw_buffers.len() as u32);

        self.inner.draw_buffers(&array);
    }

    pub fn check_framebuffer_status(&self) -> Result<(), String>
    {
        let status = self.inner.check_framebuffer_status(consts::FRAMEBUFFER);

        match status {
            consts::FRAMEBUFFER_COMPLETE => {Ok(())},
            consts::FRAMEBUFFER_INCOMPLETE_ATTACHMENT => {Err("FRAMEBUFFER_INCOMPLETE_ATTACHMENT".to_string())},
            consts::FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT => {Err("FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT".to_string())},
            consts::FRAMEBUFFER_UNSUPPORTED => {Err("FRAMEBUFFER_UNSUPPORTED".to_string())},
            consts::FRAMEBUFFER_INCOMPLETE_MULTISAMPLE => {Err("FRAMEBUFFER_INCOMPLETE_MULTISAMPLE".to_string())},
            _ => {Err("Unknown framebuffer error".to_string())}
        }
    }

    pub fn uniform1f(&self, location: UniformLocation, data: f32)
    {
        self.inner.uniform1f(Some(&location), data);
    }

    pub fn uniform1i(&self, location: UniformLocation, data: i32)
    {
        self.inner.uniform1i(Some(&location), data);
    }

    pub fn uniform2fv(&self, location: UniformLocation, data: &mut [f32])
    {
        self.inner.uniform2fv_with_f32_array(Some(&location), data);
    }

    pub fn uniform3fv(&self, location: UniformLocation, data: &mut [f32])
    {
        self.inner.uniform3fv_with_f32_array(Some(&location), data);
    }

    pub fn uniform4fv(&self, location: UniformLocation, data: &mut [f32])
    {
        self.inner.uniform4fv_with_f32_array(Some(&location), data);
    }

    pub fn uniform_matrix2fv(&self, location: UniformLocation, data: &mut [f32])
    {
        self.inner.uniform_matrix2fv_with_f32_array(Some(&location), false, data);
    }

    pub fn uniform_matrix3fv(&self, location: UniformLocation, data: &mut [f32])
    {
        self.inner.uniform_matrix3fv_with_f32_array(Some(&location), false, data);
    }

    pub fn uniform_matrix4fv(&self, location: UniformLocation, data: &mut [f32])
    {
        self.inner.uniform_matrix4fv_with_f32_array(Some(&location), false, data);
    }

    pub fn vertex_attrib_pointer(&self, location: AttributeLocation, size: u32, data_type: u32, normalized: bool, stride: u32, offset: u32)
    {
        self.inner.vertex_attrib_pointer_with_i32(location, size as i32, data_type, normalized,
                                                  byte_size_for_type(data_type, stride) as i32, byte_size_for_type(data_type, offset)  as i32);
    }
}

impl std::ops::Deref for Gl {
    type Target = InnerGl;

    fn deref(&self) -> &InnerGl {
        &self.inner
    }
}

pub fn byte_size_for_type(data_type: u32, count: u32) -> u32
{
    match data_type {
        consts::FLOAT => {
            count * std::mem::size_of::<f32>() as u32
        },
        consts::UNSIGNED_INT => {
            count * std::mem::size_of::<u32>() as u32
        }
        _ => { 0 }
    }
}