use crate::core::*;
use crate::renderer::*;
use std::collections::HashMap;
use std::sync::RwLock;

use super::BaseMesh;

///
/// Similar to [Mesh], except it is possible to render many instances of the same mesh efficiently.
///
pub struct InstancedMesh {
    context: Context,
    base_mesh: BaseMesh,
    instance_buffers: RwLock<(HashMap<String, InstanceBuffer>, Vec3)>,
    aabb: AxisAlignedBoundingBox,
    aabb_local: AxisAlignedBoundingBox,
    transformation: Mat4,
    current_transformation: Mat4,
    animation: Option<Box<dyn Fn(f32) -> Mat4>>,
    instance_count: u32,
    texture_transform: Mat3,
    instances: Instances,
}

impl InstancedMesh {
    ///
    /// Creates a new instanced 3D mesh from the given [CpuMesh].
    /// All data in the [CpuMesh] is transfered to the GPU, so make sure to remove all unnecessary data from the [CpuMesh] before calling this method.
    /// The model is rendered in as many instances as there are attributes in [Instances] given as input.
    ///
    pub fn new(context: &Context, instances: &Instances, cpu_mesh: &CpuMesh) -> Self {
        let aabb = cpu_mesh.compute_aabb();
        let mut instanced_mesh = Self {
            context: context.clone(),
            base_mesh: BaseMesh::new(context, cpu_mesh),
            instance_buffers: RwLock::new((Default::default(), vec3(0.0, 0.0, 0.0))),
            aabb,
            aabb_local: aabb,
            transformation: Mat4::identity(),
            current_transformation: Mat4::identity(),
            animation: None,
            instance_count: 0,
            texture_transform: Mat3::identity(),
            instances: instances.clone(),
        };
        instanced_mesh.set_instances(instances);
        instanced_mesh
    }

    ///
    /// Returns the local to world transformation applied to all instances.
    ///
    pub fn transformation(&self) -> Mat4 {
        self.transformation
    }

    ///
    /// Set the local to world transformation applied to all instances.
    /// This is applied before the transform for each instance.
    ///
    pub fn set_transformation(&mut self, transformation: Mat4) {
        self.transformation = transformation;
        self.current_transformation = transformation;
    }

    ///
    /// Specifies a function which takes a time parameter as input and returns a transformation that should be applied to this mesh at the given time.
    /// To actually animate this instanced mesh, call [Geometry::animate] at each frame which in turn evaluates the animation function defined by this method.
    /// This transformation is applied first, then the local to world transformation defined by [Self::set_transformation].
    ///
    pub fn set_animation(&mut self, animation: impl Fn(f32) -> Mat4 + 'static) {
        self.animation = Some(Box::new(animation));
    }

    ///
    /// Get the texture transform applied to the uv coordinates of all of the instances.
    ///
    #[deprecated]
    pub fn texture_transform(&self) -> &Mat3 {
        &self.texture_transform
    }

    ///
    /// Set the texture transform applied to the uv coordinates of all of the model instances.
    /// This is applied before the texture transform for each instance.
    ///
    #[deprecated = "Set the texture transformation of Texture2DRef for a material instead"]
    pub fn set_texture_transform(&mut self, texture_transform: Mat3) {
        self.texture_transform = texture_transform;
    }

    /// Returns the number of instances that is rendered.
    pub fn instance_count(&self) -> u32 {
        self.instance_count
    }

    /// Use this if you only want to render instance 0 through to instance `instance_count`.
    /// This is the same as changing the instances using `set_instances`, except that it is faster since it doesn't update any buffers.
    /// `instance_count` will be set to the number of instances when they are defined by `set_instances`, so all instanced are rendered by default.
    pub fn set_instance_count(&mut self, instance_count: u32) {
        self.instance_count = instance_count.min(self.instances.transformations.len() as u32);
        self.update_aabb();
    }

    ///
    /// Update the instances.
    ///
    pub fn set_instances(&mut self, instances: &Instances) {
        #[cfg(debug_assertions)]
        instances.validate().expect("invalid instances");
        self.instance_count = instances.count();
        self.instances = instances.clone();
        self.update_aabb();

        {
            let mut s = self
                .instance_buffers
                .write()
                .expect("failed acquiring write accesss");
            s.0.clear();
        }
    }

