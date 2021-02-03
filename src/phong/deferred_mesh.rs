
use crate::core::*;
use crate::objects::*;
use crate::phong::*;

pub struct PhongDeferredMesh {
    gl: Gl,
    pub name: String,
    gpu_mesh: GPUMesh,
    pub material: PhongMaterial
}

impl PhongDeferredMesh {

    pub fn new(gl: &Gl, cpu_mesh: &CPUMesh, material: &PhongMaterial) -> Result<Self, Error>
    {
        if cpu_mesh.normals.is_none() {
            Err(Error::FailedToCreateMesh {message:
              "Cannot create a mesh without normals. Consider calling compute_normals on the CPUMesh before creating the mesh.".to_string()})?
        }
        let gpu_mesh = GPUMesh::new(gl, cpu_mesh)?;
        unsafe {MESH_COUNT += 1;}
        Ok(Self {
            gl: gl.clone(),
            name: cpu_mesh.name.clone(),
            gpu_mesh,
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
        self.render_geometry(render_states, viewport, transformation, camera)
    }

    pub fn render_geometry(&self, render_states: RenderStates, viewport: Viewport, transformation: &Mat4, camera: &camera::Camera) -> Result<(), Error>
    {
        let program = match self.material.color_source {
            ColorSource::Color(_) => {
                unsafe {
                    if PROGRAM_COLOR.is_none()
                    {
                        PROGRAM_COLOR = Some(GPUMesh::create_program(&self.gl, &format!("{}\n{}",
                                                             include_str!("shaders/deferred_objects_shared.frag"),
                                                             include_str!("shaders/colored_deferred.frag")))?);
                    }
                    PROGRAM_COLOR.as_ref().unwrap()
                }
            },
            ColorSource::Texture(_) => {
                unsafe {
                    if PROGRAM_TEXTURE.is_none()
                    {
                        PROGRAM_TEXTURE = Some(GPUMesh::create_program(&self.gl, &format!("{}\n{}",
                                                             include_str!("shaders/deferred_objects_shared.frag"),
                                                             include_str!("shaders/textured_deferred.frag")))?);
                    }
                    PROGRAM_TEXTURE.as_ref().unwrap()
                }
            }
        };

        bind_material(program, &self.material, self.gpu_mesh.has_uvs())?;
        self.gpu_mesh.render(program, render_states, viewport, transformation, camera)
    }
}

impl Drop for PhongDeferredMesh {

    fn drop(&mut self) {
        unsafe {
            MESH_COUNT -= 1;
            if MESH_COUNT == 0 {
                PROGRAM_COLOR = None;
                PROGRAM_TEXTURE = None;
            }
        }
    }
}

pub struct PhongDeferredInstancedMesh {
    gl: Gl,
    pub name: String,
    gpu_mesh: InstancedGPUMesh,
    pub material: PhongMaterial
}

impl PhongDeferredInstancedMesh
{
    pub fn new(gl: &Gl, transformations: &[Mat4], cpu_mesh: &CPUMesh, material: &PhongMaterial) -> Result<Self, Error>
    {
        if cpu_mesh.normals.is_none() {
            Err(Error::FailedToCreateMesh {message:
              "Cannot create a mesh without normals. Consider calling compute_normals on the CPUMesh before creating the mesh.".to_string()})?
        }
        let gpu_mesh = InstancedGPUMesh::new(gl, transformations, cpu_mesh)?;
        unsafe {INSTANCED_MESH_COUNT += 1;}
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
        self.render_geometry(render_states, viewport, transformation, camera)
    }

    pub fn render_geometry(&self, render_states: RenderStates, viewport: Viewport, transformation: &Mat4, camera: &camera::Camera) -> Result<(), Error>
    {
        let program = match self.material.color_source {
            ColorSource::Color(_) => {
                unsafe {
                    if INSTANCED_PROGRAM_COLOR.is_none()
                    {
                        INSTANCED_PROGRAM_COLOR = Some(InstancedGPUMesh::create_program(&self.gl, &format!("{}\n{}",
                                                             include_str!("shaders/deferred_objects_shared.frag"),
                                                             include_str!("shaders/colored_deferred.frag")))?);
                    }
                    INSTANCED_PROGRAM_COLOR.as_ref().unwrap()
                }
            },
            ColorSource::Texture(_) => {
                unsafe {
                    if INSTANCED_PROGRAM_TEXTURE.is_none()
                    {
                        INSTANCED_PROGRAM_TEXTURE = Some(InstancedGPUMesh::create_program(&self.gl, &format!("{}\n{}",
                                                             include_str!("shaders/deferred_objects_shared.frag"),
                                                             include_str!("shaders/textured_deferred.frag")))?);
                    }
                    INSTANCED_PROGRAM_TEXTURE.as_ref().unwrap()
                }
            }
        };

        bind_material(program, &self.material, self.gpu_mesh.has_uvs())?;
        self.gpu_mesh.render(program, render_states, viewport, transformation, camera)
    }
}

impl Drop for PhongDeferredInstancedMesh {

    fn drop(&mut self) {
        unsafe {
            INSTANCED_MESH_COUNT -= 1;
            if INSTANCED_MESH_COUNT == 0 {
                INSTANCED_PROGRAM_COLOR = None;
                INSTANCED_PROGRAM_TEXTURE = None;
            }
        }
    }
}

static mut PROGRAM_COLOR: Option<Program> = None;
static mut PROGRAM_TEXTURE: Option<Program> = None;
static mut MESH_COUNT: u32 = 0;

static mut INSTANCED_PROGRAM_COLOR: Option<Program> = None;
static mut INSTANCED_PROGRAM_TEXTURE: Option<Program> = None;
static mut INSTANCED_MESH_COUNT: u32 = 0;

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