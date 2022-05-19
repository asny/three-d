use crate::renderer::*;

///
/// A 3D model consisting of a [Mesh] and any [material] that implements [Material].
///
pub type Model<M> = Gm<Mesh, M>;

impl Model<ColorMaterial> {
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

pub use three_d_asset::Models as CpuModels;

pub fn new_models<T: Material + FromCpuMaterial + Clone + Default>(
    context: &Context,
    cpu_models: &CpuModels,
) -> ThreeDResult<Vec<Model<T>>> {
    let mut materials = std::collections::HashMap::new();
    for m in cpu_models.materials.iter() {
        materials.insert(m.name.clone(), T::from_cpu_material(context, m)?);
    }
    let mut models: Vec<Model<T>> = Vec::new();
    for g in cpu_models.geometries.iter() {
        models.push(if let Some(material_name) = &g.material_name {
            Model::new_with_material(context, g, materials.get(material_name).unwrap().clone())?
        } else {
            Model::new_with_material(context, g, T::default())?
        });
    }
    Ok(models)
}
