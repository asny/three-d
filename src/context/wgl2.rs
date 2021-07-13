use web_sys::WebGl2RenderingContext as InnerGl;

#[allow(non_camel_case_types)]
pub type consts = InnerGl;

pub type AttributeLocation = u32;
use crate::context::{DataType, ShaderType};
pub use web_sys::WebGlActiveInfo as ActiveInfo;
pub use web_sys::WebGlBuffer as Buffer;
pub use web_sys::WebGlFramebuffer as Framebuffer;
pub use web_sys::WebGlProgram as Program;
pub use web_sys::WebGlShader as Shader;
pub use web_sys::WebGlSync as Sync;
pub use web_sys::WebGlTexture as Texture;
pub use web_sys::WebGlUniformLocation as UniformLocation;
pub use web_sys::WebGlVertexArrayObject as VertexArrayObject;

#[derive(Clone)]
pub struct Context {
    inner: std::rc::Rc<InnerGl>,
}

impl Context {
    pub fn new(webgl_context: InnerGl) -> Self {
        Self {
            inner: std::rc::Rc::new(webgl_context),
        }
    }

    pub fn finish(&self) {
        self.inner.finish();
    }

    pub fn bind_buffer_base(&self, target: u32, index: u32, buffer: &Buffer) {
        self.inner.bind_buffer_base(target, index, Some(buffer));
    }

    pub fn bind_buffer(&self, target: u32, buffer: &Buffer) {
        self.inner.bind_buffer(target, Some(buffer));
    }

    pub fn delete_buffer(&self, buffer: &Buffer) {
        self.inner.delete_buffer(Some(buffer));
    }

    pub fn unbind_buffer(&self, target: u32) {
        self.inner.bind_buffer(target, None);
    }

    pub fn buffer_data(&self, target: u32, size_in_bytes: u32, usage: u32) {
        self.inner
            .buffer_data_with_i32(target, size_in_bytes as i32, usage);
    }

    pub fn buffer_data_u8(&self, target: u32, data: &[u8], usage: u32) {
        self.inner.buffer_data_with_u8_array(target, data, usage)
    }

    pub fn buffer_data_u16(&self, target: u32, data: &[u16], usage: u32) {
        use wasm_bindgen::JsCast;
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<js_sys::WebAssembly::Memory>()
            .unwrap()
            .buffer();
        let data_location = data.as_ptr() as u32 / 2;
        let array = js_sys::Uint16Array::new(&memory_buffer)
            .subarray(data_location, data_location + data.len() as u32);

        self.inner
            .buffer_data_with_array_buffer_view(target, &array, usage);
    }

    pub fn buffer_data_u32(&self, target: u32, data: &[u32], usage: u32) {
        use wasm_bindgen::JsCast;
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<js_sys::WebAssembly::Memory>()
            .unwrap()
            .buffer();
        let data_location = data.as_ptr() as u32 / 4;
        let array = js_sys::Uint32Array::new(&memory_buffer)
            .subarray(data_location, data_location + data.len() as u32);

        self.inner
            .buffer_data_with_array_buffer_view(target, &array, usage);
    }

    pub fn buffer_data_f32(&self, target: u32, data: &[f32], usage: u32) {
        use wasm_bindgen::JsCast;
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<js_sys::WebAssembly::Memory>()
            .unwrap()
            .buffer();
        let data_location = data.as_ptr() as u32 / 4;
        let array = js_sys::Float32Array::new(&memory_buffer)
            .subarray(data_location, data_location + data.len() as u32);

        self.inner
            .buffer_data_with_array_buffer_view(target, &array, usage);
    }

    pub fn create_shader(&self, type_: ShaderType) -> Option<Shader> {
        self.inner.create_shader(type_.to_const())
    }

    pub fn compile_shader(&self, source: &str, shader: &Shader) {
        let header = "#version 300 es\nprecision highp float;\nprecision highp int;\nprecision highp sampler2DArray;\n";
        let s: &str = &[header, source].concat();

        self.inner.shader_source(shader, s);
        self.inner.compile_shader(shader);
    }

    pub fn create_program(&self) -> Program {
        self.inner.create_program().unwrap()
    }

