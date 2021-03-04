
use crate::math::*;
use crate::core::*;
use crate::camera::*;
use crate::object::*;
use crate::light::*;
use crate::phong::*;

pub struct PhongForwardInstancedMesh {
    context: Context,
    pub name: String,
    mesh: InstancedMesh,
    pub material: PhongMaterial
}

impl PhongForwardInstancedMesh
{
    pub fn new(context: &Context, transformations: &[Mat4], cpu_mesh: &CPUMesh, material: &PhongMaterial) -> Result<Self, Error>
    {
        let mesh = InstancedMesh::new(context, transformations, cpu_mesh)?;
        unsafe {
            INSTANCED_MESH_COUNT += 1;
        }
        Ok(Self {
            context: context.clone(),
            name: cpu_mesh.name.clone(),
            mesh,
            material: material.clone()
        })
    }

    pub fn render_with_ambient(&self, render_states: RenderStates, viewport: Viewport, transformation: &Mat4, camera: &Camera, ambient_light: &AmbientLight) -> Result<(), Error>
    {
        let program = match self.material.color_source {
            ColorSource::Color(_) => {
                unsafe {
                    if INSTANCED_PROGRAM_COLOR_AMBIENT.is_none()
                    {
                        INSTANCED_PROGRAM_COLOR_AMBIENT = Some(InstancedMeshProgram::new(&self.context, include_str!("shaders/colored_forward_ambient.frag"))?);
                    }
                    INSTANCED_PROGRAM_COLOR_AMBIENT.as_ref().unwrap()
                }
            },
            ColorSource::Texture(_) => {
                unsafe {
                    if INSTANCED_PROGRAM_TEXTURE_AMBIENT.is_none()
                    {
                        INSTANCED_PROGRAM_TEXTURE_AMBIENT = Some(InstancedMeshProgram::new(&self.context, include_str!("shaders/textured_forward_ambient.frag"))?);
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
                program.use_texture(texture.as_ref(),"tex")?;
            }
        }
        self.mesh.render(program, render_states, viewport, transformation, camera)
    }

    pub fn render_with_ambient_and_directional(&self, render_states: RenderStates, viewport: Viewport, transformation: &Mat4, camera: &Camera, ambient_light: &AmbientLight, directional_light: &DirectionalLight) -> Result<(), Error>
    {
        let program = match self.material.color_source {
            ColorSource::Color(_) => {
                unsafe {
                    if INSTANCED_PROGRAM_COLOR_AMBIENT_DIRECTIONAL.is_none()
                    {
                        INSTANCED_PROGRAM_COLOR_AMBIENT_DIRECTIONAL = Some(InstancedMeshProgram::new(&self.context, &format!("{}\n{}",
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
                        INSTANCED_PROGRAM_TEXTURE_AMBIENT_DIRECTIONAL = Some(InstancedMeshProgram::new(&self.context, &format!("{}\n{}",
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
        self.material.bind(program)?;
        self.mesh.render(program, render_states, viewport, transformation, camera)
    }
}

impl std::ops::Deref for PhongForwardInstancedMesh {
    type Target = InstancedMesh;

    fn deref(&self) -> &InstancedMesh {
        &self.mesh
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

static mut INSTANCED_PROGRAM_COLOR_AMBIENT: Option<InstancedMeshProgram> = None;
static mut INSTANCED_PROGRAM_COLOR_AMBIENT_DIRECTIONAL: Option<InstancedMeshProgram> = None;
static mut INSTANCED_PROGRAM_TEXTURE_AMBIENT: Option<InstancedMeshProgram> = None;
static mut INSTANCED_PROGRAM_TEXTURE_AMBIENT_DIRECTIONAL: Option<InstancedMeshProgram> = None;
static mut INSTANCED_MESH_COUNT: u32 = 0;