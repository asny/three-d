use crate::core::*;
use crate::renderer::*;

///
/// Similar to [Model], except it is possible to render many instances of the same model efficiently. See [InstancedMesh] if you need a custom render function.
///
pub struct InstancedModel {
    context: Context,
    pub(in crate::renderer) mesh: InstancedMesh,
    pub cull: Cull,
}

impl InstancedModel {
    pub fn new(context: &Context, transformations: &[Mat4], cpu_mesh: &CPUMesh) -> Result<Self> {
        let mesh = InstancedMesh::new(context, transformations, cpu_mesh)?;
        unsafe {
            MESH_COUNT += 1;
        }
        Ok(Self {
            context: context.clone(),
            mesh,
            cull: Cull::default(),
        })
    }

    ///
    /// Returns the local to world transformation applied to all instances.
    ///
    pub fn transformation(&self) -> &Mat4 {
        self.mesh.transformation()
    }

    ///
    /// Set the local to world transformation applied to all instances.
    ///
    pub fn set_transformation(&mut self, transformation: Mat4) {
        self.mesh.set_transformation(transformation);
    }

    ///
    /// Render the instanced model with a color per triangle vertex. The colors are defined when constructing the instanced model.
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    /// The transformation can be used to position, orientate and scale the instanced model.
    ///
    /// # Errors
    /// Will return an error if the instanced model has no colors.
    ///
    pub fn render_color(&self, camera: &Camera) -> Result<()> {
        let program = self.get_or_insert_program(&format!(
            "{}{}",
            include_str!("../../core/shared.frag"),
            include_str!("shaders/mesh_vertex_color.frag")
        ))?;
        Ok(self.mesh.render(
            self.render_states(self.mesh.transparent),
            program,
            camera.uniform_buffer(),
            camera.viewport(),
        )?)
    }

    ///
    /// Render the instanced model with the given color. The color is assumed to be in gamma color space (sRGBA).
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    /// The transformation can be used to position, orientate and scale the instanced model.
    ///
    #[deprecated = "Use 'render' instead"]
    pub fn render_with_color(&self, color: Color, camera: &Camera) -> Result<()> {
        self.render(
            &ColorMaterial {
                color,
                ..Default::default()
            },
            camera,
            None,
            &[],
            &[],
            &[],
        )
    }

    ///
    /// Render the instanced model with the given texture which is assumed to be in sRGB color space with or without an alpha channel.
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    /// The transformation can be used to position, orientate and scale the instanced model.
    ///
    /// # Errors
    /// Will return an error if the instanced model has no uv coordinates.
    ///
    #[deprecated = "Use 'render' instead"]
    pub fn render_with_texture(&self, texture: &impl Texture, camera: &Camera) -> Result<()> {
        let program = self.get_or_insert_program(include_str!("shaders/mesh_texture.frag"))?;
        program.use_texture("tex", texture)?;
        Ok(self.mesh.render(
            self.render_states(texture.is_transparent()),
            program,
            camera.uniform_buffer(),
            camera.viewport(),
        )?)
    }

    pub(in crate::renderer) fn render_states(&self, transparent: bool) -> RenderStates {
        if transparent {
            RenderStates {
                cull: self.cull,
                write_mask: WriteMask::COLOR,
                blend: Blend::TRANSPARENCY,
                ..Default::default()
            }
        } else {
            RenderStates {
                cull: self.cull,
                ..Default::default()
            }
        }
    }

    pub(in crate::renderer) fn get_or_insert_program(
        &self,
        fragment_shader_source: &str,
    ) -> Result<&InstancedMeshProgram> {
        unsafe {
            if PROGRAMS.is_none() {
                PROGRAMS = Some(std::collections::HashMap::new());
            }
            if !PROGRAMS
                .as_ref()
                .unwrap()
                .contains_key(fragment_shader_source)
            {
                PROGRAMS.as_mut().unwrap().insert(
                    fragment_shader_source.to_string(),
                    InstancedMeshProgram::new(&self.context, fragment_shader_source)?,
                );
            };
            Ok(PROGRAMS
                .as_ref()
                .unwrap()
                .get(fragment_shader_source)
                .unwrap())
        }
    }
}

impl Geometry for InstancedModel {
    fn render_depth_to_red(&self, camera: &Camera, max_depth: f32) -> Result<()> {
        self.render(
            &PickMaterial {
                max_distance: Some(max_depth),
                ..Default::default()
            },
            camera,
            None,
            &[],
            &[],
            &[],
        )
    }

    fn render_depth(&self, camera: &Camera) -> Result<()> {
        self.render(&DepthMaterial {}, camera, None, &[], &[], &[])
    }
}

impl Object for InstancedModel {
    fn render(
        &self,
        material: &dyn Paint,
        camera: &Camera,
        ambient_light: Option<&AmbientLight>,
        directional_lights: &[&DirectionalLight],
        spot_lights: &[&SpotLight],
        point_lights: &[&PointLight],
    ) -> Result<()> {
        let mut render_states = material.render_states();
        render_states.cull = self.cull;
        let fragment_shader_source = material.fragment_shader_source(
            ambient_light,
            directional_lights,
            spot_lights,
            point_lights,
        );
        self.context.program(
            &InstancedMesh::vertex_shader_source(&fragment_shader_source),
            &fragment_shader_source,
            |program| {
                material.bind(
                    program,
                    camera,
                    ambient_light,
                    directional_lights,
                    spot_lights,
                    point_lights,
                )?;
                self.mesh.render(
                    render_states,
                    program,
                    camera.uniform_buffer(),
                    camera.viewport(),
                )
            },
        )
    }

    fn render_deferred(
        &self,
        material: &dyn DeferredMaterial,
        camera: &Camera,
        viewport: Viewport,
    ) -> Result<()> {
        let mut render_states = material.render_states();
        render_states.cull = self.cull;
        let fragment_shader_source = material.fragment_shader_source();
        self.context.program(
            &Mesh::vertex_shader_source(&fragment_shader_source),
            &fragment_shader_source,
            |program| {
                material.bind(program)?;
                self.mesh
                    .render(render_states, program, camera.uniform_buffer(), viewport)
            },
        )
    }

    fn aabb(&self) -> AxisAlignedBoundingBox {
        AxisAlignedBoundingBox::new_infinite() // TODO: Compute bounding box
    }
}

impl ShadedGeometry for InstancedModel {
    fn geometry_pass(
        &self,
        camera: &Camera,
        viewport: Viewport,
        material: &Material,
    ) -> Result<()> {
        self.render_deferred(material, camera, viewport)
    }

    fn render_with_lighting(
        &self,
        camera: &Camera,
        material: &Material,
        lighting_model: LightingModel,
        ambient_light: Option<&AmbientLight>,
        directional_lights: &[&DirectionalLight],
        spot_lights: &[&SpotLight],
        point_lights: &[&PointLight],
    ) -> Result<()> {
        let mut mat = material.clone();
        mat.lighting_model = lighting_model;
        self.render(
            &mat,
            camera,
            ambient_light,
            directional_lights,
            spot_lights,
            point_lights,
        )
    }
}

impl Drop for InstancedModel {
    fn drop(&mut self) {
        unsafe {
            MESH_COUNT -= 1;
            if MESH_COUNT == 0 {
                PROGRAMS = None;
            }
        }
    }
}

static mut PROGRAMS: Option<std::collections::HashMap<String, InstancedMeshProgram>> = None;
static mut MESH_COUNT: u32 = 0;