    fn update_aabb(&mut self) {
        let mut aabb = AxisAlignedBoundingBox::EMPTY;
        for i in 0..self.instance_count as usize {
            let mut aabb2 = self.aabb_local;
            aabb2.transform(&(self.instances.transformations[i] * self.transformation));
            aabb.expand_with_aabb(&aabb2);
        }
        self.aabb = aabb;
    }

    /// Update the instance buffers, if depth_ordering_pose is populated depth ordering is performed
    /// using this position.
    fn update_instance_buffers(&self, depth_ordering_pose: Option<Vec3>) {
        let needs_update = {
            let s = self
                .instance_buffers
                .read()
                .expect("failed acquiring read accesss");

            // Check if we need a reorder, this only applies to transparent materials.
            let reorder_needed = if let Some(ref ordering_pose) = depth_ordering_pose {
                let camera_changed = *ordering_pose != s.1;
                let instance_count_changed = if let Some(v) = s.0.values().next() {
                    v.instance_count() != self.instance_count
                } else {
                    false
                };

                camera_changed || instance_count_changed
            } else {
                false
            };

            // Update is always needed if the instance buffers is empty; Opaque materials only.
            // Or, for transparent materials, if the camera moved or if the instance count changed.
            s.0.is_empty() || reorder_needed
        };

        if needs_update {
            let mut s = self
                .instance_buffers
                .write()
                .expect("failed acquiring mutable access");
            s.0 = self.create_instance_buffers(depth_ordering_pose);
            if let Some(ordering_pose) = depth_ordering_pose {
                s.1 = ordering_pose;
            }
        }
    }

