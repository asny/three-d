use crate::core::*;
use crate::renderer::*;

///
/// Used for debug purposes.
///
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum DebugType {
    POSITION,
    NORMAL,
    COLOR,
    DEPTH,
    ORM,
    UV,
    NONE,
}
///
/// Deferred render pipeline which can render objects (implementing the [Geometry] trait) with a [DeferredPhysicalMaterial] and lighting.
/// Supports different types of lighting models by changing the [DeferredPipeline::lighting_model] field.
/// Deferred rendering draws the geometry information into a buffer in the [DeferredPipeline::render_pass] and use that information in the [DeferredPipeline::lighting_pass].
/// This means that the lighting is only calculated once per pixel since the depth testing is happening in the render pass.
/// **Note:** Deferred rendering does not support blending and therefore does not support transparency!
///
pub struct DeferredPipeline {
    context: Context,
    ///
    /// Set this to visualize the positions, normals etc. for debug purposes.
    ///
    pub debug_type: DebugType,
    #[deprecated = "use lighting_pass where Light struct contain lighting model"]
    pub lighting_model: LightingModel,
    camera: Camera,
    geometry_pass_texture: Option<Texture2DArray<u8>>,
    geometry_pass_depth_texture: Option<DepthTargetTexture2DArray>,
}

#[allow(deprecated)]
impl DeferredPipeline {
    ///
    /// Constructor.
    ///
    pub fn new(context: &Context) -> ThreeDResult<Self> {
        let renderer = Self {
            context: context.clone(),
            camera: Camera::new_perspective(
                context,
                Viewport::new_at_origo(1, 1),
                vec3(0.0, 0.0, 1.0),
                vec3(0.0, 0.0, 0.0),
                vec3(0.0, 1.0, 0.0),
                degrees(75.0),
                0.01,
                10.0,
            )?,
            debug_type: DebugType::NONE,
            lighting_model: LightingModel::Blinn,
            geometry_pass_texture: Some(Texture2DArray::new(
                context,
                1,
                1,
                2,
                Interpolation::Nearest,
                Interpolation::Nearest,
                None,
                Wrapping::ClampToEdge,
                Wrapping::ClampToEdge,
                Format::RGBA,
            )?),
            geometry_pass_depth_texture: Some(DepthTargetTexture2DArray::new(
                context,
                1,
                1,
                1,
                Wrapping::ClampToEdge,
                Wrapping::ClampToEdge,
                DepthFormat::Depth32F,
            )?),
        };
        Ok(renderer)
    }
    ///
    /// Render the given geometry and material parameters to a buffer.
    /// This function must not be called in a render target render function and needs to be followed
    /// by a call to [DeferredPipeline::lighting_pass].
    ///
    #[deprecated = "use render_pass instead"]
    pub fn geometry_pass<G: Geometry>(
        &mut self,
        camera: &Camera,
        objects: &[(G, impl std::borrow::Borrow<PhysicalMaterial>)],
    ) -> ThreeDResult<()> {
        self.render_pass(
            camera,
            &objects
                .iter()
                .map(|(g, m)| {
                    (
                        g,
                        DeferredPhysicalMaterial::from_physical_material(m.borrow()),
                    )
                })
                .collect::<Vec<_>>(),
        )
    }

    ///
    /// Render the given geometry and material parameters to a buffer.
    /// This function must not be called in a render target render function and needs to be followed
    /// by a call to [DeferredPipeline::lighting_pass].
    ///
    pub fn render_pass<G: Geometry>(
        &mut self,
        camera: &Camera,
        objects: &[(G, impl std::borrow::Borrow<DeferredPhysicalMaterial>)],
    ) -> ThreeDResult<()> {
        let viewport = Viewport::new_at_origo(camera.viewport().width, camera.viewport().height);
        match camera.projection_type() {
            ProjectionType::Perspective { field_of_view_y } => {
                self.camera.set_perspective_projection(
                    *field_of_view_y,
                    camera.z_near(),
                    camera.z_far(),
                )?;
            }
            ProjectionType::Orthographic { height, .. } => {
                self.camera.set_orthographic_projection(
                    *height,
                    camera.z_near(),
                    camera.z_far(),
                )?;
            }
        };
        self.camera.set_viewport(viewport)?;
        self.camera
            .set_view(*camera.position(), *camera.target(), *camera.up())?;
        self.geometry_pass_texture = Some(Texture2DArray::<u8>::new(
            &self.context,
            viewport.width,
            viewport.height,
            2,
            Interpolation::Nearest,
            Interpolation::Nearest,
            None,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
            Format::RGBA,
        )?);
        self.geometry_pass_depth_texture = Some(DepthTargetTexture2DArray::new(
            &self.context,
            viewport.width,
            viewport.height,
            1,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
            DepthFormat::Depth32F,
        )?);
        RenderTargetArray::new(
            &self.context,
            self.geometry_pass_texture.as_ref().unwrap(),
            self.geometry_pass_depth_texture.as_ref().unwrap(),
        )?
        .write(&[0, 1], 0, ClearState::default(), || {
            for (geometry, material) in objects
                .iter()
                .filter(|(g, _)| self.camera.in_frustum(&g.aabb()))
            {
                geometry.render_with_material(
                    material.borrow(),
                    &self.camera,
                    &Lights::default(),
                )?;
            }
            Ok(())
        })?;
        Ok(())
    }

