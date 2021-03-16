
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
    /// Render the triangle mesh shaded with the given lights based on the Phong shading model.
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    ///
    pub fn render_with_lighting(&self, render_states: RenderStates, viewport: Viewport, transformation: &Mat4, camera: &Camera,
                                ambient_light: Option<&AmbientLight>, directional_lights: &[&DirectionalLight],
                                spot_lights: &[&SpotLight], point_lights: &[&PointLight]) -> Result<(), Error>
    {
        let key = format!("{},{},{}", directional_lights.len(), spot_lights.len(), point_lights.len());
        let program = unsafe {
            if PROGRAMS.is_none() {
                PROGRAMS = Some(std::collections::HashMap::new());
            }
            if !PROGRAMS.as_ref().unwrap().contains_key(&key) {
                let surface_functionality = format!("
                    {}
                    uniform float diffuse_intensity;
                    uniform float specular_intensity;
                    uniform float specular_power;

                    in vec3 pos;
                    in vec3 nor;

                    Surface get_surface()
                    {{
                        vec3 normal = normalize(gl_FrontFacing ? nor : -nor);
                        return Surface(pos, normal, get_surface_color(), diffuse_intensity, specular_intensity, specular_power);
                    }}",
                                                            match self.material.color_source {
                                                                ColorSource::Color(_) => {"
                            uniform vec4 surfaceColor;
                            vec4 get_surface_color()
                            {{
                                return surfaceColor;
                            }}"},
                                                                ColorSource::Texture(_) => { "
                            uniform sampler2D tex;
                            in vec2 uvs;
                            vec4 get_surface_color()
                            {{
                                return texture(tex, vec2(uvs.x, 1.0 - uvs.y));
                            }}"
                                                                }
                                                            });
                let fragment_shader_source = phong_fragment_shader(&surface_functionality,
                                                                   directional_lights.len(),
                                                                   spot_lights.len(),
                                                                   point_lights.len());
                PROGRAMS.as_mut().unwrap().insert(key.clone(), crate::MeshProgram::new(&self.context, &fragment_shader_source)?);
            };
            PROGRAMS.as_ref().unwrap().get(&key).unwrap()
        };

        crate::phong::bind_lights(program, ambient_light, directional_lights, spot_lights, point_lights)?;

        if !directional_lights.is_empty() || !spot_lights.is_empty() || !point_lights.is_empty() {
            program.use_uniform_vec3("eyePosition", &camera.position())?;
            self.material.bind(program)?;
        } else {
            match self.material.color_source {
                ColorSource::Color(ref color) => {
                    program.use_uniform_vec4("surfaceColor", color)?;
                },
                ColorSource::Texture(ref texture) => {
                    program.use_texture(texture.as_ref(),"tex")?;
                }
            }
        }
        self.mesh.render(program, render_states, viewport, transformation, camera)?;
        Ok(())
    }
}

impl std::ops::Deref for PhongMesh {
    type Target = Mesh;

    fn deref(&self) -> &Mesh {
        &self.mesh
    }
}

impl std::ops::DerefMut for PhongMesh {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.mesh
    }
}

impl Drop for PhongMesh {

    fn drop(&mut self) {
        unsafe {
            MESH_COUNT -= 1;
            if MESH_COUNT == 0 {
                PROGRAM_COLOR = None;
                PROGRAM_TEXTURE = None;
                PROGRAMS = None;
            }
        }
    }
}

static mut PROGRAM_COLOR: Option<MeshProgram> = None;
static mut PROGRAM_TEXTURE: Option<MeshProgram> = None;
static mut MESH_COUNT: u32 = 0;
static mut PROGRAMS: Option<std::collections::HashMap<String, crate::MeshProgram>> = None;