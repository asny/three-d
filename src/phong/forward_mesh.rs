
use crate::core::*;
use crate::scene::*;
use crate::phong::*;

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

    pub fn new_meshes(gl: &Gl, cpu_meshes: &[CPUMesh], materials: &[PhongMaterial]) -> Result<Vec<Self>, Error>
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

    pub fn render_depth(&self, render_states: RenderStates, viewport: Viewport, transformation: &Mat4, camera: &camera::Camera) -> Result<(), Error>
    {
        self.render_with_ambient(render_states, viewport, transformation, camera, &AmbientLight::default())
    }

    pub fn render_with_ambient(&self, render_states: RenderStates, viewport: Viewport, transformation: &Mat4, camera: &camera::Camera, ambient_light: &AmbientLight) -> Result<(), Error>
    {
        let program = match self.material.color_source {
            ColorSource::Color(_) => {
                unsafe {
                    if PROGRAM_COLOR_AMBIENT.is_none()
                    {
                        PROGRAM_COLOR_AMBIENT = Some(GPUMesh::create_program(&self.gl, include_str!("shaders/colored_forward_ambient.frag"))?);
                    }
                    PROGRAM_COLOR_AMBIENT.as_ref().unwrap()
                }
            },
            ColorSource::Texture(_) => {
                unsafe {
                    if PROGRAM_TEXTURE_AMBIENT.is_none()
                    {
                        PROGRAM_TEXTURE_AMBIENT = Some(GPUMesh::create_program(&self.gl,include_str!("shaders/textured_forward_ambient.frag"))?);
                    }
                    PROGRAM_TEXTURE_AMBIENT.as_ref().unwrap()
                }
            }
        };
        program.add_uniform_vec3("ambientColor", &(ambient_light.color * ambient_light.intensity))?;

        match self.material.color_source {
            ColorSource::Color(ref color) => {
                program.add_uniform_vec4("surfaceColor", color)?;
            },
            ColorSource::Texture(ref texture) => {
                if !self.gpu_mesh.has_uvs() {
                    Err(Error::FailedToCreateMesh {message:"Cannot use a texture as color source without uv coordinates.".to_string()})?;
                }
                program.use_texture(texture.as_ref(),"tex")?;
            }
        }
        self.gpu_mesh.render(program, render_states, viewport,transformation, camera)
    }

    pub fn render_with_ambient_and_directional(&self, render_states: RenderStates, viewport: Viewport, transformation: &Mat4, camera: &camera::Camera, ambient_light: &AmbientLight, directional_light: &DirectionalLight) -> Result<(), Error>
    {
        let program = match self.material.color_source {
            ColorSource::Color(_) => {
                unsafe {
                    if PROGRAM_COLOR_AMBIENT_DIRECTIONAL.is_none()
                    {
                        PROGRAM_COLOR_AMBIENT_DIRECTIONAL = Some(GPUMesh::create_program(&self.gl, &format!("{}\n{}",
                                                                                      &include_str!("shaders/light_shared.frag"),
                                                                                      &include_str!("shaders/colored_forward_ambient_directional.frag")))?);
                    }
                    PROGRAM_COLOR_AMBIENT_DIRECTIONAL.as_ref().unwrap()
                }
            },
            ColorSource::Texture(_) => {
                unsafe {
                    if PROGRAM_TEXTURE_AMBIENT_DIRECTIONAL.is_none()
                    {
                        PROGRAM_TEXTURE_AMBIENT_DIRECTIONAL = Some(GPUMesh::create_program(&self.gl, &format!("{}\n{}",
                                                                                    include_str!("shaders/light_shared.frag"),
                                                                                    include_str!("shaders/textured_forward_ambient_directional.frag")))?)
                    }
                    PROGRAM_TEXTURE_AMBIENT_DIRECTIONAL.as_ref().unwrap()
                }
            }
        };
        program.add_uniform_vec3("ambientColor", &(ambient_light.color * ambient_light.intensity))?;

        program.add_uniform_vec3("eyePosition", &camera.position())?;
        program.use_texture(directional_light.shadow_map(), "shadowMap")?;
        program.use_uniform_block(directional_light.buffer(), "DirectionalLightUniform");

        bind_material(program, &self.material, self.gpu_mesh.has_uvs())?;
        self.gpu_mesh.render(program, render_states, viewport, transformation, camera)
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
        let gpu_mesh = InstancedGPUMesh::new(gl, transformations, cpu_mesh)?;
        unsafe {
            INSTANCED_MESH_COUNT += 1;
        }
        Ok(Self {
            gl: gl.clone(),
            name: cpu_mesh.name.clone(),
            gpu_mesh,
            material: material.clone()
        })
    }

    pub fn update_transformations(&mut self, transformations: &[Mat4])
    {
        self.gpu_mesh.update_transformations(transformations);
    }

    pub fn render_depth(&self, render_states: RenderStates, viewport: Viewport, transformation: &Mat4, camera: &camera::Camera) -> Result<(), Error>
    {
        self.render_with_ambient(render_states, viewport, transformation, camera, &AmbientLight::default())
    }

    pub fn render_with_ambient(&self, render_states: RenderStates, viewport: Viewport, transformation: &Mat4, camera: &camera::Camera, ambient_light: &AmbientLight) -> Result<(), Error>
    {
        let program = match self.material.color_source {
            ColorSource::Color(_) => {
                unsafe {
                    if INSTANCED_PROGRAM_COLOR_AMBIENT.is_none()
                    {
                        INSTANCED_PROGRAM_COLOR_AMBIENT = Some(InstancedGPUMesh::create_program(&self.gl, include_str!("shaders/colored_forward_ambient.frag"))?);
                    }
                    INSTANCED_PROGRAM_COLOR_AMBIENT.as_ref().unwrap()
                }
            },
            ColorSource::Texture(_) => {
                unsafe {
                    if INSTANCED_PROGRAM_TEXTURE_AMBIENT.is_none()
                    {
                        INSTANCED_PROGRAM_TEXTURE_AMBIENT = Some(InstancedGPUMesh::create_program(&self.gl, include_str!("shaders/textured_forward_ambient.frag"))?);
                    }
                    INSTANCED_PROGRAM_TEXTURE_AMBIENT.as_ref().unwrap()
                }
            }
        };
        program.add_uniform_vec3("ambientColor", &(ambient_light.color * ambient_light.intensity))?;

        match self.material.color_source {
            ColorSource::Color(ref color) => {
                program.add_uniform_vec4("surfaceColor", color)?;
            },
            ColorSource::Texture(ref texture) => {
                if !self.gpu_mesh.has_uvs() {
                    Err(Error::FailedToCreateMesh {message:"Cannot use a texture as color source without uv coordinates.".to_string()})?;
                }
                program.use_texture(texture.as_ref(),"tex")?;
            }
        }
        self.gpu_mesh.render(program, render_states, viewport, transformation, camera)
    }

    pub fn render_with_ambient_and_directional(&self, render_states: RenderStates, viewport: Viewport, transformation: &Mat4, camera: &camera::Camera, ambient_light: &AmbientLight, directional_light: &DirectionalLight) -> Result<(), Error>
    {
        let program = match self.material.color_source {
            ColorSource::Color(_) => {
                unsafe {
                    if INSTANCED_PROGRAM_COLOR_AMBIENT_DIRECTIONAL.is_none()
                    {
                        INSTANCED_PROGRAM_COLOR_AMBIENT_DIRECTIONAL = Some(InstancedGPUMesh::create_program(&self.gl, &format!("{}\n{}",
                                                                                      &include_str!("shaders/light_shared.frag"),
                                                                                      &include_str!("shaders/colored_forward_ambient_directional.frag")))?);
                    }
                    INSTANCED_PROGRAM_COLOR_AMBIENT_DIRECTIONAL.as_ref().unwrap()
                }
            },
            ColorSource::Texture(_) => {
                unsafe {
                    if INSTANCED_PROGRAM_TEXTURE_AMBIENT_DIRECTIONAL.is_none()
                    {
                        INSTANCED_PROGRAM_TEXTURE_AMBIENT_DIRECTIONAL = Some(InstancedGPUMesh::create_program(&self.gl, &format!("{}\n{}",
                                                                                    include_str!("shaders/light_shared.frag"),
                                                                                    include_str!("shaders/textured_forward_ambient_directional.frag")))?)
                    }
                    INSTANCED_PROGRAM_TEXTURE_AMBIENT_DIRECTIONAL.as_ref().unwrap()
                }
            }
        };
        program.add_uniform_vec3("ambientColor", &(ambient_light.color * ambient_light.intensity))?;
        program.add_uniform_vec3("eyePosition", &camera.position())?;
        program.use_texture(directional_light.shadow_map(), "shadowMap")?;
        program.use_uniform_block(directional_light.buffer(), "DirectionalLightUniform");

        bind_material(program, &self.material, self.gpu_mesh.has_uvs())?;
        self.gpu_mesh.render(program, render_states, viewport, transformation, camera)
    }
}

