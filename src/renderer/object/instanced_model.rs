use crate::core::*;
use crate::renderer::*;

///
/// Similar to [Model], except it is possible to render many instances of the same model efficiently.
///
#[allow(deprecated)]
pub struct InstancedModel<M: ForwardMaterial> {
    context: Context,
    mesh: InstancedMesh,
    #[deprecated = "set in render states on material instead"]
    pub cull: Cull,
    aabb_local: AxisAlignedBoundingBox,
    aabb: AxisAlignedBoundingBox,
    transformation: Mat4,
    transformations: Vec<Mat4>,
    /// The material applied to the instanced model
    pub material: M,
}

impl InstancedModel<ColorMaterial> {
    ///
    /// Creates a new instanced 3D model with a triangle mesh as geometry and a default [ColorMaterial].
    /// The transformations are applied to each model instance before they are rendered.
    /// The model is rendered in as many instances as there are transformation matrices.
    ///
    pub fn new(
        context: &Context,
        transformations: &[Mat4],
        cpu_mesh: &CPUMesh,
    ) -> ThreeDResult<Self> {
        Self::new_with_material(context, transformations, cpu_mesh, ColorMaterial::default())
    }
}

#[allow(deprecated)]
impl<M: ForwardMaterial> InstancedModel<M> {
    pub fn new_with_material(
        context: &Context,
        transformations: &[Mat4],
        cpu_mesh: &CPUMesh,
        material: M,
    ) -> ThreeDResult<Self> {
        let mesh = InstancedMesh::new(context, transformations, cpu_mesh)?;
        let aabb = cpu_mesh.compute_aabb();
        let mut model = Self {
            context: context.clone(),
            mesh,
            cull: Cull::default(),
            aabb,
            aabb_local: aabb.clone(),
            transformation: Mat4::identity(),
            transformations: transformations.to_vec(),
            material,
        };
        model.update_aabb();
        Ok(model)
    }

    ///
    /// Updates the transformations applied to each model instance before they are rendered.
    /// The model is rendered in as many instances as there are transformation matrices.
    ///
    pub fn update_transformations(&mut self, transformations: &[Mat4]) {
        self.transformations = transformations.to_vec();
        self.mesh.update_transformations(transformations);
        self.update_aabb();
    }

    pub fn transformations(&self) -> &[Mat4] {
        &self.transformations
    }

    fn update_aabb(&mut self) {
        let mut aabb = AxisAlignedBoundingBox::EMPTY;
        for transform in self.transformations.iter() {
            let mut aabb2 = self.aabb_local.clone();
            aabb2.transform(&(transform * self.transformation));
            aabb.expand_with_aabb(&aabb2);
        }
        self.aabb = aabb;
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
    pub fn render_color(&self, camera: &Camera) -> ThreeDResult<()> {
        let mut mat = ColorMaterial::default();
        mat.opaque_render_states.cull = self.cull;
        mat.transparent_render_states.cull = self.cull;
        self.render_forward(&mat, camera, &Lights::default())
    }

    ///
    /// Render the instanced model with the given color. The color is assumed to be in gamma color space (sRGBA).
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    /// The transformation can be used to position, orientate and scale the instanced model.
    ///
    #[deprecated = "Use 'render_forward' instead"]
    pub fn render_with_color(&self, color: Color, camera: &Camera) -> ThreeDResult<()> {
        let mut mat = ColorMaterial {
            color,
            ..Default::default()
        };
        mat.opaque_render_states.cull = self.cull;
        mat.transparent_render_states.cull = self.cull;
        self.render_forward(&mat, camera, &Lights::default())
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
    pub fn render_with_texture(&self, texture: &impl Texture, camera: &Camera) -> ThreeDResult<()> {
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
                self.mesh.draw(
                    render_states,
                    program,
                    camera.uniform_buffer(),
                    camera.viewport(),
                    Some(self.transformation),
                )
            },
        )
    }

    ///
    /// Render the depth (scaled such that a value of 1 corresponds to max_depth) into the red channel of the current color render target which for example is used for picking.
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    ///
    #[deprecated = "Use 'render_forward' instead"]
    pub fn render_depth_to_red(&self, camera: &Camera, max_depth: f32) -> ThreeDResult<()> {
        let mut mat = DepthMaterial {
            max_distance: Some(max_depth),
            ..Default::default()
        };
        mat.render_states.write_mask = WriteMask {
            red: true,
            ..WriteMask::DEPTH
        };
        mat.render_states.cull = self.cull;
        self.render_forward(&mat, camera, &Lights::default())
    }

