//!
//! Adds functionality for physically based rendering (PBR).
//!

mod material;
#[doc(inline)]
pub use material::*;

mod deferred_pipeline;
#[doc(inline)]
pub use deferred_pipeline::*;

use crate::camera::*;
use crate::core::*;
use crate::light::*;
use crate::math::*;
use crate::object::*;

pub trait ShadedGeometry: Geometry {
    ///
    /// Render the geometry and surface material parameters of the object.
    /// Should not be called directly but used in a [deferred render pass](crate::DeferredPipeline::geometry_pass).
    ///
    fn geometry_pass(
        &self,
        render_states: RenderStates,
        viewport: Viewport,
        camera: &Camera,
    ) -> Result<(), Error>;

    ///
    /// Render the object shaded with the given lights using physically based rendering (PBR).
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    ///
    fn render_with_lighting(
        &self,
        render_states: RenderStates,
        viewport: Viewport,
        camera: &Camera,
        ambient_light: Option<&AmbientLight>,
        directional_lights: &[&DirectionalLight],
        spot_lights: &[&SpotLight],
        point_lights: &[&PointLight],
    ) -> Result<(), Error>;
}

pub(crate) fn geometry_fragment_shader(material: &Material) -> String {
    format!(
        "{}{}",
        material_shader(material),
        include_str!("shading/shaders/deferred_objects.frag")
    )
}

pub(crate) fn shaded_fragment_shader(
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
                    color.rgb += calculate_directional_light(directionalLight{}, surface_color, position, normal, metallic, roughness, directionalShadowMap{});", i, i));
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
                    color.rgb += calculate_spot_light(spotLight{}, surface_color, position, normal, metallic, roughness, spotShadowMap{});",
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
                    color.rgb += calculate_point_light(pointLight{}, surface_color, position, normal, metallic, roughness);",
            i
        ));
    }

    format!(
        "#define PHONG\n{}\n{}\n{}\n{}\n{}",
        include_str!("core/shared.frag"),
        include_str!("shading/shaders/light_shared.frag"),
        &format!(
            "
                {} // Directional lights
                {} // Spot lights
                {} // Point lights

                void calculate_lighting(inout vec4 color, vec3 surface_color, vec3 position, vec3 normal, float metallic, float roughness)
                {{
                    {} // Directional lights
                    {} // Spot lights
                    {} // Point lights
                }}
                ",
            &dir_uniform, &spot_uniform, &point_uniform, &dir_fun, &spot_fun, &point_fun
        ),
        material.map(|m| material_shader(m)).unwrap_or("#define DEFERRED\nin vec2 uv;\n"),
        include_str!("shading/shaders/lighting.frag"),
    )
}

fn material_shader(material: &Material) -> &'static str {
    match material.color_source {
        ColorSource::Color(_) => "in vec3 pos;\nin vec3 nor;\n",
        ColorSource::Texture(_) => {
            "#define USE_COLOR_TEXTURE;\nin vec3 pos;\nin vec3 nor;\nin vec2 uvs;\n"
        }
    }
}

pub(crate) fn bind_lights(
    effect: &Program,
    ambient_light: Option<&AmbientLight>,
    directional_lights: &[&DirectionalLight],
    spot_lights: &[&SpotLight],
    point_lights: &[&PointLight],
) -> Result<(), Error> {
    // Ambient light
    effect.use_uniform_vec3(
        "ambientColor",
        &ambient_light
            .map(|light| light.color * light.intensity)
            .unwrap_or(vec3(0.0, 0.0, 0.0)),
    )?;

    // Directional light
    for i in 0..directional_lights.len() {
        effect.use_texture(
            directional_lights[i].shadow_map(),
            &format!("directionalShadowMap{}", i),
        )?;
        effect.use_uniform_block(
            directional_lights[i].buffer(),
            &format!("DirectionalLightUniform{}", i),
        );
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
    Ok(())
}
