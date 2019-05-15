use gl;
use crate::program;
use crate::buffer::*;

pub fn render(gl: &gl::Gl, program: &program::Program)
{
    let buffer =
        unsafe {
            static mut FULL_SCREEN__QUAD: Option<VertexBuffer> = None;
            if FULL_SCREEN__QUAD.is_none()
            {
                let positions: Vec<f32> = vec![
                    -3.0, -1.0, 0.0,
                    3.0, -1.0, 0.0,
                    0.0, 2.0, 0.0
                ];
                let uv_coordinates: Vec<f32> = vec![
                    -1.0, 0.0,
                    2.0, 0.0,
                    0.5, 1.5
                ];
                FULL_SCREEN__QUAD = Some(VertexBuffer::new_from_attributes(&gl, &vec![Attribute::new("position", 3, positions).unwrap(),
                                          Attribute::new("uv_coordinate", 2, uv_coordinates).unwrap()]).unwrap());
            }
            FULL_SCREEN__QUAD.as_ref().unwrap()
        };

    program.use_attribute_vec3_float(buffer, "position", 0).unwrap();
    program.use_attribute_vec2_float(buffer, "uv_coordinate", 1).unwrap();

    program.draw_arrays(3);
}