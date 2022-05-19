use crate::renderer::*;

///
/// Similar to [Model], except it is possible to render many instances of the same model efficiently.
///
pub type InstancedModel<M> = Gm<InstancedMesh, M>;

impl InstancedModel<ColorMaterial> {
    ///
    /// Creates a new instanced 3D model with a triangle mesh as geometry and a default [ColorMaterial].
    /// The model is rendered in as many instances as there are attributes in [Instances] given as input.
    ///
    pub fn new(context: &Context, instances: &Instances, cpu_mesh: &CpuMesh) -> ThreeDResult<Self> {
        Self::new_with_material(context, instances, cpu_mesh, ColorMaterial::default())
    }
}

impl<M: Material> InstancedModel<M> {
    ///
    /// Creates a new instanced 3D model with a triangle mesh as geometry and the given material.
    /// The model is rendered in as many instances as there are data in the [Instances] given as input.
    ///
    pub fn new_with_material(
        context: &Context,
        instances: &Instances,
        cpu_mesh: &CpuMesh,
        material: M,
    ) -> ThreeDResult<Self> {
        Ok(Self {
            geometry: InstancedMesh::new(context, instances, cpu_mesh)?,
            material,
        })
    }
}

pub fn new_instanced_models<T: Material + FromCpuMaterial + Clone + Default>(
    context: &Context,
    instances: &Instances,
    cpu_models: &CpuModels,
) -> ThreeDResult<Vec<InstancedModel<T>>> {
    let mut materials = std::collections::HashMap::new();
    for m in cpu_models.materials.iter() {
        materials.insert(m.name.clone(), T::from_cpu_material(context, m)?);
    }
    let mut models: Vec<InstancedModel<T>> = Vec::new();
    for g in cpu_models.geometries.iter() {
        models.push(if let Some(material_name) = &g.material_name {
            InstancedModel::new_with_material(
                context,
                instances,
                g,
                materials.get(material_name).unwrap().clone(),
            )?
        } else {
            InstancedModel::new_with_material(context, instances, g, T::default())?
        });
    }
    Ok(models)
}
