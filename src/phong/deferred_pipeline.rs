
use crate::math::*;
use crate::definition::*;
use crate::core::*;
use crate::camera::*;
use crate::light::*;
use crate::effect::*;
use std::collections::HashMap;

///
/// Used for debug purposes.
///
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DebugType {POSITION, NORMAL, COLOR, DEPTH, DIFFUSE, SPECULAR, POWER, NONE}

///
/// Deferred pipeline based on the Phong reflection model supporting a performance-limited
/// amount of directional, point and spot lights with shadows. Supports colored, textured and instanced meshes.
///
pub struct PhongDeferredPipeline {
    context: Context,
    program_map: HashMap<String, ImageEffect>,
    debug_effect: Option<ImageEffect>,
    ///
    /// Set this to visualize the positions, normals etc. for debug purposes.
    ///
    pub debug_type: DebugType,
    geometry_pass_texture: Option<ColorTargetTexture2DArray>,
    geometry_pass_depth_texture: Option<DepthTargetTexture2DArray>
}

impl PhongDeferredPipeline
{
    ///
    /// Constructor.
    ///
    pub fn new(context: &Context) -> Result<Self, Error>
    {
        let renderer = Self {
            context: context.clone(),
            program_map: HashMap::new(),
            debug_effect: None,
            debug_type: DebugType::NONE,
            geometry_pass_texture: Some(ColorTargetTexture2DArray::new(context, 1, 1, 2,
                                                                       Interpolation::Nearest, Interpolation::Nearest, None, Wrapping::ClampToEdge,
                                                                       Wrapping::ClampToEdge, Format::RGBA8)?),
            geometry_pass_depth_texture: Some(DepthTargetTexture2DArray::new(context, 1, 1, 1, Wrapping::ClampToEdge,
                                                                             Wrapping::ClampToEdge, DepthFormat::Depth32F)?)
        };
        Ok(renderer)
    }

    ///
    /// Render the geometry and surface material parameters of Phong [meshes](crate::PhongMesh)
    /// or [instanced meshes](crate::PhongInstancedMesh) by calling the *render_geometry* on
    /// either type of mesh inside the **render** closure.
    /// This function must not be called in a render target render function, but needs to be followed
    /// by a call to [light_pass](Self::light_pass) which must be inside a render target render function.
    ///
    pub fn geometry_pass<F: FnOnce() -> Result<(), Error>>(&mut self, width: usize, height: usize, render: F) -> Result<(), Error>
    {
        self.geometry_pass_texture = Some(ColorTargetTexture2DArray::new(&self.context, width, height, 2,
                                                                         Interpolation::Nearest, Interpolation::Nearest, None, Wrapping::ClampToEdge,
                                                                         Wrapping::ClampToEdge, Format::RGBA8)?);
        self.geometry_pass_depth_texture = Some(DepthTargetTexture2DArray::new(&self.context, width, height, 1, Wrapping::ClampToEdge,
                                                                               Wrapping::ClampToEdge, DepthFormat::Depth32F)?);
        RenderTargetArray::new(&self.context, self.geometry_pass_texture.as_ref().unwrap(), self.geometry_pass_depth_texture.as_ref().unwrap())?
            .write(&ClearState::default(), &[0, 1], 0, render)?;
        Ok(())
    }

