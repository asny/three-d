use crate::core::Gl;
use crate::core::Error;

pub struct VertexBuffer {
    gl: Gl,
    id: gl::Buffer
}

impl VertexBuffer
{
    pub fn new_with_static_f32(gl: &Gl, data: &[f32]) -> Result<VertexBuffer, Error>
    {
        let id = gl.create_buffer().unwrap();
        let mut buffer = VertexBuffer { gl: gl.clone(), id };
        buffer.fill_with_static_f32(data);
        Ok(buffer)
    }

    pub fn fill_with_static_f32(&mut self, data: &[f32])
    {
        self.bind();
        self.gl.buffer_data_f32(gl::consts::ARRAY_BUFFER, data, gl::consts::STATIC_DRAW);
        self.gl.unbind_buffer(gl::consts::ARRAY_BUFFER);
    }

    pub fn new_with_dynamic_f32(gl: &Gl, data: &[f32]) -> Result<VertexBuffer, Error>
    {
        let id = gl.create_buffer().unwrap();
        let mut buffer = VertexBuffer { gl: gl.clone(), id };
        buffer.fill_with_dynamic_f32(data);
        Ok(buffer)
    }

    pub fn fill_with_dynamic_f32(&mut self, data: &[f32])
    {
        self.bind();
        self.gl.buffer_data_f32(gl::consts::ARRAY_BUFFER, data, gl::consts::DYNAMIC_DRAW);
        self.gl.unbind_buffer(gl::consts::ARRAY_BUFFER);
    }

    pub(crate) fn bind(&self)
    {
        self.gl.bind_buffer(gl::consts::ARRAY_BUFFER, &self.id);
    }
}

impl Drop for VertexBuffer
{
    fn drop(&mut self)
    {
        self.gl.delete_buffer(&self.id);
    }
}

pub struct ElementBuffer {
    gl: Gl,
    id: gl::Buffer,
    count: usize
}

impl ElementBuffer
{
    pub fn new_with_u32(gl: &Gl, data: &[u32]) -> Result<ElementBuffer, Error>
    {
        let id = gl.create_buffer().unwrap();
        let mut buffer = ElementBuffer{ gl: gl.clone(), id, count: 0 };
        buffer.fill_with_u32(data);
        buffer.count = data.len();
        Ok(buffer)
    }

    pub fn fill_with_u32(&mut self, data: &[u32])
    {
        self.bind();
        self.gl.buffer_data_u32(gl::consts::ELEMENT_ARRAY_BUFFER, data, gl::consts::STATIC_DRAW);
        self.gl.unbind_buffer(gl::consts::ELEMENT_ARRAY_BUFFER);

    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub(crate) fn bind(&self)
    {
        self.gl.bind_buffer(gl::consts::ELEMENT_ARRAY_BUFFER, &self.id);
    }
}

impl Drop for ElementBuffer
{
    fn drop(&mut self)
    {
        self.gl.delete_buffer(&self.id);
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

impl Drop for UniformBuffer
{
    fn drop(&mut self)
    {
        self.gl.delete_buffer(&self.id);
    }
}


