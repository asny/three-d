use crate::core::*;
use crate::renderer::*;

///
/// A triangle mesh which can be rendered with one of the standard render functions. See [Mesh] if you need a custom render function.
///
pub struct Model {
    context: Context,
    pub(in crate::renderer) mesh: Mesh,
    pub cull: Cull,
}

impl Model {
    pub fn new(context: &Context, cpu_mesh: &CPUMesh) -> Result<Self> {
        let mesh = Mesh::new(context, cpu_mesh)?;
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
    /// Returns the local to world transformation of this model.
    ///
    pub fn transformation(&self) -> &Mat4 {
        self.mesh.transformation()
    }

    ///
    /// Set the local to world transformation of this model.
    ///
    pub fn set_transformation(&mut self, transformation: Mat4) {
        self.mesh.set_transformation(transformation);
    }

    ///
    /// Render the mesh with a color per triangle vertex. The colors are defined when constructing the mesh and are assumed to be in gamma color space (sRGBA).
    /// Must be called in a render target render function, for example in the callback function of [Screen::write].
    /// Will render the model transparent if the colors contain alpha values below 255, you only need to render the model after all solid models.
    ///
    /// # Errors
    /// Will return an error if the mesh has no colors.
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
    /// Render the mesh with the given color. The color is assumed to be in gamma color space (sRGBA).
    /// Must be called in a render target render function, for example in the callback function of [Screen::write].
    /// Will render the model transparent if the color contains an alpha value below 255, you only need to render the model after all solid models.
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
    /// Render the uv coordinates of the mesh in red (u) and green (v) for debug purposes.
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write].
    ///
    /// # Errors
    /// Will return an error if the mesh has no uv coordinates.
    ///
    #[deprecated = "Use 'render' instead"]
    pub fn render_uvs(&self, camera: &Camera) -> Result<()> {
        self.render(&UVMaterial {}, camera, None, &[], &[], &[])
    }

    ///
    /// Render the normals of the mesh for debug purposes.
    /// Must be called in a render target render function, for example in the callback function of [Screen::write].
    ///
    /// # Errors
    /// Will return an error if the mesh has no normals.
    ///
    #[deprecated = "Use 'render' instead"]
    pub fn render_normals(&self, camera: &Camera) -> Result<()> {
        self.render(&NormalMaterial {}, camera, None, &[], &[], &[])
    }

    ///
    /// Render the mesh with the given texture which is assumed to be in sRGB color space with or without an alpha channel.
    /// Must be called in a render target render function, for example in the callback function of [Screen::write].
    /// Will render the model transparent if the texture contain an alpha channel (ie. the format is [Format::RGBA]), you only need to render the model after all solid models.
    ///
    /// # Errors
    /// Will return an error if the mesh has no uv coordinates.
    ///
    #[deprecated = "Use 'render' instead"]
    pub fn render_with_texture(&self, texture: &Texture2D, camera: &Camera) -> Result<()> {
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
    ) -> Result<&MeshProgram> {
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
                    MeshProgram::new(&self.context, fragment_shader_source)?,
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

impl Geometry for Model {
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

impl Object for Model {
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
            &Mesh::vertex_shader_source(&fragment_shader_source),
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
        self.mesh.aabb()
    }
}

impl ShadedGeometry for Model {
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

impl Clone for Model {
    fn clone(&self) -> Self {
        unsafe {
            MESH_COUNT += 1;
        }
        Self {
            context: self.context.clone(),
            mesh: self.mesh.clone(),
            cull: self.cull,
        }
    }
}

impl Drop for Model {
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