    ///
    /// Uses the geometry and surface material parameters written in the last [geometry_pass](Self::geometry_pass) call
    /// and all of the given lights
    /// to shade the Phong [meshes](crate::PhongMesh) or [instanced meshes](crate::PhongInstancedMesh).
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    ///
    pub fn light_pass(&mut self, viewport: Viewport, camera: &Camera, ambient_light: Option<&AmbientLight>, directional_lights: &[&DirectionalLight],
                      spot_lights: &[&SpotLight], point_lights: &[&PointLight]) -> Result<(), Error>
    {
        let render_states = RenderStates {cull: CullType::Back, depth_test: DepthTestType::LessOrEqual, ..Default::default()};

        if self.debug_type != DebugType::NONE {
            if self.debug_effect.is_none() {
                self.debug_effect = Some(ImageEffect::new(&self.context, include_str!("shaders/debug.frag")).unwrap());
            }
            self.debug_effect.as_ref().unwrap().program().use_uniform_mat4("viewProjectionInverse", &(camera.projection() * camera.view()).invert().unwrap())?;
            self.debug_effect.as_ref().unwrap().program().use_texture(self.geometry_pass_texture(), "gbuffer")?;
            self.debug_effect.as_ref().unwrap().program().use_texture(self.geometry_pass_depth_texture_array(), "depthMap")?;
            self.debug_effect.as_ref().unwrap().program().use_uniform_int("type", &(self.debug_type as i32))?;
            self.debug_effect.as_ref().unwrap().apply(render_states, viewport)?;
            return Ok(());
        }

        let key = format!("{},{},{},{}", ambient_light.is_some(), directional_lights.len(), spot_lights.len(), point_lights.len());
        if !self.program_map.contains_key(&key) {

            let mut dir_uniform = String::new();
            let mut dir_fun = String::new();
            for i in 0..directional_lights.len() {
                dir_uniform.push_str(&format!("
                uniform sampler2D directionalShadowMap{};
                layout (std140) uniform DirectionalLightUniform{}
                {{
                    DirectionalLight directionalLight{};
                }};", i, i, i));
                dir_fun.push_str(&format!("
                    color.rgb += calculate_directional_light(directionalLight{}, surface.color, surface.position, surface.normal,
                        surface.diffuse_intensity, surface.specular_intensity, surface.specular_power, directionalShadowMap{});", i, i));
            }
            let mut spot_uniform = String::new();
            let mut spot_fun = String::new();
            for i in 0..spot_lights.len() {
                spot_uniform.push_str(&format!("
                uniform sampler2D spotShadowMap{};
                layout (std140) uniform SpotLightUniform{}
                {{
                    SpotLight spotLight{};
                }};", i, i, i));
                spot_fun.push_str(&format!("
                    color.rgb += calculate_spot_light(spotLight{}, surface.color, surface.position, surface.normal,
                        surface.diffuse_intensity, surface.specular_intensity, surface.specular_power, spotShadowMap{});", i, i));
            }
            let mut point_uniform = String::new();
            let mut point_fun = String::new();
            for i in 0..point_lights.len() {
                point_uniform.push_str(&format!("
                layout (std140) uniform PointLightUniform{}
                {{
                    PointLight pointLight{};
                }};", i, i));
                point_fun.push_str(&format!("
                    color.rgb += calculate_point_light(pointLight{}, surface.color, surface.position, surface.normal,
                        surface.diffuse_intensity, surface.specular_intensity, surface.specular_power);", i));
            }

            let fragment_shader = format!("{}\n{}\n{}",
                                          &include_str!("shaders/light_shared.frag"),
                                          &include_str!("shaders/deferred_light_shared.frag"),
                                          &format!("
                uniform vec3 ambientColor;
                layout (location = 0) out vec4 color;

                {} // Directional lights
                {} // Spot lights
                {} // Point lights

                void main()
                {{
                    color = vec4(ambientColor, 1.0);
                    {} // Surface parameters
                    {} // Directional lights
                    {} // Spot lights
                    {} // Point lights
                }}
                ", &dir_uniform, &spot_uniform, &point_uniform,
                   if !directional_lights.is_empty() || !spot_lights.is_empty() || !point_lights.is_empty() {
                       "Surface surface = get_surface(); color.rgb *= surface.color;"} else {"color.rgb *= get_surface_color();"},
                   &dir_fun, &spot_fun, &point_fun));
            self.program_map.insert(key.clone(), ImageEffect::new(&self.context, &fragment_shader)?);
        };
        let effect = self.program_map.get(&key).unwrap();

        // Ambient light
        let color = ambient_light.map(|light| light.color * light.intensity).unwrap_or(vec3(0.0, 0.0, 0.0));
        effect.use_uniform_vec3("ambientColor", &color)?;

        // Directional light
        for i in 0..directional_lights.len() {
            effect.use_texture(directional_lights[i].shadow_map(), &format!("directionalShadowMap{}", i))?;
            effect.use_uniform_block(directional_lights[i].buffer(), &format!("DirectionalLightUniform{}", i));
        }

        // Spot light
        for i in 0..spot_lights.len() {
            effect.use_texture(spot_lights[i].shadow_map(), &format!("spotShadowMap{}", i))?;
            effect.use_uniform_block(spot_lights[i].buffer(), &format!("SpotLightUniform{}", i));
        }

        // Point light
        for i in 0..point_lights.len() {
            effect.use_uniform_block(point_lights[i].buffer(), &format!("PointLightUniform{}", i));
        }

        effect.use_texture(self.geometry_pass_texture(), "gbuffer")?;
        effect.use_texture(self.geometry_pass_depth_texture_array(), "depthMap")?;
        if !directional_lights.is_empty() || !spot_lights.is_empty() || !point_lights.is_empty() {
            effect.use_uniform_vec3("eyePosition", &camera.position())?;
            effect.use_uniform_mat4("viewProjectionInverse", &(camera.projection() * camera.view()).invert().unwrap())?;
        }
        effect.apply(render_states, viewport)?;
        Ok(())
    }

    pub fn geometry_pass_texture(&self) -> &dyn Texture
    {
        self.geometry_pass_texture.as_ref().unwrap()
    }
    pub fn geometry_pass_depth_texture_array(&self) -> &dyn Texture
    {
        self.geometry_pass_depth_texture.as_ref().unwrap()
    }

    pub fn geometry_pass_depth_texture(&self) -> DepthTargetTexture2D
    {
        let depth_array = self.geometry_pass_depth_texture.as_ref().unwrap();
        let depth_texture = DepthTargetTexture2D::new(&self.context, depth_array.width(), depth_array.height(),Wrapping::ClampToEdge,
                                           Wrapping::ClampToEdge, DepthFormat::Depth32F).unwrap();

        RenderTargetArray::new_depth(&self.context, depth_array).unwrap()
            .copy_depth(0, &RenderTarget::new_depth(&self.context, &depth_texture).unwrap(),
                        Viewport::new_at_origo(depth_array.width(), depth_array.height())).unwrap();
        depth_texture
    }
}