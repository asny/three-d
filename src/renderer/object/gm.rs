use crate::renderer::*;

///
/// A combination of a [Geometry] and a [Material] which implements [Object].
/// Use this to combine any [geometry] and [material] into an object that can be used in a render function for example [RenderTarget::render].
/// The only requirement is that the geometry provides all the per vertex information (normals, uv coordinates, etc.) that the material requires.
///
pub struct Gm<G: Geometry, M: Material> {
    /// The geometry
    pub geometry: G,
    /// The material applied to the geometry
    pub material: M,
}

impl<G: Geometry, M: Material> Gm<G, M> {
    ///
    /// Creates a new [Gm] from a geometry and material.
    ///
    pub fn new(geometry: G, material: M) -> Self {
        Self { geometry, material }
    }

    ///
    /// Returns an iterator over a reference to the geometry which can be used as input to for example [pick], [RenderTarget::render_with_material] or [DirectionalLight::generate_shadow_map].
    ///
    pub fn geometries(&self) -> impl Iterator<Item = &dyn Geometry> + Clone {
        std::iter::once(self as &dyn Geometry)
    }
}

impl<'a, G: Geometry, M: Material> IntoIterator for &'a Gm<G, M> {
    type Item = &'a dyn Object;
    type IntoIter = std::iter::Once<&'a dyn Object>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self)
    }
}

impl<G: Geometry, M: Material> Geometry for Gm<G, M> {
    fn aabb(&self) -> AxisAlignedBoundingBox {
        self.geometry.aabb()
    }

    fn render_with_material(
        &self,
        material: &dyn Material,
        camera: &Camera,
        lights: &[&dyn Light],
    ) {
        self.geometry.render_with_material(material, camera, lights)
    }
}

impl<G: Geometry, M: Material> Object for Gm<G, M> {
    fn render(&self, camera: &Camera, lights: &[&dyn Light]) {
        self.render_with_material(&self.material, camera, lights)
    }

    fn material_type(&self) -> MaterialType {
        self.material.material_type()
    }
}

impl<G: Geometry + Clone, M: Material + Clone> Clone for Gm<G, M> {
    fn clone(&self) -> Self {
        Self {
            geometry: self.geometry.clone(),
            material: self.material.clone(),
        }
    }
}

impl<G: Geometry, M: Material> std::ops::Deref for Gm<G, M> {
    type Target = G;
    fn deref(&self) -> &Self::Target {
        &self.geometry
    }
}

impl<G: Geometry, M: Material> std::ops::DerefMut for Gm<G, M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.geometry
    }
}
