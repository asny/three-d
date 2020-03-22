
pub mod consts {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use consts::Gl as InnerGl;

pub type AttributeLocation = u32;
pub type UniformLocation = u32;
pub type Shader = u32;
pub type Program = u32;
pub type Buffer = u32;
pub type Framebuffer = u32;
pub type Texture = u32;
pub type VertexArrayObject = u32;
pub struct ActiveInfo { size: u32, type_: u32, name: String }
impl ActiveInfo {
    pub fn new(size: u32, type_: u32, name: String) -> ActiveInfo {ActiveInfo {size, type_, name}}
    pub fn size(&self) -> i32 {self.size as i32}
    pub fn type_(&self) -> u32 {self.type_}
    pub fn name(&self) -> String {self.name.clone()}
}

pub struct Glstruct {
    inner: InnerGl
}

pub type Gl = std::rc::Rc<Glstruct>;

impl Glstruct {
    pub fn load_with<F>(loadfn: F) -> Gl
        where for<'r> F: FnMut(&'r str) -> *const consts::types::GLvoid
    {
        let gl = Glstruct { inner: InnerGl::load_with(loadfn) };
        gl.bind_vertex_array(&gl.create_vertex_array().unwrap());
        std::rc::Rc::new(gl)
    }

    pub fn finish(&self)
    {
        unsafe {
            self.inner.Finish();
        }
    }

    pub fn create_shader(&self, type_: u32) -> Option<Shader>
    {
        let id = unsafe { self.inner.CreateShader(type_) };
        Some(id)
    }

