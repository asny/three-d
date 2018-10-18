
use camera;
use traits;
use gl;
use light;
use screen;
use core::rendertarget;
use core::rendertarget::Rendertarget;
use core::state;
use core::texture::Texture;
use core::program;
use core::full_screen_quad;

#[derive(Debug)]
pub enum Error {
    Program(program::Error),
    Rendertarget(rendertarget::Error),
    Traits(traits::Error),
    LightPassRendertargetNotAvailable {message: String}
}

impl From<traits::Error> for Error {
    fn from(other: traits::Error) -> Self {
        Error::Traits(other)
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

pub struct ForwardPipeline {
    gl: gl::Gl,
    rendertarget: rendertarget::ScreenRendertarget
}

impl ForwardPipeline
{
    pub fn create(gl: &gl::Gl, screen: &screen::Screen) -> Result<ForwardPipeline, Error>
    {
        let rendertarget = rendertarget::ScreenRendertarget::create(gl, screen.width, screen.height)?;
        Ok(ForwardPipeline {gl: gl.clone(), rendertarget})
    }

    pub fn resize(&mut self, screen: &screen::Screen) -> Result<(), Error>
    {
        self.rendertarget = rendertarget::ScreenRendertarget::create(&self.gl, screen.width, screen.height)?;
        Ok(())
    }

    pub fn render_pass_begin(&self)
    {
        self.rendertarget.bind();
        self.rendertarget.clear();
    }
}

pub struct DeferredPipeline {
    gl: gl::Gl,
    light_pass_program: program::Program,
    copy_program: Option<program::Program>,
    rendertarget: rendertarget::ScreenRendertarget,
    geometry_pass_rendertarget: rendertarget::ColorRendertarget,
    light_pass_rendertarget: Option<rendertarget::ColorRendertarget>
}


impl DeferredPipeline
{

    pub fn create(gl: &gl::Gl, screen: &screen::Screen, use_light_pass_rendertarget: bool) -> Result<DeferredPipeline, Error>
    {
        DeferredPipeline::new(gl, screen.width, screen.height, use_light_pass_rendertarget)
    }

    pub fn new(gl: &gl::Gl, width: usize, height: usize, use_light_pass_rendertarget: bool) -> Result<DeferredPipeline, Error>
    {
        let light_pass_program = program::Program::from_resource(&gl, "../Dust/examples/assets/shaders/light_pass",
                                                                 "../Dust/examples/assets/shaders/light_pass")?;
        let rendertarget = rendertarget::ScreenRendertarget::create(gl, width, height)?;
        let geometry_pass_rendertarget = rendertarget::ColorRendertarget::create(&gl, width, height, 4)?;
        let mut light_pass_rendertarget= None;
        let mut copy_program = None;
        if use_light_pass_rendertarget {
            light_pass_rendertarget = Some(rendertarget::ColorRendertarget::create(&gl, width, height, 1)?);
            copy_program = Some(program::Program::from_resource(&gl, "../Dust/examples/assets/shaders/copy",
                                                                "../Dust/examples/assets/shaders/copy")?);
        }
        Ok(DeferredPipeline { gl: gl.clone(), light_pass_program, copy_program, rendertarget, geometry_pass_rendertarget, light_pass_rendertarget })
    }

    pub fn resize(&mut self, screen: &screen::Screen) -> Result<(), Error>
    {
        self.rendertarget = rendertarget::ScreenRendertarget::create(&self.gl, screen.width, screen.height)?;
        self.geometry_pass_rendertarget = rendertarget::ColorRendertarget::create(&self.gl, screen.width, screen.height, 4)?;
        Ok(())
    }

    pub fn geometry_pass_begin(&self) -> Result<(), Error>
    {
        self.geometry_pass_rendertarget.bind();
        self.geometry_pass_rendertarget.clear();

        state::depth_write(&self.gl, true);
        state::depth_test(&self.gl, state::DepthTestType::LEQUAL);
        state::cull(&self.gl, state::CullType::NONE);
        state::blend(&self.gl, state::BlendType::NONE);

        Ok(())
    }

    pub fn light_pass_begin(&self, camera: &camera::Camera) -> Result<(), Error>
    {
        match self.light_pass_rendertarget {
            Some(ref rendertarget) => {
                rendertarget.bind();
                rendertarget.clear();
            },
            None => {
                self.rendertarget.bind();
                self.rendertarget.clear();
            }
        }

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

        self.light_pass_program.add_uniform_vec3("eyePosition", &camera.position())?;

        Ok(())
    }

    pub fn shine_ambient_light(&self, light: &light::AmbientLight) -> Result<(), Error>
    {
        self.light_pass_program.add_uniform_int("shadowMap", &5)?;
        self.light_pass_program.add_uniform_int("shadowCubeMap", &6)?;

        self.light_pass_program.add_uniform_int("lightType", &0)?;
        self.light_pass_program.add_uniform_vec3("ambientLight.base.color", &light.base.color)?;
        self.light_pass_program.add_uniform_float("ambientLight.base.intensity", &light.base.intensity)?;

        full_screen_quad::render(&self.gl, &self.light_pass_program);
        Ok(())
    }

    pub fn shine_directional_light(&self, light: &light::DirectionalLight) -> Result<(), Error>
    {
        if let Ok(shadow_camera) = light.shadow_camera() {
            use camera::Camera;
            let bias_matrix = ::Mat4::new(
                                 0.5, 0.0, 0.0, 0.0,
                                 0.0, 0.5, 0.0, 0.0,
                                 0.0, 0.0, 0.5, 0.0,
                                 0.5, 0.5, 0.5, 1.0).transpose();
            self.light_pass_program.add_uniform_mat4("shadowMVP", &(bias_matrix * *shadow_camera.get_projection() * *shadow_camera.get_view()))?;

            light.shadow_rendertarget.as_ref().unwrap().target.bind(5);
        }

        self.light_pass_program.add_uniform_int("shadowMap", &5)?;
        self.light_pass_program.add_uniform_int("shadowCubeMap", &6)?;

        self.light_pass_program.add_uniform_int("lightType", &1)?;
        self.light_pass_program.add_uniform_vec3("directionalLight.direction", &light.direction)?;
        self.light_pass_program.add_uniform_vec3("directionalLight.base.color", &light.base.color)?;
        self.light_pass_program.add_uniform_float("directionalLight.base.intensity", &light.base.intensity)?;

        full_screen_quad::render(&self.gl, &self.light_pass_program);
        Ok(())
    }

    pub fn shine_point_light(&self, light: &light::PointLight) -> Result<(), Error>
    {
        self.light_pass_program.add_uniform_int("shadowMap", &5)?;
        self.light_pass_program.add_uniform_int("shadowCubeMap", &6)?;

        self.light_pass_program.add_uniform_int("lightType", &2)?;
        self.light_pass_program.add_uniform_vec3("pointLight.position", &light.position)?;
        self.light_pass_program.add_uniform_vec3("pointLight.base.color", &light.base.color)?;
        self.light_pass_program.add_uniform_float("pointLight.base.intensity", &light.base.intensity)?;
        self.light_pass_program.add_uniform_float("pointLight.attenuation.constant", &light.attenuation.constant)?;
        self.light_pass_program.add_uniform_float("pointLight.attenuation.linear", &light.attenuation.linear)?;
        self.light_pass_program.add_uniform_float("pointLight.attenuation.exp", &light.attenuation.exp)?;

        full_screen_quad::render(&self.gl, &self.light_pass_program);
        Ok(())
    }

    pub fn shine_spot_light(&self, light: &light::SpotLight) -> Result<(), Error>
    {
        if let Ok(shadow_camera) = light.shadow_camera() {
            use camera::Camera;
            let bias_matrix = ::Mat4::new(
                                 0.5, 0.0, 0.0, 0.0,
                                 0.0, 0.5, 0.0, 0.0,
                                 0.0, 0.0, 0.5, 0.0,
                                 0.5, 0.5, 0.5, 1.0).transpose();
            self.light_pass_program.add_uniform_mat4("shadowMVP", &(bias_matrix * *shadow_camera.get_projection() * *shadow_camera.get_view()))?;

            light.shadow_rendertarget.as_ref().unwrap().target.bind(5);
        }

        self.light_pass_program.add_uniform_int("shadowMap", &5)?;
        self.light_pass_program.add_uniform_int("shadowCubeMap", &6)?;

        self.light_pass_program.add_uniform_int("lightType", &3)?;
        self.light_pass_program.add_uniform_vec3("spotLight.position", &light.position)?;
        self.light_pass_program.add_uniform_vec3("spotLight.direction", &light.direction)?;
        self.light_pass_program.add_uniform_vec3("spotLight.base.color", &light.base.color)?;
        self.light_pass_program.add_uniform_float("spotLight.base.intensity", &light.base.intensity)?;
        self.light_pass_program.add_uniform_float("spotLight.attenuation.constant", &light.attenuation.constant)?;
        self.light_pass_program.add_uniform_float("spotLight.attenuation.linear", &light.attenuation.linear)?;
        self.light_pass_program.add_uniform_float("spotLight.attenuation.exp", &light.attenuation.exp)?;
        self.light_pass_program.add_uniform_float("spotLight.cutoff", &light.cutoff.cos())?;

        full_screen_quad::render(&self.gl, &self.light_pass_program);
        Ok(())
    }

    pub fn copy_to_screen(&self) -> Result<(), Error>
    {
        let program = self.copy_program()?;
        self.rendertarget.bind();
        self.rendertarget.clear();

        state::depth_write(&self.gl,true);
        state::depth_test(&self.gl, state::DepthTestType::LEQUAL);
        state::cull(&self.gl,state::CullType::BACK);
        state::blend(&self.gl, state::BlendType::NONE);

        self.light_pass_color_texture()?.bind(0);
        program.add_uniform_int("colorMap", &0)?;

        self.geometry_pass_depth_texture().bind(1);
        program.add_uniform_int("depthMap", &1)?;

        full_screen_quad::render(&self.gl, program);
        Ok(())
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
        &self.geometry_pass_rendertarget.depth_target
    }

    pub fn light_pass_color_texture(&self) -> Result<&Texture, Error>
    {
        match self.light_pass_rendertarget {
            Some(ref rendertarget) => { return Ok(&rendertarget.targets[0]) },
            None => {
                return Err(Error::LightPassRendertargetNotAvailable{message: format!("Light pass render target is not available, consider creating the pipeline with 'use_light_pass_rendertarget' set to true")})
            }
        }
    }

    pub fn copy_program(&self) -> Result<&program::Program, Error>
    {
        match self.copy_program {
            Some(ref program) => { return Ok(program) },
            None => {
                return Err(Error::LightPassRendertargetNotAvailable{message: format!("Light pass render target is not available, consider creating the pipeline with 'use_light_pass_rendertarget' set to true")})
            }
        }
    }
}