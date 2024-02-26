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
    animation: Option<Box<dyn Fn(f32) -> Mat4 + Send + Sync>>,
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
    pub fn set_animation(&mut self, animation: impl Fn(f32) -> Mat4 + Send + Sync + 'static) {
        self.animation = Some(Box::new(animation));
    }

    /// Returns the number of instances that is rendered.
    pub fn instance_count(&self) -> u32 {
        self.instances.count()
    }

    ///
    /// Update the instances.
    ///
    pub fn set_instances(&mut self, instances: &Instances) {
        #[cfg(debug_assertions)]
        instances.validate().expect("invalid instances");
        self.instances = instances.clone();
        self.update_aabb();

        self.update_instance_buffers(None);
    }

    fn update_aabb(&mut self) {
        let mut aabb = AxisAlignedBoundingBox::EMPTY;
        for transformation in self.instances.transformations.iter() {
            let mut aabb2 = self.aabb_local;
            aabb2.transform(&(transformation * self.transformation));
            aabb.expand_with_aabb(&aabb2);
        }
        self.aabb = aabb;
    }

    ///
    /// This function creates the instance buffers, ordering them by distance to the camera
    ///
    fn update_instance_buffers(&self, camera: Option<&Camera>) {
        let mut s = self.instance_buffers.write().unwrap();
        let indices = if let Some(position) = camera.map(|c| *c.position()) {
            s.1 = position;
            // Need to order by using the position.
            let distances = self
                .instances
                .transformations
                .iter()
                .map(|m| (self.transformation * m).w.truncate().distance2(position))
                .collect::<Vec<_>>();
            let mut indices = (0..self.instance_count() as usize).collect::<Vec<usize>>();
            indices.sort_by(|a, b| {
                distances[*b]
                    .partial_cmp(&distances[*a])
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
            indices
        } else {
            // No need to order, just return the indices as is.
            (0..self.instances.transformations.len()).collect::<Vec<usize>>()
        };

        // Next, we can compute the instance buffers with that ordering.
        let instance_buffers = &mut s.0;
        instance_buffers.clear();

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
                .map(|i| instance_colors[*i].to_linear_srgb())
                .collect::<Vec<_>>();
            instance_buffers.insert(
                "instance_color".to_string(),
                InstanceBuffer::new_with_data(&self.context, &ordered_instance_colors),
            );
        }
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
    fn draw(
        &self,
        camera: &Camera,
        program: &Program,
        render_states: RenderStates,
        attributes: FragmentAttributes,
    ) {
        // Check if we need a reorder, this only applies to transparent materials.
        if render_states.blend != Blend::Disabled
            && *camera.position() != self.instance_buffers.read().unwrap().1
        {
            self.update_instance_buffers(Some(camera));
        }

        let instance_buffers = &self.instance_buffers.read().unwrap().0;
        if attributes.normal && instance_buffers.contains_key("instance_translation") {
            if let Some(inverse) = self.current_transformation.invert() {
                program.use_uniform_if_required("normalMatrix", inverse.transpose());
            } else {
                // determinant is float zero
                return;
            }
        }
        program.use_uniform("viewProjection", camera.projection() * camera.view());
        program.use_uniform("modelMatrix", self.current_transformation);

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
            self.instance_count(),
        );
    }

    fn vertex_shader_source(&self, required_attributes: FragmentAttributes) -> String {
        let instance_buffers = &self.instance_buffers.read().unwrap().0;
        format!(
            "{}{}{}{}{}{}{}{}{}",
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
            if required_attributes.color && self.base_mesh.colors.is_some() {
                "#define USE_VERTEX_COLORS\n"
            } else {
                ""
            },
            if required_attributes.color && instance_buffers.contains_key("instance_color") {
                "#define USE_INSTANCE_COLORS\n"
            } else {
                ""
            },
            if instance_buffers.contains_key("instance_translation") {
                "#define USE_INSTANCE_TRANSLATIONS\n"
            } else {
                "#define USE_INSTANCE_TRANSFORMS\n"
            },
            if required_attributes.uv && instance_buffers.contains_key("tex_transform_row1") {
                "#define USE_INSTANCE_TEXTURE_TRANSFORMATION\n"
            } else {
                ""
            },
            include_str!("../../core/shared.frag"),
            include_str!("shaders/mesh.vert"),
        )
    }

    fn id(&self, required_attributes: FragmentAttributes) -> u16 {
        let instance_buffers = &self
            .instance_buffers
            .read()
            .expect("failed to acquire read access")
            .0;
        let mut id = 0b1u16 << 15 | 0b1u16 << 7;
        if required_attributes.normal {
            id |= 0b1u16;
        }
        if required_attributes.tangents {
            id |= 0b1u16 << 1;
        }
        if required_attributes.uv {
            id |= 0b1u16 << 2;
        }
        if required_attributes.color && self.base_mesh.colors.is_some() {
            id |= 0b1u16 << 3;
        }
        if required_attributes.color && instance_buffers.contains_key("instance_color") {
            id |= 0b1u16 << 4;
        }
        if instance_buffers.contains_key("instance_translation") {
            id |= 0b1u16 << 5;
        }
        if required_attributes.uv && instance_buffers.contains_key("tex_transform_row1") {
            id |= 0b1u16 << 6;
        }
        id
    }

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
        render_with_material(&self.context, camera, self, material, lights)
    }

    fn render_with_effect(
        &self,
        material: &dyn Effect,
        camera: &Camera,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) {
        render_with_effect(
            &self.context,
            camera,
            self,
            material,
            lights,
            color_texture,
            depth_texture,
        )
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
    pub colors: Option<Vec<Srgba>>,
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
