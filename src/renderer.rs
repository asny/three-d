
use crate::*;
use crate::objects::FullScreen;

const MAX_NO_LIGHTS: usize = 3;

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Program(program::Error),
    Rendertarget(rendertarget::Error),
    Texture(texture::Error),
    Buffer(buffer::Error),
    LightPassRendertargetNotAvailable {message: String}
}

impl From<std::io::Error> for Error {
    fn from(other: std::io::Error) -> Self {
        Error::IO(other)
    }
}

impl From<program::Error> for Error {
    fn from(other: program::Error) -> Self {
        Error::Program(other)
    }
}

impl From<rendertarget::Error> for Error {
    fn from(other: rendertarget::Error) -> Self {
        Error::Rendertarget(other)
    }
}

impl From<texture::Error> for Error {
    fn from(other: texture::Error) -> Self {
        Error::Texture(other)
    }
}

impl From<buffer::Error> for Error {
    fn from(other: buffer::Error) -> Self {
        Error::Buffer(other)
    }
}

pub struct DeferredPipeline {
    gl: Gl,
    light_pass_program: program::Program,
    rendertarget: rendertarget::ColorRendertarget,
    geometry_pass_rendertarget: rendertarget::ColorRendertarget,
    full_screen: FullScreen,
    light_buffer: UniformBuffer,
    shadow_rendertarget: DepthRenderTargetArray,
    shadow_cameras: [Option<Camera>; MAX_NO_LIGHTS],
    pub background_color: Vec4,
    pub camera: Camera
}


impl DeferredPipeline
{
    pub fn new(gl: &Gl, screen_width: usize, screen_height: usize, background_color: Vec4) -> Result<DeferredPipeline, Error>
    {
        let light_pass_program = program::Program::from_source(gl,
                                                               include_str!("shaders/light_pass.vert"),
                                                               include_str!("shaders/light_pass.frag"))?;
        let rendertarget = rendertarget::ColorRendertarget::default(gl, screen_width, screen_height)?;
        let shadow_rendertarget = DepthRenderTargetArray::new(gl, screen_width, screen_height, MAX_NO_LIGHTS)?;
        let geometry_pass_rendertarget = rendertarget::ColorRendertarget::new(gl, screen_width, screen_height, 4, true)?;

        let sizes: Vec<u32> = [3u32, 1, 3, 1, 16].iter().cloned().cycle().take(5*MAX_NO_LIGHTS).collect();
        dbg!(&sizes);
        let light_buffer = UniformBuffer::new(gl, &sizes)?;

        let camera = Camera::new_perspective(gl, vec3(5.0, 5.0, 5.0), vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0),
                                                    degrees(45.0), screen_width as f32 / screen_height as f32, 0.1, 1000.0);

        let mut pipeline = DeferredPipeline { gl: gl.clone(), light_pass_program, rendertarget,shadow_rendertarget,
            geometry_pass_rendertarget, full_screen: FullScreen::new(gl), light_buffer, shadow_cameras: [None, None, None],
            background_color, camera };

        for light_id in 0..MAX_NO_LIGHTS {
            pipeline.set_directional_light_intensity(light_id, 0.0)?;
            pipeline.set_directional_light_color(light_id, &vec3(1.0, 1.0, 1.0))?;
            pipeline.set_directional_light_direction(light_id, &vec3(0.0, -1.0, 0.0))?;
        }

