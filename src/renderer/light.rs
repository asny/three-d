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

///
/// Specifies how the intensity of a light fades over distance.
/// The light intensity is scaled by ``` 1 / max(1, constant + distance * linear + distance * distance * quadratic) ```.
///
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Attenuation {
    /// Constant attenuation factor.
    pub constant: f32,
    /// Linear attenuation factor.
    pub linear: f32,
    /// Quadratic attenuation factor.
    pub quadratic: f32,
}

impl Default for Attenuation {
    fn default() -> Self {
        Self {
            constant: 1.0,
            linear: 0.0,
            quadratic: 0.0,
        }
    }
}

#[deprecated = "use slice of lights instead when making a render call"]
#[allow(missing_docs)]
pub struct Lights {
    pub ambient: Option<AmbientLight>,
    pub directional: Vec<DirectionalLight>,
    pub spot: Vec<SpotLight>,
    pub point: Vec<PointLight>,
    #[deprecated = "the lighting model is specified on each physical material"]
    pub lighting_model: LightingModel,
}

#[allow(deprecated, missing_docs)]
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

#[allow(deprecated, missing_docs)]
pub struct LightsIterator<'a> {
    lights: &'a Lights,
    index: usize,
}

#[allow(deprecated, missing_docs)]
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

/// Represents a light source.
pub trait Light {
    /// The fragment shader source for calculating this lights contribution to the color in a fragment.
    /// It should contain a function with this signature
    /// ```
    /// vec3 calculate_lighting{}(vec3 surface_color, vec3 position, vec3 normal, vec3 view_direction, float metallic, float roughness, float occlusion)
    /// ```
    /// Where {} is replaced with the number i given as input.
    /// This function should return the color contribution for this light on the surface with the given surface parameters.
    fn shader_source(&self, i: u32) -> String;
    /// Should bind the uniforms that is needed for calculating this lights contribution to the color in [Light::shader_source].
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

impl<T: Light + ?Sized> Light for &mut T {
    fn shader_source(&self, i: u32) -> String {
        (**self).shader_source(i)
    }
    fn use_uniforms(&self, program: &Program, i: u32) -> ThreeDResult<()> {
        (**self).use_uniforms(program, i)
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

pub(crate) fn lights_fragment_shader_source(
    lights: &[&dyn Light],
    lighting_model: LightingModel,
) -> String {
    let mut shader_source = lighting_model.shader().to_string();
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
