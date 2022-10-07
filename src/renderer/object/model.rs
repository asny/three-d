use crate::renderer::*;

pub use three_d_asset::Model as CpuModel;

///
/// A 3D model consisting of a set of [Gm]s with [Mesh]es as the geometries and a [material] type specified by the generic parameter.
///
pub struct Model<M: Material>(Vec<Gm<Mesh, M>>);

impl<'a, M: Material> IntoIterator for &'a Model<M> {
    type Item = &'a dyn Object;
    type IntoIter = std::vec::IntoIter<&'a dyn Object>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
            .map(|m| m as &dyn Object)
            .collect::<Vec<_>>()
            .into_iter()
    }
}

impl<M: Material + FromCpuMaterial + Clone + Default> Model<M> {
    ///
    /// Constructs a [Model] from a [CpuModel], ie. constructs a list of [Gm]s with a [Mesh] as geometry (constructed from the [CpuMesh]es in the [CpuModel]) and
    /// a [material] type specified by the generic parameter which implement [FromCpuMaterial] (constructed from the [CpuMaterial]s in the [CpuModel]).
    ///
    pub fn new(context: &Context, cpu_model: &CpuModel) -> Result<Self, RendererError> {
        let mut materials = std::collections::HashMap::new();
        for m in cpu_model.materials.iter() {
            materials.insert(m.name.clone(), M::from_cpu_material(context, m));
        }
        let mut gms = Vec::new();
        for g in cpu_model.geometries.iter() {
            gms.push(if let Some(material_name) = &g.material_name {
                Gm {
                    geometry: Mesh::new(context, g),
                    material: materials
                        .get(material_name)
                        .ok_or(RendererError::MissingMaterial(
                            material_name.clone(),
                            g.name.clone(),
                        ))?
                        .clone(),
                }
            } else {
                Gm {
                    geometry: Mesh::new(context, g),
                    material: M::default(),
                }
            });
        }
        Ok(Self(gms))
    }
}

impl<M: Material> std::ops::Deref for Model<M> {
    type Target = Vec<Gm<Mesh, M>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<M: Material> std::ops::DerefMut for Model<M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
