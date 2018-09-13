use gl;
use gust::static_mesh::StaticMesh;
use core::surface;
use core::program;

pub fn render(gl: &gl::Gl, program: &program::Program)
{
    unsafe {
        static mut FULL_SCREEN__QUAD: Option<surface::TriangleSurface> = None;
        match FULL_SCREEN__QUAD
        {
            None => {
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
                let mut mesh = StaticMesh::create((0..3).collect(), positions).unwrap();
                mesh.add_vec2_attribute("uv_coordinate", uv_coordinates).unwrap();

                let mut model = surface::TriangleSurface::create(gl, &mesh).unwrap();
                model.add_attributes(&mesh, &program,&vec!["uv_coordinate"], &vec!["position"]).unwrap();
                model.render().unwrap();
                FULL_SCREEN__QUAD = Some(model);
            },
            Some(ref x) => {x.render().unwrap();}
        }
    }
}