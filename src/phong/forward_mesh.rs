
use crate::*;
use std::rc::Rc;

static mut PROGRAM_COLOR_AMBIENT: Option<Program> = None;
static mut PROGRAM_COLOR_AMBIENT_DIRECTIONAL: Option<Program> = None;
static mut PROGRAM_TEXTURE_AMBIENT: Option<Program> = None;
static mut PROGRAM_TEXTURE_AMBIENT_DIRECTIONAL: Option<Program> = None;
static mut MESH_COUNT: u32 = 0;

pub struct PhongForwardMesh {
    gl: Gl,
    pub name: String,
    gpu_mesh: GPUMesh,
    pub material: PhongMaterial
}

impl PhongForwardMesh
{
    pub fn new(gl: &Gl, cpu_mesh: &CPUMesh, material: &PhongMaterial) -> Result<Self, Error>
    {
        if cpu_mesh.normals.is_none() {
            Err(Error::FailedToCreateMesh {message:
              "Cannot create a mesh without normals. Consider calling compute_normals on the CPUMesh before creating the mesh.".to_string()})?
        }
        unsafe {
            MESH_COUNT += 1;
        }
        Ok(Self {
            gl: gl.clone(),
            name: cpu_mesh.name.clone(),
            gpu_mesh: GPUMesh::new(gl, cpu_mesh)?,
            material: material.clone()
        })
    }

    pub fn new_meshes(gl: &Gl, cpu_meshes: &Vec<CPUMesh>, materials: &Vec<PhongMaterial>) -> Result<Vec<PhongForwardMesh>, Error>
    {
        let mut meshes = Vec::new();
        for cpu_mesh in cpu_meshes {
            let material = cpu_mesh.material_name.as_ref().map(|material_name|
                materials.iter().filter(|m| &m.name == material_name).last()
                .map(|m| m.clone()).unwrap_or_else(|| PhongMaterial::default()))
                .unwrap_or_else(|| PhongMaterial::default());
            meshes.push(Self::new(gl,cpu_mesh, &material)?);
        }
        Ok(meshes)
    }

    pub fn render_depth(&self, transformation: &Mat4, camera: &camera::Camera) -> Result<(), Error>
    {
        self.render_with_ambient(transformation, camera, &AmbientLight::new(&self.gl, 0.0, &vec3(0.0, 0.0, 0.0))?)
    }

    pub fn render_with_ambient(&self, transformation: &Mat4, camera: &camera::Camera, ambient_light: &AmbientLight) -> Result<(), Error>
    {
        let program = match self.material.color_source {
            ColorSource::Color(_) => Self::program_color_ambient(&self.gl)?,
            ColorSource::Texture(_) => Self::program_texture_ambient(&self.gl)?
        };
        program.add_uniform_vec3("ambientLight.color", &ambient_light.color())?;
        program.add_uniform_float("ambientLight.intensity", &ambient_light.intensity())?;

        bind_material(program, &self.material, self.gpu_mesh.has_uvs())?;
        self.gpu_mesh.render(program, transformation, camera)
    }

    pub fn render_with_ambient_and_directional(&self, transformation: &Mat4, camera: &camera::Camera, ambient_light: &AmbientLight, directional_light: &DirectionalLight) -> Result<(), Error>
    {
        let program = match self.material.color_source {
            ColorSource::Color(_) => Self::program_color_ambient_directional(&self.gl)?,
            ColorSource::Texture(_) => Self::program_texture_ambient_directional(&self.gl)?
        };
        program.add_uniform_vec3("ambientLight.color", &ambient_light.color())?;
        program.add_uniform_float("ambientLight.intensity", &ambient_light.intensity())?;

        program.add_uniform_vec3("eyePosition", &camera.position())?;
        program.use_texture(directional_light.shadow_map(), "shadowMap")?;
        program.use_uniform_block(directional_light.buffer(), "DirectionalLightUniform");

        bind_material(program, &self.material, self.gpu_mesh.has_uvs())?;
        self.gpu_mesh.render(program, transformation, camera)
    }

    fn program_color_ambient(gl: &Gl) -> Result<&Program, Error>
    {
        unsafe {
            if PROGRAM_COLOR_AMBIENT.is_none()
            {
                PROGRAM_COLOR_AMBIENT = Some(GPUMesh::create_program(gl, &format!("{}\n{}",
                                                                              &include_str!("shaders/light_shared.frag"),
                                                                              &include_str!("shaders/colored_forward_ambient.frag")))?);
            }
            Ok(PROGRAM_COLOR_AMBIENT.as_ref().unwrap())
        }
    }

