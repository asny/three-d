
// GL

#[cfg(target_arch = "x86_64")]
pub mod consts {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[cfg(target_arch = "x86_64")]
use crate::consts::*;

#[cfg(target_arch = "x86_64")]
use crate::consts::Gl as InnerGl;

// WEBGL

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use web_sys::WebGl2RenderingContext as InnerGl;

#[cfg(target_arch = "wasm32")]
#[allow(non_camel_case_types)]
pub type consts = InnerGl;

#[cfg(target_arch = "x86_64")]
mod defines
{
    pub type AttributeLocation = u32;
    pub type UniformLocation = u32;
    pub type Shader = u32;
    pub type Program = u32;
    pub type Buffer = u32;
    pub type Framebuffer = u32;
    pub type Texture = u32;
    pub type VertexArrayObject = u32;
}

#[cfg(target_arch = "wasm32")]
mod defines
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

pub use crate::defines::*;


use std::rc::Rc;

#[derive(Clone)]
pub struct Gl {
    inner: Rc<InnerGl>,
}

#[cfg(target_arch = "wasm32")]
impl Gl {
    pub fn new(webgl_context: InnerGl) -> Gl
    {
        Gl {
            inner: Rc::new(webgl_context)
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

    pub fn create_program(&self) -> Program
    {
        self.inner.create_program().unwrap()
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
        unimplemented!();
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
            consts::FRAMEBUFFER_INCOMPLETE_ATTACHMENT => {Err("Any of the framebuffer attachment points are framebuffer incomplete.".to_string())},
            consts::FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT => {Err("The framebuffer does not have at least one image attached to it.".to_string())},
            consts::FRAMEBUFFER_UNSUPPORTED => {Err("The combination of internal formats of the attached images violates an implementation-dependent set of restrictions.".to_string())},
            consts::FRAMEBUFFER_INCOMPLETE_MULTISAMPLE => {Err("The value of GL_RENDERBUFFER_SAMPLES is not the same for all attached renderbuffers; if the value of GL_TEXTURE_SAMPLES is the not same for all attached textures; or, if the attached images are a mix of renderbuffers and textures, the value of GL_RENDERBUFFER_SAMPLES does not match the value of GL_TEXTURE_SAMPLES.".to_string())},
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

#[cfg(target_arch = "x86_64")]
impl Gl {
    pub fn load_with<F>(loadfn: F) -> Gl
        where for<'r> F: FnMut(&'r str) -> *const types::GLvoid
    {
        Gl {
            inner: Rc::new(InnerGl::load_with(loadfn))
        }
    }

    pub fn delete_shader(&self, shader: Option<&Shader>)
    {
        unsafe {
            self.inner.DeleteShader(*shader.unwrap());
        }
    }

    pub fn attach_shader(&self, program: &Program, shader: &Shader)
    {
        unsafe {
            self.inner.AttachShader(*program, *shader);
        }
    }

    pub fn detach_shader(&self, program: &Program, shader: &Shader)
    {
        unsafe {
            self.inner.DetachShader(*program, *shader);
        }
    }

    pub fn create_buffer(&self) -> Option<Buffer>
    {
        let mut id: u32 = 0;
        unsafe {
            self.inner.GenBuffers(1, &mut id);
        }
        Some(id)
    }

    pub fn bind_buffer(&self, target: u32, buffer: Option<&Buffer>)
    {
        unsafe {
            static mut CURRENTLY_USED: u32 = std::u32::MAX;
            let id = *buffer.unwrap();
            if id != CURRENTLY_USED
            {
                self.inner.BindBuffer(target, id);
                CURRENTLY_USED = id;
            }
        }
    }

    pub fn buffer_data_u32(&self, target: u32, data: &[u32], usage: u32)
    {
        unsafe {
            self.inner.BufferData(
                target,
                (data.len() * std::mem::size_of::<u32>()) as types::GLsizeiptr, // size of data in bytes
                data.as_ptr() as *const types::GLvoid, // pointer to data
                usage
            );
        }
    }

    pub fn buffer_data_f32(&self, target: u32, data: &[f32], usage: u32)
    {
        unsafe {
            self.inner.BufferData(
                target,
                (data.len() * std::mem::size_of::<f32>()) as types::GLsizeiptr, // size of data in bytes
                data.as_ptr() as *const types::GLvoid, // pointer to data
                usage
            );
        }
    }

    pub fn create_vertex_array(&self) -> Option<VertexArrayObject>
    {
        let mut id: u32 = 0;
        unsafe {
            self.inner.GenVertexArrays(1, &mut id);
        }
        Some(id)
    }

    pub fn bind_vertex_array(&self, array: &VertexArrayObject)
    {
        unsafe {
            static mut CURRENTLY_USED: u32 = std::u32::MAX;
            let id = *array;
            if id != CURRENTLY_USED
            {
                self.inner.BindVertexArray(id);
                CURRENTLY_USED = id;
            }
        }
    }

    pub fn create_program(&self) -> Program
    {
        unsafe { self.inner.CreateProgram() }
    }

    pub fn use_program(&self, program: &Program)
    {
        unsafe {
            static mut CURRENTLY_USED: u32 = std::u32::MAX;
            let id = *program;
            if id != CURRENTLY_USED
            {
                self.inner.UseProgram(id);
                CURRENTLY_USED = id;
            }
        }
    }

    pub fn delete_program(&self, program: &Program)
    {
        unsafe {
            self.inner.DeleteProgram(*program);
        }
    }

    pub fn get_attrib_location(&self, program: &Program, name: &str) -> Option<AttributeLocation>
    {
        let c_str = std::ffi::CString::new(name).unwrap();
        let location = unsafe {
            self.inner.GetAttribLocation(*program, c_str.as_ptr())
        };
        if location == -1 { None } else { Some(location as AttributeLocation) }
    }

    pub fn enable_vertex_attrib_array(&self, location: AttributeLocation)
    {
        unsafe {
            self.inner.EnableVertexAttribArray(location);
        }
    }

    pub fn vertex_attrib_pointer(&self, location: AttributeLocation, size: u32, data_type: u32, normalized: bool, stride: u32, offset: u32)
    {
        unsafe {
            self.inner.VertexAttribPointer(
                location as types::GLuint, // index of the generic vertex attribute
                size as types::GLint, // the number of components per generic vertex attribute
                data_type as types::GLenum, // data type
                normalized as types::GLboolean, // normalized (int-to-float conversion)
                byte_size_for_type(data_type, stride) as types::GLint, // stride (byte offset between consecutive attributes)
                byte_size_for_type(data_type, offset) as *const types::GLvoid // offset of the first component
            );
        }
    }

    pub fn vertex_attrib_divisor(&self, location: AttributeLocation, divisor: u32)
    {
        unsafe {
            self.inner.VertexAttribDivisor(location as types::GLuint, divisor as types::GLuint);
        }
    }

    pub fn get_uniform_location(&self, program: &Program, name: &str) -> Option<UniformLocation>
    {
        let c_str = std::ffi::CString::new(name).unwrap();
        let location = unsafe {
            self.inner.GetUniformLocation(*program, c_str.as_ptr())
        };
        if location == -1 { None } else { Some(location as UniformLocation) }
    }

    pub fn uniform1i(&self, location: UniformLocation, data: i32)
    {
        unsafe {
            self.inner.Uniform1i(location as i32, data);
        }
    }

    pub fn uniform1f(&self, location: UniformLocation, data: f32)
    {
        unsafe {
            self.inner.Uniform1f(location as i32, data);
        }
    }

    pub fn uniform2fv(&self, location: UniformLocation, data: &[f32])
    {
        unsafe {
            self.inner.Uniform2fv(location as i32, 1, data.as_ptr());
        }
    }

    pub fn uniform3fv(&self, location: UniformLocation, data: &[f32])
    {
        unsafe {
            self.inner.Uniform3fv(location as i32, 1, data.as_ptr());
        }
    }

    pub fn uniform4fv(&self, location: UniformLocation, data: &[f32])
    {
        unsafe {
            self.inner.Uniform4fv(location as i32, 1, data.as_ptr());
        }
    }

    pub fn uniform_matrix2fv(&self, location: UniformLocation, data: &[f32])
    {
        unsafe {
            self.inner.UniformMatrix2fv(location as i32, 1, FALSE, data.as_ptr());
        }
    }

    pub fn uniform_matrix3fv(&self, location: UniformLocation, data: &[f32])
    {
        unsafe {
            self.inner.UniformMatrix3fv(location as i32, 1, FALSE, data.as_ptr());
        }
    }

    pub fn uniform_matrix4fv(&self, location: UniformLocation, data: &[f32])
    {
        unsafe {
            self.inner.UniformMatrix4fv(location as i32, 1, FALSE, data.as_ptr());
        }
    }

    pub fn draw_buffers(&self, draw_buffers: &[u32])
    {
        unsafe {
            self.inner.DrawBuffers(draw_buffers.len() as i32, draw_buffers.as_ptr());
        }
    }

    pub fn create_framebuffer(&self) -> Option<Framebuffer>
    {
        let mut id: u32 = 0;
        unsafe {
            self.inner.GenFramebuffers(1, &mut id);
        }
        Some(id)
    }

    pub fn bind_framebuffer(&self, target: u32, framebuffer: Option<&Framebuffer>)
    {
        let id = if let Some(fb) = framebuffer {*fb} else {0};
        unsafe {
            static mut CURRENTLY_USED: u32 = std::u32::MAX;
            if id != CURRENTLY_USED
            {
                self.inner.BindFramebuffer(target, id);
                CURRENTLY_USED = id;
            }
        }
    }

    pub fn delete_framebuffer(&self, framebuffer: Option<&Framebuffer>)
    {
        let id = if let Some(fb) = framebuffer {fb} else {&0};
        unsafe {
            self.inner.DeleteFramebuffers(1, id);
        }
    }

    pub fn check_framebuffer_status(&self) -> Result<(), String>
    {
        let status = unsafe {
            self.inner.CheckFramebufferStatus(consts::FRAMEBUFFER)
        };

        match status {
            consts::FRAMEBUFFER_COMPLETE => {Ok(())},
            consts::FRAMEBUFFER_INCOMPLETE_ATTACHMENT => {Err("Any of the framebuffer attachment points are framebuffer incomplete.".to_string())},
            consts::FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER => {Err("The value of GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE is GL_NONE for any color attachment point(s) named by GL_DRAW_BUFFERi.".to_string())},
            consts::FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT => {Err("The framebuffer does not have at least one image attached to it.".to_string())},
            consts::FRAMEBUFFER_UNSUPPORTED => {Err("The combination of internal formats of the attached images violates an implementation-dependent set of restrictions.".to_string())},
            consts::FRAMEBUFFER_UNDEFINED => {Err("The specified framebuffer is the default read or draw framebuffer, but the default framebuffer does not exist.".to_string())},
            consts::FRAMEBUFFER_INCOMPLETE_READ_BUFFER => {Err("GL_READ_BUFFER is not GL_NONE and the value of GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE is GL_NONE for the color attachment point named by GL_READ_BUFFER.".to_string())},
            consts::FRAMEBUFFER_INCOMPLETE_MULTISAMPLE => {Err("The value of GL_RENDERBUFFER_SAMPLES is not the same for all attached renderbuffers; if the value of GL_TEXTURE_SAMPLES is the not same for all attached textures; or, if the attached images are a mix of renderbuffers and textures, the value of GL_RENDERBUFFER_SAMPLES does not match the value of GL_TEXTURE_SAMPLES.".to_string())},
            consts::FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS => {Err("Any framebuffer attachment is layered, and any populated attachment is not layered, or if all populated color attachments are not from textures of the same target.".to_string())},
            _ => {Err("Unknown framebuffer error".to_string())}
        }
    }

    pub fn viewport(&self, x: i32, y: i32, width: i32, height: i32)
    {
        unsafe {
            self.inner.Viewport(x, y, width, height);
        }
    }

    pub fn clear_color(&self, red: f32, green: f32, blue: f32, alpha: f32)
    {
        unsafe {
            self.inner.ClearColor(red, green, blue, alpha);
        }
    }

    pub fn clear(&self, mask: u32)
    {
        unsafe {
            self.inner.Clear(mask);
        }
    }

    pub fn enable(&self, cap: u32)
    {
        unsafe {
            self.inner.Enable(cap);
        }
    }

    pub fn disable(&self, cap: u32)
    {
        unsafe {
            self.inner.Disable(cap);
        }
    }

    pub fn blend_func(&self, sfactor: u32, dfactor: u32)
    {
        unsafe {
            self.inner.BlendFunc(sfactor, dfactor);
        }
    }

    pub fn cull_face(&self, mode: u32)
    {
        unsafe {
            self.inner.CullFace(mode);
        }
    }

    pub fn depth_func(&self, func: u32)
    {
        unsafe {
            self.inner.DepthFunc(func);
        }
    }

    pub fn depth_mask(&self, flag: bool)
    {
        unsafe {
            if flag
            {
                self.inner.DepthMask(TRUE);
            }
            else {
                self.inner.DepthMask(FALSE);
            }
        }
    }

    pub fn create_texture(&self) -> Option<Texture>
    {
        let mut id: u32 = 0;
        unsafe {
            self.inner.GenTextures(1, &mut id);
        }
        Some(id)
    }

    pub fn active_texture(&self, texture: u32)
    {
        unsafe {
            self.inner.ActiveTexture(texture);
        }
    }

    pub fn bind_texture(&self, target: u32, texture: &Texture)
    {
        unsafe {
            self.inner.BindTexture(target, *texture);
        }
    }

    pub fn tex_image_2d(&self, target: u32, level: u32, internalformat: u32, width: u32, height: u32, border: u32, format: u32, data_type: u32)
    {
        unsafe {
            self.inner.TexImage2D(target, level as i32, internalformat as i32, width as i32, height as i32, border as i32, format, data_type, std::ptr::null() as *const types::GLvoid);
        }
    }

    pub fn tex_image_2d_with_u8_data(&self, target: u32, level: u32, internalformat: u32, width: u32, height: u32, border: u32, format: u32, data_type: u32, pixels: &mut [u8])
    {
        unsafe {
            self.inner.TexImage2D(target, level as i32, internalformat as i32, width as i32, height as i32, border as i32, format, data_type, pixels.as_ptr() as *const types::GLvoid);
        }
    }

    pub fn tex_image_2d_with_f32_data(&self, target: u32, level: u32, internalformat: u32, width: u32, height: u32, border: u32, format: u32, data_type: u32, pixels: &mut [f32])
    {
        unsafe {
            self.inner.TexImage2D(target, level as i32, internalformat as i32, width as i32, height as i32, border as i32, format, data_type, pixels.as_ptr() as *const types::GLvoid);
        }
    }

    pub fn tex_parameteri(&self, target: u32, pname: u32, param: i32)
    {
        unsafe {
            self.inner.TexParameteri(target, pname, param);
        }
    }

    pub fn delete_texture(&self, texture: &Texture)
    {
        unsafe {
            self.inner.DeleteTextures(1, texture);
        }
    }

    pub fn framebuffer_texture_2d(&self, target: u32, attachment: u32, textarget: u32, texture: &Texture, level: u32)
    {
        unsafe {
            self.inner.FramebufferTexture2D(target, attachment, textarget, *texture, level as i32);
        }
    }

    pub fn draw_elements(&self, mode: u32, count: u32, data_type: u32, offset: u32)
    {
        unsafe {
            self.inner.DrawElements(
                mode as types::GLenum,
                count as types::GLint, // number of indices to be rendered
                data_type as types::GLenum,
                byte_size_for_type(data_type, offset) as *const types::GLvoid, // starting index in the enabled arrays
            );
        }
    }

    pub fn draw_elements_instanced(&self, mode: u32, count: u32, data_type: u32, offset: u32, instance_count: u32)
    {
        unsafe {
            self.inner.DrawElementsInstanced(
                mode as types::GLenum,
                count as types::GLint, // number of indices to be rendered
                data_type as types::GLenum,
                byte_size_for_type(data_type, offset) as *const types::GLvoid, // starting index in the enabled arrays
                instance_count as types::GLint
            );
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl std::ops::Deref for Gl {
    type Target = InnerGl;

    fn deref(&self) -> &InnerGl {
        &self.inner
    }
}

#[cfg(target_arch = "wasm32")]
pub fn shader_from_source(
    gl: &Gl,
    source: &str,
    shader_type: u32
) -> Result<Shader, String> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    let header = "#version 300 es\nprecision mediump float;\n";
    let s: &str = &[header, source].concat();
    gl.shader_source(&shader, s);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, consts::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(gl
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| "Unknown error creating shader".into()))
    }
}

#[cfg(target_arch = "x86_64")]
pub fn shader_from_source(
    gl: &Gl,
    source: &str,
    shader_type: types::GLenum
) -> Result<Shader, String>
{
    let header = "#version 330 core\nprecision mediump float;\n";
    let s: &str = &[header, source].concat();

    use std::ffi::{CStr, CString};
    let c_str: &CStr = &CString::new(s).unwrap();

    let id = unsafe { gl.inner.CreateShader(shader_type) };
    unsafe {
        gl.inner.ShaderSource(id, 1, &c_str.as_ptr(), std::ptr::null());
        gl.inner.CompileShader(id);
    }

    let mut success: types::GLint = 1;
    unsafe {
        gl.inner.GetShaderiv(id, COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        let mut len: types::GLint = 0;
        unsafe {
            gl.inner.GetShaderiv(id, INFO_LOG_LENGTH, &mut len);
        }

        let error = create_whitespace_cstring_with_len(len as usize);

        unsafe {
            gl.inner.GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut types::GLchar
            );
        }

        return Err(format!("Failed to compile shader due to error: {}", error.to_string_lossy().into_owned()));
    }

    Ok(id)
}

#[cfg(target_arch = "x86_64")]
pub fn link_program(gl: &Gl, program: &Program) -> Result<(), String>
{
    unsafe { gl.inner.LinkProgram(*program); }

    let mut success: types::GLint = 1;
    unsafe {
        gl.inner.GetProgramiv(*program, LINK_STATUS, &mut success);
    }

    if success == 0 {
        let mut len: types::GLint = 0;
        unsafe {
            gl.inner.GetProgramiv(*program, INFO_LOG_LENGTH, &mut len);
        }

        let error = create_whitespace_cstring_with_len(len as usize);

        unsafe {
            gl.inner.GetProgramInfoLog(
                *program,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut types::GLchar
            );
        }

        return Err(error.to_string_lossy().into_owned());;
    }
    Ok(())
}

#[cfg(target_arch = "wasm32")]
pub fn link_program(gl: &Gl, program: &Program) -> Result<(), String>
{
    gl.link_program(program);

    if gl
        .get_program_parameter(program, consts::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(())
    } else {
        Err(gl
            .get_program_info_log(program)
            .unwrap_or_else(|| "Unknown error creating program object".into()))
    }
}

#[cfg(target_arch = "x86_64")]
fn create_whitespace_cstring_with_len(len: usize) -> std::ffi::CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { std::ffi::CString::from_vec_unchecked(buffer) }
}

fn byte_size_for_type(data_type: u32, count: u32) -> u32
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