        Ok(pipeline)
    }

    pub fn resize(&mut self, screen_width: usize, screen_height: usize) -> Result<(), Error>
    {
        self.rendertarget = rendertarget::ColorRendertarget::default(&self.gl, screen_width, screen_height)?;
        self.geometry_pass_rendertarget = rendertarget::ColorRendertarget::new(&self.gl, screen_width, screen_height, 4, true)?;
        Ok(())
    }

    pub fn shadow_pass<F>(&self, render_scene: F) -> Result<(), Error>
        where F: Fn(&Camera)
    {
        for light_id in 0..MAX_NO_LIGHTS {
            if let Some(ref camera) = self.shadow_cameras[light_id]
            {
                self.shadow_rendertarget.bind(light_id);
                self.shadow_rendertarget.clear();
                render_scene(camera);
            }
        }
        Ok(())
    }

    pub fn geometry_pass<F>(&self, render_scene: F) -> Result<(), Error>
        where F: Fn(&Camera)
    {
        self.geometry_pass_rendertarget.bind();
        self.geometry_pass_rendertarget.clear(&self.background_color);

        state::depth_write(&self.gl, true);
        state::depth_test(&self.gl, state::DepthTestType::LEQUAL);
        state::cull(&self.gl, state::CullType::NONE);
        state::blend(&self.gl, state::BlendType::NONE);

        render_scene(&self.camera);
        Ok(())
    }

    pub fn light_pass(&self) -> Result<(), Error>
    {
        self.light_pass_render_to(&self.rendertarget)?;
        Ok(())
    }

    pub fn light_pass_render_to(&self, rendertarget: &ColorRendertarget) -> Result<(), Error>
    {
        rendertarget.bind();
        rendertarget.clear(&vec4(0.0, 0.0, 0.0, 0.0));

        state::depth_write(&self.gl,false);
        state::depth_test(&self.gl, state::DepthTestType::NONE);
        state::cull(&self.gl,state::CullType::BACK);
        state::blend(&self.gl, state::BlendType::ONE__ONE);

        self.geometry_pass_color_texture().bind(0);
        self.light_pass_program.add_uniform_int("colorMap", &0)?;

        self.geometry_pass_position_texture().bind(1);
        self.light_pass_program.add_uniform_int("positionMap", &1)?;

        self.geometry_pass_normal_texture().bind(2);
        self.light_pass_program.add_uniform_int("normalMap", &2)?;

        self.geometry_pass_surface_parameters_texture().bind(3);
        self.light_pass_program.add_uniform_int("surfaceParametersMap", &3)?;

        self.geometry_pass_depth_texture().bind(4);
        self.light_pass_program.add_uniform_int("depthMap", &4)?;

        self.shadow_rendertarget.target.bind(5);
        self.light_pass_program.add_uniform_int("shadowMaps", &5)?;

        self.light_pass_program.add_uniform_vec3("eyePosition", &self.camera.position())?;

        self.light_pass_program.use_uniform_block(&self.light_buffer, "Lights");

        self.full_screen.render(&self.light_pass_program);

        Ok(())
    }

    pub fn set_directional_light_color(&mut self, light_id: usize, color: &Vec3) -> Result<(), Error>
    {
        self.light_buffer.update(light_id * 5, &color.to_slice())?;
        Ok(())
    }

    pub fn set_directional_light_intensity(&mut self, light_id: usize, intensity: f32) -> Result<(), Error>
    {
        self.light_buffer.update(light_id * 5 + 1, &[intensity])?;
        Ok(())
    }

    pub fn set_directional_light_direction(&mut self, light_id: usize, direction: &Vec3) -> Result<(), Error>
    {
        self.light_buffer.update(light_id * 5 + 2, &direction.to_slice())?;

        if let Some(ref mut camera) = self.shadow_cameras[light_id]
        {
            let up = Self::compute_up_direction(*direction);
            camera.set_view(- *direction, vec3(0.0, 0.0, 0.0), up);

            let bias_matrix = crate::Mat4::new(
                                 0.5, 0.0, 0.0, 0.0,
                                 0.0, 0.5, 0.0, 0.0,
                                 0.0, 0.0, 0.5, 0.0,
                                 0.5, 0.5, 0.5, 1.0);
            let shadow_matrix = bias_matrix * camera.get_projection() * camera.get_view();
            self.light_buffer.update(light_id * 5 + 4, &shadow_matrix.to_slice())?;
        }
        Ok(())
    }

    fn compute_up_direction(direction: Vec3) -> Vec3
    {
        if vec3(1.0, 0.0, 0.0).dot(direction).abs() > 0.9
        {
            (vec3(0.0, 1.0, 0.0).cross(direction)).normalize()
        }
        else {
            (vec3(1.0, 0.0, 0.0).cross(direction)).normalize()
        }
    }

    pub fn enable_shadows(&mut self, light_id: usize) -> Result<(), Error>
    {
        let d = self.light_buffer.get(light_id * 5 + 2)?;
        let dir = vec3(d[0], d[1], d[2]);
        let radius = 2.0;
        let depth = 10.0;
        self.shadow_cameras[light_id] = Some(Camera::new_orthographic(&self.gl, vec3(0.0, 0.0, 0.0), vec3(0.0, 0.0, 0.0), vec3(0.0, 0.0, 0.0),
                                                                  2.0 * radius, 2.0 * radius, 2.0 * depth));
        self.set_directional_light_direction(light_id, &dir)?;
        Ok(())
    }

    pub fn disable_shadows(&mut self, light_id: usize) -> Result<(), Error>
    {
        self.shadow_cameras[light_id] = None;
        Ok(())
    }

    /*pub fn shine_ambient_light(&self, light: &light::AmbientLight) -> Result<(), Error>
    {
        self.light_pass_program.add_uniform_int("lightType", &0)?;
        self.light_pass_program.add_uniform_vec3("ambientLight.base.color", &light.base.color)?;
        self.light_pass_program.add_uniform_float("ambientLight.base.intensity", &light.base.intensity)?;

        self.full_screen.render(&self.light_pass_program);
        Ok(())
    }

    pub fn shine_point_light(&self, light: &light::PointLight) -> Result<(), Error>
    {
        //self.light_pass_program.add_uniform_int("shadowMap", &5)?;
        //self.light_pass_program.add_uniform_int("shadowCubeMap", &6)?;

        self.light_pass_program.add_uniform_int("lightType", &2)?;
        self.light_pass_program.add_uniform_vec3("pointLight.position", &light.position)?;
        self.light_pass_program.add_uniform_vec3("pointLight.base.color", &light.base.color)?;
        self.light_pass_program.add_uniform_float("pointLight.base.intensity", &light.base.intensity)?;
        self.light_pass_program.add_uniform_float("pointLight.attenuation.constant", &light.attenuation.constant)?;
        self.light_pass_program.add_uniform_float("pointLight.attenuation.linear", &light.attenuation.linear)?;
        self.light_pass_program.add_uniform_float("pointLight.attenuation.exp", &light.attenuation.exp)?;

        self.full_screen.render(&self.light_pass_program);
        Ok(())
    }

    pub fn shine_spot_light(&self, light: &light::SpotLight) -> Result<(), Error>
    {
        if let Ok(shadow_camera) = light.shadow_camera() {
            let bias_matrix = crate::Mat4::new(
                                 0.5, 0.0, 0.0, 0.0,
                                 0.0, 0.5, 0.0, 0.0,
                                 0.0, 0.0, 0.5, 0.0,
                                 0.5, 0.5, 0.5, 1.0);
            self.light_pass_program.add_uniform_mat4("shadowMVP", &(bias_matrix * *shadow_camera.get_projection() * *shadow_camera.get_view()))?;

            light.shadow_rendertarget.as_ref().unwrap().target.bind(5);
            self.light_pass_program.add_uniform_int("shadowMap", &5)?;
        }

        //self.light_pass_program.add_uniform_int("shadowCubeMap", &6)?;

        self.light_pass_program.add_uniform_int("lightType", &3)?;
        self.light_pass_program.add_uniform_vec3("spotLight.position", &light.position)?;
        self.light_pass_program.add_uniform_vec3("spotLight.direction", &light.direction)?;
        self.light_pass_program.add_uniform_vec3("spotLight.base.color", &light.base.color)?;
        self.light_pass_program.add_uniform_float("spotLight.base.intensity", &light.base.intensity)?;
        self.light_pass_program.add_uniform_float("spotLight.attenuation.constant", &light.attenuation.constant)?;
        self.light_pass_program.add_uniform_float("spotLight.attenuation.linear", &light.attenuation.linear)?;
        self.light_pass_program.add_uniform_float("spotLight.attenuation.exp", &light.attenuation.exp)?;
        self.light_pass_program.add_uniform_float("spotLight.cutoff", &light.cutoff.cos())?;

        self.full_screen.render(&self.light_pass_program);
        Ok(())
    }*/

    pub fn full_screen(&self) -> &FullScreen
    {
        &self.full_screen
    }

    pub fn screen_rendertarget(&self) -> &ColorRendertarget
    {
        &self.rendertarget
    }

    pub fn geometry_pass_color_texture(&self) -> &Texture
    {
        &self.geometry_pass_rendertarget.targets[0]
    }

    pub fn geometry_pass_position_texture(&self) -> &Texture
    {
        &self.geometry_pass_rendertarget.targets[1]
    }

    pub fn geometry_pass_normal_texture(&self) -> &Texture
    {
        &self.geometry_pass_rendertarget.targets[2]
    }

    pub fn geometry_pass_surface_parameters_texture(&self) -> &Texture
    {
        &self.geometry_pass_rendertarget.targets[3]
    }

    pub fn geometry_pass_depth_texture(&self) -> &Texture
    {
        self.geometry_pass_rendertarget.depth_target.as_ref().unwrap()
    }
}