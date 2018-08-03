use gl;
use std;
use gust::attribute;
pub use std::slice::Iter;

#[derive(Debug)]
pub enum Error {

}

pub struct Att {
    pub name: String,
    pub offset: usize,
    pub no_components: usize
}

pub struct VertexBuffer {
    gl: gl::Gl,
    id: u32,
    stride: usize,
    map: Vec<Att>
}

impl VertexBuffer
{
    pub fn create(gl: &gl::Gl) -> Result<VertexBuffer, Error>
    {
        let mut id: u32 = 0;
        unsafe {
            gl.GenBuffers(1, &mut id);
        }
        let buffer = VertexBuffer{gl: gl.clone(), id, stride:0, map: Vec::new() };
        buffer.bind();
        Ok(buffer)
    }

    pub fn bind(&self)
    {
        bind(&self.gl, self.id, gl::ARRAY_BUFFER);
    }

    pub fn stride(&self) -> usize
    {
        self.stride
    }

    pub fn attributes_iter(&self) -> Iter<Att>
    {
        self.map.iter()
    }

    pub fn fill_from(&mut self, attributes: &Vec<&attribute::Attribute>)
    {
        self.stride = 0;
        self.map = Vec::new();
        let mut no_vertices = 0;
        for attribute in attributes
        {
            self.stride += attribute.no_components();
            no_vertices = attribute.data().len() / attribute.no_components();
        }

        let mut data: Vec<f32> = vec![0.0; self.stride * no_vertices];
        let mut offset = 0;
        for attribute in attributes.iter()
        {
            self.map.push(Att {name: String::from(attribute.name()), offset, no_components: attribute.no_components()});
            for vertex_id in 0..no_vertices
            {
                for d in 0..attribute.no_components()
                {
                    data[offset + vertex_id * self.stride + d] = attribute.data()[vertex_id * attribute.no_components() + d];
                }
            }
            offset = offset + attribute.no_components();
        }

        self.fill_with(&data);
    }

    pub fn fill_with(&mut self, data: &Vec<f32>)
    {
        self.bind();
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

    pub fn fill_with(&self, data: &Vec<u32>)
    {
        bind(&self.gl, self.id, gl::ELEMENT_ARRAY_BUFFER);
        unsafe {
            self.gl.BufferData(
                gl::ELEMENT_ARRAY_BUFFER, // target
                (data.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr, // size of data in bytes
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
