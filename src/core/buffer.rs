use gl;
use std;
use gust::attribute;

#[derive(Debug)]
pub enum Error {

}

pub struct VertexBuffer {
    gl: gl::Gl,
    id: u32,
}

impl VertexBuffer
{
    pub fn create(gl: &gl::Gl) -> Result<VertexBuffer, Error>
    {
        let mut id: u32 = 0;
        unsafe {
            gl.GenBuffers(1, &mut id);
        }
        let buffer = VertexBuffer{gl: gl.clone(), id };
        bind(&buffer.gl, buffer.id, gl::ARRAY_BUFFER);
        Ok(buffer)
    }

    pub fn fill_from(&mut self, attributes: &Vec<&attribute::Attribute>)
    {
        let mut stride = 0;
        let mut no_vertices = 0;
        for attribute in attributes
        {
            stride += attribute.no_components();
            no_vertices = attribute.data().len() / attribute.no_components();
        }

        let mut data: Vec<f32> = Vec::with_capacity(stride * no_vertices);
        unsafe { data.set_len(stride * no_vertices); }
        let mut offset = 0;
        for attribute in attributes.iter()
        {
            for vertex_id in 0..no_vertices
            {
                for d in 0..attribute.no_components()
                {
                    data[offset + vertex_id * stride + d] = attribute.data()[vertex_id * attribute.no_components() + d];
                }
            }
            offset = offset + attribute.no_components();
        }

        self.fill_with(&data);
    }

    pub fn fill_with(&mut self, data: &Vec<f32>)
    {
        bind(&self.gl, self.id, gl::ARRAY_BUFFER);
        unsafe {
            self.gl.BufferData(
                gl::ARRAY_BUFFER, // target
                (data.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
                data.as_ptr() as *const gl::types::GLvoid, // pointer to data
                gl::STATIC_DRAW // usage
            );
        }
    }
}


pub struct ElementBuffer {
    gl: gl::Gl,
    id: u32,
}

impl ElementBuffer
{
    pub fn create(gl: &gl::Gl) -> Result<ElementBuffer, Error>
    {
        let mut id: u32 = 0;
        unsafe {
            gl.GenBuffers(1, &mut id);
        }
        let buffer = ElementBuffer{gl: gl.clone(), id };
        bind(&buffer.gl, buffer.id, gl::ELEMENT_ARRAY_BUFFER);
        Ok(buffer)
    }

    pub fn fill_with(&self, data: &Vec<u16>)
    {
        bind(&self.gl, self.id, gl::ELEMENT_ARRAY_BUFFER);
        unsafe {
            self.gl.BufferData(
                gl::ELEMENT_ARRAY_BUFFER, // target
                (data.len() * std::mem::size_of::<u16>()) as gl::types::GLsizeiptr, // size of data in bytes
                data.as_ptr() as *const gl::types::GLvoid, // pointer to data
                gl::STATIC_DRAW // usage
            );
        }
    }
}



fn bind(gl: &gl::Gl, id: u32, buffer_type: u32)
{
    unsafe {
        static mut CURRENTLY_USED: u32 = std::u32::MAX;
        if id != CURRENTLY_USED
        {
            gl.BindBuffer(buffer_type, id);
            CURRENTLY_USED = id;
        }
    }
}
