//!
//! Adds functionality for rendering objects with lighting including support for physically based rendering (PBR).
//! To render an object implementing the [ShadedGeometry] trait, either call the [ShadedGeometry::render_with_lighting] method or use the [ForwardPipeline] or [DeferredPipeline].
//!

mod forward_pipeline;
#[doc(inline)]
pub use forward_pipeline::*;

mod deferred_pipeline;
#[doc(inline)]
pub use deferred_pipeline::*;

use crate::core::*;
use crate::renderer::*;

pub trait ShadedGeometry: Geometry {
    ///
    /// Render the geometry and surface material parameters of the object.
    /// Should not be called directly but used in a [deferred render pass](crate::DeferredPipeline::geometry_pass).
    ///
    fn geometry_pass(&self, camera: &Camera, viewport: Viewport, material: &Material)
        -> Result<()>;

    ///
    /// Render the object shaded with the given lights using physically based rendering (PBR).
    /// Must be called in a render target render function, for example in the callback function of [Screen::write].
    /// Will render transparent if the material contain an albedo color with alpha value below 255 or if the albedo texture contain an alpha channel (ie. the format is [Format::RGBA]),
    /// you only need to render the model after all solid models.
    ///
    fn render_with_lighting(
        &self,
        camera: &Camera,
        material: &Material,
        lighting_model: LightingModel,
        ambient_light: Option<&AmbientLight>,
        directional_lights: &[&DirectionalLight],
        spot_lights: &[&SpotLight],
        point_lights: &[&PointLight],
    ) -> Result<()>;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LightingModel {
    Phong,
    Blinn,
    Cook(NormalDistributionFunction, GeometryFunction),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GeometryFunction {
    SmithSchlickGGX,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum NormalDistributionFunction {
    Blinn,
    Beckmann,
    TrowbridgeReitzGGX,
}

pub(in crate::renderer) fn geometry_fragment_shader(material: &Material) -> String {
    format!(
        "in vec3 pos;\nin vec3 nor;\n{}{}",
        material_shader(material),
        include_str!("shading/shaders/deferred_objects.frag")
    )
}

pub(in crate::renderer) fn shaded_fragment_shader(
    lighting_model: LightingModel,
    material: Option<&Material>,
    directional_lights: usize,
    spot_lights: usize,
    point_lights: usize,
) -> String {
    let mut dir_uniform = String::new();
    let mut dir_fun = String::new();
    for i in 0..directional_lights {
        dir_uniform.push_str(&format!(
            "
                uniform sampler2D directionalShadowMap{};
                layout (std140) uniform DirectionalLightUniform{}
                {{
                    DirectionalLight directionalLight{};
                }};",
            i, i, i
        ));
        dir_fun.push_str(&format!("
                    color += calculate_directional_light(directionalLight{}, surface_color, position, normal, metallic, roughness, directionalShadowMap{});", i, i));
    }
    let mut spot_uniform = String::new();
    let mut spot_fun = String::new();
    for i in 0..spot_lights {
        spot_uniform.push_str(&format!(
            "
                uniform sampler2D spotShadowMap{};
                layout (std140) uniform SpotLightUniform{}
                {{
                    SpotLight spotLight{};
                }};",
            i, i, i
        ));
        spot_fun.push_str(&format!(
            "
                    color += calculate_spot_light(spotLight{}, surface_color, position, normal, metallic, roughness, spotShadowMap{});",
            i, i
        ));
    }
    let mut point_uniform = String::new();
    let mut point_fun = String::new();
    for i in 0..point_lights {
        point_uniform.push_str(&format!(
            "
                layout (std140) uniform PointLightUniform{}
                {{
                    PointLight pointLight{};
                }};",
            i, i
        ));
        point_fun.push_str(&format!(
            "
                    color += calculate_point_light(pointLight{}, surface_color, position, normal, metallic, roughness);",
            i
        ));
    }

    let model = match lighting_model {
        LightingModel::Phong => "#define PHONG",
        LightingModel::Blinn => "#define BLINN",
        LightingModel::Cook(normal, _) => match normal {
            NormalDistributionFunction::Blinn => "#define COOK\n#define COOK_BLINN\n",
            NormalDistributionFunction::Beckmann => "#define COOK\n#define COOK_BECKMANN\n",
            NormalDistributionFunction::TrowbridgeReitzGGX => "#define COOK\n#define COOK_GGX\n",
        },
    };

    format!(
        "{}\n{}\n{}\n{}\nin vec3 pos;\nin vec3 nor;\n{}\n{}",
        model,
        include_str!("../core/shared.frag"),
        include_str!("shading/shaders/light_shared.frag"),
        &format!(
            "
                uniform vec3 ambientColor;
                {} // Directional lights
                {} // Spot lights
                {} // Point lights

                vec3 calculate_lighting(vec3 surface_color, vec3 position, vec3 normal, float metallic, float roughness)
                {{
                    vec3 color = ambientColor * mix(surface_color, vec3(0.0), metallic); // Ambient light
                    {} // Directional lights
                    {} // Spot lights
                    {} // Point lights
                    return color;
                }}
                ",
            &dir_uniform, &spot_uniform, &point_uniform, &dir_fun, &spot_fun, &point_fun
        ),
        material.map(|m| material_shader(m)).unwrap_or("#define DEFERRED\nin vec2 uv;\n".to_string()),
        include_str!("shading/shaders/lighting.frag"),
    )
}

fn material_shader(material: &Material) -> String {
    let mut output = String::new();
    if material.albedo_texture.is_some() || material.metallic_roughness_texture.is_some() {
        output.push_str("in vec2 uvs;\n");
        if material.albedo_texture.is_some() {
            output.push_str("#define USE_ALBEDO_TEXTURE;\n");
        }
        if material.metallic_roughness_texture.is_some() {
            output.push_str("#define USE_METALLIC_ROUGHNESS_TEXTURE;\n");
        }
    }
    output
}

pub(in crate::renderer) fn bind_lights(
    program: &Program,
    ambient_light: Option<&AmbientLight>,
    directional_lights: &[&DirectionalLight],
    spot_lights: &[&SpotLight],
    point_lights: &[&PointLight],
    camera_position: &Vec3,
) -> Result<()> {
    // Ambient light
    program.use_uniform_vec3(
        "ambientColor",
        &ambient_light
            .map(|light| light.color.to_vec3() * light.intensity)
            .unwrap_or(vec3(0.0, 0.0, 0.0)),
    )?;

    if !directional_lights.is_empty() || !spot_lights.is_empty() || !point_lights.is_empty() {
        program.use_uniform_vec3("eyePosition", camera_position)?;
    }

    // Directional light
    for i in 0..directional_lights.len() {
        program.use_texture(
            &format!("directionalShadowMap{}", i),
            directional_lights[i].shadow_map(),
        )?;
        program.use_uniform_block(
            &format!("DirectionalLightUniform{}", i),
            directional_lights[i].buffer(),
        );
    }

    // Spot light
    for i in 0..spot_lights.len() {
        program.use_texture(&format!("spotShadowMap{}", i), spot_lights[i].shadow_map())?;
        program.use_uniform_block(&format!("SpotLightUniform{}", i), spot_lights[i].buffer());
    }

    // Point light
    for i in 0..point_lights.len() {
        program.use_uniform_block(&format!("PointLightUniform{}", i), point_lights[i].buffer());
    }
    Ok(())
}
