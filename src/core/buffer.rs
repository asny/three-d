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

pub struct Att {
    pub name: String,
    pub no_components: usize
}

pub struct VertexBuffer {
    gl: gl::Gl,
    id: gl::Buffer,
    stride: usize,
    attributes_infos: Vec<Att>
}

impl VertexBuffer
{
    pub fn new(gl: &gl::Gl) -> Result<VertexBuffer, Error>
    {
        let id = gl.create_buffer().unwrap();
        let buffer = VertexBuffer{gl: gl.clone(), id, stride:0, attributes_infos: Vec::new() };
        buffer.bind();
        Ok(buffer)
    }

    pub fn bind(&self)
    {
        bind(&self.gl, &self.id, gl::consts::ARRAY_BUFFER);
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
        self.gl.buffer_data_f32(gl::consts::ARRAY_BUFFER, data, gl::consts::STATIC_DRAW);
    }
}


pub struct ElementBuffer {
    gl: gl::Gl,
    id: gl::Buffer,
}

impl ElementBuffer
{
    pub fn new(gl: &gl::Gl) -> Result<ElementBuffer, Error>
    {
        let id = gl.create_buffer().unwrap();
        let buffer = ElementBuffer{gl: gl.clone(), id };
        bind(&buffer.gl, &buffer.id, gl::consts::ELEMENT_ARRAY_BUFFER);
        Ok(buffer)
    }

    pub fn fill_with(&self, data: &[u32])
    {
        bind(&self.gl, &self.id, gl::consts::ELEMENT_ARRAY_BUFFER);
        self.gl.buffer_data_u32(gl::consts::ELEMENT_ARRAY_BUFFER, data, gl::consts::STATIC_DRAW);
    }
}

fn bind(gl: &gl::Gl, id: &gl::Buffer, buffer_type: u32)
{
    gl.bind_buffer(buffer_type, Some(id));
}
