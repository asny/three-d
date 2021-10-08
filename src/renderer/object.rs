//!
//! A collection of objects that can be rendered, for example a mesh.
//!

pub use crate::core::{AxisAlignedBoundingBox, CPUMesh};

mod model;
#[doc(inline)]
pub use model::*;

mod instanced_model;
#[doc(inline)]
pub use instanced_model::*;

mod line;
#[doc(inline)]
pub use line::*;

mod rectangle;
#[doc(inline)]
pub use rectangle::*;

mod circle;
#[doc(inline)]
pub use circle::*;

mod skybox;
#[doc(inline)]
pub use skybox::*;

mod imposters;
#[doc(inline)]
pub use imposters::*;

mod axes;
#[doc(inline)]
pub use axes::*;

mod particles;
#[doc(inline)]
pub use particles::*;

use crate::core::*;
use crate::renderer::*;

pub struct Glue<'a, T: Shadable + Cullable> {
    pub geometry: &'a T,
    pub material: &'a dyn ForwardMaterial,
}

impl<'a, T: Shadable + Cullable> Drawable for Glue<'a, T> {
    fn render(&self, camera: &Camera) -> Result<()> {
        self.geometry.render_forward(self.material, camera)
    }
}

impl<'a, T: Shadable + Cullable> Cullable for Glue<'a, T> {
    fn in_frustum(&self, camera: &Camera) -> bool {
        self.geometry.in_frustum(camera)
    }
}

pub trait Shadable {
    fn render_forward(&self, material: &dyn ForwardMaterial, camera: &Camera) -> Result<()>;
}

pub trait Cullable {
    fn in_frustum(&self, camera: &Camera) -> bool;
}

pub trait Object2D {
    fn render(&self, material: &dyn ForwardMaterial, viewport: Viewport) -> Result<()>;
}

pub trait Drawable {
    ///
    /// Render the object.
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    ///
    fn render(&self, camera: &Camera) -> Result<()>;
}

pub trait DeferredGeometry {
    fn render_deferred(
        &self,
        material: &dyn DeferredMaterial,
        camera: &Camera,
        viewport: Viewport,
    ) -> Result<()>;
}

#[deprecated]
pub trait Geometry {
    ///
    /// Render only the depth into the current depth render target which is useful for shadow maps or depth pre-pass.
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    ///
    #[deprecated = "Use 'render_forward' instead"]
    fn render_depth(&self, camera: &Camera) -> Result<()>;

    ///
    /// Render the depth (scaled such that a value of 1 corresponds to max_depth) into the red channel of the current color render target which for example is used for picking.
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    ///
    #[deprecated = "Use 'render_forward' instead"]
    fn render_depth_to_red(&self, camera: &Camera, max_depth: f32) -> Result<()>;

    #[deprecated]
    fn aabb(&self) -> AxisAlignedBoundingBox;
}

#[deprecated]
pub trait ShadedGeometry: Geometry {
    ///
    /// Render the geometry and surface material parameters of the object.
    /// Should not be called directly but used in a [deferred render pass](crate::DeferredPipeline::geometry_pass).
    ///
    #[deprecated = "Use 'render_deferred' instead"]
    fn geometry_pass(
        &self,
        camera: &Camera,
        viewport: Viewport,
        material: &PhysicalMaterial,
    ) -> Result<()>;

    ///
    /// Render the object shaded with the given lights using physically based rendering (PBR).
    /// Must be called in a render target render function, for example in the callback function of [Screen::write].
    /// Will render transparent if the material contain an albedo color with alpha value below 255 or if the albedo texture contain an alpha channel (ie. the format is [Format::RGBA]),
    /// you only need to render the model after all solid models.
    ///
    #[deprecated = "Use 'render_forward' instead"]
    fn render_with_lighting(
        &self,
        camera: &Camera,
        material: &PhysicalMaterial,
        lighting_model: LightingModel,
        ambient_light: Option<&AmbientLight>,
        directional_lights: &[&DirectionalLight],
        spot_lights: &[&SpotLight],
        point_lights: &[&PointLight],
    ) -> Result<()>;
}