    /// Sort function to order the indices up to instance count by their distance, back to front.
    fn ordered_indices_back_to_front(instance_count: usize, distances: &[f32]) -> Vec<usize> {
        // Then, we can sort the indices based on those distances.
        let mut indices = (0..instance_count).collect::<Vec<usize>>();
        indices.sort_by(|a, b| {
            distances[*b]
                .partial_cmp(&distances[*a])
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        indices
    }

    ///
    /// This function creates the instance buffers, ordering them by distance to the camera
    ///
    fn create_instance_buffers(
        &self,
        depth_ordering: Option<Vec3>,
    ) -> HashMap<String, InstanceBuffer> {
        let indices = if let Some(position) = depth_ordering {
            // Need to order by using the position.
            let distances = self
                .instances
                .transformations
                .iter()
                .map(|m| (self.transformation * m).w.truncate().distance2(position))
                .collect::<Vec<_>>();
            Self::ordered_indices_back_to_front(self.instance_count as usize, &distances)
        } else {
            // No need to order, just return the indices as is.
            (0..self.instances.transformations.len()).collect::<Vec<usize>>()
        };

        // Next, we can compute the instance buffers with that ordering.
        let mut instance_buffers: HashMap<String, InstanceBuffer> = Default::default();

        if indices
            .iter()
            .map(|i| self.instances.transformations[*i])
            .all(|t| Mat3::from_cols(t.x.truncate(), t.y.truncate(), t.z.truncate()).is_identity())
        {
            instance_buffers.insert(
                "instance_translation".to_string(),
                InstanceBuffer::new_with_data(
                    &self.context,
                    &indices
                        .iter()
                        .map(|i| self.instances.transformations[*i])
                        .map(|t| t.w.truncate())
                        .collect::<Vec<_>>(),
                ),
            );
        } else {
            let mut row1 = Vec::new();
            let mut row2 = Vec::new();
            let mut row3 = Vec::new();
            for transformation in indices.iter().map(|i| self.instances.transformations[*i]) {
                row1.push(transformation.row(0));
                row2.push(transformation.row(1));
                row3.push(transformation.row(2));
            }

            instance_buffers.insert(
                "row1".to_string(),
                InstanceBuffer::new_with_data(&self.context, &row1),
            );
            instance_buffers.insert(
                "row2".to_string(),
                InstanceBuffer::new_with_data(&self.context, &row2),
            );
            instance_buffers.insert(
                "row3".to_string(),
                InstanceBuffer::new_with_data(&self.context, &row3),
            );
        }

        if let Some(texture_transforms) = &self.instances.texture_transformations {
            let mut instance_tex_transform1 = Vec::new();
            let mut instance_tex_transform2 = Vec::new();
            for texture_transform in indices.iter().map(|i| texture_transforms[*i]) {
                instance_tex_transform1.push(vec3(
                    texture_transform.x.x,
                    texture_transform.y.x,
                    texture_transform.z.x,
                ));
                instance_tex_transform2.push(vec3(
                    texture_transform.x.y,
                    texture_transform.y.y,
                    texture_transform.z.y,
                ));
            }
            instance_buffers.insert(
                "tex_transform_row1".to_string(),
                InstanceBuffer::new_with_data(&self.context, &instance_tex_transform1),
            );
            instance_buffers.insert(
                "tex_transform_row2".to_string(),
                InstanceBuffer::new_with_data(&self.context, &instance_tex_transform2),
            );
        }
        if let Some(instance_colors) = &self.instances.colors {
            // Create the re-ordered color buffer by depth.
            let ordered_instance_colors = indices
                .iter()
                .map(|i| instance_colors[*i])
                .collect::<Vec<Color>>();
            instance_buffers.insert(
                "instance_color".to_string(),
                InstanceBuffer::new_with_data(&self.context, &ordered_instance_colors),
            );
        }
        instance_buffers
    }

    fn draw(
        &self,
        program: &Program,
        render_states: RenderStates,
        camera: &Camera,
        attributes: FragmentAttributes,
        instance_buffers: &HashMap<String, InstanceBuffer>,
    ) {
        if attributes.normal && instance_buffers.contains_key("instance_translation") {
            if let Some(inverse) = self.current_transformation.invert() {
                program.use_uniform("normalMatrix", inverse.transpose());
            } else {
                // determinant is float zero
                return;
            }
        }
        program.use_uniform("viewProjection", camera.projection() * camera.view());
        program.use_uniform("modelMatrix", self.current_transformation);

        if attributes.uv {
            program.use_uniform("textureTransform", self.texture_transform);
        }

        for attribute_name in [
            "instance_translation",
            "row1",
            "row2",
            "row3",
            "tex_transform_row1",
            "tex_transform_row2",
            "instance_color",
        ] {
            if program.requires_attribute(attribute_name) {
                program.use_instance_attribute(
                    attribute_name,
                    instance_buffers
                    .get(attribute_name).unwrap_or_else(|| panic!("the render call requires the {} instance buffer which is missing on the given geometry", attribute_name))
                );
            }
        }
        self.base_mesh.draw_instanced(
            program,
            render_states,
            camera,
            attributes,
            self.instance_count,
        );
    }

    fn vertex_shader_source(
        &self,
        required_attributes: FragmentAttributes,
        instance_buffers: &HashMap<String, InstanceBuffer>,
    ) -> String {
        format!(
            "{}{}{}{}{}{}{}{}",
            if instance_buffers.contains_key("instance_translation") {
                "#define USE_INSTANCE_TRANSLATIONS\n"
            } else {
                "#define USE_INSTANCE_TRANSFORMS\n"
            },
            if required_attributes.normal {
                "#define USE_NORMALS\n"
            } else {
                ""
            },
            if required_attributes.tangents {
                "#define USE_TANGENTS\n"
            } else {
                ""
            },
            if required_attributes.uv {
                "#define USE_UVS\n"
            } else {
                ""
            },
            if instance_buffers.contains_key("instance_color") && self.base_mesh.colors.is_some() {
                "#define USE_VERTEX_COLORS\n#define USE_INSTANCE_COLORS\n"
            } else if instance_buffers.contains_key("instance_color") {
                "#define USE_INSTANCE_COLORS\n"
            } else if self.base_mesh.colors.is_some() {
                "#define USE_VERTEX_COLORS\n"
            } else {
                ""
            },
            if instance_buffers.contains_key("tex_transform_row1") {
                "#define USE_INSTANCE_TEXTURE_TRANSFORMATION\n"
            } else {
                ""
            },
            include_str!("../../core/shared.frag"),
            include_str!("shaders/mesh.vert"),
        )
    }
}

impl<'a> IntoIterator for &'a InstancedMesh {
    type Item = &'a dyn Geometry;
    type IntoIter = std::iter::Once<&'a dyn Geometry>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self)
    }
}

impl Geometry for InstancedMesh {
    fn aabb(&self) -> AxisAlignedBoundingBox {
        let mut aabb = self.aabb;
        aabb.transform(&self.current_transformation);
        aabb
    }

