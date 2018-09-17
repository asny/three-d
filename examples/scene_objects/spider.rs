use dust::core::program;
use gl;
use dust::traits;
use gust::*;
use dust::core::surface;
use dust::camera;
use scene_objects::environment::Environment;

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
        self.program.add_uniform_vec3("color", &vec3(1.0, 1.0, 1.0))?;
        self.program.add_uniform_mat4("modelMatrix", &self.local2world)?;
        self.program.add_uniform_mat4("viewMatrix", &camera.get_view())?;
        self.program.add_uniform_mat4("projectionMatrix", &camera.get_projection())?;
        self.program.add_uniform_mat4("normalMatrix", &self.local2world.try_inverse().unwrap().transpose())?;
        self.model.render()?;
        Ok(())
    }
}

impl Spider
{
    pub fn create(gl: &gl::Gl) -> Result<Spider, traits::Error>
    {
        let mesh = loader::load_obj_as_static_mesh("/examples/assets/models/spider.obj").unwrap();
        let program = program::Program::from_resource(&gl, "examples/assets/shaders/standard")?;
        let mut model = surface::TriangleSurface::create(gl, &mesh)?;
        model.add_attributes(&mesh, &program,&vec!["position", "normal"])?;

        Ok(Spider { program, model, position: vec3(0.0, 0.0, 5.0), view_direction: vec3(0.0, 0.0, -1.0), local2world: Mat4::identity(),
        is_moving_backward: false, is_moving_forward: false, is_rotating_left: false, is_rotating_right: false, is_jumping: false})
    }

    pub fn get_position(&self, environment: &Environment) -> Vec3
    {
        static HEIGHT_ABOVE_GROUND: f32 = 0.3;
        vec3(self.position.x, environment.get_height_at(self.position.x, self.position.z) + HEIGHT_ABOVE_GROUND, self.position.z)
    }

    pub fn get_view_direction(&self, environment: &Environment) -> Vec3
    {
        let height0 = environment.get_height_at(self.position.x, self.position.z);
        let height1 = environment.get_height_at(self.position.x + 0.5 * self.view_direction.x, self.position.z + 0.5 * self.view_direction.z);
        let height2 = environment.get_height_at(self.position.x + self.view_direction.x, self.position.z + self.view_direction.z);
        let y_view_dir = 0.25 * ((height2 - height0) + (height1 - height0));
        vec3(self.view_direction.x, y_view_dir, self.view_direction.z).normalize()
    }

    pub fn update(&mut self, time: f32, environment: &Environment)
    {
        static SPEED: f32 = 2.0;
        static ANGULAR_SPEED: f32 = 1.0;
        static GRAVITY: f32 = -9.82;

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
            let m = Mat4::new_rotation( time * ANGULAR_SPEED * vec3(0.0, 1.0, 0.0) );
            let v = m * vec4(self.view_direction.x, self.view_direction.y, self.view_direction.z, 1.0);
            self.view_direction = vec3(v.x, v.y, v.z);
        }
        if self.is_rotating_right
        {
            let m = Mat4::new_rotation( - time * ANGULAR_SPEED * vec3(0.0, 1.0, 0.0) );
            let v = m * vec4(self.view_direction.x, self.view_direction.y, self.view_direction.z, 1.0);
            self.view_direction = vec3(v.x, v.y, v.z);
        }

        let spider_translation;
        {
            // Get world position and view direction
            let world_position = self.get_position(environment);
            let world_view_direction = self.get_view_direction(environment);

            // Compute spider model matrix
            //let spider_rotation_yaw = orientation(normalize(vec3(world_view_direction.x, 0.0, world_view_direction.z)), vec3(0.0, 0.0, 1.0));
            //let spider_rotation_pitch = orientation(normalize(vec3(0.0, world_view_direction.y, 1.0)), vec3(0.0, 0.0, 1.0));
            spider_translation = Mat4::identity();//translate(&Mat4::one(), world_position);
        }
        self.local2world = spider_translation;// * spider_rotation_yaw * spider_rotation_pitch;
    }
}
