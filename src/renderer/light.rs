//!
//! A collection of light types.
//! Currently implemented light types are ambient light, directional light, spot light and point light.
//! Directional and spot lights can cast shadows.
//!

mod directional_light;
#[doc(inline)]
pub use directional_light::*;

mod spot_light;
#[doc(inline)]
pub use spot_light::*;

mod point_light;
#[doc(inline)]
pub use point_light::*;

mod ambient_light;
#[doc(inline)]
pub use ambient_light::*;

mod environment;
#[doc(inline)]
pub use environment::*;

use crate::core::*;

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

impl LightingModel {
    pub(crate) fn shader(&self) -> &str {
        match self {
            LightingModel::Phong => "#define PHONG",
            LightingModel::Blinn => "#define BLINN",
            LightingModel::Cook(normal, _) => match normal {
                NormalDistributionFunction::Blinn => "#define COOK\n#define COOK_BLINN\n",
                NormalDistributionFunction::Beckmann => "#define COOK\n#define COOK_BECKMANN\n",
                NormalDistributionFunction::TrowbridgeReitzGGX => {
                    "#define COOK\n#define COOK_GGX\n"
                }
            },
        }
    }
}

#[deprecated = "Use slice of lights instead when making a render call"]
pub struct Lights {
    pub ambient: Option<AmbientLight>,
    pub directional: Vec<DirectionalLight>,
    pub spot: Vec<SpotLight>,
    pub point: Vec<PointLight>,
    pub lighting_model: LightingModel,
}

#[allow(deprecated)]
impl Lights {
    pub fn iter<'a>(&'a self) -> LightsIterator<'a> {
        LightsIterator::new(self)
    }

    pub fn to_vec<'a>(&'a self) -> Vec<&'a dyn Light> {
        self.iter().collect::<Vec<_>>()
    }
}

#[allow(deprecated)]
impl<'a> IntoIterator for &'a Lights {
    type Item = &'a dyn Light;
    type IntoIter = LightsIterator<'a>;
    fn into_iter(self) -> LightsIterator<'a> {
        LightsIterator::new(&self)
    }
}

#[allow(deprecated)]
impl Default for Lights {
    fn default() -> Self {
        Self {
            ambient: None,
            directional: Vec::new(),
            spot: Vec::new(),
            point: Vec::new(),
            lighting_model: LightingModel::Blinn,
        }
    }
}

#[allow(deprecated)]
pub struct LightsIterator<'a> {
    lights: &'a Lights,
    index: usize,
}

#[allow(deprecated)]
impl<'a> LightsIterator<'a> {
    pub fn new(lights: &'a Lights) -> Self {
        Self { index: 0, lights }
    }
}

impl<'a> std::clone::Clone for LightsIterator<'a> {
    fn clone(&self) -> Self {
        Self::new(self.lights)
    }
}

#[allow(deprecated)]
impl<'a> Iterator for LightsIterator<'a> {
    type Item = &'a dyn Light;
    fn next(&mut self) -> Option<Self::Item> {
        let mut count = 0;
        let result = self
            .lights
            .ambient
            .as_ref()
            .filter(|_| self.index == 0)
            .map(|l| l as &dyn Light);
        if self.lights.ambient.is_some() {
            count += 1;
        }

        let result = result.or_else(|| {
            self.lights
                .directional
                .get(self.index - count)
                .map(|l| l as &dyn Light)
        });
        count += self.lights.directional.len();

        let result = result.or_else(|| {
            self.lights
                .spot
                .get(self.index - count)
                .map(|l| l as &dyn Light)
        });
        count += self.lights.spot.len();

        let result = result.or_else(|| {
            self.lights
                .point
                .get(self.index - count)
                .map(|l| l as &dyn Light)
        });

        self.index += 1;
        result
    }
}

pub trait Light {
    fn shader_source(&self, i: u32) -> String;
    fn use_uniforms(&self, program: &Program, i: u32) -> ThreeDResult<()>;
}

impl<T: Light + ?Sized> Light for &T {
    fn shader_source(&self, i: u32) -> String {
        (*self).shader_source(i)
    }
    fn use_uniforms(&self, program: &Program, i: u32) -> ThreeDResult<()> {
        (*self).use_uniforms(program, i)
    }
}

impl<T: Light> Light for Box<T> {
    fn shader_source(&self, i: u32) -> String {
        self.as_ref().shader_source(i)
    }
    fn use_uniforms(&self, program: &Program, i: u32) -> ThreeDResult<()> {
        self.as_ref().use_uniforms(program, i)
    }
}

impl<T: Light> Light for std::rc::Rc<T> {
    fn shader_source(&self, i: u32) -> String {
        self.as_ref().shader_source(i)
    }
    fn use_uniforms(&self, program: &Program, i: u32) -> ThreeDResult<()> {
        self.as_ref().use_uniforms(program, i)
    }
}

pub(crate) fn lights_fragment_shader_source(lights: &[&dyn Light]) -> String {
    let mut shader_source = LightingModel::Cook(
        NormalDistributionFunction::TrowbridgeReitzGGX,
        GeometryFunction::SmithSchlickGGX,
    )
    .shader()
    .to_string();
    shader_source.push_str(include_str!("../core/shared.frag"));
    shader_source.push_str(include_str!("light/shaders/light_shared.frag"));
    let mut dir_fun = String::new();
    for (i, light) in lights.iter().enumerate() {
        shader_source.push_str(&light.shader_source(i as u32));
        dir_fun.push_str(&format!("color += calculate_lighting{}(surface_color, position, normal, view_direction, metallic, roughness, occlusion);\n", i))
    }
    shader_source.push_str(&format!(
        "
            uniform vec3 eyePosition;
            vec3 calculate_lighting(vec3 surface_color, vec3 position, vec3 normal, float metallic, float roughness, float occlusion)
            {{
                vec3 color = vec3(0.0, 0.0, 0.0);
                vec3 view_direction = normalize(eyePosition - position);
                {}
                return color;
            }}
            ",
        &dir_fun
    ));
    shader_source
}

fn shadow_matrix(camera: &Camera) -> Mat4 {
    let bias_matrix = crate::Mat4::new(
        0.5, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.5, 0.5, 0.5, 1.0,
    );
    bias_matrix * camera.projection() * camera.view()
}

fn compute_up_direction(direction: Vec3) -> Vec3 {
    if vec3(1.0, 0.0, 0.0).dot(direction).abs() > 0.9 {
        (vec3(0.0, 1.0, 0.0).cross(direction)).normalize()
    } else {
        (vec3(1.0, 0.0, 0.0).cross(direction)).normalize()
    }
}
