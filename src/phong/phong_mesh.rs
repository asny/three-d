
use crate::math::*;
use crate::definition::*;
use crate::core::*;
use crate::camera::*;
use crate::object::*;
use crate::light::*;
use crate::phong::*;

///
/// A triangle mesh that adds additional lighting functionality based on the Phong shading model to a [Mesh](crate::Mesh).
///
pub struct PhongMesh {
    context: Context,
    pub name: String,
    mesh: Mesh,
    pub material: PhongMaterial
}

impl PhongMesh
{
    pub fn new(context: &Context, cpu_mesh: &CPUMesh, material: &PhongMaterial) -> Result<Self, Error>
    {
        let mesh = Mesh::new(context, cpu_mesh)?;
        unsafe {
            MESH_COUNT += 1;
        }
        Ok(Self {
            context: context.clone(),
            name: cpu_mesh.name.clone(),
            mesh,
            material: material.clone()
        })
    }

    ///
    /// Render the geometry and surface material parameters of the mesh, ie. the first part of a deferred render pass.
    /// Must be called inside the **render** closure given to [PhongDeferredPipeline::geometry_pass](crate::PhongDeferredPipeline::geometry_pass).
    ///
    pub fn render_geometry(&self, render_states: RenderStates, viewport: Viewport, transformation: &Mat4, camera: &Camera) -> Result<(), Error>
    {
        let program = match self.material.color_source {
            ColorSource::Color(_) => {
                unsafe {
                    if PROGRAM_COLOR.is_none()
                    {
                        PROGRAM_COLOR = Some(MeshProgram::new(&self.context, &format!("{}\n{}",
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
                        PROGRAM_TEXTURE = Some(MeshProgram::new(&self.context, &format!("{}\n{}",
                                                                                        include_str!("shaders/deferred_objects_shared.frag"),
                                                                                        include_str!("shaders/textured_deferred.frag")))?);
                    }
                    PROGRAM_TEXTURE.as_ref().unwrap()
                }
            }
        };
        self.material.bind(program)?;
        self.mesh.render(program, render_states, viewport, transformation, camera)
    }

    ///
    /// Render the triangle mesh shaded with an ambient light.
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    ///
    pub fn render_with_ambient(&self, render_states: RenderStates, viewport: Viewport, transformation: &Mat4, camera: &Camera, ambient_light: &AmbientLight) -> Result<(), Error>
    {
        let program = match self.material.color_source {
            ColorSource::Color(_) => {
                unsafe {
                    if PROGRAM_COLOR_AMBIENT.is_none()
                    {
                        PROGRAM_COLOR_AMBIENT = Some(MeshProgram::new(&self.context, include_str!("shaders/colored_forward_ambient.frag"))?);
                    }
                    PROGRAM_COLOR_AMBIENT.as_ref().unwrap()
                }
            },
            ColorSource::Texture(_) => {
                unsafe {
                    if PROGRAM_TEXTURE_AMBIENT.is_none()
                    {
                        PROGRAM_TEXTURE_AMBIENT = Some(MeshProgram::new(&self.context,include_str!("shaders/textured_forward_ambient.frag"))?);
                    }
                    PROGRAM_TEXTURE_AMBIENT.as_ref().unwrap()
                }
            }
        };
        program.use_uniform_vec3("ambientColor", &(ambient_light.color * ambient_light.intensity))?;

        match self.material.color_source {
            ColorSource::Color(ref color) => {
                program.use_uniform_vec4("surfaceColor", color)?;
            },
            ColorSource::Texture(ref texture) => {
                program.use_texture(texture.as_ref(),"tex")?;
            }
        }
        self.mesh.render(program, render_states, viewport,transformation, camera)
    }

    ///
    /// Render the triangle mesh shaded with an ambient and a directional light.
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    ///
    pub fn render_with_ambient_and_directional(&self, render_states: RenderStates, viewport: Viewport, transformation: &Mat4, camera: &Camera, ambient_light: &AmbientLight, directional_light: &DirectionalLight) -> Result<(), Error>
    {
        let program = match self.material.color_source {
            ColorSource::Color(_) => {
                unsafe {
                    if PROGRAM_COLOR_AMBIENT_DIRECTIONAL.is_none()
                    {
                        PROGRAM_COLOR_AMBIENT_DIRECTIONAL = Some(MeshProgram::new(&self.context, &format!("{}\n{}",
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
                        PROGRAM_TEXTURE_AMBIENT_DIRECTIONAL = Some(MeshProgram::new(&self.context, &format!("{}\n{}",
                                                                                    include_str!("shaders/light_shared.frag"),
                                                                                    include_str!("shaders/textured_forward_ambient_directional.frag")))?)
                    }
                    PROGRAM_TEXTURE_AMBIENT_DIRECTIONAL.as_ref().unwrap()
                }
            }
        };
        program.use_uniform_vec3("ambientColor", &(ambient_light.color * ambient_light.intensity))?;

        program.use_uniform_vec3("eyePosition", &camera.position())?;
        program.use_texture(directional_light.shadow_map(), "shadowMap")?;
        program.use_uniform_block(directional_light.buffer(), "DirectionalLightUniform");
        self.material.bind(program)?;
        self.mesh.render(program, render_states, viewport, transformation, camera)
    }
}

impl std::ops::Deref for PhongMesh {
    type Target = Mesh;

    fn deref(&self) -> &Mesh {
        &self.mesh
    }
}

impl Drop for PhongMesh {

    fn drop(&mut self) {
        unsafe {
            MESH_COUNT -= 1;
            if MESH_COUNT == 0 {
                PROGRAM_COLOR = None;
                PROGRAM_TEXTURE = None;
                PROGRAM_COLOR_AMBIENT = None;
                PROGRAM_COLOR_AMBIENT_DIRECTIONAL = None;
                PROGRAM_TEXTURE_AMBIENT = None;
                PROGRAM_TEXTURE_AMBIENT_DIRECTIONAL = None;
            }
        }
    }
}

static mut PROGRAM_COLOR: Option<MeshProgram> = None;
static mut PROGRAM_TEXTURE: Option<MeshProgram> = None;
static mut PROGRAM_COLOR_AMBIENT: Option<MeshProgram> = None;
static mut PROGRAM_COLOR_AMBIENT_DIRECTIONAL: Option<MeshProgram> = None;
static mut PROGRAM_TEXTURE_AMBIENT: Option<MeshProgram> = None;
static mut PROGRAM_TEXTURE_AMBIENT_DIRECTIONAL: Option<MeshProgram> = None;
static mut MESH_COUNT: u32 = 0;
