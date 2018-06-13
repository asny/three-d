use dust::core::program;
use gl;
use dust::traits;
use gust::mesh;
use dust::core::surface;
use std::rc::Rc;
use dust::camera;
use glm;

pub struct Triangle {
    program: program::Program,
    model: surface::TriangleSurface
}

impl traits::Reflecting for Triangle
{
    fn reflect(&self, transformation: &glm::Mat4, camera: &camera::Camera) -> Result<(), traits::Error>
    {
        self.program.add_uniform_mat4("viewMatrix", &camera.get_view())?;
        self.program.add_uniform_mat4("projectionMatrix", &camera.get_projection())?;
        self.model.render()?;
        Ok(())
    }
}

impl Triangle
{
    pub fn create(gl: &gl::Gl) -> Result<Rc<traits::Reflecting>, traits::Error>
    {
        let positions: Vec<glm::Vec3> = vec![
            glm::vec3(0.5, -0.5, 0.0), // bottom right
            glm::vec3(-0.5, -0.5, 0.0),// bottom left
            glm::vec3(0.0,  0.5, 0.0) // top
        ];
        let colors: Vec<glm::Vec3> = vec![
            glm::vec3(1.0, 0.0, 0.0),   // bottom right
            glm::vec3(0.0, 1.0, 0.0),   // bottom left
            glm::vec3(0.0, 0.0, 1.0)    // top
        ];
        let mut mesh = mesh::Mesh::create(positions).unwrap();
        mesh.add_custom_vec3_attribute("color", colors).unwrap();
        let program = program::Program::from_resource(&gl, "examples/assets/shaders/color")?;
        let model = surface::TriangleSurface::create(gl, &mesh, &program)?;

        Ok(Rc::new(Triangle { program, model }))
    }
}
