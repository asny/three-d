use crate::renderer::*;

///
/// Similar to [Model], except it is possible to render many instances of the same model efficiently.
///
pub struct InstancedModel<M: Material>(Vec<Gm<InstancedMesh, M>>);

impl<M: Material> InstancedModel<M> {
    ///
    /// Returns a list of references to the objects in this instanced model which can be used as input to a render function, for example [RenderTarget::render].
    ///
    pub fn to_objects(&self) -> Vec<&dyn Object> {
        self.0.iter().map(|m| m as &dyn Object).collect::<Vec<_>>()
    }

    ///
    /// Returns a list of references to the geometries in this instanced which can be used as input to for example [pick], [RenderTarget::render_with_material] or [DirectionalLight::generate_shadow_map].
    ///
    pub fn to_geometries(&self) -> Vec<&dyn Geometry> {
        self.0
            .iter()
            .map(|m| m as &dyn Geometry)
            .collect::<Vec<_>>()
    }
}

impl<T: Material + FromCpuMaterial + Clone + Default> InstancedModel<T> {
    ///
    /// Constructs an [InstancedModel] from a [CpuModel] and the given [Instances] attributes, ie. constructs a list of [Gm]s with a [InstancedMesh] as geometry (constructed from the [CpuMesh]es in the [CpuModel]) and
    /// a [material] type specified by the generic parameter which implement [FromCpuMaterial] (constructed from the [CpuMaterial]s in the [CpuModel]).
    ///
    pub fn new(
        context: &Context,
        instances: &Instances,
        cpu_model: &CpuModel,
    ) -> Result<Self, CoreError> {
        let mut materials = std::collections::HashMap::new();
        for m in cpu_model.materials.iter() {
            materials.insert(m.name.clone(), T::from_cpu_material(context, m));
        }
        let mut gms: Vec<Gm<InstancedMesh, T>> = Vec::new();
        for g in cpu_model.geometries.iter() {
            gms.push(if let Some(material_name) = &g.material_name {
                Gm {
                    geometry: InstancedMesh::new(context, instances, g),
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
                    geometry: InstancedMesh::new(context, instances, g),
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
