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

pub use three_d_asset::Model as CpuModel;

pub fn new_models(
    context: &Context,
    cpu_model: &CpuModel,
) -> ThreeDResult<Vec<Model<PhysicalMaterial>>> {
    let mut materials = Vec::new();
    for m in cpu_model.materials.iter() {
        materials.push(PhysicalMaterial::new(context, m)?);
    }
    let mut models = Vec::new();
    for g in cpu_model.geometries.iter() {
        models.push(Gm {
            geometry: Mesh::new(context, g)?,
            material: materials
                .iter()
                .find(|m| Some(&m.name) == g.material_name.as_ref())
                .unwrap()
                .clone(),
        });
    }
    Ok(models)
}
