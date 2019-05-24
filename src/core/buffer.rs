use gl;
pub use std::slice::Iter;

#[derive(Debug)]
pub enum Error {
}

pub struct VertexBuffer {
    gl: gl::Gl,
    id: gl::Buffer,
    stride: usize,
    offsets: Vec<usize>,
    data: Vec<f32>
}

impl VertexBuffer
{
    pub(crate) fn new(gl: &gl::Gl) -> Result<VertexBuffer, Error>
    {
        let id = gl.create_buffer().unwrap();
        let buffer = VertexBuffer {gl: gl.clone(), id, stride: 0, offsets: Vec::new(), data: Vec::new() };
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

    pub fn offset_from(&self, index: usize) -> usize
    {
        self.offsets[index]
    }

    /*pub fn optimize_data_layout(&mut self)
    {
        // TODO: use interleaved when not the same count (no_vertices)
        self.lengths = Vec::new();
        self.offsets = Vec::new();
        self.stride = 0;
        for (data, no_components) in self.data {

        }
        let mut out_data = vec![0.0; self.stride * self.lengths[0]];
        let mut offset = 0;
        for (data, no_components) in self.data
        {
            let mut index = offset;
            for i in 0..no_vertices {
                for j in 0..no_components {
                    out_data[index + j] = data[i * no_components + j];
                }
                index += stride;
            }
            offset += no_components;
        }
        out_data
    }*/

    pub fn clear(&mut self)
    {
        self.data.clear();
        self.offsets.clear();
        self.stride = 0;
        //TODO: Unbind data on gpu
    }

    pub fn add(&mut self, data: &[f32], no_components: usize)
    {
        self.offsets.push(self.data.len());
        self.data.extend_from_slice(data);
    }
}


pub struct StaticVertexBuffer {
    buffer: VertexBuffer
}

impl StaticVertexBuffer {

    pub fn new(gl: &gl::Gl) -> Result<StaticVertexBuffer, Error>
    {
        let buffer = VertexBuffer::new(gl)?;
        Ok(StaticVertexBuffer { buffer })
    }

    pub fn new_with_vec3(gl: &gl::Gl, attribute: &[f32]) -> Result<StaticVertexBuffer, Error>
    {
        let mut buffer = StaticVertexBuffer::new(gl)?;
        buffer.buffer.add(attribute, 3);
        buffer.send_data_at();
        Ok(buffer)
    }

    pub fn new_with_vec3_vec3(gl: &gl::Gl, attribute0: &[f32], attribute1: &[f32]) -> Result<StaticVertexBuffer, Error>
    {
        let mut buffer = StaticVertexBuffer::new(gl)?;
        buffer.buffer.add(attribute0, 3);
        buffer.buffer.add(attribute1, 3);
        buffer.send_data_at();
        Ok(buffer)
    }

    pub fn new_with_vec3_vec2(gl: &gl::Gl, attribute0: &[f32], attribute1: &[f32]) -> Result<StaticVertexBuffer, Error>
    {
        let mut buffer = StaticVertexBuffer::new(gl)?;
        buffer.buffer.add(attribute0, 3);
        buffer.buffer.add(attribute1, 2);
        buffer.send_data_at();
        Ok(buffer)
    }

    pub fn send_data_at(&mut self)
    {
        //TODO: self.buffer.optimize_data_layout();
        self.buffer.bind();
        self.buffer.gl.buffer_data_f32(gl::consts::ARRAY_BUFFER, &self.buffer.data, gl::consts::STATIC_DRAW);
    }
}

impl std::ops::Deref for StaticVertexBuffer {
    type Target = VertexBuffer;

    fn deref(&self) -> &VertexBuffer {
        &self.buffer
    }
}

pub struct DynamicVertexBuffer {
    buffer: VertexBuffer
}

impl DynamicVertexBuffer {

    pub fn new(gl: &gl::Gl) -> Result<DynamicVertexBuffer, Error>
    {
        let buffer = VertexBuffer::new(gl)?;
        Ok(DynamicVertexBuffer { buffer })
    }

    pub fn send_data_at(&self)
    {
        self.buffer.bind();
        //TODO: Unbind data on gpu: https://www.khronos.org/opengl/wiki/Buffer_Object_Streaming
        self.buffer.gl.buffer_data_f32(gl::consts::ARRAY_BUFFER, &self.buffer.data, gl::consts::DYNAMIC_DRAW);
    }

    pub fn update_data_at(&mut self, index: usize, data: &[f32])
    {
        let offset = self.buffer.offset_from(index);
        for i in 0..data.len() {
            self.buffer.data[i + offset] = data[i]
        }
    }

    /*pub fn update_and_send_data_at(&mut self, index: usize, data: &[f32])
    {
        self.update_data_at(index, data);
        TODO: self.buffer.gl.buffer_sub_data_f32()
    }*/
}

impl std::ops::Deref for DynamicVertexBuffer {
    type Target = VertexBuffer;

    fn deref(&self) -> &VertexBuffer {
        &self.buffer
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
