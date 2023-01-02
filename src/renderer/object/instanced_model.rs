use crate::renderer::*;

///
/// Part of an [InstancedModel] consisting of a [InstancedMesh], some type of [material] and a set of possible animations.
///
pub struct InstancedModelPart<M: Material> {
    gm: Gm<InstancedMesh, M>,
    animations: Vec<KeyFrameAnimation>,
}

impl<M: Material> InstancedModelPart<M> {
    ///
    /// Returns a list of unique names for the animations for this model part. Use these names as input to [Self::choose_animation].
    ///
    pub fn animations(&self) -> Vec<Option<String>> {
        self.animations
            .iter()
            .map(|animation| animation.name.clone())
            .collect()
    }

    ///
    /// Specifies the animation to use when [Geometry::animate] is called. Use the [Self::animations] method to get a list of possible animations.
    ///
    pub fn choose_animation(&mut self, animation_name: Option<&str>) {
        if let Some(animation) = self
            .animations
            .iter()
            .find(|a| animation_name == a.name.as_deref())
            .cloned()
        {
            self.set_animation(move |time| animation.transformation(time));
        }
    }
}

impl<M: Material> std::ops::Deref for InstancedModelPart<M> {
    type Target = Gm<InstancedMesh, M>;
    fn deref(&self) -> &Self::Target {
        &self.gm
    }
}

impl<M: Material> std::ops::DerefMut for InstancedModelPart<M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.gm
    }
}

impl<M: Material> Geometry for InstancedModelPart<M> {
    fn render_with_material(
        &self,
        material: &dyn Material,
        camera: &Camera,
        lights: &[&dyn Light],
    ) {
        self.gm.render_with_material(material, camera, lights)
    }

    fn render_with_post_material(
        &self,
        material: &dyn PostMaterial,
        camera: &Camera,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) {
        self.gm
            .render_with_post_material(material, camera, lights, color_texture, depth_texture)
    }

    fn aabb(&self) -> AxisAlignedBoundingBox {
        self.gm.aabb()
    }
    fn animate(&mut self, time: f32) {
        self.gm.animate(time)
    }
}

impl<M: Material> Object for InstancedModelPart<M> {
    fn render(&self, camera: &Camera, lights: &[&dyn Light]) {
        self.gm.render(camera, lights)
    }

    fn material_type(&self) -> MaterialType {
        self.gm.material_type()
    }
}

impl<'a, M: Material> IntoIterator for &'a InstancedModelPart<M> {
    type Item = &'a dyn Object;
    type IntoIter = std::iter::Once<&'a dyn Object>;

    fn into_iter(self) -> Self::IntoIter {
        self.gm.into_iter()
    }
}

///
/// Similar to [Model], except it is possible to render many instances of the same model efficiently.
///
pub struct InstancedModel<M: Material>(Vec<InstancedModelPart<M>>);

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
        let mut gms = Vec::new();
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
                let mut gm = Gm {
                    geometry: InstancedMesh::new(context, instances, geometry),
                    material,
                };
                gm.set_transformation(primitive.transformation);
                gms.push(InstancedModelPart {
                    gm,
                    animations: primitive.animations.clone(),
                });
            }
        }
        let mut model = Self(gms);
        if let Some(animation_name) = model.animations().first().cloned() {
            model.choose_animation(animation_name.as_deref());
        }
        Ok(model)
    }

    ///
    /// Returns a list of unique names for the animations in this model. Use these names as input to [Self::choose_animation].
    ///
    pub fn animations(&self) -> Vec<Option<String>> {
        let mut set = std::collections::HashSet::new();
        for model_part in self.0.iter() {
            set.extend(model_part.animations());
        }
        set.into_iter().collect()
    }

    ///
    /// Specifies the animation to use when [Geometry::animate] is called. Use the [Self::animations] method to get a list of possible animations.
    ///
    pub fn choose_animation(&mut self, animation_name: Option<&str>) {
        for part in self.0.iter_mut() {
            part.choose_animation(animation_name);
        }
    }

    ///
    /// For updating the animation. The time parameter should be some continious time, for example the time since start.
    ///
    pub fn animate(&mut self, time: f32) {
        self.iter_mut().for_each(|m| m.animate(time));
    }
}

impl<M: Material> std::ops::Deref for InstancedModel<M> {
    type Target = Vec<InstancedModelPart<M>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<M: Material> std::ops::DerefMut for InstancedModel<M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
