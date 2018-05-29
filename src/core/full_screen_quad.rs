use gl;
use glm;
use mesh;
use core::surface;
use core::program;

pub fn render(gl: &gl::Gl, program: &program::Program)
{
    unsafe {
        static mut FULL_SCREEN__QUAD: Option<surface::TriangleSurface> = None;
        match FULL_SCREEN__QUAD
        {
            None => {
                let positions: Vec<glm::Vec3> = vec![
                    glm::vec3(-3.0, -1.0, 0.0),
                    glm::vec3(3.0, -1.0, 0.0),
                    glm::vec3(0.0, 2.0, 0.0)
                ];
                let uv_coordinates: Vec<glm::Vec2> = vec![
                    glm::vec2(-1.0, 0.0),
                    glm::vec2(2.0, 0.0),
                    glm::vec2(0.5, 1.5)
                ];
                let mut mesh = mesh::Mesh::create(&vec![0, 1, 2], positions).unwrap();
                mesh.add_custom_vec2_attribute("uv_coordinate", uv_coordinates).unwrap();

                let surface = surface::TriangleSurface::create(gl, &mesh, program).unwrap();
                surface.render().unwrap();
                FULL_SCREEN__QUAD = Some(surface);
            },
            Some(ref x) => {x.render().unwrap();}
        }
    }
}