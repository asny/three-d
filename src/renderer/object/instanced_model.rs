use crate::core::*;
use crate::renderer::*;

///
/// Similar to [Model], except it is possible to render many instances of the same model efficiently. See [InstancedMesh] if you need a custom render function.
///
pub struct InstancedModel {
    context: Context,
    mesh: InstancedMesh,
    #[deprecated = "set in render states on material instead"]
    pub cull: Cull,
}

impl InstancedModel {
    pub fn new(context: &Context, transformations: &[Mat4], cpu_mesh: &CPUMesh) -> Result<Self> {
        Ok(Self {
            context: context.clone(),
            mesh: InstancedMesh::new(context, transformations, cpu_mesh)?,
            cull: Cull::default(),
        })
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
    #[deprecated = "Use 'render_forward' instead"]
    pub fn render_color(&self, camera: &Camera) -> Result<()> {
        let mut mat = ColorMaterial::default();
        mat.render_states.cull = self.cull;
        mat.transparent_render_states.cull = self.cull;
        self.render_forward(&mat, camera, &[])
    }

    ///
    /// Render the instanced model with the given color. The color is assumed to be in gamma color space (sRGBA).
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    /// The transformation can be used to position, orientate and scale the instanced model.
    ///
    #[deprecated = "Use 'render_forward' instead"]
    pub fn render_with_color(&self, color: Color, camera: &Camera) -> Result<()> {
        let mut mat = ColorMaterial {
            color,
            ..Default::default()
        };
        mat.render_states.cull = self.cull;
        mat.transparent_render_states.cull = self.cull;
        self.render_forward(&mat, camera, &[])
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
    #[deprecated = "Use 'render_forward' instead"]
    pub fn render_with_texture(&self, texture: &impl Texture, camera: &Camera) -> Result<()> {
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

impl Geometry for InstancedModel {
    fn render_depth_to_red(&self, camera: &Camera, max_depth: f32) -> Result<()> {
        let mut mat = DepthMaterial {
            max_distance: Some(max_depth),
            ..Default::default()
        };
        mat.render_states.write_mask = WriteMask {
            red: true,
            ..WriteMask::DEPTH
        };
        mat.render_states.cull = self.cull;
        self.render_forward(&mat, camera, &[])
    }

    fn render_depth(&self, camera: &Camera) -> Result<()> {
        let mut mat = DepthMaterial {
            render_states: RenderStates {
                write_mask: WriteMask::DEPTH,
                ..Default::default()
            },
            ..Default::default()
        };
        mat.render_states.cull = self.cull;
        self.render_forward(&mat, camera, &[])
    }

    fn transformation(&self) -> &Mat4 {
        self.mesh.transformation()
    }

    fn set_transformation(&mut self, transformation: Mat4) {
        self.mesh.set_transformation(transformation);
    }

    fn aabb(&self) -> AxisAlignedBoundingBox {
        AxisAlignedBoundingBox::new_infinite() // TODO: Compute bounding box
    }
}

impl Object for InstancedModel {
    fn render_forward(
        &self,
        material: &dyn ForwardMaterial,
        camera: &Camera,
        lights: &[&dyn Light],
    ) -> Result<()> {
        let render_states = material.render_states(
            self.mesh
                .mesh
                .color_buffer
                .as_ref()
                .map(|(_, transparent)| *transparent)
                .unwrap_or(false),
        );
        let fragment_shader_source =
            material.fragment_shader_source(lights, self.mesh.mesh.color_buffer.is_some());
        self.context.program(
            &InstancedMesh::vertex_shader_source(&fragment_shader_source),
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
        let render_states = material.render_states();
        let fragment_shader_source =
            material.fragment_shader_source(self.mesh.mesh.color_buffer.is_some());
        self.context.program(
            &InstancedMesh::vertex_shader_source(&fragment_shader_source),
            &fragment_shader_source,
            |program| {
                material.bind(program)?;
                self.mesh
                    .render(render_states, program, camera.uniform_buffer(), viewport)
            },
        )
    }
}

impl ShadedGeometry for InstancedModel {
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
        mat.render_states.cull = self.cull;
        mat.transparent_render_states.cull = self.cull;
        let mut lights = Vec::new();
        if let Some(light) = ambient_light {
            lights.push(light as &dyn Light)
        }
        for light in directional_lights {
            lights.push(*light as &dyn Light);
        }
        for light in spot_lights {
            lights.push(*light as &dyn Light);
        }
        for light in point_lights {
            lights.push(*light as &dyn Light);
        }
        self.render_forward(&mat, camera, &lights)
    }
}
