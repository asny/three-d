use camera;
use scene;
use traits;
use glm;
use gl;
use light;
use num_traits::identities::One;
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
    Traits(traits::Error)
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
    width: usize,
    height: usize,
    rendertarget: rendertarget::ScreenRendertarget
}

impl ForwardPipeline
{
    pub fn create(gl: &gl::Gl, width: usize, height: usize) -> Result<ForwardPipeline, Error>
    {
        let rendertarget = rendertarget::ScreenRendertarget::create(gl, width, height)?;
        Ok(ForwardPipeline {gl: gl.clone(), width, height, rendertarget})
    }

    pub fn resize(&mut self, width: usize, height: usize) -> Result<(), Error>
    {
        self.rendertarget = rendertarget::ScreenRendertarget::create(&self.gl, width, height)?;
        self.width = width;
        self.height = height;
        Ok(())
    }

    pub fn render(&self, camera: &camera::Camera, scene: &scene::Scene) -> Result<(), Error>
    {
        self.rendertarget.bind();
        self.rendertarget.clear();

        for model in &scene.models {
            let transformation = glm::Matrix4::one();
            model.reflect(&transformation, camera)?;
        }

        Ok(())
    }
}

pub struct DeferredPipeline {
    gl: gl::Gl,
    pub width: usize,
    pub height: usize,
    light_pass_program: program::Program,
    rendertarget: rendertarget::ScreenRendertarget,
    geometry_pass_rendertarget: rendertarget::ColorRendertarget
}


impl DeferredPipeline
{
    pub fn create(gl: &gl::Gl, width: usize, height: usize) -> Result<DeferredPipeline, Error>
    {
        let light_pass_program = program::Program::from_resource(&gl, "examples/assets/shaders/light_pass")?;
        let rendertarget = rendertarget::ScreenRendertarget::create(gl, width, height)?;
        let geometry_pass_rendertarget = rendertarget::ColorRendertarget::create(&gl, width, height, 3)?;
        Ok(DeferredPipeline { gl: gl.clone(), width, height, light_pass_program, rendertarget, geometry_pass_rendertarget })
    }

    pub fn resize(&mut self, width: usize, height: usize) -> Result<(), Error>
    {
        self.rendertarget = rendertarget::ScreenRendertarget::create(&self.gl, width, height)?;
        self.geometry_pass_rendertarget = rendertarget::ColorRendertarget::create(&self.gl, width, height, 3)?;
        self.width = width;
        self.height = height;
        Ok(())
    }

    pub fn geometry_pass_begin(&self) -> Result<(), Error>
    {
        state::depth_write(&self.gl, true);
        state::depth_test(&self.gl, true);
        state::cull(&self.gl, state::CullType::NONE);
        state::blend(&self.gl, false);

        self.geometry_pass_rendertarget.bind();
        self.geometry_pass_rendertarget.clear();
        Ok(())
    }

    pub fn light_pass_begin(&self, camera: &camera::Camera) -> Result<(), Error>
    {
        self.rendertarget.bind();
        self.rendertarget.clear();

        state::depth_write(&self.gl,false);
        state::depth_test(&self.gl, false);
        state::cull(&self.gl,state::CullType::BACK);
        state::blend(&self.gl, true);
        unsafe {
            self.gl.DepthFunc(gl::LEQUAL);
            self.gl.BlendFunc(gl::ONE, gl::ONE);
        }

        self.geometry_pass_rendertarget.targets[0].bind(0);
        self.light_pass_program.add_uniform_int("colorMap", &0)?;

        self.geometry_pass_rendertarget.targets[1].bind(1);
        self.light_pass_program.add_uniform_int("positionMap", &1)?;

        self.geometry_pass_rendertarget.targets[2].bind(2);
        self.light_pass_program.add_uniform_int("normalMap", &2)?;

        self.geometry_pass_rendertarget.depth_target.bind(3);
        self.light_pass_program.add_uniform_int("depthMap", &3)?;

        self.light_pass_program.add_uniform_vec3("eyePosition", &camera.position)?;

        Ok(())
    }

    pub fn shine_light(&self, directional_light: &light::DirectionalLight) -> Result<(), Error>
    {
        /*shadow_render_target.bind_texture_for_reading(4);
        GLUniform::use(shader, "shadowMap", 4);
        GLUniform::use(shader, "shadowCubeMap", 5);
        GLUniform::use(shader, "shadowMVP", bias_matrix * get_projection() * get_view());*/

        self.light_pass_program.add_uniform_int("lightType", &1)?;
        self.light_pass_program.add_uniform_vec3("directionalLight.direction", &directional_light.direction)?;
        self.light_pass_program.add_uniform_vec3("directionalLight.base.color", &directional_light.base.color)?;
        self.light_pass_program.add_uniform_float("directionalLight.base.ambientIntensity", &directional_light.base.ambient_intensity)?;
        self.light_pass_program.add_uniform_float("directionalLight.base.diffuseIntensity", &directional_light.base.diffuse_intensity)?;

        full_screen_quad::render(&self.gl, &self.light_pass_program);
        Ok(())
    }

    pub fn forward_pass_begin(&self)
    {
        state::blend(&self.gl, true);
        unsafe {
            self.gl.BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
    }
}