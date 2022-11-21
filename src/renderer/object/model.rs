use crate::renderer::*;

pub use three_d_asset::Model as CpuModel;
use three_d_asset::{Animation, KeyFrames};

///
/// A 3D model consisting of a set of [Gm]s with [Mesh]es as the geometries and a [material] type specified by the generic parameter.
///
pub struct Model<M: Material> {
    gms: Vec<Gm<Mesh, M>>,
    key_frames_indices: Vec<Vec<usize>>,
    animations: Vec<Animation>,
}

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

impl<M: Material> Model<M> {
    pub fn update_animation(&mut self, time: f32) {
        let animation = self.animations.get(0).unwrap();
        let time = (0.001 * time) % animation.loop_time;
        for i in 0..self.gms.len() {
            let mut transformation = Mat4::identity();
            for key_frames_index in self.key_frames_indices[i].iter() {
                transformation =
                    animation.key_frames[*key_frames_index].transformation(time) * transformation;
            }
            self.gms[i].set_transformation(transformation);
        }
    }
}

impl<M: Material + FromCpuMaterial + Clone + Default> Model<M> {
    ///
    /// Constructs a [Model] from a [CpuModel], ie. constructs a list of [Gm]s with a [Mesh] as geometry (constructed from the [CpuMesh]es in the [CpuModel]) and
    /// a [material] type specified by the generic parameter which implement [FromCpuMaterial] (constructed from the [CpuMaterial]s in the [CpuModel]).
    ///
    pub fn new(context: &Context, cpu_model: &CpuModel) -> Result<Self, RendererError> {
        let materials = cpu_model
            .materials
            .iter()
            .map(|m| M::from_cpu_material(context, m))
            .collect::<Vec<_>>();
        let mut gms = Vec::new();
        let mut key_frames_indices = Vec::new();
        for part in cpu_model.parts.iter() {
            let material = if let Some(material_index) = part.material_index {
                materials
                    .get(material_index)
                    .ok_or(RendererError::MissingMaterial(
                        material_index.to_string(),
                        part.name.clone(),
                    ))?
                    .clone()
            } else {
                M::default()
            };
            gms.push(Gm {
                geometry: Mesh::new(context, &part.geometry),
                material,
            });
            key_frames_indices.push(part.key_frames_indices.clone().unwrap_or(Vec::new()));
        }
        Ok(Self {
            gms,
            animations: cpu_model.animations.clone(),
            key_frames_indices,
        })
    }
}

impl<M: Material> std::ops::Deref for Model<M> {
    type Target = Vec<Gm<Mesh, M>>;
    fn deref(&self) -> &Self::Target {
        &self.gms
    }
}

impl<M: Material> std::ops::DerefMut for Model<M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.gms
    }
}
