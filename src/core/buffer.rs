use gl;
pub use std::slice::Iter;

#[derive(Debug)]
pub enum Error {
    AttributeNotFound {message: String},
    AttributeHasZeroLength {message: String}
}

#[derive(Clone, Debug)]
pub struct Attribute {
    pub name: String,
    pub no_components: usize,
    pub data: Vec<f32>
}

impl Attribute {
    pub fn new(name: &str, no_components: usize, data: Vec<f32>) -> Result<Attribute, Error>
    {
        if data.len() == 0 { return Err(Error::AttributeHasZeroLength {message: format!("The attribute {} does not contain any data.", name)}); }
        Ok(Attribute {name: name.to_string(), no_components, data})
    }
}

pub struct VertexBuffer {
    gl: gl::Gl,
    id: gl::Buffer,
    stride: usize,
    count: usize,
    offsets: Vec<usize>
}

impl VertexBuffer
{
    pub fn new(gl: &gl::Gl) -> Result<VertexBuffer, Error>
    {
        let id = gl.create_buffer().unwrap();
        let buffer = VertexBuffer{gl: gl.clone(), id, stride: 0, count: 0, offsets: Vec::new() };
        Ok(buffer)
    }

    pub fn new_from_attributes(gl: &gl::Gl, attributes: &[Attribute]) -> Result<VertexBuffer, Error>
    {
        let mut buffer = VertexBuffer::new(gl)?;
        buffer.fill_from_attributes(attributes)?;
        Ok(buffer)
    }

    pub fn bind(&self)
    {
        bind(&self.gl, &self.id, gl::consts::ARRAY_BUFFER);
    }

    pub fn count(&self) -> usize
    {
        self.count
    }

    pub fn stride(&self) -> usize
    {
        self.stride
    }

    pub fn offset_from(&self, index: usize) -> usize
    {
        self.offsets[index]
    }

    pub fn fill_from_attributes(&mut self, attributes: &[Attribute]) -> Result<(), Error>
    {
        self.offsets = Vec::new();
        self.stride = 0;
        self.count = 0;
        for attribute in attributes {
            self.stride = self.stride + attribute.no_components;
            self.count = attribute.data.len() / attribute.no_components;
        }

        let mut data: Vec<f32> = vec![0.0; self.stride * self.count];
        let mut offset = 0;
        for attribute in attributes
        {
            let no_components = attribute.no_components;
            self.offsets.push(offset);
            let mut index = offset;
            for i in 0..self.count {
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
        self.gl.buffer_data_f32(gl::consts::ARRAY_BUFFER, data, gl::consts::STATIC_DRAW);
    }
}


pub struct ElementBuffer {
    gl: gl::Gl,
    id: gl::Buffer,
    count: usize
}

impl ElementBuffer
{
    pub fn new(gl: &gl::Gl) -> Result<ElementBuffer, Error>
    {
        let id = gl.create_buffer().unwrap();
        let buffer = ElementBuffer{ gl: gl.clone(), id, count: 0 };
        Ok(buffer)
    }

    pub fn new_with(gl: &gl::Gl, data: &[u32]) -> Result<ElementBuffer, Error>
    {
        let mut buffer = ElementBuffer::new(gl)?;
        buffer.fill_with(data);
        buffer.count = data.len();
        Ok(buffer)
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn bind(&self)
    {
        bind(&self.gl, &self.id, gl::consts::ELEMENT_ARRAY_BUFFER);
    }

    pub fn fill_with(&mut self, data: &[u32])
    {
        self.bind();
        self.gl.buffer_data_u32(gl::consts::ELEMENT_ARRAY_BUFFER, data, gl::consts::STATIC_DRAW);
    }
}

fn bind(gl: &gl::Gl, id: &gl::Buffer, buffer_type: u32)
{
    gl.bind_buffer(buffer_type, Some(id));
}