    fn program_color_ambient_directional(gl: &Gl) -> Result<&Program, Error>
    {
        unsafe {
            if PROGRAM_COLOR_AMBIENT_DIRECTIONAL.is_none()
            {
                PROGRAM_COLOR_AMBIENT_DIRECTIONAL = Some(GPUMesh::create_program(gl, &format!("{}\n{}",
                                                                              &include_str!("shaders/light_shared.frag"),
                                                                              &include_str!("shaders/colored_forward_ambient_directional.frag")))?);
            }
            Ok(PROGRAM_COLOR_AMBIENT_DIRECTIONAL.as_ref().unwrap())
        }
    }

    fn program_texture_ambient(gl: &Gl) -> Result<&Program, Error>
    {
        unsafe {
            if PROGRAM_TEXTURE_AMBIENT.is_none()
            {
                PROGRAM_TEXTURE_AMBIENT = Some(GPUMesh::create_program(gl, &format!("{}\n{}",
                                                                                include_str!("shaders/light_shared.frag"),
                                                                                include_str!("shaders/textured_forward_ambient.frag")))?);
            }
            Ok(PROGRAM_TEXTURE_AMBIENT.as_ref().unwrap())
        }
    }

    fn program_texture_ambient_directional(gl: &Gl) -> Result<&Program, Error>
    {
        unsafe {
            if PROGRAM_TEXTURE_AMBIENT_DIRECTIONAL.is_none()
            {
                PROGRAM_TEXTURE_AMBIENT_DIRECTIONAL = Some(GPUMesh::create_program(gl, &format!("{}\n{}",
                                                                            include_str!("shaders/light_shared.frag"),
                                                                            include_str!("shaders/textured_forward_ambient_directional.frag")))?)
            }
            Ok(PROGRAM_TEXTURE_AMBIENT_DIRECTIONAL.as_ref().unwrap())
        }
    }
}

impl Drop for PhongForwardMesh {

    fn drop(&mut self) {
        unsafe {
            MESH_COUNT -= 1;
            if MESH_COUNT == 0 {
                PROGRAM_COLOR_AMBIENT = None;
                PROGRAM_COLOR_AMBIENT_DIRECTIONAL = None;
                PROGRAM_TEXTURE_AMBIENT = None;
                PROGRAM_TEXTURE_AMBIENT_DIRECTIONAL = None;
            }
        }
    }
}

pub struct PhongForwardInstancedMesh {
    gl: Gl,
    pub name: String,
    program_color_ambient: Rc<Program>,
    program_color_ambient_directional: Rc<Program>,
    program_texture_ambient: Rc<Program>,
    program_texture_ambient_directional: Rc<Program>,
    gpu_mesh: InstancedGPUMesh,
    pub material: PhongMaterial
}

impl PhongForwardInstancedMesh
{
    pub fn new(gl: &Gl, transformations: &[Mat4], cpu_mesh: &CPUMesh, material: &PhongMaterial) -> Result<Self, Error>
    {
        if cpu_mesh.normals.is_none() {
            Err(Error::FailedToCreateMesh {message:
              "Cannot create a mesh without normals. Consider calling compute_normals on the CPUMesh before creating the mesh.".to_string()})?
        }
        Self::new_with_programs(gl, transformations, Self::program_color_ambient(gl)?,
                                Self::program_color_ambient_directional(gl)?,
                                Self::program_texture_ambient(gl)?,
                                Self::program_texture_ambient_directional(gl)?, cpu_mesh, material)
    }

    pub fn update_transformations(&mut self, transformations: &[Mat4])
    {
        self.gpu_mesh.update_transformations(transformations);
    }

    pub fn render_depth(&self, transformation: &Mat4, camera: &camera::Camera) -> Result<(), Error>
    {
        self.render_with_ambient(transformation, camera, &AmbientLight::new(&self.gl, 0.0, &vec3(0.0, 0.0, 0.0))?)
    }