    ///
    /// Uses the geometry and surface material parameters written in the last [DeferredPipeline::render_pass] call
    /// and all of the given lights to render the objects.
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write].
    ///
    #[deprecated = "use lighting_pass instead"]
    pub fn light_pass(
        &mut self,
        camera: &Camera,
        ambient_light: Option<&AmbientLight>,
        directional_lights: &[&DirectionalLight],
        spot_lights: &[&SpotLight],
        point_lights: &[&PointLight],
    ) -> ThreeDResult<()> {
        let render_states = RenderStates {
            depth_test: DepthTest::LessOrEqual,
            ..Default::default()
        };

        if self.debug_type != DebugType::NONE {
            return self.context.effect(
                &format!(
                    "{}{}",
                    include_str!("../core/shared.frag"),
                    include_str!("material/shaders/debug.frag")
                ),
                |debug_effect| {
                    debug_effect.use_uniform(
                        "viewProjectionInverse",
                        (camera.projection() * camera.view()).invert().unwrap(),
                    )?;
                    debug_effect.use_texture_array("gbuffer", self.geometry_pass_texture())?;
                    debug_effect
                        .use_texture_array("depthMap", self.geometry_pass_depth_texture_array())?;
                    if self.debug_type == DebugType::DEPTH {
                        debug_effect.use_uniform("zNear", camera.z_near())?;
                        debug_effect.use_uniform("zFar", camera.z_far())?;
                        debug_effect.use_uniform("cameraPosition", camera.position())?;
                    }
                    debug_effect.use_uniform("type", &(self.debug_type as i32))?;
                    debug_effect.apply(render_states, camera.viewport())?;
                    Ok(())
                },
            );
        }
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

        let mut fragment_shader =
            lights_fragment_shader_source(&mut lights.clone().into_iter(), self.lighting_model);
        fragment_shader.push_str(include_str!("material/shaders/deferred_lighting.frag"));

        self.context.effect(&fragment_shader, |effect| {
            effect.use_uniform("eyePosition", camera.position())?;
            for (i, light) in lights.iter().enumerate() {
                light.use_uniforms(effect, i as u32)?;
            }
            effect.use_texture_array("gbuffer", self.geometry_pass_texture())?;
            effect.use_texture_array("depthMap", self.geometry_pass_depth_texture_array())?;
            if !directional_lights.is_empty() || !spot_lights.is_empty() || !point_lights.is_empty()
            {
                effect.use_uniform(
                    "viewProjectionInverse",
                    (camera.projection() * camera.view()).invert().unwrap(),
                )?;
            }
            effect.apply(render_states, camera.viewport())?;
            Ok(())
        })
    }

    ///
    /// Uses the geometry and surface material parameters written in the last [DeferredPipeline::render_pass] call
    /// and all of the given lights to render the objects.
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write].
    ///
    pub fn lighting_pass(&mut self, camera: &Camera, lights: &Lights) -> ThreeDResult<()> {
        let render_states = RenderStates {
            depth_test: DepthTest::LessOrEqual,
            ..Default::default()
        };

        if self.debug_type != DebugType::NONE {
            return self.context.effect(
                &format!(
                    "{}{}",
                    include_str!("../core/shared.frag"),
                    include_str!("material/shaders/debug.frag")
                ),
                |debug_effect| {
                    debug_effect.use_uniform(
                        "viewProjectionInverse",
                        (camera.projection() * camera.view()).invert().unwrap(),
                    )?;
                    debug_effect.use_texture_array("gbuffer", self.geometry_pass_texture())?;
                    debug_effect
                        .use_texture_array("depthMap", self.geometry_pass_depth_texture_array())?;
                    if self.debug_type == DebugType::DEPTH {
                        debug_effect.use_uniform("zNear", camera.z_near())?;
                        debug_effect.use_uniform("zFar", camera.z_far())?;
                        debug_effect.use_uniform("cameraPosition", camera.position())?;
                    }
                    debug_effect.use_uniform("type", self.debug_type as i32)?;
                    debug_effect.apply(render_states, camera.viewport())?;
                    Ok(())
                },
            );
        }

        let mut fragment_shader = lights.fragment_shader_source();
        fragment_shader.push_str(include_str!("material/shaders/deferred_lighting.frag"));

        self.context.effect(&fragment_shader, |effect| {
            lights.use_uniforms(effect, camera)?;
            effect.use_texture_array("gbuffer", self.geometry_pass_texture())?;
            effect.use_texture_array("depthMap", self.geometry_pass_depth_texture_array())?;
            if !lights.directional.is_empty() || !lights.spot.is_empty() || !lights.point.is_empty()
            {
                effect.use_uniform(
                    "viewProjectionInverse",
                    (camera.projection() * camera.view()).invert().unwrap(),
                )?;
            }
            effect.apply(render_states, camera.viewport())?;
            Ok(())
        })
    }

    pub fn geometry_pass_texture(&self) -> &Texture2DArray<u8> {
        self.geometry_pass_texture.as_ref().unwrap()
    }
    pub fn geometry_pass_depth_texture_array(&self) -> &DepthTargetTexture2DArray {
        self.geometry_pass_depth_texture.as_ref().unwrap()
    }

    pub fn geometry_pass_depth_texture(&self) -> DepthTargetTexture2D {
        let depth_array: &DepthTargetTexture2DArray =
            self.geometry_pass_depth_texture.as_ref().unwrap();
        let mut depth_texture = DepthTargetTexture2D::new(
            &self.context,
            depth_array.width(),
            depth_array.height(),
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
            DepthFormat::Depth32F,
        )
        .unwrap();

        RenderTarget::new_depth(&self.context, &mut depth_texture)
            .unwrap()
            .copy_from_array::<u8>(
                None,
                Some((&depth_array, 0)),
                Viewport::new_at_origo(depth_array.width(), depth_array.height()),
                WriteMask::default(),
            )
            .unwrap();
        depth_texture
    }
}