    pub fn link_program(&self, program: &Program) -> bool {
        self.inner.link_program(program);
        self.inner
            .get_program_parameter(program, consts::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
    }

    pub fn bind_vertex_array(&self, array: &VertexArrayObject) {
        self.inner.bind_vertex_array(Some(array));
    }

    pub fn delete_texture(&self, texture: &Texture) {
        self.inner.delete_texture(Some(texture));
    }

    pub fn bind_texture(&self, target: u32, texture: &Texture) {
        self.inner.bind_texture(target, Some(texture));
    }

    pub fn tex_storage_2d(
        &self,
        target: u32,
        level: u32,
        internalformat: u32,
        width: u32,
        height: u32,
    ) {
        self.inner.tex_storage_2d(
            target,
            level as i32,
            internalformat,
            width as i32,
            height as i32,
        );
    }

    pub fn tex_storage_3d(
        &self,
        target: u32,
        level: u32,
        internalformat: u32,
        width: u32,
        height: u32,
        depth: u32,
    ) {
        self.inner.tex_storage_3d(
            target,
            level as i32,
            internalformat,
            width as i32,
            height as i32,
            depth as i32,
        );
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
        self.inner
            .tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
                target,
                level as i32,
                internalformat as i32,
                width as i32,
                height as i32,
                border as i32,
                format,
                data_type.to_const(),
                None,
            )
            .unwrap();
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
        self.inner
            .tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_opt_u8_array(
                target,
                level as i32,
                x_offset as i32,
                y_offset as i32,
                width as i32,
                height as i32,
                format,
                data_type.to_const(),
                Some(pixels),
            )
            .unwrap();
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
        self.inner
            .tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
                target,
                level as i32,
                internalformat as i32,
                width as i32,
                height as i32,
                border as i32,
                format,
                data_type.to_const(),
                Some(pixels),
            )
            .unwrap();
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
        use wasm_bindgen::JsCast;
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<js_sys::WebAssembly::Memory>()
            .unwrap()
            .buffer();
        let data_location = pixels.as_ptr() as u32 / 4;
        let array = js_sys::Float32Array::new(&memory_buffer)
            .subarray(data_location, data_location + pixels.len() as u32);

        self.inner
            .tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_opt_array_buffer_view(
                target,
                level as i32,
                x_offset as i32,
                y_offset as i32,
                width as i32,
                height as i32,
                format,
                data_type.to_const(),
                Some(&array),
            )
            .unwrap();
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
        use wasm_bindgen::JsCast;
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<js_sys::WebAssembly::Memory>()
            .unwrap()
            .buffer();
        let data_location = pixels.as_ptr() as u32 / 4;
        let array = js_sys::Uint32Array::new(&memory_buffer)
            .subarray(data_location, data_location + pixels.len() as u32);

        self.inner
            .tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_opt_array_buffer_view(
                target,
                level as i32,
                x_offset as i32,
                y_offset as i32,
                width as i32,
                height as i32,
                format,
                data_type.to_const(),
                Some(&array),
            )
            .unwrap();
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
        use wasm_bindgen::JsCast;
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<js_sys::WebAssembly::Memory>()
            .unwrap()
            .buffer();
        let data_location = pixels.as_ptr() as u32 / 4;
        let array = js_sys::Float32Array::new(&memory_buffer)
            .subarray(data_location, data_location + pixels.len() as u32);

