use crate::renderer::*;

///
/// Similar to [Model], except it is possible to render many instances of the same model efficiently.
/// Usually constructed from a [CpuModel].
///
pub struct InstancedModel<M: Material>(Vec<Gm<InstancedMesh, M>>);

impl<M: Material> InstancedModel<M> {
    ///
    /// Creates a new instanced 3D model with a triangle mesh as geometry and the given material.
    /// The model is rendered in as many instances as there are data in the [Instances] given as input.
    ///
    #[deprecated = "Use Gm::new(InstancedMesh::new(&context, &instances, &cpu_mesh)?, material);"]
    pub fn new_with_material(
        context: &Context,
        instances: &Instances,
        cpu_mesh: &CpuMesh,
        material: M,
    ) -> ThreeDResult<Gm<InstancedMesh, M>> {
        Ok(Gm {
            geometry: InstancedMesh::new(context, instances, cpu_mesh)?,
            material,
        })
    }

    pub fn to_objects(&self) -> Vec<&dyn Object> {
        self.0.iter().map(|m| m as &dyn Object).collect::<Vec<_>>()
    }
}

impl<T: Material + FromCpuMaterial + Clone + Default> InstancedModel<T> {
    ///
    /// Constructs an [InstancedModel] from a [CpuModel] and the given [Instances] attributes.
    ///
    pub fn new(
        context: &Context,
        instances: &Instances,
        cpu_model: &CpuModel,
    ) -> ThreeDResult<Self> {
        let mut materials = std::collections::HashMap::new();
        for m in cpu_model.materials.iter() {
            materials.insert(m.name.clone(), T::from_cpu_material(context, m)?);
        }
        let mut gms: Vec<Gm<InstancedMesh, T>> = Vec::new();
        for g in cpu_model.geometries.iter() {
            gms.push(if let Some(material_name) = &g.material_name {
                Gm {
                    geometry: InstancedMesh::new(context, instances, g)?,
                    material: materials
                        .get(material_name)
                        .ok_or(CoreError::MissingMaterial(
                            material_name.clone(),
                            g.name.clone(),
                        ))?
                        .clone(),
                }
            } else {
                Gm {
                    geometry: InstancedMesh::new(context, instances, g)?,
                    material: T::default(),
                }
            });
        }
        Ok(Self(gms))
    }
}

impl<M: Material> std::ops::Deref for InstancedModel<M> {
    type Target = Vec<Gm<InstancedMesh, M>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<M: Material> std::ops::DerefMut for InstancedModel<M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
