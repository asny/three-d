use crate::renderer::*;

//pub type Model<M> = Gm<Mesh, M>;
pub use three_d_asset::Model as CpuModel;

///
/// A 3D model consisting of a set of [Mesh]es and applied [material] (see [Gm]).
/// Usually constructed from a [CpuModel].
///
pub struct Model<M: Material>(Vec<Gm<Mesh, M>>);

impl<M: Material> Model<M> {
    ///
    /// Creates a new 3D model with a [Mesh] as geometry and the given material.
    ///
    #[deprecated = "Use Gm::new(Mesh::new(&context, &cpu_mesh)?, material);"]
    pub fn new_with_material(
        context: &Context,
        cpu_mesh: &CpuMesh,
        material: M,
    ) -> ThreeDResult<Gm<Mesh, M>> {
        Ok(Gm {
            geometry: Mesh::new(context, cpu_mesh)?,
            material,
        })
    }

    pub fn to_objects(&self) -> Vec<&dyn Object> {
        self.0.iter().map(|m| m as &dyn Object).collect::<Vec<_>>()
    }
}

impl<M: Material + FromCpuMaterial + Clone + Default> Model<M> {
    ///
    /// Constructs a [Model] from [CpuModel].
    ///
    pub fn new(context: &Context, cpu_model: &CpuModel) -> ThreeDResult<Model<M>> {
        let mut materials = std::collections::HashMap::new();
        for m in cpu_model.materials.iter() {
            materials.insert(m.name.clone(), M::from_cpu_material(context, m)?);
        }
        let mut gms = Vec::new();
        for g in cpu_model.geometries.iter() {
            gms.push(if let Some(material_name) = &g.material_name {
                Gm {
                    geometry: Mesh::new(context, g)?,
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
                    geometry: Mesh::new(context, g)?,
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
