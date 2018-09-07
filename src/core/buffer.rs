use gl;
use std;
use gust::mesh::Mesh;
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

    pub fn fill_from_attributes(&mut self, mesh: &Mesh, vec2_attributes: &Vec<&str>, vec3_attributes: &Vec<&str>)
    {
        self.stride = vec2_attributes.len() * 2 + vec3_attributes.len() * 3;
        self.map = Vec::new();

        let mut data: Vec<f32> = vec![0.0; self.stride * mesh.no_vertices()];
        let mut offset = 0;
        for attribute_name in vec2_attributes.iter()
        {
            self.map.push(Att {name: attribute_name.to_string(), offset, no_components: 2});
            let mut index = offset;
            for vertex_id in mesh.vertex_iterator()
            {
                let value = mesh.get_vec2_attribute_at(attribute_name, &vertex_id).unwrap();
                data[index] = value.x;
                data[index + 1] = value.y;

                index += self.stride;
            }
            offset = offset + 2;
        }

        for attribute_name in vec3_attributes.iter()
        {
            self.map.push(Att {name: attribute_name.to_string(), offset, no_components: 3});
            let mut index = offset;
            for vertex_id in mesh.vertex_iterator()
            {
                let value = mesh.get_vec3_attribute_at(attribute_name, &vertex_id).unwrap();
                data[index] = value.x;
                data[index + 1] = value.y;
                data[index + 2] = value.z;

                index += self.stride;
            }
            offset = offset + 3;
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