    ///
    /// Render only the depth into the current depth render target which is useful for shadow maps or depth pre-pass.
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    ///
    #[deprecated = "Use 'render_forward' instead"]
    pub fn render_depth(&self, camera: &Camera) -> ThreeDResult<()> {
        let mut mat = DepthMaterial {
            render_states: RenderStates {
                write_mask: WriteMask::DEPTH,
                ..Default::default()
            },
            ..Default::default()
        };
        mat.render_states.cull = self.cull;
        self.render_forward(&mat, camera, &Lights::default())
    }
}

#[allow(deprecated)]
impl<M: ForwardMaterial> ShadedGeometry for InstancedModel<M> {
    fn render_with_lighting(
        &self,
        camera: &Camera,
        material: &PhysicalMaterial,
        lighting_model: LightingModel,
        ambient_light: Option<&AmbientLight>,
        directional_lights: &[&DirectionalLight],
        spot_lights: &[&SpotLight],
        point_lights: &[&PointLight],
    ) -> ThreeDResult<()> {
        let mut mat = material.clone();
        mat.opaque_render_states.cull = self.cull;
        mat.transparent_render_states.cull = self.cull;

        let mut lights: Vec<&dyn Light> = Vec::new();
        if let Some(light) = ambient_light {
            lights.push(light)
        }
        for light in directional_lights {
            lights.push(light);
        }
        for light in spot_lights {
            lights.push(light);
        }
        for light in point_lights {
            lights.push(light);
        }
        let mut fragment_shader_source =
            lights_fragment_shader_source(&mut lights.clone().into_iter(), lighting_model);
        fragment_shader_source
            .push_str(&mat.fragment_shader_source_internal(self.mesh.color_buffer.is_some()));
        self.context.program(
            &InstancedMesh::vertex_shader_source(&fragment_shader_source),
            &fragment_shader_source,
            |program| {
                for (i, light) in lights.iter().enumerate() {
                    light.use_uniforms(program, camera, i as u32)?;
                }
                mat.use_uniforms_internal(program)?;
                self.mesh.draw(
                    mat.render_states(),
                    program,
                    camera.uniform_buffer(),
                    camera.viewport(),
                    Some(self.transformation),
                )
            },
        )
    }

    fn geometry_pass(
        &self,
        camera: &Camera,
        viewport: Viewport,
        material: &PhysicalMaterial,
    ) -> ThreeDResult<()> {
        self.render_deferred(material, camera, viewport)
    }
}

impl<M: ForwardMaterial> Geometry for InstancedModel<M> {
    fn aabb(&self) -> AxisAlignedBoundingBox {
        self.aabb
    }

    fn transformation(&self) -> Mat4 {
        self.transformation
    }
}

impl<M: ForwardMaterial> GeometryMut for InstancedModel<M> {
    fn set_transformation(&mut self, transformation: Mat4) {
        self.transformation = transformation;
        self.update_aabb();
    }
}

#[allow(deprecated)]
impl<M: ForwardMaterial> Shadable for InstancedModel<M> {
    fn render_forward(
        &self,
        material: &dyn ForwardMaterial,
        camera: &Camera,
        lights: &Lights,
    ) -> ThreeDResult<()> {
        let fragment_shader_source =
            material.fragment_shader_source(self.mesh.color_buffer.is_some(), lights);
        self.context.program(
            &InstancedMesh::vertex_shader_source(&fragment_shader_source),
            &fragment_shader_source,
            |program| {
                material.use_uniforms(program, camera, lights)?;
                self.mesh.draw(
                    material.render_states(),
                    program,
                    camera.uniform_buffer(),
                    camera.viewport(),
                    Some(self.transformation),
                )
            },
        )
    }

    fn render_deferred(
        &self,
        material: &dyn DeferredMaterial,
        camera: &Camera,
        viewport: Viewport,
    ) -> ThreeDResult<()> {
        let fragment_shader_source =
            material.fragment_shader_source_deferred(self.mesh.mesh.color_buffer.is_some());
        self.context.program(
            &InstancedMesh::vertex_shader_source(&fragment_shader_source),
            &fragment_shader_source,
            |program| {
                material.use_uniforms(program, camera, &Lights::default())?;
                self.mesh.draw(
                    material.render_states(),
                    program,
                    camera.uniform_buffer(),
                    viewport,
                    Some(self.transformation),
                )
            },
        )
    }
}

impl<M: ForwardMaterial> Object for InstancedModel<M> {
    fn render(&self, camera: &Camera, lights: &Lights) -> ThreeDResult<()> {
        self.render_forward(&self.material, camera, lights)
    }

    fn is_transparent(&self) -> bool {
        self.material.is_transparent()
    }
}
