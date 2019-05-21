use gl;
pub use std::slice::Iter;

#[derive(Debug)]
pub enum Error {
    AttributeNotFound {message: String},
    AttributeHasZeroLength {message: String}
}

pub struct VertexBufferBuilder {
    gl: gl::Gl,
    data: Vec<(Vec<f32>, usize)>
}

impl VertexBufferBuilder {
    pub fn new(gl: &gl::Gl) -> Result<VertexBufferBuilder, Error>
    {
        Ok(VertexBufferBuilder{gl: gl.clone(), data: Vec::new() })
    }

    pub fn add(&mut self, data: Vec<f32>, no_components: usize)
    {
        self.data.push((data, no_components));
    }

    pub fn build(&self) -> Result<VertexBuffer, Error>
    {
        // TODO: not use interleaved when not the same count (no_vertices)
        let mut no_vertices = 0;
        let mut offsets = Vec::new();
        let mut stride = 0;
        for (data, no_components) in self.data.iter() {

            no_vertices = data.len() / no_components;
            offsets.push(stride);
            stride += no_components;
        }

        let mut data_out: Vec<f32> = vec![0.0; stride * no_vertices];
        let mut offset = 0;
        for (data, no_components) in self.data.iter()
        {
            let mut index = offset;
            for i in 0..no_vertices {
                for j in 0..*no_components {
                    data_out[index + j] = data[i * no_components + j];
                }
                index += stride;
            }
            offset += no_components;
        }

        VertexBuffer::new_with_data(&self.gl, no_vertices, stride, offsets, &data_out)
    }

    pub fn new_with_vec3(gl: &gl::Gl, attribute: Vec<f32>) -> Result<VertexBuffer, Error>
    {
        let mut builder = VertexBufferBuilder::new(gl)?;
        builder.add(attribute, 3);
        builder.build()
    }

    pub fn new_with_vec3_vec3(gl: &gl::Gl, attribute0: Vec<f32>, attribute1: Vec<f32>) -> Result<VertexBuffer, Error>
    {
        let mut builder = VertexBufferBuilder::new(gl)?;
        builder.add(attribute0, 3);
        builder.add(attribute1, 3);
        builder.build()
    }

    pub fn new_with_vec3_vec2(gl: &gl::Gl, attribute0: Vec<f32>, attribute1: Vec<f32>) -> Result<VertexBuffer, Error>
    {
        let mut builder = VertexBufferBuilder::new(gl)?;
        builder.add(attribute0, 3);
        builder.add(attribute1, 2);
        builder.build()
    }
}

pub struct VertexBuffer {
    gl: gl::Gl,
    id: gl::Buffer,
    stride: usize,
    no_vertices: usize,
    offsets: Vec<usize>
}

impl VertexBuffer
{
    pub(crate) fn new_with_data(gl: &gl::Gl, no_vertices: usize, stride: usize, offsets: Vec<usize>, data: &[f32]) -> Result<VertexBuffer, Error>
    {
        let id = gl.create_buffer().unwrap();
        let mut buffer = VertexBuffer {gl: gl.clone(), id, stride, no_vertices, offsets };
        buffer.fill_with(data);
        Ok(buffer)
    }

    pub(crate) fn new(gl: &gl::Gl) -> Result<VertexBuffer, Error>
    {
        let id = gl.create_buffer().unwrap();
        let buffer = VertexBuffer {gl: gl.clone(), id, stride: 0, no_vertices: 0, offsets: Vec::new() };
        Ok(buffer)
    }

    pub(crate) fn fill_with(&mut self, data: &[f32])
    {
        self.bind();
        self.gl.buffer_data_f32(gl::consts::ARRAY_BUFFER, data, gl::consts::STATIC_DRAW);
    }

    pub fn bind(&self)
    {
        bind(&self.gl, &self.id, gl::consts::ARRAY_BUFFER);
    }

    pub fn count(&self) -> usize
    {
        self.no_vertices
    }

    pub fn stride(&self) -> usize
    {
        self.stride
    }

    pub fn offset_from(&self, index: usize) -> usize
    {
        self.offsets[index]
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
