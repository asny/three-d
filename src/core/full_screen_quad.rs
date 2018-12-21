use gl;
use crate::static_mesh::{Attribute};
use crate::core::surface;
use crate::core::program;

pub fn render(gl: &gl::Gl, program: &program::Program)
{
    unsafe {
        static mut FULL_SCREEN__QUAD: Option<surface::TriangleSurface> = None;
        match FULL_SCREEN__QUAD
        {
            None => {
                let indices: Vec<u32> = (0..3).collect();
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
                let attributes = vec![Attribute::new("position", 3, positions),
                                      Attribute::new("uv_coordinate", 2, uv_coordinates)];

                let mut model = surface::TriangleSurface::create(gl, &indices).unwrap();
                model.add_attributes(&program, &attributes).unwrap();
                model.render().unwrap();
                FULL_SCREEN__QUAD = Some(model);
            },
            Some(ref x) => {x.render().unwrap();}
        }
    }
}