    pub fn compile_shader(&self, source: &str, shader: &Shader) -> Result<(), String>
    {
        let header = "#version 330 core\n";
        let s: &str = &[header, source].concat();

        use std::ffi::{CStr, CString};
        let c_str: &CStr = &CString::new(s).unwrap();

        unsafe {
            self.inner.ShaderSource(*shader, 1, &c_str.as_ptr(), std::ptr::null());
            self.inner.CompileShader(*shader);
        }

        let mut success: consts::types::GLint = 1;
        unsafe {
            self.inner.GetShaderiv(*shader, consts::COMPILE_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: consts::types::GLint = 0;
            unsafe {
                self.inner.GetShaderiv(*shader, consts::INFO_LOG_LENGTH, &mut len);
            }
            let error = create_whitespace_cstring_with_len(len as usize);
            unsafe {
                self.inner.GetShaderInfoLog(*shader, len, std::ptr::null_mut(), error.as_ptr() as *mut consts::types::GLchar);
            }
            return Err(format!("Failed to compile shader due to error: {}", error.to_string_lossy().into_owned()));
        }
        Ok(())
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

    pub fn get_program_parameter(&self, program: &Program, pname: u32) -> u32
    {
        let mut out = 0;
        unsafe {
            self.inner.GetProgramiv(*program, pname, &mut out);
        }
        out as u32
    }

    pub fn get_active_attrib(&self, program: &Program, index: u32) -> ActiveInfo
    {
        let mut length = 128;
        let mut size = 0;
        let mut _type = 0;
        let name = create_whitespace_cstring_with_len(length as usize);
        unsafe {
            self.inner.GetActiveAttrib(*program, index, length, &mut length, &mut size, &mut _type, name.as_ptr() as *mut consts::types::GLchar);
        }

        let mut s = name.to_string_lossy().into_owned();
        s.truncate(length as usize);
        ActiveInfo::new(size as u32, _type as u32, s)
    }

    pub fn get_active_uniform(&self, program: &Program, index: u32) -> ActiveInfo
    {
        let mut length = 128;
        let mut size = 0;
        let mut _type = 0;
        let name = create_whitespace_cstring_with_len(length as usize);
        unsafe {
            self.inner.GetActiveUniform(*program, index, length, &mut length, &mut size, &mut _type, name.as_ptr() as *mut consts::types::GLchar);
        }

        let mut s = name.to_string_lossy().into_owned();
        s.truncate(length as usize);
        ActiveInfo::new(size as u32, _type as u32, s)
    }

    pub fn create_buffer(&self) -> Option<Buffer>
    {
        let mut id: u32 = 0;
        unsafe {
            self.inner.GenBuffers(1, &mut id);
        }
        Some(id)
    }

    pub fn delete_buffer(&self, buffer: &Buffer)
    {
        unsafe {
            self.inner.DeleteBuffers(1, [*buffer].as_ptr());
        }
    }

    pub fn bind_buffer_base(&self, target: u32, index: u32, buffer: &Buffer)
    {
        let pname = match target {
            consts::ARRAY_BUFFER => consts::ARRAY_BUFFER_BINDING,
            consts::ELEMENT_ARRAY_BUFFER => consts::ELEMENT_ARRAY_BUFFER_BINDING,
            consts::UNIFORM_BUFFER => consts::UNIFORM_BUFFER_BINDING,
            _ => unreachable!()
        };

        unsafe {
            let mut current = -1;
            self.inner.GetIntegerv(pname, &mut current);
            if current != 0
            {
                println!("{}", current);
                panic!();
            }
            self.inner.BindBufferBase(target, index, *buffer);
        }
    }

    pub fn bind_buffer(&self, target: u32, buffer: &Buffer)
    {
        let pname = match target {
            consts::ARRAY_BUFFER => consts::ARRAY_BUFFER_BINDING,
            consts::ELEMENT_ARRAY_BUFFER => consts::ELEMENT_ARRAY_BUFFER_BINDING,
            consts::UNIFORM_BUFFER => consts::UNIFORM_BUFFER_BINDING,
            _ => unreachable!()
        };

        unsafe {
            let mut current = -1;
            self.inner.GetIntegerv(pname, &mut current);
            if current != 0
            {
                println!("{}", current);
                panic!();
            }
            self.inner.BindBuffer(target, *buffer);
        }
    }

    pub fn unbind_buffer(&self, target: u32)
    {
        unsafe {
            self.inner.BindBuffer(target, 0);
        }
    }

    pub fn get_uniform_block_index(&self, program: &Program, name: &str) -> u32
    {
        let c_str = std::ffi::CString::new(name).unwrap();
        unsafe {
            self.inner.GetUniformBlockIndex(*program, c_str.as_ptr())
        }
    }

    pub fn uniform_block_binding(&self, program: &Program, location: u32, index: u32)
    {
        unsafe {
            self.inner.UniformBlockBinding(*program, location, index);
        }
    }

    pub fn buffer_data_u32(&self, target: u32, data: &[u32], usage: u32)
    {
        unsafe {
            self.inner.BufferData(
                target,
                (data.len() * std::mem::size_of::<u32>()) as consts::types::GLsizeiptr, // size of data in bytes
                data.as_ptr() as *const consts::types::GLvoid, // pointer to data
                usage
            );
        }
    }

    pub fn buffer_data_f32(&self, target: u32, data: &[f32], usage: u32)
    {
        unsafe {
            self.inner.BufferData(
                target,
                (data.len() * std::mem::size_of::<f32>()) as consts::types::GLsizeiptr, // size of data in bytes
                data.as_ptr() as *const consts::types::GLvoid, // pointer to data
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
            self.inner.BindVertexArray(*array);
        }
    }

    pub fn create_program(&self) -> Program
    {
        unsafe { self.inner.CreateProgram() }
    }

    pub fn link_program(&self, program: &Program) -> Result<(), String>
    {
        unsafe { self.inner.LinkProgram(*program); }

        let mut success: consts::types::GLint = 1;
        unsafe {
            self.inner.GetProgramiv(*program, consts::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: consts::types::GLint = 0;
            unsafe {
                self.inner.GetProgramiv(*program, consts::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                self.inner.GetProgramInfoLog(
                    *program,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut consts::types::GLchar
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }
        Ok(())
    }

    pub fn use_program(&self, program: &Program)
    {
        unsafe {
            let mut current = -1;
            self.inner.GetIntegerv(consts::CURRENT_PROGRAM, &mut current);
            if current != 0
            {
                println!("{}", current);
                panic!();
            }
            self.inner.UseProgram(*program);
        }
    }

    pub fn unuse_program(&self)
    {
        unsafe {
            self.inner.UseProgram(0);
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

    pub fn disable_vertex_attrib_array(&self, location: AttributeLocation)
    {
        unsafe {
            self.inner.DisableVertexAttribArray(location);
        }
    }

    pub fn vertex_attrib_pointer(&self, location: AttributeLocation, size: u32, data_type: u32, normalized: bool, stride: u32, offset: u32)
    {
        unsafe {
            self.inner.VertexAttribPointer(
                location as consts::types::GLuint, // index of the generic vertex attribute
                size as consts::types::GLint, // the number of components per generic vertex attribute
                data_type as consts::types::GLenum, // data type
                normalized as consts::types::GLboolean, // normalized (int-to-float conversion)
                byte_size_for_type(data_type, stride) as consts::types::GLint, // stride (byte offset between consecutive attributes)
                byte_size_for_type(data_type, offset) as *const consts::types::GLvoid // offset of the first component
            );
        }
    }

    pub fn vertex_attrib_divisor(&self, location: AttributeLocation, divisor: u32)
    {
        unsafe {
            self.inner.VertexAttribDivisor(location as consts::types::GLuint, divisor as consts::types::GLuint);
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

    pub fn uniform1i(&self, location: &UniformLocation, data: i32)
    {
        unsafe {
            self.inner.Uniform1i(*location as i32, data);
        }
    }

    pub fn uniform1f(&self, location: &UniformLocation, data: f32)
    {
        unsafe {
            self.inner.Uniform1f(*location as i32, data);
        }
    }

    pub fn uniform2fv(&self, location: &UniformLocation, data: &[f32])
    {
        unsafe {
            self.inner.Uniform2fv(*location as i32, 1, data.as_ptr());
        }
    }

    pub fn uniform3fv(&self, location: &UniformLocation, data: &[f32])
    {
        unsafe {
            self.inner.Uniform3fv(*location as i32, 1, data.as_ptr());
        }
    }

    pub fn uniform4fv(&self, location: &UniformLocation, data: &[f32])
    {
        unsafe {
            self.inner.Uniform4fv(*location as i32, 1, data.as_ptr());
        }
    }

    pub fn uniform_matrix2fv(&self, location: &UniformLocation, data: &[f32])
    {
        unsafe {
            self.inner.UniformMatrix2fv(*location as i32, 1, consts::FALSE, data.as_ptr());
        }
    }

    pub fn uniform_matrix3fv(&self, location: &UniformLocation, data: &[f32])
    {
        unsafe {
            self.inner.UniformMatrix3fv(*location as i32, 1, consts::FALSE, data.as_ptr());
        }
    }

    pub fn uniform_matrix4fv(&self, location: &UniformLocation, data: &[f32])
    {
        unsafe {
            self.inner.UniformMatrix4fv(*location as i32, 1, consts::FALSE, data.as_ptr());
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
        let id = match framebuffer { Some(fb) => *fb, None => 0 };
        unsafe {
            self.inner.BindFramebuffer(target, id);
        }
    }

    pub fn delete_framebuffer(&self, framebuffer: Option<&Framebuffer>)
    {
        let id = match framebuffer { Some(fb) => fb, None => &0 };
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
            consts::FRAMEBUFFER_INCOMPLETE_ATTACHMENT => {Err("FRAMEBUFFER_INCOMPLETE_ATTACHMENT".to_string())},
            consts::FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER => {Err("FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER".to_string())},
            consts::FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT => {Err("FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT".to_string())},
            consts::FRAMEBUFFER_UNSUPPORTED => {Err("FRAMEBUFFER_UNSUPPORTED".to_string())},
            consts::FRAMEBUFFER_UNDEFINED => {Err("FRAMEBUFFER_UNDEFINED".to_string())},
            consts::FRAMEBUFFER_INCOMPLETE_READ_BUFFER => {Err("FRAMEBUFFER_INCOMPLETE_READ_BUFFER".to_string())},
            consts::FRAMEBUFFER_INCOMPLETE_MULTISAMPLE => {Err("FRAMEBUFFER_INCOMPLETE_MULTISAMPLE".to_string())},
            consts::FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS => {Err("FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS".to_string())},
            _ => {Err("Unknown framebuffer error".to_string())}
        }
    }

    pub fn blit_framebuffer(&self, src_x0: u32, src_y0: u32, src_x1: u32, src_y1: u32,
                                    dst_x0: u32, dst_y0: u32, dst_x1: u32, dst_y1: u32, mask: u32, filter: u32)
    {
        unsafe {
            self.inner.BlitFramebuffer(src_x0 as i32, src_y0 as i32, src_x1 as i32, src_y1 as i32,
                                        dst_x0 as i32, dst_y0 as i32, dst_x1 as i32, dst_y1 as i32, mask, filter);
        }
    }

    pub fn viewport(&self, x: i32, y: i32, width: usize, height: usize)
    {
        unsafe {
            self.inner.Viewport(x, y, width as i32, height as i32);
        }
    }

    pub fn clear_color(&self, red: f32, green: f32, blue: f32, alpha: f32)
    {
        unsafe {
            self.inner.ClearColor(red, green, blue, alpha);
        }
    }

    pub fn clear_depth(&self, depth: f32)
    {
        unsafe {
            self.inner.ClearDepth(depth as f64);
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
                self.inner.DepthMask(consts::TRUE);
            }
            else {
                self.inner.DepthMask(consts::FALSE);
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

    pub fn tex_storage_2d(&self, target: u32, level: u32, internalformat: u32, width: u32, height: u32)
    {
        unsafe {
            self.inner.TexStorage2D(target, level as i32, internalformat, width as i32, height as i32);
        }
    }

    pub fn tex_storage_3d(&self, target: u32, level: u32, internalformat: u32, width: u32, height: u32, depth: u32)
    {
        unsafe {
            self.inner.TexStorage3D(target, level as i32, internalformat, width as i32, height as i32, depth as i32);
        }
    }

    pub fn tex_image_2d(&self, target: u32, level: u32, internalformat: u32, width: u32, height: u32, border: u32, format: u32, data_type: u32)
    {
        unsafe {
            self.inner.TexImage2D(target, level as i32, internalformat as i32, width as i32, height as i32, border as i32, format, data_type, std::ptr::null() as *const consts::types::GLvoid);
        }
    }

    pub fn tex_image_2d_with_u8_data(&self, target: u32, level: u32, internalformat: u32, width: u32, height: u32, border: u32, format: u32, data_type: u32, pixels: &mut [u8])
    {
        unsafe {
            self.inner.TexImage2D(target, level as i32, internalformat as i32, width as i32, height as i32, border as i32, format, data_type, pixels.as_ptr() as *const consts::types::GLvoid);
        }
    }

    pub fn tex_image_2d_with_f32_data(&self, target: u32, level: u32, internalformat: u32, width: u32, height: u32, border: u32, format: u32, data_type: u32, pixels: &mut [f32])
    {
        unsafe {
            self.inner.TexImage2D(target, level as i32, internalformat as i32, width as i32, height as i32, border as i32, format, data_type, pixels.as_ptr() as *const consts::types::GLvoid);
        }
    }

    pub fn tex_image_3d(&self, target: u32, level: u32, internalformat: u32, width: u32, height: u32, depth: u32, format: u32, data_type: u32)
    {
        unsafe {
            self.inner.TexImage3D(target, level as i32, internalformat as i32, width as i32, height as i32, depth as i32, 0, format, data_type, std::ptr::null() as *const consts::types::GLvoid);
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

    pub fn framebuffer_texture_layer(&self, target: u32, attachment: u32, texture: &Texture, level: u32, layer: u32)
    {
        unsafe {
            self.inner.FramebufferTextureLayer(target, attachment, *texture, level as i32, layer as i32);
        }
    }

    pub fn draw_arrays(&self, mode: u32, first: u32, count: u32)
    {
        unsafe {
            self.inner.DrawArrays(
                mode as consts::types::GLenum,
                first as i32, // starting index in the enabled arrays
                count as i32 // number of vertices to be rendered
            );
        }
    }

    pub fn draw_arrays_instanced(&self, mode: u32, first: u32, count: u32, instance_count: u32)
    {
        unsafe {
            self.inner.DrawArraysInstanced(
                mode as consts::types::GLenum,
                first as i32, // starting index in the enabled arrays
                count as consts::types::GLint, // number of vertices to be rendered
                instance_count as consts::types::GLint
            );
        }
    }

    pub fn draw_elements(&self, mode: u32, count: u32, data_type: u32, offset: u32)
    {
        unsafe {
            self.inner.DrawElements(
                mode as consts::types::GLenum,
                count as consts::types::GLint, // number of indices to be rendered
                data_type as consts::types::GLenum,
                byte_size_for_type(data_type, offset) as *const consts::types::GLvoid, // starting index in the enabled arrays
            );
        }
    }

    pub fn draw_elements_instanced(&self, mode: u32, count: u32, data_type: u32, offset: u32, instance_count: u32)
    {
        unsafe {
            self.inner.DrawElementsInstanced(
                mode as consts::types::GLenum,
                count as consts::types::GLint, // number of indices to be rendered
                data_type as consts::types::GLenum,
                byte_size_for_type(data_type, offset) as *const consts::types::GLvoid, // starting index in the enabled arrays
                instance_count as consts::types::GLint
            );
        }
    }

    pub fn read_pixels(&self, x: u32, y: u32, width: u32, height: u32, format: u32, data_type: u32, dst_data: &mut [u8])
    {
        unsafe {
            self.inner.ReadPixels(x as i32, y as i32, width as i32, height as i32, format, data_type, dst_data.as_ptr() as *mut consts::types::GLvoid)
        }
    }

    pub fn read_depths(&self, x: u32, y: u32, width: u32, height: u32, format: u32, data_type: u32, dst_data: &mut [f32])
    {
        unsafe {
            self.inner.ReadPixels(x as i32, y as i32, width as i32, height as i32, format, data_type, dst_data.as_ptr() as *mut consts::types::GLvoid)
        }
    }
}

fn create_whitespace_cstring_with_len(len: usize) -> std::ffi::CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { std::ffi::CString::from_vec_unchecked(buffer) }
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