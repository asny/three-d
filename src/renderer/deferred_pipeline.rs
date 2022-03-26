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
/// Deferred rendering draws the geometry information into a buffer in the [DeferredPipeline::render_pass] and use that information in the [DeferredPipeline::lighting_pass].
/// This means that the lighting is only calculated once per pixel since the depth testing is happening in the render pass.
/// For now only supports a cook-torrance [LightingModel].
/// **Note:** Deferred rendering does not support blending and therefore does not support transparency!
///
pub struct DeferredPipeline {
    context: Context,
    ///
    /// Set this to visualize the positions, normals etc. for debug purposes.
    ///
    pub debug_type: DebugType,
    camera: Camera,
    geometry_pass_texture: Option<Texture2DArray>,
    geometry_pass_depth_texture: Option<DepthTargetTexture2D>,
}

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
            geometry_pass_texture: Some(Texture2DArray::new_empty::<Vector4<u8>>(
                context,
                1,
                1,
                2,
                Interpolation::Nearest,
                Interpolation::Nearest,
                None,
                Wrapping::ClampToEdge,
                Wrapping::ClampToEdge,
            )?),
            geometry_pass_depth_texture: Some(DepthTargetTexture2D::new(
                context,
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
    pub fn render_pass(
        &mut self,
        camera: &Camera,
        objects: &[(impl Geometry, &DeferredPhysicalMaterial)],
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
        self.geometry_pass_texture = Some(Texture2DArray::new_empty::<Vector4<u8>>(
            &self.context,
            viewport.width,
            viewport.height,
            2,
            Interpolation::Nearest,
            Interpolation::Nearest,
            None,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
        )?);
        self.geometry_pass_depth_texture = Some(DepthTargetTexture2D::new(
            &self.context,
            viewport.width,
            viewport.height,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
            DepthFormat::Depth32F,
        )?);
        RenderTarget::new(
            &self.context,
            ColorTarget::Texture2DArray {
                texture: self.geometry_pass_texture.as_mut().unwrap(),
                layers: &[0, 1],
                mip_level: None,
            },
            DepthTarget::Texture2D {
                texture: self.geometry_pass_depth_texture.as_mut().unwrap(),
            },
        )?
        .clear(ClearState::default())?
        .write(|| {
            for (geometry, material) in objects
                .iter()
                .filter(|(g, _)| self.camera.in_frustum(&g.aabb()))
            {
                geometry.render_with_material(material, &self.camera, &[])?;
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
    pub fn lighting_pass(&mut self, camera: &Camera, lights: &[&dyn Light]) -> ThreeDResult<()> {
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
                    debug_effect.use_texture("depthMap", self.geometry_pass_depth_texture())?;
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

        let mut fragment_shader = lights_fragment_shader_source(
            lights,
            LightingModel::Cook(
                NormalDistributionFunction::TrowbridgeReitzGGX,
                GeometryFunction::SmithSchlickGGX,
            ),
        );
        fragment_shader.push_str(include_str!("material/shaders/deferred_lighting.frag"));

        self.context.effect(&fragment_shader, |effect| {
            effect.use_uniform("eyePosition", camera.position())?;
            for (i, light) in lights.iter().enumerate() {
                light.use_uniforms(effect, i as u32)?;
            }
            effect.use_texture_array("gbuffer", self.geometry_pass_texture())?;
            effect.use_texture("depthMap", self.geometry_pass_depth_texture())?;
            effect.use_uniform(
                "viewProjectionInverse",
                (camera.projection() * camera.view()).invert().unwrap(),
            )?;
            effect.apply(render_states, camera.viewport())?;
            Ok(())
        })
    }

    /// Returns the geometry pass texture
    pub fn geometry_pass_texture(&self) -> &Texture2DArray {
        self.geometry_pass_texture.as_ref().unwrap()
    }

    /// Returns the geometry pass depth texture
    pub fn geometry_pass_depth_texture(&self) -> &DepthTargetTexture2D {
        self.geometry_pass_depth_texture.as_ref().unwrap()
    }
}
