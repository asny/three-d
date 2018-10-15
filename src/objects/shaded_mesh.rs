
use gl;
use ::*;

pub struct ShadedMesh {
    program: program::Program,
    model: surface::TriangleSurface,
    pub color: Vec3,
    pub texture: Option<texture::Texture2D>,
    pub diffuse_intensity: f32,
    pub specular_intensity: f32,
    pub specular_power: f32
}

impl ShadedMesh
{
    pub fn create(gl: &gl::Gl, mesh: &Renderable) -> ShadedMesh
    {
        let program = program::Program::from_resource(&gl, "../Dust/src/objects/shaders/mesh_shaded",
                                                      "../Dust/src/objects/shaders/shaded").unwrap();
        let mut model = surface::TriangleSurface::create(gl, mesh).unwrap();
        model.add_attributes(mesh, &program, &vec!["position", "normal"]).unwrap();

        ShadedMesh { program, model, color: vec3(1.0, 1.0, 1.0), texture: None, diffuse_intensity: 0.5, specular_intensity: 0.2, specular_power: 5.0 }
    }

    pub fn render(&self, transformation: &Mat4, camera: &camera::Camera)
    {
        self.program.cull(state::CullType::BACK);
        self.program.depth_test(state::DepthTestType::LEQUAL);
        self.program.depth_write(true);
        self.program.polygon_mode(state::PolygonType::Fill);

        self.program.add_uniform_float("diffuse_intensity", &self.diffuse_intensity).unwrap();
        self.program.add_uniform_float("specular_intensity", &self.specular_intensity).unwrap();
        self.program.add_uniform_float("specular_power", &self.specular_power).unwrap();

        if let Some(ref tex) = self.texture
        {
            self.program.add_uniform_int("use_texture", &1).unwrap();
            tex.bind(0);
            self.program.add_uniform_int("tex", &0).unwrap();
        }
        else {
            self.program.add_uniform_int("use_texture", &0).unwrap();
            self.program.add_uniform_vec3("color", &self.color).unwrap();
        }

        self.program.add_uniform_mat4("modelMatrix", &transformation).unwrap();
        self.program.add_uniform_mat4("viewMatrix", &camera.get_view()).unwrap();
        self.program.add_uniform_mat4("projectionMatrix", &camera.get_projection()).unwrap();
        self.program.add_uniform_mat4("normalMatrix", &transformation.try_inverse().unwrap().transpose()).unwrap();
        self.model.render().unwrap();
    }
}
