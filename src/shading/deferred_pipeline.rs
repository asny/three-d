use crate::camera::*;
use crate::core::*;
use crate::definition::*;
use crate::effect::*;
use crate::light::*;
use crate::math::*;
use crate::shading::*;
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
    DIFFUSE,
    SPECULAR,
    POWER,
    NONE,
}

///
/// Deferred pipeline based on the Phong reflection model supporting a performance-limited
/// amount of directional, point and spot lights with shadows.
///
pub struct DeferredPipeline {
    context: Context,
    program_map: HashMap<String, ImageEffect>,
    debug_effect: Option<ImageEffect>,
    ///
    /// Set this to visualize the positions, normals etc. for debug purposes.
    ///
    pub debug_type: DebugType,
    geometry_pass_texture: Option<ColorTargetTexture2DArray<u8>>,
    geometry_pass_depth_texture: Option<DepthTargetTexture2DArray>,
}

impl DeferredPipeline {
    ///
    /// Constructor.
    ///
    pub fn new(context: &Context) -> Result<Self, Error> {
        let renderer = Self {
            context: context.clone(),
            program_map: HashMap::new(),
            debug_effect: None,
            debug_type: DebugType::NONE,
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
    /// Render the geometry and surface material parameters of the given [Phong geometries](crate::PhongGeometry).
    /// This function must not be called in a render target render function and needs to be followed
    /// by a call to [light_pass](Self::light_pass) which must be inside a render target render function.
    ///
    pub fn geometry_pass(
        &mut self,
        width: u32,
        height: u32,
        camera: &Camera,
        geometries: &[&dyn ShadedGeometry],
    ) -> Result<(), Error> {
        self.geometry_pass_texture = Some(ColorTargetTexture2DArray::<u8>::new(
            &self.context,
            width,
            height,
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
            width,
            height,
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
            for geometry in geometries {
                if geometry
                    .aabb()
                    .map(|aabb| camera.in_frustum(&aabb))
                    .unwrap_or(true)
                {
                    geometry.geometry_pass(
                        RenderStates::default(),
                        Viewport::new_at_origo(width, height),
                        camera,
                    )?;
                }
            }
            Ok(())
        })?;
        Ok(())
    }

    ///
    /// Uses the geometry and surface material parameters written in the last [geometry_pass](Self::geometry_pass) call
    /// and all of the given lights to shade the [Phong geometries](crate::PhongGeometry).
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    ///
    pub fn light_pass(
        &mut self,
        viewport: Viewport,
        camera: &Camera,
        ambient_light: Option<&AmbientLight>,
        directional_lights: &[&DirectionalLight],
        spot_lights: &[&SpotLight],
        point_lights: &[&PointLight],
    ) -> Result<(), Error> {
        let render_states = RenderStates {
            depth_test: DepthTestType::LessOrEqual,
            ..Default::default()
        };

        if self.debug_type != DebugType::NONE {
            if self.debug_effect.is_none() {
                self.debug_effect = Some(
                    ImageEffect::new(&self.context, include_str!("shaders/debug.frag")).unwrap(),
                );
            }
            self.debug_effect.as_ref().unwrap().use_uniform_mat4(
                "viewProjectionInverse",
                &(camera.projection() * camera.view()).invert().unwrap(),
            )?;
            self.debug_effect
                .as_ref()
                .unwrap()
                .use_texture_array(self.geometry_pass_texture(), "gbuffer")?;
            self.debug_effect
                .as_ref()
                .unwrap()
                .use_texture_array(self.geometry_pass_depth_texture_array(), "depthMap")?;
            self.debug_effect
                .as_ref()
                .unwrap()
                .use_uniform_int("type", &(self.debug_type as i32))?;
            self.debug_effect
                .as_ref()
                .unwrap()
                .apply(render_states, viewport)?;
            return Ok(());
        }

        let key = format!(
            "{},{},{},{}",
            ambient_light.is_some(),
            directional_lights.len(),
            spot_lights.len(),
            point_lights.len()
        );
        if !self.program_map.contains_key(&key) {
            self.program_map.insert(
                key.clone(),
                ImageEffect::new(
                    &self.context,
                    &shaded_fragment_shader(
                        "#define DEFERRED\nin vec2 uv;\n",
                        directional_lights.len(),
                        spot_lights.len(),
                        point_lights.len(),
                    ),
                )?,
            );
        };
        let effect = self.program_map.get(&key).unwrap();

        bind_lights(
            effect,
            ambient_light,
            directional_lights,
            spot_lights,
            point_lights,
        )?;

        effect.use_texture_array(self.geometry_pass_texture(), "gbuffer")?;
        effect.use_texture_array(self.geometry_pass_depth_texture_array(), "depthMap")?;
        if !directional_lights.is_empty() || !spot_lights.is_empty() || !point_lights.is_empty() {
            effect.use_uniform_vec3("eyePosition", &camera.position())?;
            effect.use_uniform_mat4(
                "viewProjectionInverse",
                &(camera.projection() * camera.view()).invert().unwrap(),
            )?;
        }
        effect.apply(render_states, viewport)?;
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
