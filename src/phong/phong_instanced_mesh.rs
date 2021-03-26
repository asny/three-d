use crate::camera::*;
use crate::core::*;
use crate::definition::*;
use crate::light::*;
use crate::math::*;
use crate::object::*;
use crate::phong::*;

///
/// An instanced triangle mesh that adds additional lighting functionality based on the Phong shading model to a [InstancedMesh](crate::InstancedMesh).
///
pub struct PhongInstancedMesh {
    context: Context,
    pub name: String,
    mesh: InstancedMesh,
    pub material: PhongMaterial,
}

impl PhongInstancedMesh {
    pub fn new(
        context: &Context,
        transformations: &[Mat4],
        cpu_mesh: &CPUMesh,
        material: &PhongMaterial,
    ) -> Result<Self, Error> {
        let mesh = InstancedMesh::new(context, transformations, cpu_mesh)?;
        unsafe {
            INSTANCED_MESH_COUNT += 1;
        }
        Ok(Self {
            context: context.clone(),
            name: cpu_mesh.name.clone(),
            mesh,
            material: material.clone(),
        })
    }

    ///
    /// Render the geometry and surface material parameters of the instanced mesh, ie. the first part of a deferred render pass.
    /// Must be called inside the **render** closure given to [PhongDeferredPipeline::geometry_pass](crate::PhongDeferredPipeline::geometry_pass).
    ///
    pub fn render_geometry(
        &self,
        render_states: RenderStates,
        viewport: Viewport,
        camera: &Camera,
    ) -> Result<(), Error> {
        let program = unsafe {
            if PROGRAMS.is_none() {
                PROGRAMS = Some(std::collections::HashMap::new());
            }
            let key = match self.material.color_source {
                ColorSource::Color(_) => "ColorDeferred",
                ColorSource::Texture(_) => "TextureDeferred",
            };
            if !PROGRAMS.as_ref().unwrap().contains_key(key) {
                PROGRAMS.as_mut().unwrap().insert(
                    key.to_string(),
                    match self.material.color_source {
                        ColorSource::Color(_) => InstancedMeshProgram::new(
                            &self.context,
                            &format!(
                                "{}\n{}",
                                include_str!("shaders/deferred_objects_shared.frag"),
                                include_str!("shaders/deferred_color.frag")
                            ),
                        )?,
                        ColorSource::Texture(_) => InstancedMeshProgram::new(
                            &self.context,
                            &format!(
                                "{}\n{}",
                                include_str!("shaders/deferred_objects_shared.frag"),
                                include_str!("shaders/deferred_texture.frag")
                            ),
                        )?,
                    },
                );
            };
            PROGRAMS.as_ref().unwrap().get(key).unwrap()
        };
        self.material.bind(program)?;
        self.mesh
            .render(program, render_states, viewport, camera)
    }

    ///
    /// Render the instanced triangle mesh shaded with the given lights based on the Phong shading model.
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    ///
    pub fn render_with_lighting(
        &self,
        render_states: RenderStates,
        viewport: Viewport,
        camera: &Camera,
        ambient_light: Option<&AmbientLight>,
        directional_lights: &[&DirectionalLight],
        spot_lights: &[&SpotLight],
        point_lights: &[&PointLight],
    ) -> Result<(), Error> {
        let key = format!(
            "{},{},{},{}",
            self.material.color_source,
            directional_lights.len(),
            spot_lights.len(),
            point_lights.len()
        );
        let program = unsafe {
            if PROGRAMS.is_none() {
                PROGRAMS = Some(std::collections::HashMap::new());
            }
            if !PROGRAMS.as_ref().unwrap().contains_key(&key) {
                let surface_functionality = match self.material.color_source {
                    ColorSource::Color(_) => {
                        include_str!("shaders/forward_color_surface.frag")
                    }
                    ColorSource::Texture(_) => {
                        include_str!("shaders/forward_texture_surface.frag")
                    }
                };
                let fragment_shader_source = phong_fragment_shader(
                    &surface_functionality,
                    directional_lights.len(),
                    spot_lights.len(),
                    point_lights.len(),
                );
                PROGRAMS.as_mut().unwrap().insert(
                    key.clone(),
                    InstancedMeshProgram::new(&self.context, &fragment_shader_source)?,
                );
            };
            PROGRAMS.as_ref().unwrap().get(&key).unwrap()
        };

        crate::phong::bind_lights(
            program,
            ambient_light,
            directional_lights,
            spot_lights,
            point_lights,
        )?;

        if !directional_lights.is_empty() || !spot_lights.is_empty() || !point_lights.is_empty() {
            program.use_uniform_vec3("eyePosition", &camera.position())?;
            self.material.bind(program)?;
        } else {
            match self.material.color_source {
                ColorSource::Color(ref color) => {
                    program.use_uniform_vec4("surfaceColor", color)?;
                }
                ColorSource::Texture(ref texture) => {
                    program.use_texture(texture.as_ref(), "tex")?;
                }
            }
        }
        self.mesh
            .render(program, render_states, viewport, camera)?;
        Ok(())
    }
}

impl std::ops::Deref for PhongInstancedMesh {
    type Target = InstancedMesh;

    fn deref(&self) -> &InstancedMesh {
        &self.mesh
    }
}

impl std::ops::DerefMut for PhongInstancedMesh {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.mesh
    }
}

impl Drop for PhongInstancedMesh {
    fn drop(&mut self) {
        unsafe {
            INSTANCED_MESH_COUNT -= 1;
            if INSTANCED_MESH_COUNT == 0 {
                PROGRAMS = None;
            }
        }
    }
}

static mut PROGRAMS: Option<std::collections::HashMap<String, InstancedMeshProgram>> = None;
static mut INSTANCED_MESH_COUNT: u32 = 0;