        self.inner
            .tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
                target,
                level as i32,
                internalformat as i32,
                width as i32,
                height as i32,
                border as i32,
                format,
                data_type.to_const(),
                Some(&array),
            )
            .unwrap();
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
        use wasm_bindgen::JsCast;
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<js_sys::WebAssembly::Memory>()
            .unwrap()
            .buffer();
        let data_location = pixels.as_ptr() as u32 / 2;
        let array = js_sys::Uint16Array::new(&memory_buffer)
            .subarray(data_location, data_location + pixels.len() as u32);

        self.inner
            .tex_image_3d_with_opt_array_buffer_view(
                target,
                level as i32,
                internalformat as i32,
                width as i32,
                height as i32,
                depth as i32,
                border as i32,
                format,
                data_type.to_const(),
                Some(&array),
            )
            .unwrap();
    }

    pub fn framebuffer_texture_2d(
        &self,
        target: u32,
        attachment: u32,
        textarget: u32,
        texture: &Texture,
        level: u32,
    ) {
        self.inner.framebuffer_texture_2d(
            target,
            attachment,
            textarget,
            Some(texture),
            level as i32,
        );
    }

    pub fn framebuffer_texture_layer(
        &self,
        target: u32,
        attachment: u32,
        texture: &Texture,
        level: u32,
        layer: u32,
    ) {
        self.inner.framebuffer_texture_layer(
            target,
            attachment,
            Some(texture),
            level as i32,
            layer as i32,
        );
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
        self.inner
            .read_pixels_with_opt_u8_array(
                x as i32,
                y as i32,
                width as i32,
                height as i32,
                format,
                data_type.to_const(),
                Some(dst_data),
            )
            .unwrap()
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
        use wasm_bindgen::JsCast;
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<js_sys::WebAssembly::Memory>()
            .unwrap()
            .buffer();
        let data_location = dst_data.as_ptr() as u32 / 4;
        let array = js_sys::Float32Array::new(&memory_buffer)
            .subarray(data_location, data_location + dst_data.len() as u32);
        self.inner
            .read_pixels_with_opt_array_buffer_view(
                x as i32,
                y as i32,
                width as i32,
                height as i32,
                format,
                data_type.to_const(),
                Some(&array),
            )
            .unwrap();
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
        use wasm_bindgen::JsCast;
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<js_sys::WebAssembly::Memory>()
            .unwrap()
            .buffer();
        let data_location = dst_data.as_ptr() as u32 / 4;
        let array = js_sys::Uint32Array::new(&memory_buffer)
            .subarray(data_location, data_location + dst_data.len() as u32);
        self.inner
            .read_pixels_with_opt_array_buffer_view(
                x as i32,
                y as i32,
                width as i32,
                height as i32,
                format,
                data_type.to_const(),
                Some(&array),
            )
            .unwrap();
    }

    pub fn get_attrib_location(&self, program: &Program, name: &str) -> Option<AttributeLocation> {
        Some(self.inner.get_attrib_location(program, name) as u32)
    }

    pub fn use_program(&self, program: &Program) {
        self.inner.use_program(Some(program));
    }

    pub fn unuse_program(&self) {
        self.inner.use_program(None);
    }

    pub fn delete_program(&self, program: &Program) {
        self.inner.delete_program(Some(program));
    }

    pub fn draw_arrays(&self, mode: u32, first: u32, count: u32) {
        self.inner.draw_arrays(
            mode,
            first as i32, // starting index in the enabled arrays
            count as i32, // number of vertices to be rendered
        );
    }

    pub fn draw_arrays_instanced(&self, mode: u32, first: u32, count: u32, instance_count: u32) {
        self.inner.draw_arrays_instanced(
            mode,
            first as i32, // starting index in the enabled arrays
            count as i32, // number of vertices to be rendered
            instance_count as i32,
        );
    }

    pub fn draw_elements(&self, mode: u32, count: u32, data_type: DataType, offset: u32) {
        self.inner
            .draw_elements_with_i32(mode, count as i32, data_type.to_const(), offset as i32);
    }

    pub fn draw_elements_instanced(
        &self,
        mode: u32,
        count: u32,
        data_type: DataType,
        offset: u32,
        instance_count: u32,
    ) {
        self.inner.draw_elements_instanced_with_i32(
            mode,
            count as i32,
            data_type.to_const(),
            offset as i32,
            instance_count as i32,
        );
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
        self.inner.blit_framebuffer(
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

    pub fn draw_buffers(&self, draw_buffers: &[u32]) {
        use wasm_bindgen::JsCast;
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<js_sys::WebAssembly::Memory>()
            .unwrap()
            .buffer();
        let data_location = draw_buffers.as_ptr() as u32 / 4;
        let array = js_sys::Uint32Array::new(&memory_buffer)
            .subarray(data_location, data_location + draw_buffers.len() as u32);

        self.inner.draw_buffers(&array);
    }

    pub fn check_framebuffer_status(&self) -> Result<(), String> {
        let status = self.inner.check_framebuffer_status(consts::FRAMEBUFFER);

        match status {
            consts::FRAMEBUFFER_COMPLETE => Ok(()),
            consts::FRAMEBUFFER_INCOMPLETE_ATTACHMENT => {
                Err("FRAMEBUFFER_INCOMPLETE_ATTACHMENT".to_string())
            }
            consts::FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT => {
                Err("FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT".to_string())
            }
            consts::FRAMEBUFFER_UNSUPPORTED => Err("FRAMEBUFFER_UNSUPPORTED".to_string()),
            consts::FRAMEBUFFER_INCOMPLETE_MULTISAMPLE => {
                Err("FRAMEBUFFER_INCOMPLETE_MULTISAMPLE".to_string())
            }
            _ => Err("Unknown framebuffer error".to_string()),
        }
    }

    pub fn uniform1f(&self, location: &UniformLocation, data: f32) {
        self.inner.uniform1f(Some(location), data);
    }

    pub fn uniform1i(&self, location: &UniformLocation, data: i32) {
        self.inner.uniform1i(Some(location), data);
    }

    pub fn uniform2fv(&self, location: &UniformLocation, data: &[f32]) {
        self.inner.uniform2fv_with_f32_array(Some(location), data);
    }

    pub fn uniform3fv(&self, location: &UniformLocation, data: &[f32]) {
        self.inner.uniform3fv_with_f32_array(Some(location), data);
    }

    pub fn uniform4fv(&self, location: &UniformLocation, data: &[f32]) {
        self.inner.uniform4fv_with_f32_array(Some(location), data);
    }

    pub fn uniform_matrix2fv(&self, location: &UniformLocation, data: &[f32]) {
        self.inner
            .uniform_matrix2fv_with_f32_array(Some(location), false, data);
    }

    pub fn uniform_matrix3fv(&self, location: &UniformLocation, data: &[f32]) {
        self.inner
            .uniform_matrix3fv_with_f32_array(Some(location), false, data);
    }

    pub fn uniform_matrix4fv(&self, location: &UniformLocation, data: &[f32]) {
        self.inner
            .uniform_matrix4fv_with_f32_array(Some(location), false, data);
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
        self.inner.vertex_attrib_pointer_with_i32(
            location,
            size as i32,
            data_type.to_const(),
            normalized,
            (stride * data_type.byte_size()) as i32,
            (offset * data_type.byte_size()) as i32,
        );
    }

    pub fn get_program_parameter(&self, program: &Program, pname: u32) -> u32 {
        let result = self.inner.get_program_parameter(program, pname);
        result.as_f64().unwrap() as u32
    }

    pub fn get_active_attrib(&self, program: &Program, index: u32) -> ActiveInfo {
        self.inner.get_active_attrib(program, index).unwrap()
    }

    pub fn get_active_uniform(&self, program: &Program, index: u32) -> ActiveInfo {
        self.inner.get_active_uniform(program, index).unwrap()
    }

    pub fn fence_sync(&self) -> Sync {
        self.inner
            .fence_sync(consts::SYNC_GPU_COMMANDS_COMPLETE, 0)
            .unwrap()
    }

    pub fn client_wait_sync(&self, sync: &Sync, flags: u32, timeout: u32) -> u32 {
        self.inner.client_wait_sync_with_u32(sync, flags, timeout)
    }

    pub fn delete_sync(&self, sync: &Sync) {
        self.inner.delete_sync(Some(sync));
    }
}

impl std::ops::Deref for Context {
    type Target = InnerGl;

    fn deref(&self) -> &InnerGl {
        &self.inner
    }
}

impl ShaderType {
    fn to_const(&self) -> u32 {
        match self {
            ShaderType::Vertex => consts::VERTEX_SHADER,
            ShaderType::Fragment => consts::FRAGMENT_SHADER,
        }
    }
}

impl DataType {
    fn to_const(&self) -> u32 {
        match self {
            DataType::Float => consts::FLOAT,
            DataType::Byte => consts::BYTE,
            DataType::UnsignedByte => consts::UNSIGNED_BYTE,
            DataType::Short => consts::SHORT,
            DataType::UnsignedShort => consts::UNSIGNED_SHORT,
            DataType::Int => consts::INT,
            DataType::UnsignedInt => consts::UNSIGNED_INT,
        }
    }
}
