use crate::renderer::*;

///
/// Similar to [Model], except it is possible to render many instances of the same model efficiently.
///
pub struct InstancedModel<M: Material>(Vec<Gm<InstancedMesh, M>>);

impl<'a, M: Material> IntoIterator for &'a InstancedModel<M> {
    type Item = &'a dyn Object;
    type IntoIter = std::vec::IntoIter<&'a dyn Object>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
            .map(|m| m as &dyn Object)
            .collect::<Vec<_>>()
            .into_iter()
    }
}

impl<M: Material + FromCpuMaterial + Clone + Default> InstancedModel<M> {
    ///
    /// Constructs an [InstancedModel] from a [CpuModel] and the given [Instances] attributes, ie. constructs a list of [Gm]s with a [InstancedMesh] as geometry (constructed from the [CpuMesh]es in the [CpuModel]) and
    /// a [material] type specified by the generic parameter which implement [FromCpuMaterial] (constructed from the [CpuMaterial]s in the [CpuModel]).
    ///
    pub fn new(
        context: &Context,
        instances: &Instances,
        cpu_model: &CpuModel,
    ) -> Result<Self, RendererError> {
        let materials = cpu_model
            .materials
            .iter()
            .map(|m| M::from_cpu_material(context, m))
            .collect::<Vec<_>>();
        let mut gms: Vec<Gm<InstancedMesh, M>> = Vec::new();
        for primitive in cpu_model.geometries.iter() {
            if let CpuGeometry::Triangles(geometry) = &primitive.geometry {
                let material = if let Some(material_index) = primitive.material_index {
                    materials
                        .get(material_index)
                        .ok_or_else(|| {
                            RendererError::MissingMaterial(
                                material_index.to_string(),
                                primitive.name.clone(),
                            )
                        })?
                        .clone()
                } else {
                    M::default()
                };
                gms.push(Gm {
                    geometry: InstancedMesh::new(context, instances, &geometry),
                    material,
                });
            }
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