    fn animate(&mut self, time: f32) {
        if let Some(animation) = &self.animation {
            self.current_transformation = self.transformation * animation(time);
        }
    }

    fn render_with_material(
        &self,
        material: &dyn Material,
        camera: &Camera,
        lights: &[&dyn Light],
    ) {
        // Update the instance buffers if required.
        let update_pose = if material.material_type() == MaterialType::Transparent {
            Some(*camera.position())
        } else {
            None
        };

        self.update_instance_buffers(update_pose);
        let instance_buffers = &self
            .instance_buffers
            .read()
            .expect("failed to acquire read access")
            .0;

        let fragment_shader = material.fragment_shader(lights);
        let vertex_shader_source =
            self.vertex_shader_source(fragment_shader.attributes, instance_buffers);
        self.context
            .program(vertex_shader_source, fragment_shader.source, |program| {
                material.use_uniforms(program, camera, lights);
                self.draw(
                    program,
                    material.render_states(),
                    camera,
                    fragment_shader.attributes,
                    instance_buffers,
                );
            })
            .expect("Failed compiling shader");
    }

    fn render_with_post_material(
        &self,
        material: &dyn PostMaterial,
        camera: &Camera,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) {
        // Update the instance buffers if required.
        let update_pose = if material.material_type() == MaterialType::Transparent {
            Some(*camera.position())
        } else {
            None
        };

        self.update_instance_buffers(update_pose);
        let instance_buffers = &self
            .instance_buffers
            .read()
            .expect("failed to acquire read access")
            .0;

        let fragment_shader = material.fragment_shader(lights, color_texture, depth_texture);
        let vertex_shader_source =
            self.vertex_shader_source(fragment_shader.attributes, instance_buffers);
        self.context
            .program(vertex_shader_source, fragment_shader.source, |program| {
                material.use_uniforms(program, camera, lights, color_texture, depth_texture);
                self.draw(
                    program,
                    material.render_states(),
                    camera,
                    fragment_shader.attributes,
                    instance_buffers,
                );
            })
            .expect("Failed compiling shader");
    }
}

///
/// Defines the attributes for the instances of the model defined in [InstancedMesh] or [InstancedModel].
///
/// Each list of attributes must contain the same number of elements as the number of instances.
/// The attributes are applied to each instance before they are rendered.
/// The [Instances::transformations] are applied after the transformation applied to all instances (see [InstancedMesh::set_transformation]).
///
#[derive(Clone, Debug, Default)]
pub struct Instances {
    /// The transformations applied to each instance.
    pub transformations: Vec<Mat4>,
    /// The texture transform applied to the uv coordinates of each instance.
    pub texture_transformations: Option<Vec<Mat3>>,
    /// Colors multiplied onto the base color of each instance.
    pub colors: Option<Vec<Color>>,
}

impl Instances {
    ///
    /// Returns an error if the instances is not valid.
    ///
    pub fn validate(&self) -> Result<(), RendererError> {
        let instance_count = self.count();
        let buffer_check = |length: Option<usize>, name: &str| -> Result<(), RendererError> {
            if let Some(length) = length {
                if length < instance_count as usize {
                    Err(RendererError::InvalidBufferLength(
                        name.to_string(),
                        instance_count as usize,
                        length,
                    ))?;
                }
            }
            Ok(())
        };

        buffer_check(
            self.texture_transformations.as_ref().map(|b| b.len()),
            "texture transformations",
        )?;
        buffer_check(Some(self.transformations.len()), "transformations")?;
        buffer_check(self.colors.as_ref().map(|b| b.len()), "colors")?;

        Ok(())
    }

    /// Returns the number of instances.
    pub fn count(&self) -> u32 {
        self.transformations.len() as u32
    }
}

impl From<PointCloud> for Instances {
    fn from(points: PointCloud) -> Self {
        Self {
            transformations: points
                .positions
                .to_f32()
                .into_iter()
                .map(Mat4::from_translation)
                .collect(),
            colors: points.colors,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_ordered_indices_back_to_front() {
        let distances = vec![10.0, 5.0, 5.1, 3.0, 1.0];
        let res = InstancedMesh::ordered_indices_back_to_front(distances.len(), &distances);
        assert_eq!(res, vec![0, 2, 1, 3, 4]);
    }
}
