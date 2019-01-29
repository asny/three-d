
// GL

#[cfg(target_arch = "x86_64")]
pub mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[cfg(target_arch = "x86_64")]
pub use bindings::*;

#[cfg(target_arch = "x86_64")]
use bindings::Gl as InnerGl;

// WEBGL

#[cfg(target_arch = "wasm32")]
use web_sys::{WebGlRenderingContext};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use WebGlRenderingContext as InnerGl;

#[cfg(target_arch = "wasm32")]
pub type bindings = WebGlRenderingContext;


#[cfg(target_arch = "x86_64")]
mod defines
{
    pub type AttributeLocation = u32;
    pub type UniformLocation = u32;
    pub type Shader = u32;
    pub type Program = u32;
    pub type Buffer = u32;
}

#[cfg(target_arch = "wasm32")]
mod defines
{
    pub type AttributeLocation = u32;
    pub use web_sys::WebGlUniformLocation as UniformLocation;
    pub use web_sys::WebGlShader as Shader;
    pub use web_sys::WebGlProgram as Program;
    pub use web_sys::WebGlBuffer as Buffer;
}

pub use defines::*;


use std::rc::Rc;
use std::ops::Deref;

#[derive(Clone)]
pub struct Gl {
    inner: Rc<InnerGl>,
}

#[cfg(target_arch = "wasm32")]
impl Gl {
    pub fn new(webgl_context: WebGlRenderingContext) -> Gl
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

    pub fn create_program(&self) -> Program
    {
        unsafe { self.inner.CreateProgram() }
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
}

//#[cfg(target_arch = "wasm32")]
impl Deref for Gl {
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
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
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
    use std::ffi::{CStr, CString};
    let c_str: &CStr = &CString::new(source).unwrap();

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
        .get_program_parameter(program, WebGlRenderingContext::LINK_STATUS)
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