use crate::core::*;
use crate::renderer::*;

///
/// A triangle mesh which can be rendered with one of the standard render functions. See [Mesh] if you need a custom render function.
///
#[derive(Clone)]
pub struct Model {
    context: Context,
    pub(in crate::renderer) mesh: Mesh,
    pub cull: Cull,
}

impl Model {
    pub fn new(context: &Context, cpu_mesh: &CPUMesh) -> Result<Self> {
        Ok(Self {
            context: context.clone(),
            mesh: Mesh::new(context, cpu_mesh)?,
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
    #[deprecated = "Use 'render_forward' instead"]
    pub fn render_color(&self, camera: &Camera) -> Result<()> {
        self.render_forward(
            &ColorMaterial {
                vertex_colors: true,
                ..Default::default()
            },
            camera,
            &Lights::NONE,
        )
    }

    ///
    /// Render the mesh with the given color. The color is assumed to be in gamma color space (sRGBA).
    /// Must be called in a render target render function, for example in the callback function of [Screen::write].
    /// Will render the model transparent if the color contains an alpha value below 255, you only need to render the model after all solid models.
    ///
    #[deprecated = "Use 'render_forward' instead"]
    pub fn render_with_color(&self, color: Color, camera: &Camera) -> Result<()> {
        self.render_forward(
            &ColorMaterial {
                color,
                ..Default::default()
            },
            camera,
            &Lights::NONE,
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
    #[deprecated = "Use 'render_forward' instead"]
    pub fn render_uvs(&self, camera: &Camera) -> Result<()> {
        self.render_forward(&UVMaterial {}, camera, &Lights::NONE)
    }

    ///
    /// Render the normals of the mesh for debug purposes.
    /// Must be called in a render target render function, for example in the callback function of [Screen::write].
    ///
    /// # Errors
    /// Will return an error if the mesh has no normals.
    ///
    #[deprecated = "Use 'render_forward' instead"]
    pub fn render_normals(&self, camera: &Camera) -> Result<()> {
        self.render_forward(&NormalMaterial::default(), camera, &Lights::NONE)
    }

    ///
    /// Render the mesh with the given texture which is assumed to be in sRGB color space with or without an alpha channel.
    /// Must be called in a render target render function, for example in the callback function of [Screen::write].
    /// Will render the model transparent if the texture contain an alpha channel (ie. the format is [Format::RGBA]), you only need to render the model after all solid models.
    ///
    /// # Errors
    /// Will return an error if the mesh has no uv coordinates.
    ///
    #[deprecated = "Use 'render_forward' instead"]
    pub fn render_with_texture(&self, texture: &Texture2D, camera: &Camera) -> Result<()> {
        let render_states = if texture.is_transparent() {
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
        };
        let fragment_shader_source = include_str!("shaders/mesh_texture.frag");
        self.context.program(
            &Mesh::vertex_shader_source(fragment_shader_source),
            fragment_shader_source,
            |program| {
                program.use_texture("tex", texture)?;
                self.mesh.render(
                    render_states,
                    program,
                    camera.uniform_buffer(),
                    camera.viewport(),
                )
            },
        )
    }
}

impl Object for Model {
    fn render_forward(
        &self,
        material: &dyn ForwardMaterial,
        camera: &Camera,
        lights: &Lights,
    ) -> Result<()> {
        let mut render_states = material.render_states();
        render_states.cull = self.cull;
        let fragment_shader_source = material.fragment_shader_source(lights);
        self.context.program(
            &Mesh::vertex_shader_source(&fragment_shader_source),
            &fragment_shader_source,
            |program| {
                material.bind(program, camera, lights)?;
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
}

impl Geometry for Model {
    fn render_depth_to_red(&self, camera: &Camera, max_depth: f32) -> Result<()> {
        self.render_forward(
            &PickMaterial {
                max_distance: Some(max_depth),
                ..Default::default()
            },
            camera,
            &Lights::NONE,
        )
    }

    fn render_depth(&self, camera: &Camera) -> Result<()> {
        self.render_forward(&DepthMaterial {}, camera, &Lights::NONE)
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
        material: &PhysicalMaterial,
    ) -> Result<()> {
        self.render_deferred(material, camera, viewport)
    }

    fn render_with_lighting(
        &self,
        camera: &Camera,
        material: &PhysicalMaterial,
        lighting_model: LightingModel,
        ambient_light: Option<&AmbientLight>,
        directional_lights: &[&DirectionalLight],
        spot_lights: &[&SpotLight],
        point_lights: &[&PointLight],
    ) -> Result<()> {
        let mut mat = material.clone();
        mat.lighting_model = lighting_model;
        self.render_forward(
            &mat,
            camera,
            &Lights {
                ambient_light: ambient_light.map(|l| l.clone()),
                directional_lights: directional_lights.iter().map(|l| (*l).clone()).collect(),
                spot_lights: spot_lights.iter().map(|l| (*l).clone()).collect(),
                point_lights: point_lights.iter().map(|l| (*l).clone()).collect(),
            },
        )
    }
}
