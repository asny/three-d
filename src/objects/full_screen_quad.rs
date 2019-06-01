use crate::Gl;
use crate::program;
use crate::buffer::*;

pub struct FullScreen
{
    buffer: StaticVertexBuffer
}

impl FullScreen {

    pub fn new(gl: &Gl) -> FullScreen
    {
        let positions = vec![
            -3.0, -1.0, 0.0,
            3.0, -1.0, 0.0,
            0.0, 2.0, 0.0
        ];
        let uvs = vec![
            -1.0, 0.0,
            2.0, 0.0,
            0.5, 1.5
        ];
        let buffer = StaticVertexBuffer::new_with_vec3_vec2(&gl, &positions, &uvs).unwrap();
        FullScreen {buffer}
    }

    pub fn render(&self, program: &program::Program)
    {
        program.use_attribute_vec3_float(&self.buffer, "position", 0).unwrap();
        program.use_attribute_vec2_float(&self.buffer, "uv_coordinate", 1).unwrap();
        program.draw_arrays(3);
    }
}