    pub fn render_with_ambient(&self, transformation: &Mat4, camera: &camera::Camera, ambient_light: &AmbientLight) -> Result<(), Error>
    {
        let program = match self.material.color_source {
            ColorSource::Color(_) => self.program_color_ambient.as_ref(),
            ColorSource::Texture(_) => self.program_texture_ambient.as_ref()
        };
        program.add_uniform_vec3("ambientLight.color", &ambient_light.color())?;
        program.add_uniform_float("ambientLight.intensity", &ambient_light.intensity())?;

        bind_material(program, &self.material, self.gpu_mesh.has_uvs())?;
        self.gpu_mesh.render(program, transformation, camera)
    }

    pub fn render_with_ambient_and_directional(&self, transformation: &Mat4, camera: &camera::Camera, ambient_light: &AmbientLight, directional_light: &DirectionalLight) -> Result<(), Error>
    {
        let program = match self.material.color_source {
            ColorSource::Color(_) => self.program_color_ambient_directional.as_ref(),
            ColorSource::Texture(_) => self.program_texture_ambient_directional.as_ref()
        };
        program.add_uniform_vec3("ambientLight.color", &ambient_light.color())?;
        program.add_uniform_float("ambientLight.intensity", &ambient_light.intensity())?;
        program.add_uniform_vec3("eyePosition", &camera.position())?;
        program.use_texture(directional_light.shadow_map(), "shadowMap")?;
        program.use_uniform_block(directional_light.buffer(), "DirectionalLightUniform");

        bind_material(program, &self.material, self.gpu_mesh.has_uvs())?;
        self.gpu_mesh.render(program, transformation, camera)
    }

    pub(crate) fn program_color_ambient(gl: &Gl) -> Result<Rc<Program>, Error>
    {
        Ok(Rc::new(InstancedGPUMesh::create_program(gl, &format!("{}\n{}",
                                                                              &include_str!("shaders/light_shared.frag"),
                                                                              &include_str!("shaders/colored_forward_ambient.frag")))?))
    }

    pub(crate) fn program_color_ambient_directional(gl: &Gl) -> Result<Rc<Program>, Error>
    {
        Ok(Rc::new(InstancedGPUMesh::create_program(gl, &format!("{}\n{}",
                                                                              &include_str!("shaders/light_shared.frag"),
                                                                              &include_str!("shaders/colored_forward_ambient_directional.frag")))?))
    }

    pub(crate) fn program_texture_ambient(gl: &Gl) -> Result<Rc<Program>, Error>
    {
        Ok(Rc::new(InstancedGPUMesh::create_program(gl, &format!("{}\n{}",
                                                                                include_str!("shaders/light_shared.frag"),
                                                                                include_str!("shaders/textured_forward_ambient.frag")))?))
    }

    pub(crate) fn program_texture_ambient_directional(gl: &Gl) -> Result<Rc<Program>, Error>
    {
        Ok(Rc::new(InstancedGPUMesh::create_program(gl, &format!("{}\n{}",
                                                                                include_str!("shaders/light_shared.frag"),
                                                                                include_str!("shaders/textured_forward_ambient_directional.frag")))?))
    }

    pub(crate) fn new_with_programs(gl: &Gl, transformations: &[Mat4], program_color_ambient: Rc<Program>, program_color_ambient_directional: Rc<Program>,
                                    program_texture_ambient: Rc<Program>, program_texture_ambient_directional: Rc<Program>,
                   cpu_mesh: &CPUMesh, material: &PhongMaterial) -> Result<Self, Error>
    {
        Ok(Self { gl: gl.clone(), name: cpu_mesh.name.clone(), gpu_mesh: InstancedGPUMesh::new(gl, transformations, cpu_mesh)?,
            program_color_ambient, program_color_ambient_directional, program_texture_ambient,
            program_texture_ambient_directional, material: material.clone() })
    }
}

fn bind_material(program: &Program, material: &PhongMaterial, has_uvs: bool) -> Result<(), Error> {
    program.add_uniform_float("diffuse_intensity", &material.diffuse_intensity)?;
    program.add_uniform_float("specular_intensity", &material.specular_intensity)?;
    program.add_uniform_float("specular_power", &material.specular_power)?;

    match material.color_source {
        ColorSource::Color(ref color) => {
            program.add_uniform_vec4("color", color)?;
        },
        ColorSource::Texture(ref texture) => {
            if !has_uvs {
                Err(Error::FailedToCreateMesh {message:"Cannot use a texture as color source without uv coordinates.".to_string()})?;
            }
            program.use_texture(texture.as_ref(),"tex")?;
        }
    }
    Ok(())
}