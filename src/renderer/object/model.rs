use crate::renderer::*;

///
/// A 3D model consisting of a [Mesh] and any material that implements [Material].
///
pub type Model<M> = Shape<Mesh, M>;

impl Model<ColorMaterial<std::rc::Rc<Texture2D<u8>>>> {
    ///
    /// Creates a new 3D model with a [Mesh] as geometry and a default [ColorMaterial].
    ///
    pub fn new(context: &Context, cpu_mesh: &CpuMesh) -> ThreeDResult<Self> {
        Self::new_with_material(context, cpu_mesh, ColorMaterial::default())
    }
}

impl<M: Material> Model<M> {
    ///
    /// Creates a new 3D model with a [Mesh] as geometry and the given material.
    ///
    pub fn new_with_material(
        context: &Context,
        cpu_mesh: &CpuMesh,
        material: M,
    ) -> ThreeDResult<Self> {
        Ok(Self {
            geometry: Mesh::new(context, cpu_mesh)?,
            material,
        })
    }
}
