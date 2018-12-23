use gl;
pub use std::slice::Iter;

#[derive(Debug)]
pub enum Error {
    AttributeNotFound {message: String}
}

#[derive(Clone, Debug)]
pub struct Attribute {
    pub name: String,
    pub no_components: usize,
    pub data: Vec<f32>
}

impl Attribute {
    pub fn new(name: &str, no_components: usize, data: Vec<f32>) -> Attribute
    {
        Attribute {name: name.to_string(), no_components, data}
    }
}

pub struct Att {
    pub name: String,
    pub no_components: usize
}

pub struct VertexBuffer {
    gl: gl::Gl,
    id: u32,
    stride: usize,
    attributes_infos: Vec<Att>
}

impl VertexBuffer
{
    pub fn create(gl: &gl::Gl) -> Result<VertexBuffer, Error>
    {
        let mut id: u32 = 0;
        unsafe {
            gl.GenBuffers(1, &mut id);
        }
        let buffer = VertexBuffer{gl: gl.clone(), id, stride:0, attributes_infos: Vec::new() };
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
        self.attributes_infos.iter()
    }

    pub fn fill_from_attributes(&mut self, attributes: &[Attribute]) -> Result<(), Error>
    {
        self.attributes_infos = Vec::new();
        let mut no_vertices = 0;
        self.stride = 0;
        for attribute in attributes {
            self.stride = self.stride + attribute.no_components;
            no_vertices = attribute.data.len() / attribute.no_components;
        }

        let mut data: Vec<f32> = vec![0.0; self.stride * no_vertices];
        let mut offset = 0;
        for attribute in attributes
        {
            let no_components = attribute.no_components;
            self.attributes_infos.push(Att {name: attribute.name.clone(), no_components});
            let mut index = offset;
            for i in 0..no_vertices {
                for j in 0..no_components {
                    data[index + j] = attribute.data[i * no_components + j];
                }
                index += self.stride;
            }
            offset = offset + no_components;
        }

        self.fill_with(&data);
        Ok(())
    }

    pub fn fill_with(&mut self, data: &[f32])
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

    pub fn fill_with(&self, data: &[u32])
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
