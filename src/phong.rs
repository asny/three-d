//!
//! Lighting functionality based on the phong reflection model.
//!

mod material;
#[doc(inline)]
pub use material::*;

mod forward_pipeline;
#[doc(inline)]
pub use forward_pipeline::*;

mod geometry;
#[doc(inline)]
pub use geometry::*;

mod deferred_pipeline;
#[doc(inline)]
pub use deferred_pipeline::*;

mod phong_mesh;
#[doc(inline)]
pub use phong_mesh::*;

mod phong_instanced_mesh;
#[doc(inline)]
pub use phong_instanced_mesh::*;

fn phong_fragment_shader(
    surface_functionality: &str,
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
                    color.rgb += calculate_directional_light(directionalLight{}, surface.color.rgb, surface.position, surface.normal,
                        surface.diffuse_intensity, surface.specular_intensity, surface.specular_power, directionalShadowMap{});", i, i));
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
        spot_fun.push_str(&format!("
                    color.rgb += calculate_spot_light(spotLight{}, surface.color.rgb, surface.position, surface.normal,
                        surface.diffuse_intensity, surface.specular_intensity, surface.specular_power, spotShadowMap{});", i, i));
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
        point_fun.push_str(&format!("
                    color.rgb += calculate_point_light(pointLight{}, surface.color.rgb, surface.position, surface.normal,
                        surface.diffuse_intensity, surface.specular_intensity, surface.specular_power);", i));
    }

    format!(
        "{}\n{}\n{}",
        &include_str!("phong/shaders/light_shared.frag"),
        surface_functionality,
        &format!(
            "
                uniform vec3 ambientColor;
                layout (location = 0) out vec4 color;

                {} // Directional lights
                {} // Spot lights
                {} // Point lights

                void main()
                {{
                    {} // Surface parameters
                    color = vec4(ambientColor * surfaceColor.rgb, surfaceColor.a);
                    {} // Directional lights
                    {} // Spot lights
                    {} // Point lights
                }}
                ",
            &dir_uniform,
            &spot_uniform,
            &point_uniform,
            if directional_lights > 0 || spot_lights > 0 || point_lights > 0 {
                "Surface surface = get_surface(); vec4 surfaceColor = surface.color;"
            } else {
                "vec4 surfaceColor = get_surface_color();"
            },
            &dir_fun,
            &spot_fun,
            &point_fun
        )
    )
}

use crate::core::*;
use crate::light::*;
use crate::math::*;
fn bind_lights(
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
