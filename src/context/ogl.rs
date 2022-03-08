pub mod consts {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use std::rc::Rc;

use crate::context::{DataType, ShaderType};
use consts::Gl as InnerGl;

#[derive(Copy, Clone, Debug)]
pub struct AttributeLocation(u32);
#[derive(Copy, Clone, Debug)]
pub struct UniformLocation(u32);
#[derive(Copy, Clone, Debug)]
pub struct Shader(u32);
#[derive(Copy, Clone, Debug)]
pub struct Program(u32);
#[derive(Copy, Clone, Debug)]
pub struct Buffer(u32);
#[derive(Copy, Clone, Debug)]
pub struct Framebuffer(u32);
#[derive(Copy, Clone, Debug)]
pub struct Texture(u32);
#[derive(Copy, Clone, Debug)]
pub struct VertexArrayObject(u32);

pub type Sync = consts::types::GLsync;

pub struct ActiveInfo {
    size: u32,
    type_: u32,
    name: String,
}
impl ActiveInfo {
    pub fn new(size: u32, type_: u32, name: String) -> ActiveInfo {
        ActiveInfo { size, type_, name }
    }
    pub fn size(&self) -> i32 {
        self.size as i32
    }
    pub fn type_(&self) -> u32 {
        self.type_
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

///
/// Contains the graphics API for almost direct calls to OpenGL/WebGL.
/// Used internally in the higher level features and can safely be ignored unless you want more control.
///
/// Calls to this API can be combined with higher level features.
///
#[derive(Clone)]
pub struct GLContext {
    inner: Rc<InnerGl>,
}

impl GLContext {
    pub fn load_with<F>(loadfn: F) -> Self
    where
        for<'r> F: FnMut(&'r str) -> *const consts::types::GLvoid,
    {
        let gl = Self {
            inner: Rc::new(InnerGl::load_with(loadfn)),
        };
        gl.bind_vertex_array(&gl.create_vertex_array().unwrap());
        gl.enable(consts::TEXTURE_CUBE_MAP_SEAMLESS);
        gl
    }

    pub fn finish(&self) {
        unsafe {
            self.inner.Finish();
        }
    }

    pub fn create_shader(&self, type_: ShaderType) -> Option<Shader> {
        let id = unsafe { self.inner.CreateShader(type_.to_const()) };
        Some(Shader(id))
    }

    pub fn compile_shader(&self, source: &str, shader: &Shader) {
        let header = "#version 330 core\n";
        let s: &str = &[header, source].concat();

        use std::ffi::{CStr, CString};
        let c_str: &CStr = &CString::new(s).unwrap();

        unsafe {
            self.inner
                .ShaderSource(shader.0, 1, &c_str.as_ptr(), std::ptr::null());
            self.inner.CompileShader(shader.0);
        }
    }

    pub fn get_shader_info_log(&self, shader: &Shader) -> Option<String> {
        let mut len: consts::types::GLint = 0;
        unsafe {
            self.inner
                .GetShaderiv(shader.0, consts::INFO_LOG_LENGTH, &mut len);
        }

        if len == 0 {
            None
        } else {
            let error = create_whitespace_cstring_with_len(len as usize);
            unsafe {
                self.inner.GetShaderInfoLog(
                    shader.0,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut consts::types::GLchar,
                );
            }
            Some(error.to_string_lossy().into_owned())
        }
    }

    pub fn delete_shader(&self, shader: Option<&Shader>) {
        unsafe {
            self.inner.DeleteShader(shader.unwrap().0);
        }
    }

    pub fn attach_shader(&self, program: &Program, shader: &Shader) {
        unsafe {
            self.inner.AttachShader(program.0, shader.0);
        }
    }

    pub fn detach_shader(&self, program: &Program, shader: &Shader) {
        unsafe {
            self.inner.DetachShader(program.0, shader.0);
        }
    }

    pub fn get_program_parameter(&self, program: &Program, pname: u32) -> u32 {
        let mut out = 0;
        unsafe {
            self.inner.GetProgramiv(program.0, pname, &mut out);
        }
        out as u32
    }

    pub fn get_active_attrib(&self, program: &Program, index: u32) -> ActiveInfo {
        let mut length = 128;
        let mut size = 0;
        let mut _type = 0;
        let name = create_whitespace_cstring_with_len(length as usize);
        unsafe {
            self.inner.GetActiveAttrib(
                program.0,
                index,
                length,
                &mut length,
                &mut size,
                &mut _type,
                name.as_ptr() as *mut consts::types::GLchar,
            );
        }

        let mut s = name.to_string_lossy().into_owned();
        s.truncate(length as usize);
        ActiveInfo::new(size as u32, _type as u32, s)
    }

    pub fn get_active_uniform(&self, program: &Program, index: u32) -> ActiveInfo {
        let mut length = 128;
        let mut size = 0;
        let mut _type = 0;
        let name = create_whitespace_cstring_with_len(length as usize);
        unsafe {
            self.inner.GetActiveUniform(
                program.0,
                index,
                length,
                &mut length,
                &mut size,
                &mut _type,
                name.as_ptr() as *mut consts::types::GLchar,
            );
        }

        let mut s = name.to_string_lossy().into_owned();
        s.truncate(length as usize);
        ActiveInfo::new(size as u32, _type as u32, s)
    }

    pub fn create_buffer(&self) -> Option<Buffer> {
        let mut id: u32 = 0;
        unsafe {
            self.inner.GenBuffers(1, &mut id);
        }
        Some(Buffer(id))
    }

    pub fn delete_buffer(&self, buffer: &Buffer) {
        unsafe {
            self.inner.DeleteBuffers(1, [buffer.0].as_ptr());
        }
    }

    pub fn bind_buffer_base(&self, target: u32, index: u32, buffer: &Buffer) {
        let pname = match target {
            consts::ARRAY_BUFFER => consts::ARRAY_BUFFER_BINDING,
            consts::ELEMENT_ARRAY_BUFFER => consts::ELEMENT_ARRAY_BUFFER_BINDING,
            consts::UNIFORM_BUFFER => consts::UNIFORM_BUFFER_BINDING,
            _ => unreachable!(),
        };

        unsafe {
            let mut current = -1;
            self.inner.GetIntegerv(pname, &mut current);
            if current != 0 {
                println!("{}", current);
                panic!();
            }
            self.inner.BindBufferBase(target, index, buffer.0);
        }
    }

    pub fn bind_buffer(&self, target: u32, buffer: &Buffer) {
        unsafe {
            self.inner.BindBuffer(target, buffer.0);
        }
    }

    pub fn unbind_buffer(&self, target: u32) {
        unsafe {
            self.inner.BindBuffer(target, 0);
        }
    }

    pub fn get_uniform_block_index(&self, program: &Program, name: &str) -> u32 {
        let c_str = std::ffi::CString::new(name).unwrap();
        unsafe { self.inner.GetUniformBlockIndex(program.0, c_str.as_ptr()) }
    }

    pub fn uniform_block_binding(&self, program: &Program, location: u32, index: u32) {
        unsafe {
            self.inner.UniformBlockBinding(program.0, location, index);
        }
    }

    pub fn buffer_data(&self, target: u32, size_in_bytes: u32, usage: u32) {
        unsafe {
            self.inner.BufferData(
                target,
                size_in_bytes as consts::types::GLsizeiptr, // size of data in bytes
                std::ptr::null() as *const consts::types::GLvoid, // pointer to data
                usage,
            );
        }
    }

    pub fn buffer_data_u8(&self, target: u32, data: &[u8], usage: u32) {
        unsafe {
            self.inner.BufferData(
                target,
                (data.len() * std::mem::size_of::<u8>()) as consts::types::GLsizeiptr, // size of data in bytes
                data.as_ptr() as *const consts::types::GLvoid, // pointer to data
                usage,
            );
        }
    }

    pub fn buffer_data_u16(&self, target: u32, data: &[u16], usage: u32) {
        unsafe {
            self.inner.BufferData(
                target,
                (data.len() * std::mem::size_of::<u16>()) as consts::types::GLsizeiptr, // size of data in bytes
                data.as_ptr() as *const consts::types::GLvoid, // pointer to data
                usage,
            );
        }
    }

    pub fn buffer_data_u32(&self, target: u32, data: &[u32], usage: u32) {
        unsafe {
            self.inner.BufferData(
                target,
                (data.len() * std::mem::size_of::<u32>()) as consts::types::GLsizeiptr, // size of data in bytes
                data.as_ptr() as *const consts::types::GLvoid, // pointer to data
                usage,
            );
        }
    }

    pub fn buffer_data_f32(&self, target: u32, data: &[f32], usage: u32) {
        unsafe {
            self.inner.BufferData(
                target,
                (data.len() * std::mem::size_of::<f32>()) as consts::types::GLsizeiptr, // size of data in bytes
                data.as_ptr() as *const consts::types::GLvoid, // pointer to data
                usage,
            );
        }
    }

    pub fn create_vertex_array(&self) -> Option<VertexArrayObject> {
        let mut id: u32 = 0;
        unsafe {
            self.inner.GenVertexArrays(1, &mut id);
        }
        Some(VertexArrayObject(id))
    }

    pub fn bind_vertex_array(&self, array: &VertexArrayObject) {
        unsafe {
            self.inner.BindVertexArray(array.0);
        }
    }

    pub fn create_program(&self) -> Program {
        unsafe { Program(self.inner.CreateProgram()) }
    }

    pub fn link_program(&self, program: &Program) -> bool {
        unsafe {
            self.inner.LinkProgram(program.0);
        }

        let mut success: consts::types::GLint = 1;
        unsafe {
            self.inner
                .GetProgramiv(program.0, consts::LINK_STATUS, &mut success);
        }
        success == 1
    }

    pub fn get_program_info_log(&self, program: &Program) -> Option<String> {
        let mut len: consts::types::GLint = 0;
        unsafe {
            self.inner
                .GetProgramiv(program.0, consts::INFO_LOG_LENGTH, &mut len);
        }

        if len == 0 {
            None
        } else {
            let error = create_whitespace_cstring_with_len(len as usize);
            unsafe {
                self.inner.GetProgramInfoLog(
                    program.0,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut consts::types::GLchar,
                );
            }
            Some(error.to_string_lossy().into_owned())
        }
    }

    pub fn use_program(&self, program: &Program) {
        unsafe {
            self.inner.UseProgram(program.0);
        }
    }

    pub fn unuse_program(&self) {
        unsafe {
            self.inner.UseProgram(0);
        }
    }

    pub fn delete_program(&self, program: &Program) {
        unsafe {
            self.inner.DeleteProgram(program.0);
        }
    }

    pub fn get_attrib_location(&self, program: &Program, name: &str) -> Option<AttributeLocation> {
        let c_str = std::ffi::CString::new(name).unwrap();
        let location = unsafe { self.inner.GetAttribLocation(program.0, c_str.as_ptr()) };
        if location == -1 {
            None
        } else {
            Some(AttributeLocation(location as _))
        }
    }

    pub fn enable_vertex_attrib_array(&self, location: AttributeLocation) {
        unsafe {
            self.inner.EnableVertexAttribArray(location.0);
        }
    }

    pub fn disable_vertex_attrib_array(&self, location: AttributeLocation) {
        unsafe {
            self.inner.DisableVertexAttribArray(location.0);
        }
    }

    pub fn vertex_attrib_pointer(
        &self,
        location: AttributeLocation,
        size: u32,
        data_type: DataType,
        normalized: bool,
        stride: u32,
        offset: u32,
    ) {
        unsafe {
            self.inner.VertexAttribPointer(
                location.0 as consts::types::GLuint, // index of the generic vertex attribute
                size as consts::types::GLint, // the number of components per generic vertex attribute
                data_type.to_const(),         // data type
                normalized as consts::types::GLboolean, // normalized (int-to-float conversion)
                (stride * data_type.byte_size()) as consts::types::GLint, // stride (byte offset between consecutive attributes)
                (offset * data_type.byte_size()) as *const consts::types::GLvoid, // offset of the first component
            );
        }
    }

    pub fn vertex_attrib_divisor(&self, location: AttributeLocation, divisor: u32) {
        unsafe {
            self.inner.VertexAttribDivisor(
                location.0 as consts::types::GLuint,
                divisor as consts::types::GLuint,
            );
        }
    }

    pub fn get_uniform_location(&self, program: &Program, name: &str) -> Option<UniformLocation> {
        let c_str = std::ffi::CString::new(name).unwrap();
        let location = unsafe { self.inner.GetUniformLocation(program.0, c_str.as_ptr()) };
        if location == -1 {
            None
        } else {
            Some(UniformLocation(location as _))
        }
    }

    pub fn uniform1i(&self, location: &UniformLocation, data: i32) {
        unsafe {
            self.inner.Uniform1i(location.0 as i32, data);
        }
    }

    pub fn uniform1iv(&self, location: &UniformLocation, data: &[i32]) {
        unsafe {
            self.inner
                .Uniform1iv(location.0 as i32, data.len() as i32, data.as_ptr());
        }
    }

    pub fn uniform1f(&self, location: &UniformLocation, data: f32) {
        unsafe {
            self.inner.Uniform1f(location.0 as i32, data);
        }
    }

    pub fn uniform1fv(&self, location: &UniformLocation, data: &[f32]) {
        unsafe {
            self.inner
                .Uniform1fv(location.0 as i32, data.len() as i32, data.as_ptr());
        }
    }

    pub fn uniform2fv(&self, location: &UniformLocation, data: &[f32]) {
        unsafe {
            self.inner
                .Uniform2fv(location.0 as i32, data.len() as i32 / 2, data.as_ptr());
        }
    }

    pub fn uniform3fv(&self, location: &UniformLocation, data: &[f32]) {
        unsafe {
            self.inner
                .Uniform3fv(location.0 as i32, data.len() as i32 / 3, data.as_ptr());
        }
    }

    pub fn uniform4fv(&self, location: &UniformLocation, data: &[f32]) {
        unsafe {
            self.inner
                .Uniform4fv(location.0 as i32, data.len() as i32 / 4, data.as_ptr());
        }
    }

    pub fn uniform_matrix2fv(&self, location: &UniformLocation, data: &[f32]) {
        unsafe {
            self.inner.UniformMatrix2fv(
                location.0 as i32,
                data.len() as i32 / 4,
                consts::FALSE,
                data.as_ptr(),
            );
        }
    }

    pub fn uniform_matrix3fv(&self, location: &UniformLocation, data: &[f32]) {
        unsafe {
            self.inner.UniformMatrix3fv(
                location.0 as i32,
                data.len() as i32 / 9,
                consts::FALSE,
                data.as_ptr(),
            );
        }
    }

    pub fn uniform_matrix4fv(&self, location: &UniformLocation, data: &[f32]) {
        unsafe {
            self.inner.UniformMatrix4fv(
                location.0 as i32,
                data.len() as i32 / 16,
                consts::FALSE,
                data.as_ptr(),
            );
        }
    }

    pub fn draw_buffers(&self, draw_buffers: &[u32]) {
        unsafe {
            self.inner
                .DrawBuffers(draw_buffers.len() as i32, draw_buffers.as_ptr());
        }
    }

    pub fn create_framebuffer(&self) -> Option<Framebuffer> {
        let mut id: u32 = 0;
        unsafe {
            self.inner.GenFramebuffers(1, &mut id);
        }
        Some(Framebuffer(id))
    }

    pub fn bind_framebuffer(&self, target: u32, framebuffer: Option<&Framebuffer>) {
        let id = match framebuffer {
            Some(fb) => fb.0,
            None => 0,
        };
        unsafe {
            self.inner.BindFramebuffer(target, id);
        }
    }

    pub fn delete_framebuffer(&self, framebuffer: Option<&Framebuffer>) {
        let id = match framebuffer {
            Some(fb) => &fb.0,
            None => &0,
        };
        unsafe {
            self.inner.DeleteFramebuffers(1, id);
        }
    }

    pub fn check_framebuffer_status(&self) -> Result<(), String> {
        let status = unsafe { self.inner.CheckFramebufferStatus(consts::FRAMEBUFFER) };

        match status {
            consts::FRAMEBUFFER_COMPLETE => Ok(()),
            consts::FRAMEBUFFER_INCOMPLETE_ATTACHMENT => {
                Err("FRAMEBUFFER_INCOMPLETE_ATTACHMENT".to_string())
            }
            consts::FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER => {
                Err("FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER".to_string())
            }
            consts::FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT => {
                Err("FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT".to_string())
            }
            consts::FRAMEBUFFER_UNSUPPORTED => Err("FRAMEBUFFER_UNSUPPORTED".to_string()),
            consts::FRAMEBUFFER_UNDEFINED => Err("FRAMEBUFFER_UNDEFINED".to_string()),
            consts::FRAMEBUFFER_INCOMPLETE_READ_BUFFER => {
                Err("FRAMEBUFFER_INCOMPLETE_READ_BUFFER".to_string())
            }
            consts::FRAMEBUFFER_INCOMPLETE_MULTISAMPLE => {
                Err("FRAMEBUFFER_INCOMPLETE_MULTISAMPLE".to_string())
            }
            consts::FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS => {
                Err("FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS".to_string())
            }
            _ => Err("Unknown framebuffer error".to_string()),
        }
    }

    pub fn blit_framebuffer(
        &self,
        src_x0: u32,
        src_y0: u32,
        src_x1: u32,
        src_y1: u32,
        dst_x0: u32,
        dst_y0: u32,
        dst_x1: u32,
        dst_y1: u32,
        mask: u32,
        filter: u32,
    ) {
        unsafe {
            self.inner.BlitFramebuffer(
                src_x0 as i32,
                src_y0 as i32,
                src_x1 as i32,
                src_y1 as i32,
                dst_x0 as i32,
                dst_y0 as i32,
                dst_x1 as i32,
                dst_y1 as i32,
                mask,
                filter,
            );
        }
    }

    pub fn viewport(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            self.inner.Viewport(x, y, width, height);
        }
    }

    pub fn clear_color(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        unsafe {
            self.inner.ClearColor(red, green, blue, alpha);
        }
    }

    pub fn clear_depth(&self, depth: f32) {
        unsafe {
            self.inner.ClearDepth(depth as f64);
        }
    }

    pub fn clear(&self, mask: u32) {
        unsafe {
            self.inner.Clear(mask);
        }
    }

    pub fn enable(&self, cap: u32) {
        unsafe {
            self.inner.Enable(cap);
        }
    }

    pub fn disable(&self, cap: u32) {
        unsafe {
            self.inner.Disable(cap);
        }
    }

    pub fn blend_func(&self, sfactor: u32, dfactor: u32) {
        unsafe {
            self.inner.BlendFunc(sfactor, dfactor);
        }
    }

    pub fn blend_func_separate(&self, src_rgb: u32, dst_rgb: u32, src_alpha: u32, dst_alpha: u32) {
        unsafe {
            self.inner
                .BlendFuncSeparate(src_rgb, dst_rgb, src_alpha, dst_alpha);
        }
    }

    pub fn blend_equation(&self, mode: u32) {
        unsafe {
            self.inner.BlendEquation(mode);
        }
    }

    pub fn blend_equation_separate(&self, mode_rgb: u32, mode_alpha: u32) {
        unsafe {
            self.inner.BlendEquationSeparate(mode_rgb, mode_alpha);
        }
    }

    pub fn cull_face(&self, mode: u32) {
        unsafe {
            self.inner.CullFace(mode);
        }
    }

    pub fn scissor(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            self.inner.Scissor(x, y, width, height);
        }
    }

    pub fn depth_func(&self, func: u32) {
        unsafe {
            self.inner.DepthFunc(func);
        }
    }

    pub fn color_mask(&self, red: bool, green: bool, blue: bool, alpha: bool) {
        unsafe {
            self.inner.ColorMask(
                if red { consts::TRUE } else { consts::FALSE },
                if green { consts::TRUE } else { consts::FALSE },
                if blue { consts::TRUE } else { consts::FALSE },
                if alpha { consts::TRUE } else { consts::FALSE },
            );
        }
    }

    pub fn depth_mask(&self, flag: bool) {
        unsafe {
            if flag {
                self.inner.DepthMask(consts::TRUE);
            } else {
                self.inner.DepthMask(consts::FALSE);
            }
        }
    }

    pub fn create_texture(&self) -> Option<Texture> {
        let mut id: u32 = 0;
        unsafe {
            self.inner.GenTextures(1, &mut id);
        }
        Some(Texture(id))
    }

    pub fn active_texture(&self, texture: u32) {
        unsafe {
            self.inner.ActiveTexture(texture);
        }
    }

    pub fn bind_texture(&self, target: u32, texture: &Texture) {
        unsafe {
            self.inner.BindTexture(target, texture.0);
        }
    }

    pub fn generate_mipmap(&self, target: u32) {
        unsafe {
            self.inner.GenerateMipmap(target);
        }
    }

    pub fn tex_storage_2d(
        &self,
        target: u32,
        levels: u32,
        internalformat: u32,
        width: u32,
        height: u32,
    ) {
        unsafe {
            self.inner.TexStorage2D(
                target,
                levels as i32,
                internalformat,
                width as i32,
                height as i32,
            );
        }
    }

    pub fn tex_storage_3d(
        &self,
        target: u32,
        levels: u32,
        internalformat: u32,
        width: u32,
        height: u32,
        depth: u32,
    ) {
        unsafe {
            self.inner.TexStorage3D(
                target,
                levels as i32,
                internalformat,
                width as i32,
                height as i32,
                depth as i32,
            );
        }
    }

    pub fn tex_image_2d(
        &self,
        target: u32,
        level: u32,
        internalformat: u32,
        width: u32,
        height: u32,
        border: u32,
        format: u32,
        data_type: DataType,
    ) {
        unsafe {
            self.inner.TexImage2D(
                target,
                level as i32,
                internalformat as i32,
                width as i32,
                height as i32,
                border as i32,
                format,
                data_type.to_const(),
                std::ptr::null() as *const consts::types::GLvoid,
            );
        }
    }

    pub fn tex_image_2d_with_u8_data(
        &self,
        target: u32,
        level: u32,
        internalformat: u32,
        width: u32,
        height: u32,
        border: u32,
        format: u32,
        data_type: DataType,
        pixels: &[u8],
    ) {
        unsafe {
            self.inner.TexImage2D(
                target,
                level as i32,
                internalformat as i32,
                width as i32,
                height as i32,
                border as i32,
                format,
                data_type.to_const(),
                pixels.as_ptr() as *const consts::types::GLvoid,
            );
        }
    }

    pub fn tex_sub_image_2d_with_u8_data(
        &self,
        target: u32,
        level: u32,
        x_offset: u32,
        y_offset: u32,
        width: u32,
        height: u32,
        format: u32,
        data_type: DataType,
        pixels: &[u8],
    ) {
        unsafe {
            self.inner.TexSubImage2D(
                target,
                level as i32,
                x_offset as i32,
                y_offset as i32,
                width as i32,
                height as i32,
                format,
                data_type.to_const(),
                pixels.as_ptr() as *const consts::types::GLvoid,
            );
        }
    }

    pub fn tex_image_2d_with_f32_data(
        &self,
        target: u32,
        level: u32,
        internalformat: u32,
        width: u32,
        height: u32,
        border: u32,
        format: u32,
        data_type: DataType,
        pixels: &[f32],
    ) {
        unsafe {
            self.inner.TexImage2D(
                target,
                level as i32,
                internalformat as i32,
                width as i32,
                height as i32,
                border as i32,
                format,
                data_type.to_const(),
                pixels.as_ptr() as *const consts::types::GLvoid,
            );
        }
    }

    pub fn tex_sub_image_2d_with_f32_data(
        &self,
        target: u32,
        level: u32,
        x_offset: u32,
        y_offset: u32,
        width: u32,
        height: u32,
        format: u32,
        data_type: DataType,
        pixels: &[f32],
    ) {
        unsafe {
            self.inner.TexSubImage2D(
                target,
                level as i32,
                x_offset as i32,
                y_offset as i32,
                width as i32,
                height as i32,
                format,
                data_type.to_const(),
                pixels.as_ptr() as *const consts::types::GLvoid,
            );
        }
    }

    pub fn tex_sub_image_2d_with_u16_data(
        &self,
        target: u32,
        level: u32,
        x_offset: u32,
        y_offset: u32,
        width: u32,
        height: u32,
        format: u32,
        data_type: DataType,
        pixels: &[u16],
    ) {
        unsafe {
            self.inner.TexSubImage2D(
                target,
                level as i32,
                x_offset as i32,
                y_offset as i32,
                width as i32,
                height as i32,
                format,
                data_type.to_const(),
                pixels.as_ptr() as *const consts::types::GLvoid,
            );
        }
    }

    pub fn tex_sub_image_2d_with_u32_data(
        &self,
        target: u32,
        level: u32,
        x_offset: u32,
        y_offset: u32,
        width: u32,
        height: u32,
        format: u32,
        data_type: DataType,
        pixels: &[u32],
    ) {
        unsafe {
            self.inner.TexSubImage2D(
                target,
                level as i32,
                x_offset as i32,
                y_offset as i32,
                width as i32,
                height as i32,
                format,
                data_type.to_const(),
                pixels.as_ptr() as *const consts::types::GLvoid,
            );
        }
    }

    pub fn tex_image_3d(
        &self,
        target: u32,
        level: u32,
        internalformat: u32,
        width: u32,
        height: u32,
        depth: u32,
        border: u32,
        format: u32,
        data_type: DataType,
    ) {
        unsafe {
            self.inner.TexImage3D(
                target,
                level as i32,
                internalformat as i32,
                width as i32,
                height as i32,
                depth as i32,
                border as i32,
                format,
                data_type.to_const(),
                std::ptr::null() as *const consts::types::GLvoid,
            );
        }
    }

    pub fn tex_sub_image_3d_with_u8_data(
        &self,
        target: u32,
        level: u32,
        x_offset: i32,
        y_offset: i32,
        z_offset: i32,
        width: u32,
        height: u32,
        depth: u32,
        format: u32,
        data_type: DataType,
        pixels: &[u8],
    ) {
        unsafe {
            self.inner.TexSubImage3D(
                target,
                level as i32,
                x_offset as i32,
                y_offset as i32,
                z_offset as i32,
                width as i32,
                height as i32,
                depth as i32,
                format,
                data_type.to_const(),
                pixels.as_ptr() as *const consts::types::GLvoid,
            );
        }
    }

    pub fn tex_image_3d_with_u16_data(
        &self,
        target: u32,
        level: u32,
        internalformat: u32,
        width: u32,
        height: u32,
        depth: u32,
        border: u32,
        format: u32,
        data_type: DataType,
        pixels: &[u16],
    ) {
        unsafe {
            self.inner.TexImage3D(
                target,
                level as i32,
                internalformat as i32,
                width as i32,
                height as i32,
                depth as i32,
                border as i32,
                format,
                data_type.to_const(),
                pixels.as_ptr() as *const consts::types::GLvoid,
            );
        }
    }

    pub fn tex_parameteri(&self, target: u32, pname: u32, param: i32) {
        unsafe {
            self.inner.TexParameteri(target, pname, param);
        }
    }

    pub fn delete_texture(&self, texture: &Texture) {
        unsafe {
            self.inner.DeleteTextures(1, &texture.0);
        }
    }

    pub fn framebuffer_texture_2d(
        &self,
        target: u32,
        attachment: u32,
        textarget: u32,
        texture: &Texture,
        level: u32,
    ) {
        unsafe {
            self.inner
                .FramebufferTexture2D(target, attachment, textarget, texture.0, level as i32);
        }
    }

    pub fn framebuffer_texture_layer(
        &self,
        target: u32,
        attachment: u32,
        texture: &Texture,
        level: u32,
        layer: u32,
    ) {
        unsafe {
            self.inner.FramebufferTextureLayer(
                target,
                attachment,
                texture.0,
                level as i32,
                layer as i32,
            );
        }
    }

    pub fn draw_arrays(&self, mode: u32, first: u32, count: u32) {
        unsafe {
            self.inner.DrawArrays(
                mode as consts::types::GLenum,
                first as i32, // starting index in the enabled arrays
                count as i32, // number of vertices to be rendered
            );
        }
    }

    pub fn draw_arrays_instanced(&self, mode: u32, first: u32, count: u32, instance_count: u32) {
        unsafe {
            self.inner.DrawArraysInstanced(
                mode as consts::types::GLenum,
                first as i32,                  // starting index in the enabled arrays
                count as consts::types::GLint, // number of vertices to be rendered
                instance_count as consts::types::GLint,
            );
        }
    }

    pub fn draw_elements(&self, mode: u32, count: u32, data_type: DataType, offset: u32) {
        unsafe {
            self.inner.DrawElements(
                mode as consts::types::GLenum,
                count as consts::types::GLint, // number of indices to be rendered
                data_type.to_const(),
                (offset * data_type.byte_size()) as *const consts::types::GLvoid, // starting index in the enabled arrays
            );
        }
    }

    pub fn draw_elements_instanced(
        &self,
        mode: u32,
        count: u32,
        data_type: DataType,
        offset: u32,
        instance_count: u32,
    ) {
        unsafe {
            self.inner.DrawElementsInstanced(
                mode as consts::types::GLenum,
                count as consts::types::GLint, // number of indices to be rendered
                data_type.to_const(),
                (offset * data_type.byte_size()) as *const consts::types::GLvoid, // starting index in the enabled arrays
                instance_count as consts::types::GLint,
            );
        }
    }

    pub fn read_pixels_with_u8_data(
        &self,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        format: u32,
        data_type: DataType,
        dst_data: &mut [u8],
    ) {
        unsafe {
            self.inner.ReadPixels(
                x as i32,
                y as i32,
                width as i32,
                height as i32,
                format,
                data_type.to_const(),
                dst_data.as_ptr() as *mut consts::types::GLvoid,
            )
        }
    }

    pub fn read_pixels_with_u16_data(
        &self,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        format: u32,
        data_type: DataType,
        dst_data: &mut [u16],
    ) {
        unsafe {
            self.inner.ReadPixels(
                x as i32,
                y as i32,
                width as i32,
                height as i32,
                format,
                data_type.to_const(),
                dst_data.as_ptr() as *mut consts::types::GLvoid,
            )
        }
    }

    pub fn read_pixels_with_f32_data(
        &self,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        format: u32,
        data_type: DataType,
        dst_data: &mut [f32],
    ) {
        unsafe {
            self.inner.ReadPixels(
                x as i32,
                y as i32,
                width as i32,
                height as i32,
                format,
                data_type.to_const(),
                dst_data.as_ptr() as *mut consts::types::GLvoid,
            )
        }
    }

    pub fn read_pixels_with_u32_data(
        &self,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        format: u32,
        data_type: DataType,
        dst_data: &mut [u32],
    ) {
        unsafe {
            self.inner.ReadPixels(
                x as i32,
                y as i32,
                width as i32,
                height as i32,
                format,
                data_type.to_const(),
                dst_data.as_ptr() as *mut consts::types::GLvoid,
            )
        }
    }

    pub fn flush(&self) {
        unsafe {
            self.inner.Flush();
        }
    }

    pub fn fence_sync(&self) -> Sync {
        unsafe { self.inner.FenceSync(consts::SYNC_GPU_COMMANDS_COMPLETE, 0) }
    }

    pub fn client_wait_sync(&self, sync: &Sync, flags: u32, timeout: u32) -> u32 {
        unsafe { self.inner.ClientWaitSync(*sync, flags, timeout as u64) }
    }

    pub fn delete_sync(&self, sync: &Sync) {
        unsafe {
            self.inner.DeleteSync(*sync);
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

impl ShaderType {
    fn to_const(&self) -> u32 {
        match self {
            ShaderType::Vertex => consts::VERTEX_SHADER,
            ShaderType::Fragment => consts::FRAGMENT_SHADER,
        }
    }
}
