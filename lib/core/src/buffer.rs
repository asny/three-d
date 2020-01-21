use crate::Gl;

#[derive(Debug)]
pub enum Error {
    BufferUpdateFailed {message: String}
}

pub struct VertexBuffer {
    gl: Gl,
    id: gl::Buffer,
    stride: usize,
    offsets: Vec<usize>,
    data: Vec<f32>
}

impl VertexBuffer
{
    pub fn new(gl: &Gl) -> Result<VertexBuffer, Error>
    {
        let id = gl.create_buffer().unwrap();
        let buffer = VertexBuffer {gl: gl.clone(), id, stride: 0, offsets: Vec::new(), data: Vec::new() };
        Ok(buffer)
    }

    pub fn new_with_one_static_attribute(gl: &Gl, data: &[f32]) -> Result<VertexBuffer, Error>
    {
        let mut buffer = VertexBuffer::new(gl)?;
        buffer.add(data);
        buffer.send_static_data();
        Ok(buffer)
    }

    pub fn new_with_two_static_attributes(gl: &Gl, data0: &[f32], data1: &[f32]) -> Result<VertexBuffer, Error>
    {
        let mut buffer = VertexBuffer::new(gl)?;
        buffer.add(data0);
        buffer.add(data1);
        buffer.send_static_data();
        Ok(buffer)
    }

    pub(crate) fn bind(&self)
    {
        self.gl.bind_buffer(gl::consts::ARRAY_BUFFER, &self.id);
    }

    pub fn stride(&self) -> usize
    {
        self.stride
    }

    pub fn offset_from(&self, index: usize) -> usize
    {
        self.offsets[index]
    }

    pub fn clear(&mut self)
    {
        self.data.clear();
        self.offsets.clear();
        self.stride = 0;
    }

    pub fn add(&mut self, data: &[f32])
    {
        self.offsets.push(self.data.len());
        self.data.extend_from_slice(data);
    }

    pub fn update_data_at(&mut self, index: usize, data: &[f32])
    {
        let offset = self.offset_from(index);
        for i in 0..data.len() {
            self.data[i + offset] = data[i];
        }
    }

    pub fn send_static_data(&mut self)
    {
        self.bind();
        self.gl.buffer_data_f32(gl::consts::ARRAY_BUFFER, &self.data, gl::consts::STATIC_DRAW);
        self.gl.unbind_buffer(gl::consts::ARRAY_BUFFER);
    }

    pub fn send_dynamic_data(&mut self)
    {
        self.bind();
        self.gl.buffer_data_f32(gl::consts::ARRAY_BUFFER, &self.data, gl::consts::DYNAMIC_DRAW);
        self.gl.unbind_buffer(gl::consts::ARRAY_BUFFER);
    }
}

pub struct ElementBuffer {
    gl: Gl,
    id: gl::Buffer,
    count: usize
}

impl ElementBuffer
{
    pub fn new(gl: &Gl) -> Result<ElementBuffer, Error>
    {
        let id = gl.create_buffer().unwrap();
        let buffer = ElementBuffer{ gl: gl.clone(), id, count: 0 };
        Ok(buffer)
    }

    pub fn new_with(gl: &Gl, data: &[u32]) -> Result<ElementBuffer, Error>
    {
        let mut buffer = ElementBuffer::new(gl)?;
        buffer.fill_with(data);
        buffer.count = data.len();
        Ok(buffer)
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub(crate) fn bind(&self)
    {
        self.gl.bind_buffer(gl::consts::ELEMENT_ARRAY_BUFFER, &self.id);
    }

    pub fn fill_with(&mut self, data: &[u32])
    {
        self.bind();
        self.gl.buffer_data_u32(gl::consts::ELEMENT_ARRAY_BUFFER, data, gl::consts::STATIC_DRAW);
        self.gl.unbind_buffer(gl::consts::ELEMENT_ARRAY_BUFFER);

    }
}

pub struct UniformBuffer {
    gl: Gl,
    id: gl::Buffer,
    offsets: Vec<usize>,
    data: Vec<f32>
}

impl UniformBuffer
{
    pub fn new(gl: &Gl, sizes: &[u32]) -> Result<UniformBuffer, Error>
    {
        let id = gl.create_buffer().unwrap();

        let mut offsets = Vec::new();
        let mut length = 0;
        for size in sizes
        {
            offsets.push(length);
            length += *size as usize;
        }
        Ok(UniformBuffer{ gl: gl.clone(), id, offsets, data: vec![0.0; length as usize] })
    }

    pub(crate) fn bind(&self, id: u32)
    {
        self.gl.bind_buffer_base(gl::consts::UNIFORM_BUFFER, id, &self.id);
    }

    pub fn update(&mut self, index: usize, data: &[f32]) -> Result<(), Error>
    {
        let (offset, length) = self.offset_length(index)?;
        if data.len() != length
        {
            return Err(Error::BufferUpdateFailed {message: format!("The uniform buffer data for index {} has length {} but it must be {}.", index, data.len(), length)})
        }
        self.data.splice(offset..offset+length, data.iter().cloned());
        self.send();
        //TODO: Send to GPU (glBufferSubData)
        Ok(())
    }

    pub fn get(&self, index: usize) -> Result<&[f32], Error>
    {
        let (offset, length) = self.offset_length(index)?;
        Ok(&self.data[offset..offset+length])
    }

    fn offset_length(&self, index: usize) -> Result<(usize, usize), Error>
    {
        if index >= self.offsets.len()
        {
            return Err(Error::BufferUpdateFailed {message: format!("The uniform buffer index {} is outside the range 0-{}", index, self.offsets.len()-1)})
        }
        let offset = self.offsets[index];
        let length = if index + 1 == self.offsets.len() {self.data.len()} else {self.offsets[index+1]}  - offset;
        Ok((offset, length))
    }

    fn send(&self)
    {
        self.gl.bind_buffer(gl::consts::UNIFORM_BUFFER, &self.id);
        self.gl.buffer_data_f32(gl::consts::UNIFORM_BUFFER, &self.data, gl::consts::STATIC_DRAW);
        self.gl.unbind_buffer(gl::consts::UNIFORM_BUFFER);
    }
}


