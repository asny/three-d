
use crate::buffer::*;
use crate::core::Gl;

pub struct Mesh {
    position_buffer: StaticVertexBuffer,
    normal_buffer: StaticVertexBuffer,
    index_buffer: ElementBuffer
}

impl Mesh
{
    pub fn new(gl: &Gl, indices: &[u32], positions: &[f32], normals: &[f32]) -> Result<Mesh, Error>
    {
        let position_buffer = StaticVertexBuffer::new_with_vec3(gl, positions)?;
        let normal_buffer = StaticVertexBuffer::new_with_vec3(gl, normals)?;
        let index_buffer = ElementBuffer::new_with(gl, indices)?;

        Ok(Mesh { index_buffer, position_buffer, normal_buffer})
    }

    pub fn new_from_obj_source(gl: &Gl, source: String) -> Result<Mesh, Error>
    {
        let objs = wavefront_obj::obj::parse(source).unwrap();
        let obj = objs.objects.first().unwrap();

        let mut positions = Vec::new();
        obj.vertices.iter().for_each(|v| {positions.push(v.x as f32); positions.push(v.y as f32); positions.push(v.z as f32);});
        let mut normals = vec![0.0f32; obj.vertices.len()*3];
        let mut indices = Vec::new();
        for shape in obj.geometry.first().unwrap().shapes.iter() {
            match shape.primitive {
                wavefront_obj::obj::Primitive::Triangle(i0, i1, i2) => {
                    indices.push(i0.0 as u32);
                    indices.push(i1.0 as u32);
                    indices.push(i2.0 as u32);

                    let mut normal = obj.normals[i0.2.unwrap()];
                    normals[i0.0*3] = normal.x as f32;
                    normals[i0.0*3+1] = normal.y as f32;
                    normals[i0.0*3+2] = normal.z as f32;

                    normal = obj.normals[i1.2.unwrap()];
                    normals[i1.0*3] = normal.x as f32;
                    normals[i1.0*3+1] = normal.y as f32;
                    normals[i1.0*3+2] = normal.z as f32;

                    normal = obj.normals[i2.2.unwrap()];
                    normals[i2.0*3] = normal.x as f32;
                    normals[i2.0*3+1] = normal.y as f32;
                    normals[i2.0*3+2] = normal.z as f32;
                },
                _ => {}
            }
        }
        Self::new(&gl, &indices, &positions, &normals)
    }

    pub fn update_positions(&mut self, positions: &[f32]) -> Result<(), Error>
    {
        self.position_buffer.clear();
        self.position_buffer.add(positions, 3);
        self.position_buffer.send_data();
        Ok(())
    }

    pub fn update_normals(&mut self, normals: &[f32]) -> Result<(), Error>
    {
        self.normal_buffer.clear();
        self.normal_buffer.add(normals, 3);
        self.normal_buffer.send_data();
        Ok(())
    }

    pub fn position_buffer(&self) -> &VertexBuffer
    {
        &self.position_buffer
    }

    pub fn normal_buffer(&self) -> &VertexBuffer
    {
        &self.normal_buffer
    }

    pub fn index_buffer(&self) -> &ElementBuffer
    {
        &self.index_buffer
    }
}
