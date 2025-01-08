use crate::core::*;
use crate::renderer::*;
use std::sync::RwLock;

use super::BaseMesh;

///
/// Similar to [Mesh], except it is possible to render many instances of the same mesh efficiently.
///
pub struct InstancedMesh {
    context: Context,
    base_mesh: BaseMesh,
    transform: RwLock<(
        InstanceBuffer<Vec4>,
        InstanceBuffer<Vec4>,
        InstanceBuffer<Vec4>,
    )>,
    tex_transform: RwLock<Option<(InstanceBuffer<Vec3>, InstanceBuffer<Vec3>)>>,
    instance_color: RwLock<Option<InstanceBuffer<Vec4>>>,
    last_camera_position: RwLock<Vec3>,
    aabb: AxisAlignedBoundingBox, // The AABB for the base mesh without transformations applied
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
            transform: RwLock::new((
                InstanceBuffer::<Vec4>::new(context),
                InstanceBuffer::<Vec4>::new(context),
                InstanceBuffer::<Vec4>::new(context),
            )),
            tex_transform: RwLock::new(None),
            instance_color: RwLock::new(None),
            last_camera_position: RwLock::new(vec3(0.0, 0.0, 0.0)),
            aabb,
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

        self.update_instance_buffers(None);
    }

    ///
    /// This function creates the instance buffers, ordering them by distance to the camera
    ///
    fn update_instance_buffers(&self, viewer: Option<&dyn Viewer>) {
        let indices = if let Some(position) = viewer.map(|c| c.position()) {
            *self.last_camera_position.write().unwrap() = position;
            // Need to order by using the position.
            let distances = self
                .instances
                .transformations
                .iter()
                .map(|m| {
                    (self.current_transformation * m)
                        .w
                        .truncate()
                        .distance2(position)
                })
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
        let mut row1 = Vec::new();
        let mut row2 = Vec::new();
        let mut row3 = Vec::new();
        for transformation in indices.iter().map(|i| self.instances.transformations[*i]) {
            row1.push(transformation.row(0));
            row2.push(transformation.row(1));
            row3.push(transformation.row(2));
        }

        *self.transform.write().unwrap() = (
            InstanceBuffer::new_with_data(&self.context, &row1),
            InstanceBuffer::new_with_data(&self.context, &row2),
            InstanceBuffer::new_with_data(&self.context, &row3),
        );

        *self.tex_transform.write().unwrap() =
            self.instances
                .texture_transformations
                .as_ref()
                .map(|texture_transforms| {
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
                    (
                        InstanceBuffer::new_with_data(&self.context, &instance_tex_transform1),
                        InstanceBuffer::new_with_data(&self.context, &instance_tex_transform2),
                    )
                });
        *self.instance_color.write().unwrap() =
            self.instances.colors.as_ref().map(|instance_colors| {
                let ordered_instance_colors = indices
                    .iter()
                    .map(|i| instance_colors[*i].to_linear_srgb())
                    .collect::<Vec<_>>();
                InstanceBuffer::new_with_data(&self.context, &ordered_instance_colors)
            });
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
    fn draw(&self, viewer: &dyn Viewer, program: &Program, render_states: RenderStates) {
        // Check if we need a reorder, this only applies to transparent materials.
        if render_states.blend != Blend::Disabled
            && viewer.position() != *self.last_camera_position.read().unwrap()
        {
            self.update_instance_buffers(Some(viewer));
        }

        program.use_uniform("viewProjection", viewer.projection() * viewer.view());
        program.use_uniform("modelMatrix", self.current_transformation);

        let (row1, row2, row3) = &*self.transform.read().unwrap();
        program.use_instance_attribute("row1", row1);
        program.use_instance_attribute("row2", row2);
        program.use_instance_attribute("row3", row3);

        if program.requires_attribute("tex_transform_row1") {
            if let Some((row1, row2)) = &*self.tex_transform.read().unwrap() {
                program.use_instance_attribute("tex_transform_row1", row1);
                program.use_instance_attribute("tex_transform_row2", row2);
            }
        }

        if program.requires_attribute("instance_color") {
            if let Some(color) = &*self.instance_color.read().unwrap() {
                program.use_instance_attribute("instance_color", color);
            }
        }

        self.base_mesh
            .draw_instanced(program, render_states, viewer, self.instance_count());
    }

    fn vertex_shader_source(&self) -> String {
        format!(
            "#define USE_INSTANCE_TRANSFORMS\n{}{}{}",
            if self.instance_color.read().unwrap().is_some() {
                "#define USE_INSTANCE_COLORS\n"
            } else {
                ""
            },
            if self.tex_transform.read().unwrap().is_some() {
                "#define USE_INSTANCE_TEXTURE_TRANSFORMATION\n"
            } else {
                ""
            },
            self.base_mesh.vertex_shader_source()
        )
    }

    fn id(&self) -> GeometryId {
        GeometryId::InstancedMesh(
            self.base_mesh.normals.is_some(),
            self.base_mesh.tangents.is_some(),
            self.base_mesh.uvs.is_some(),
            self.base_mesh.colors.is_some(),
            self.instance_color.read().unwrap().is_some(),
            self.tex_transform.read().unwrap().is_some(),
        )
    }

    fn aabb(&self) -> AxisAlignedBoundingBox {
        let mut aabb = AxisAlignedBoundingBox::EMPTY;
        let local_aabb = self.aabb.transformed(self.current_transformation);
        for transformation in &self.instances.transformations {
            aabb.expand_with_aabb(local_aabb.transformed(*transformation));
        }
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
        viewer: &dyn Viewer,
        lights: &[&dyn Light],
    ) {
        render_with_material(&self.context, viewer, self, material, lights)
    }

    fn render_with_effect(
        &self,
        material: &dyn Effect,
        viewer: &dyn Viewer,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) {
        render_with_effect(
            &self.context,
            viewer,
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
