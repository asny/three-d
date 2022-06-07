use crate::renderer::*;

pub use three_d_asset::Model as CpuModel;

///
/// A 3D model consisting of a set of [Gm]s with [Mesh]es as the geometries and a [material] type specified by the generic parameter.
///
pub struct Model<M: Material>(Vec<Gm<Mesh, M>>);

impl<M: Material> Model<M> {
    ///
    /// Returns a list of references to the objects in this model which can be used as input to a render function, for example [RenderTarget::render].
    ///
    pub fn to_objects(&self) -> Vec<&dyn Object> {
        self.0.iter().map(|m| m as &dyn Object).collect::<Vec<_>>()
    }

    ///
    /// Returns a list of references to the geometries in this model which can be used as input to for example [pick], [RenderTarget::render_with_material] or [DirectionalLight::generate_shadow_map].
    ///
    pub fn to_geometries(&self) -> Vec<&dyn Geometry> {
        self.0
            .iter()
            .map(|m| m as &dyn Geometry)
            .collect::<Vec<_>>()
    }
}

impl<M: Material + FromCpuMaterial + Clone + Default> Model<M> {
    ///
    /// Constructs a [Model] from a [CpuModel], ie. constructs a list of [Gm]s with a [Mesh] as geometry (constructed from the [CpuMesh]es in the [CpuModel]) and
    /// a [material] type specified by the generic parameter which implement [FromCpuMaterial] (constructed from the [CpuMaterial]s in the [CpuModel]).
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
