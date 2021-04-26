use crate::camera::*;
use crate::core::*;
use crate::definition::*;
use crate::light::*;
use crate::math::*;
use crate::object::*;
use crate::phong::*;

///
/// A triangle mesh that adds additional lighting functionality based on the Phong shading model to a [Mesh](crate::Mesh).
///
pub struct PhongMesh {
    context: Context,
    mesh: Mesh,
    pub material: PhongMaterial,
}

impl PhongMesh {
    pub fn new(
        context: &Context,
        cpu_mesh: &CPUMesh,
        material: &PhongMaterial,
    ) -> Result<Self, Error> {
        let mesh = Mesh::new(context, cpu_mesh)?;
        unsafe {
            MESH_COUNT += 1;
        }
        Ok(Self {
            context: context.clone(),
            mesh,
            material: material.clone(),
        })
    }

    ///
    /// Render the triangle mesh shaded with the given lights based on the Phong shading model.
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
                    MeshProgram::new(&self.context, &fragment_shader_source)?,
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
        self.mesh.render(program, render_states, viewport, camera)?;
        Ok(())
    }
}

impl Geometry for PhongMesh {
    fn render_depth_to_red(
        &self,
        render_states: RenderStates,
        viewport: Viewport,
        camera: &Camera,
        max_depth: f32,
    ) -> Result<(), Error> {
        self.mesh
            .render_depth_to_red(render_states, viewport, camera, max_depth)
    }

    fn render_depth(
        &self,
        render_states: RenderStates,
        viewport: Viewport,
        camera: &Camera,
    ) -> Result<(), Error> {
        self.mesh.render_depth(render_states, viewport, camera)
    }

    fn aabb(&self) -> Option<AxisAlignedBoundingBox> {
        self.mesh.aabb()
    }
}

impl PhongGeometry for PhongMesh {
    fn geometry_pass(
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
                        ColorSource::Color(_) => MeshProgram::new(
                            &self.context,
                            &format!(
                                "{}\n{}",
                                include_str!("shaders/deferred_objects_shared.frag"),
                                include_str!("shaders/deferred_color.frag")
                            ),
                        )?,
                        ColorSource::Texture(_) => MeshProgram::new(
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
        self.mesh.render(program, render_states, viewport, camera)
    }
}

impl Clone for PhongMesh {
    fn clone(&self) -> Self {
        unsafe {
            MESH_COUNT += 1;
        }
        Self {
            context: self.context.clone(),
            mesh: self.mesh.clone(),
            material: self.material.clone(),
        }
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
                PROGRAMS = None;
            }
        }
    }
}

static mut MESH_COUNT: u32 = 0;
static mut PROGRAMS: Option<std::collections::HashMap<String, MeshProgram>> = None;
