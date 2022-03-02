use crate::core::*;
use crate::renderer::*;

///
/// Similar to [Model], except it is possible to render many instances of the same model efficiently.
///
pub type InstancedModel<M> = Shape<InstancedMesh, M>;

impl InstancedModel<ColorMaterial<std::rc::Rc<Texture2D<u8>>>> {
    ///
    /// Creates a new instanced 3D model with a triangle mesh as geometry and a default [ColorMaterial].
    /// The model is rendered in as many instances as there are transformation matrices.
    /// The transformation and texture transform in [ModelInstance] are applied to each model instance before they are rendered.
    ///
    pub fn new(
        context: &Context,
        instances: &[ModelInstance],
        cpu_mesh: &CpuMesh,
    ) -> ThreeDResult<Self> {
        Self::new_with_material(context, instances, cpu_mesh, ColorMaterial::default())
    }
}

impl<M: Material> InstancedModel<M> {
    ///
    /// Creates a new instanced 3D model with a triangle mesh as geometry and the given material.
    /// The model is rendered in as many instances as there are [ModelInstance] structs given as input.
    /// The transformation and texture transform in [ModelInstance] are applied to each model instance before they are rendered.
    ///
    pub fn new_with_material(
        context: &Context,
        instances: &[ModelInstance],
        cpu_mesh: &CpuMesh,
        material: M,
    ) -> ThreeDResult<Self> {
        Ok(Shape {
            geometry: InstancedMesh::new(context, instances, cpu_mesh)?,
            material,
        })
    }
}

impl<M: Material> std::ops::Deref for InstancedModel<M> {
    type Target = InstancedMesh;
    fn deref(&self) -> &Self::Target {
        &self.geometry
    }
}

impl<M: Material> std::ops::DerefMut for InstancedModel<M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.geometry
    }
}
