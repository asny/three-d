use crate::core::*;
use crate::renderer::*;
use std::collections::HashMap;

///
/// Used for debug purposes.
///
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DebugType {
    POSITION,
    NORMAL,
    COLOR,
    DEPTH,
    ORM,
    NONE,
}
///
/// Deferred render pipeline which can render objects (implementing the [Object] trait) with materials (implementing the [DeferredMaterial] trait) and lighting.
/// Supports different types of lighting models by changing the [DeferredPipeline::lighting_model] field.
/// Deferred rendering draws the geometry information into a buffer in the [DeferredPipeline::geometry_pass] and use that information in the [DeferredPipeline::light_pass].
/// This means that the lighting is only calculated once per pixel since the depth testing is happening in the geometry pass.
/// **Note:** Deferred rendering does not support blending and therefore does not support transparency!
///
pub struct DeferredPipeline {
    context: Context,
    program_map: HashMap<String, ImageEffect>,
    debug_effect: Option<ImageEffect>,
    ///
    /// Set this to visualize the positions, normals etc. for debug purposes.
    ///
    pub debug_type: DebugType,
    pub lighting_model: LightingModel,
    geometry_pass_texture: Option<ColorTargetTexture2DArray<u8>>,
    geometry_pass_depth_texture: Option<DepthTargetTexture2DArray>,
}

impl DeferredPipeline {
    ///
    /// Constructor.
    ///
    pub fn new(context: &Context) -> Result<Self> {
        let renderer = Self {
            context: context.clone(),
            program_map: HashMap::new(),
            debug_effect: None,
            debug_type: DebugType::NONE,
            lighting_model: LightingModel::Blinn,
            geometry_pass_texture: Some(ColorTargetTexture2DArray::new(
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
    /// by a call to [DeferredPipeline::light_pass].
    ///
    pub fn geometry_pass(
        &mut self,
        camera: &Camera,
        objects: &[(&dyn Object, &dyn DeferredMaterial)],
    ) -> Result<()> {
        let viewport = Viewport::new_at_origo(camera.viewport().width, camera.viewport().height);
        self.geometry_pass_texture = Some(ColorTargetTexture2DArray::<u8>::new(
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
            for (object, material) in objects {
                if in_frustum(&camera, object) {
                    object.render_deferred(*material, camera, viewport)?;
                }
            }
            Ok(())
        })?;
        Ok(())
    }

    ///
    /// Uses the geometry and surface material parameters written in the last [DeferredPipeline::geometry_pass] call
    /// and all of the given lights to render the [Object].
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write].
    ///
    pub fn light_pass(
        &mut self,
        camera: &Camera,
        ambient_light: Option<&AmbientLight>,
        directional_lights: &[&DirectionalLight],
        spot_lights: &[&SpotLight],
        point_lights: &[&PointLight],
    ) -> Result<()> {
        let render_states = RenderStates {
            depth_test: DepthTest::LessOrEqual,
            ..Default::default()
        };

        if self.debug_type != DebugType::NONE {
            if self.debug_effect.is_none() {
                self.debug_effect = Some(
                    ImageEffect::new(&self.context, include_str!("material/shaders/debug.frag"))
                        .unwrap(),
                );
            }
            self.debug_effect.as_ref().unwrap().use_uniform_mat4(
                "viewProjectionInverse",
                &(camera.projection() * camera.view()).invert().unwrap(),
            )?;
            self.debug_effect
                .as_ref()
                .unwrap()
                .use_texture_array("gbuffer", self.geometry_pass_texture())?;
            self.debug_effect
                .as_ref()
                .unwrap()
                .use_texture_array("depthMap", self.geometry_pass_depth_texture_array())?;
            self.debug_effect
                .as_ref()
                .unwrap()
                .use_uniform_int("type", &(self.debug_type as i32))?;
            self.debug_effect
                .as_ref()
                .unwrap()
                .apply(render_states, camera.viewport())?;
            return Ok(());
        }

        let mut fragment_shader = shaded_fragment_shader(self.lighting_model);
        fragment_shader.push_str(include_str!("material/shaders/deferred_lighting.frag"));

        if !self.program_map.contains_key(&fragment_shader) {
            self.program_map.insert(
                fragment_shader.clone(),
                ImageEffect::new(&self.context, &fragment_shader)?,
            );
        };
        let effect = self.program_map.get(&fragment_shader).unwrap();

        let lights = Lights {
            ambient: ambient_light.map(|l| l.clone()),
            directional: directional_lights.iter().map(|l| (*l).clone()).collect(),
            spot: spot_lights.iter().map(|l| (*l).clone()).collect(),
            point: point_lights.iter().map(|l| (*l).clone()).collect(),
        };
        bind_lights(&self.context, effect, &lights, camera.position())?;

        effect.use_texture_array("gbuffer", self.geometry_pass_texture())?;
        effect.use_texture_array("depthMap", self.geometry_pass_depth_texture_array())?;
        if !directional_lights.is_empty() || !spot_lights.is_empty() || !point_lights.is_empty() {
            effect.use_uniform_mat4(
                "viewProjectionInverse",
                &(camera.projection() * camera.view()).invert().unwrap(),
            )?;
        }
        effect.apply(render_states, camera.viewport())?;
        Ok(())
    }

    pub fn geometry_pass_texture(&self) -> &ColorTargetTexture2DArray<u8> {
        self.geometry_pass_texture.as_ref().unwrap()
    }
    pub fn geometry_pass_depth_texture_array(&self) -> &DepthTargetTexture2DArray {
        self.geometry_pass_depth_texture.as_ref().unwrap()
    }

    pub fn geometry_pass_depth_texture(&self) -> DepthTargetTexture2D {
        let depth_array = self.geometry_pass_depth_texture.as_ref().unwrap();
        let depth_texture = DepthTargetTexture2D::new(
            &self.context,
            depth_array.width(),
            depth_array.height(),
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
            DepthFormat::Depth32F,
        )
        .unwrap();

        depth_array
            .copy_to(
                0,
                CopyDestination::<u8>::DepthTexture(&depth_texture),
                Viewport::new_at_origo(depth_array.width(), depth_array.height()),
            )
            .unwrap();
        depth_texture
    }
}