impl Drop for PhongForwardInstancedMesh {

    fn drop(&mut self) {
        unsafe {
            INSTANCED_MESH_COUNT -= 1;
            if INSTANCED_MESH_COUNT == 0 {
                INSTANCED_PROGRAM_COLOR_AMBIENT = None;
                INSTANCED_PROGRAM_COLOR_AMBIENT_DIRECTIONAL = None;
                INSTANCED_PROGRAM_TEXTURE_AMBIENT = None;
                INSTANCED_PROGRAM_TEXTURE_AMBIENT_DIRECTIONAL = None;
            }
        }
    }
}

static mut PROGRAM_COLOR_AMBIENT: Option<Program> = None;
static mut PROGRAM_COLOR_AMBIENT_DIRECTIONAL: Option<Program> = None;
static mut PROGRAM_TEXTURE_AMBIENT: Option<Program> = None;
static mut PROGRAM_TEXTURE_AMBIENT_DIRECTIONAL: Option<Program> = None;
static mut MESH_COUNT: u32 = 0;

static mut INSTANCED_PROGRAM_COLOR_AMBIENT: Option<Program> = None;
static mut INSTANCED_PROGRAM_COLOR_AMBIENT_DIRECTIONAL: Option<Program> = None;
static mut INSTANCED_PROGRAM_TEXTURE_AMBIENT: Option<Program> = None;
static mut INSTANCED_PROGRAM_TEXTURE_AMBIENT_DIRECTIONAL: Option<Program> = None;
static mut INSTANCED_MESH_COUNT: u32 = 0;

fn bind_material(program: &Program, material: &PhongMaterial, has_uvs: bool) -> Result<(), Error> {
    program.add_uniform_float("diffuse_intensity", &material.diffuse_intensity)?;
    program.add_uniform_float("specular_intensity", &material.specular_intensity)?;
    program.add_uniform_float("specular_power", &material.specular_power)?;

    match material.color_source {
        ColorSource::Color(ref color) => {
            program.add_uniform_vec4("surfaceColor", color)?;
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