use dust::core::program;
use gl;
use glm::*;
use glm::ext::*;
use dust::traits;
use gust;
use dust::core::surface;
use dust::camera;
use scene_objects::terrain;
use num_traits::identities::One;

pub struct Spider {
    program: program::Program,
    model: surface::TriangleSurface,
    position: Vec3,
    view_direction: Vec3,
    local2world: Mat4,
    // Move states
    pub is_moving_forward: bool,
    pub is_moving_backward: bool,
    pub is_rotating_right: bool,
    pub is_rotating_left: bool,
    pub is_jumping: bool
}

impl traits::Reflecting for Spider
{
    fn reflect(&self, transformation: &Mat4, camera: &camera::Camera) -> Result<(), traits::Error>
    {
        /*self.program.add_uniform_vec3("color", &vec3(1.0, 1.0, 1.0))?;
        self.program.add_uniform_mat4("modelMatrix", &self.local2world)?;
        self.program.add_uniform_mat4("viewMatrix", &camera.get_view())?;
        self.program.add_uniform_mat4("projectionMatrix", &camera.get_projection())?;
        self.program.add_uniform_mat4("normalMatrix", &transpose(&inverse(transformation)))?;
        self.model.render()?;*/
        Ok(())
    }
}

impl Spider
{
    pub fn create(gl: &gl::Gl) -> Result<Spider, traits::Error>
    {
        let mesh = gust::loader::load_obj("/examples/assets/models/spider.obj").unwrap();
        let program = program::Program::from_resource(&gl, "examples/assets/shaders/standard")?;
        let model = surface::TriangleSurface::create(gl, &mesh, &program)?;

        Ok(Spider { program, model, position: vec3(0.0, 0.0, 5.0), view_direction: vec3(0.0, 0.0, -1.0), local2world: Matrix4::one(),
        is_moving_backward: false, is_moving_forward: false, is_rotating_left: false, is_rotating_right: false, is_jumping: false})
    }

    pub fn get_position(&self, terrain: &terrain::Terrain) -> &Vec3
    {
        &self.position
    }

    pub fn get_view_direction(&self, terrain: &terrain::Terrain) -> &Vec3
    {
        &self.view_direction
    }

    pub fn update(&mut self, time: f32, terrain: &terrain::Terrain)
    {
        static SPEED: f32 = 2.0;
        static ANGULAR_SPEED: f32 = 1.0;
        static GRAVITY: f32 = -9.82;
        static HEIGHT: f32 = 0.3;

        if self.is_moving_forward
        {
            self.position = self.position + self.view_direction * time * SPEED;
        }
        if self.is_moving_backward
        {
            self.position = self.position - self.view_direction * time * SPEED;
        }
        if self.is_rotating_left
        {
            let v = ext::rotate(&Matrix4::one(), time * ANGULAR_SPEED, vec3(0.0, 1.0, 0.0)) * vec4(self.view_direction.x, self.view_direction.y, self.view_direction.z, 1.0);
            self.view_direction = vec3(v.x, v.y, v.z);
        }
        if self.is_rotating_right
        {
            let v = ext::rotate(&Matrix4::one(), -time * ANGULAR_SPEED, vec3(0.0, 1.0, 0.0)) * vec4(self.view_direction.x, self.view_direction.y, self.view_direction.z, 1.0);
            self.view_direction = vec3(v.x, v.y, v.z);
        }

        let spider_translation;
        {
            // Get world position and view direction
            let world_position = self.get_position(terrain);
            let world_view_direction = self.get_view_direction(terrain);

            // Compute spider model matrix
            //let spider_rotation_yaw = orientation(normalize(vec3(world_view_direction.x, 0.0, world_view_direction.z)), vec3(0.0, 0.0, 1.0));
            //let spider_rotation_pitch = orientation(normalize(vec3(0.0, world_view_direction.y, 1.0)), vec3(0.0, 0.0, 1.0));
            spider_translation = translate(&Matrix4::one(), *world_position);
        }
        self.local2world = spider_translation;// * spider_rotation_yaw * spider_rotation_pitch;
    }
}
