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

///
/// A list of [InstancedModel]s, usually constructed from [CpuModels].
///
pub struct InstancedModels<T: Material>(pub Vec<InstancedModel<T>>);

impl<T: Material + FromCpuMaterial + Clone + Default> InstancedModels<T> {
    ///
    /// Constructs a list of [InstancedModel]s from [CpuModels] and the given [Instances] attributes.
    ///
    pub fn new(
        context: &Context,
        instances: &Instances,
        cpu_models: &CpuModels,
    ) -> ThreeDResult<InstancedModels<T>> {
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
                    materials
                        .get(material_name)
                        .ok_or(CoreError::MissingMaterial(
                            material_name.clone(),
                            g.name.clone(),
                        ))?
                        .clone(),
                )?
            } else {
                InstancedModel::new_with_material(context, instances, g, T::default())?
            });
        }
        Ok(Self(models))
    }
}

impl<T: Material> std::ops::Deref for InstancedModels<T> {
    type Target = Vec<InstancedModel<T>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Material> std::ops::DerefMut for InstancedModels<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
