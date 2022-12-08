use crate::core::*;
use crate::renderer::*;
use std::collections::HashMap;
use std::sync::RwLock;

/// Internal struct to store and track the current state of the instance buffers.
struct InstanceBufferState {
    /// The actual instance buffers.
    instance_buffers: HashMap<String, InstanceBuffer>,

    /// Camera position for which the buffers are correct, only relevant if non opaque.
    camera_position: Vec3,

    /// Indicates buffers should be rewritten completely. Instances changed.
    is_dirty: bool,

    /// Indicates instances[0..instance_count] has at least one a != 255.
    instance_transparency: bool,

    /// The instance count for which buffers are correct, if material is opaque and there is an
    /// instance that is transparent, we need to recalculate the buffers.
    current_instance_count: u32,
}

impl Default for InstanceBufferState {
    fn default() -> Self {
        InstanceBufferState {
            instance_buffers: Default::default(),
            camera_position: vec3(0.0, 0.0, 0.0),
            is_dirty: true,
            instance_transparency: false,
            current_instance_count: 0,
        }
    }
}

#[derive(PartialEq, Eq)]
enum InstanceSorting {
    None,
    BackToFront,
    OpaqueFirstTransparentBackToFront,
}

///
/// Similar to [Mesh], except it is possible to render many instances of the same mesh efficiently.
///
pub struct InstancedMesh {
    context: Context,
    vertex_buffers: HashMap<String, VertexBuffer>,
    index_buffer: Option<ElementBuffer>,
    instance_state: RwLock<InstanceBufferState>,
    aabb_local: AxisAlignedBoundingBox,
    aabb: AxisAlignedBoundingBox,
    transformation: Mat4,
    instance_transforms: Vec<Mat4>,
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
            index_buffer: super::index_buffer_from_mesh(context, cpu_mesh),
            vertex_buffers: super::vertex_buffers_from_mesh(context, cpu_mesh),
            instance_state: RwLock::new(InstanceBufferState::default()),
            aabb,
            aabb_local: aabb.clone(),
            transformation: Mat4::identity(),
            instance_count: 0,
            instance_transforms: Vec::new(),
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
        self.update_aabb();
    }

    ///
    /// Get the texture transform applied to the uv coordinates of all of the instances.
    ///
    pub fn texture_transform(&self) -> &Mat3 {
        &self.texture_transform
    }

    ///
    /// Set the texture transform applied to the uv coordinates of all of the model instances.
    /// This is applied before the texture transform for each instance.
    ///
    pub fn set_texture_transform(&mut self, texture_transform: Mat3) {
        self.texture_transform = texture_transform;
    }

    /// Returns the number of instances that is rendered.
    pub fn instance_count(&self) -> u32 {
        self.instance_count
    }

    /// Use this if you only want to render instance 0 through to instance `instance_count`.
    /// This is the same as changing the instances using `set_instances`, except that for opaque materials (with opaque instances) it is faster since it doesn't update any buffers.
    /// `instance_count` will be set to the number of instances when they are defined by `set_instances`, so all instanced are rendered by default.
    pub fn set_instance_count(&mut self, instance_count: u32) {
        self.instance_count = instance_count.min(self.instance_transforms.len() as u32);
        self.update_aabb();
    }

    ///
    /// Update the instances.
    ///
    pub fn set_instances(&mut self, instances: &Instances) {
        // For code review; should this be here > I dev with --release, that hides this and then
        // it fails elsewhere.
        #[cfg(debug_assertions)]
        instances.validate().expect("invalid instances");
        self.instance_count = instances.count();
        self.instances = instances.clone();
        self.instance_transforms = instances.transformations.clone();

        self.instance_state
            .write()
            .expect("failed acquiring write accesss")
            .is_dirty = true;

        if let Some(ref colors) = self.instances.colors {
            let mut state = self
                .instance_state
                .write()
                .expect("failed acquiring write accesss");
            state.instance_transparency = colors.iter().any(|c| c.a != 255);
        }

        self.update_aabb();
    }

    fn update_aabb(&mut self) {
        let mut aabb = AxisAlignedBoundingBox::EMPTY;
        for i in 0..self.instance_count as usize {
            let mut aabb2 = self.aabb_local.clone();
            aabb2.transform(&(self.instance_transforms[i] * self.transformation));
            aabb.expand_with_aabb(&aabb2);
        }
        self.aabb = aabb;
    }

    fn update_instance_buffers(&self, camera: &Camera, is_material_transparent: bool) {
        let (is_dirty, camera_pos, instance_transparency, current_instance_count) = {
            let s = self
                .instance_state
                .read()
                .expect("failed acquiring read accesss");
            (
                s.is_dirty,
                s.camera_position,
                s.instance_transparency,
                s.current_instance_count,
            )
        };

        /*
            See: https://github.com/asny/three-d/pull/297#issuecomment-1340557016
            - If dirty; always update.
            - If material transparent; update if camera changed, or if instance count changed.
                => Order ALL back to front, regardless of alpha.
            - If material is not transparent.
                -> If not instance_transparency, even if count changed -> nothing to do, buffers ok.
                -> If instance_transparency, update if camera changed, or instance count changed.
                    => Order a==255 first, then transparency back to front.
        */
        let camera_changed = camera.position() != &camera_pos;
        let instance_count_changed = current_instance_count != self.instance_count;
        let should_update = is_dirty
            || if is_material_transparent {
                camera_changed || instance_count_changed
            } else {
                // Material is not transparent
                if !instance_transparency {
                    false // nothing to do, everything is opaque, can truncate buffers.
                } else {
                    // instance transparency, need to order with the camera.
                    camera_changed || instance_count_changed
                }
            };

        // Two sorting styles;
        // sort back to front, regardless of alpha
        // alpha=255 first, then back to front.
        let sorting = if is_material_transparent {
            InstanceSorting::BackToFront
        } else {
            // Material is not transparent, instances may be
            if instance_transparency {
                InstanceSorting::OpaqueFirstTransparentBackToFront
            } else {
                // all is opaque, use order as is.
                InstanceSorting::None
            }
        };

        if should_update {
            let mut state = self
                .instance_state
                .write()
                .expect("failed acquiring mutable access");
            state.instance_buffers = self.create_instance_buffers(camera, sorting);

            // State it is no longer dirty and update the state-tracking.
            state.is_dirty = false;
            state.camera_position = *camera.position();
            state.current_instance_count = self.instance_count;
        }
    }

    ///
    /// This function creates the instance buffers, ordering them by distance to the camera
    ///
    fn create_instance_buffers(
        &self,
        camera: &Camera,
        sorting: InstanceSorting,
    ) -> HashMap<String, InstanceBuffer> {

        let distances = ||{
            self.instance_transforms
                .iter()
                .map(|m| {
                    (self.transformation * m)
                        .w
                        .truncate()
                        .distance2(*camera.position())
                })
                .collect::<Vec<_>>()
        };

        let indices = match sorting {
            InstanceSorting::None => (0..self.instance_transforms.len()).collect::<Vec<usize>>(),
            InstanceSorting::BackToFront => {
                // First, create a vector of distances from the camera to each instance.
                let distances = distances();
                // Then, we can sort the indices based on those distances.
                let mut indices = (0..distances.len()).collect::<Vec<usize>>();
                indices.sort_by(|a, b| {
                    distances[*b]
                        .partial_cmp(&distances[*a])
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
                indices
            },
            InstanceSorting::OpaqueFirstTransparentBackToFront => {
                let distances = distances();
                // Now, if there exists alpha, we obtain it, otherwise assume opaque
                let opaque_mask = if let Some(ref colors) = self.instances.colors {
                    colors.iter().map(|v|{v.a == 255}).collect::<Vec<_>>()
                } else {
                    // Don't think we can get here, as we only end up in this branch if there is
                    // instance transparency, which can only happen if colors is populated.
                    vec![false; distances.len()]
                };
                // Then, we can sort the indices based on those distances.
                let mut indices = (0..distances.len()).collect::<Vec<usize>>();
                indices.sort_by(|a, b| {
                    // If both opaque, ordering is equal.
                    if opaque_mask[*a] && opaque_mask[*b] {
                        std::cmp::Ordering::Equal
                    } else if opaque_mask[*a] && !opaque_mask[*b] {
                        // a is opaque, b is not, a is less than b.
                        std::cmp::Ordering::Less
                    } else if !opaque_mask[*a] && opaque_mask[*b] {
                        // a is not opaque, b is, a is greater than b.
                        std::cmp::Ordering::Greater
                    } else {
                        // both are false, not opaque, order by distance.
                        distances[*b]
                            .partial_cmp(&distances[*a])
                            .unwrap_or(std::cmp::Ordering::Equal)
                    }
                });
                indices
            },

        };
        // Next, we can compute the instance buffers with that ordering.
        let mut instance_buffers: HashMap<String, InstanceBuffer> = Default::default();

        if indices
            .iter()
            .map(|i| self.instance_transforms[*i])
            .all(|t| Mat3::from_cols(t.x.truncate(), t.y.truncate(), t.z.truncate()).is_identity())
        {
            instance_buffers.insert(
                "instance_translation".to_string(),
                InstanceBuffer::new_with_data(
                    &self.context,
                    &self
                        .instance_transforms
                        .iter()
                        .map(|t| t.w.truncate())
                        .collect::<Vec<_>>(),
                ),
            );
        } else {
            let mut row1 = Vec::new();
            let mut row2 = Vec::new();
            let mut row3 = Vec::new();
            for transformation in indices.iter().map(|i| self.instance_transforms[*i]) {
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

        if let Some(texture_transforms) = &self.instances.texture_transforms {
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

    fn draw(&self, program: &Program, render_states: RenderStates, camera: &Camera) {
        let instance_buffers = &self
            .instance_state
            .read()
            .expect("failed acquiring mutable access")
            .instance_buffers;

        program.use_uniform("viewProjection", camera.projection() * camera.view());
        program.use_uniform("modelMatrix", &self.transformation);
        program.use_uniform_if_required("textureTransform", &self.texture_transform);
        program.use_uniform_if_required(
            "normalMatrix",
            &self.transformation.invert().unwrap().transpose(),
        );

        for attribute_name in ["position", "normal", "tangent", "color", "uv_coordinates"] {
            if program.requires_attribute(attribute_name) {
                program.use_vertex_attribute(
                    attribute_name,
                    self.vertex_buffers
                        .get(attribute_name).expect(&format!("the render call requires the {} vertex buffer which is missing on the given geometry", attribute_name))
                );
            }
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
                    .get(attribute_name).expect(&format!("the render call requires the {} instance buffer which is missing on the given geometry", attribute_name))
                );
            }
        }

        if let Some(ref index_buffer) = self.index_buffer {
            program.draw_elements_instanced(
                render_states,
                camera.viewport(),
                index_buffer,
                self.instance_count,
            )
        } else {
            program.draw_arrays_instanced(
                render_states,
                camera.viewport(),
                self.vertex_buffers.get("position").unwrap().vertex_count() as u32,
                self.instance_count,
            )
        }
    }

    fn vertex_shader_source(
        &self,
        fragment_shader_source: &str,
        instance_buffers: &HashMap<String, InstanceBuffer>,
    ) -> String {
        let use_positions = fragment_shader_source.find("in vec3 pos;").is_some();
        let use_normals = fragment_shader_source.find("in vec3 nor;").is_some();
        let use_tangents = fragment_shader_source.find("in vec3 tang;").is_some();
        let use_uvs = fragment_shader_source.find("in vec2 uvs;").is_some();
        let use_colors = fragment_shader_source.find("in vec4 col;").is_some();
        format!(
            "{}{}{}{}{}{}{}{}{}",
            if instance_buffers.contains_key("instance_translation") {
                "#define USE_INSTANCE_TRANSLATIONS\n"
            } else {
                "#define USE_INSTANCE_TRANSFORMS\n"
            },
            if use_positions {
                "#define USE_POSITIONS\n"
            } else {
                ""
            },
            if use_normals {
                "#define USE_NORMALS\n"
            } else {
                ""
            },
            if use_tangents {
                if fragment_shader_source.find("in vec3 bitang;").is_none() {
                    panic!("if the fragment shader defined 'in vec3 tang' it also needs to define 'in vec3 bitang'");
                }
                "#define USE_TANGENTS\n"
            } else {
                ""
            },
            if use_uvs { "#define USE_UVS\n" } else { "" },
            if use_colors {
                if instance_buffers.contains_key("instance_color")
                    && self.vertex_buffers.contains_key("color")
                {
                    "#define USE_COLORS\n#define USE_VERTEX_COLORS\n#define USE_INSTANCE_COLORS\n"
                } else if instance_buffers.contains_key("instance_color") {
                    "#define USE_COLORS\n#define USE_INSTANCE_COLORS\n"
                } else {
                    "#define USE_COLORS\n#define USE_VERTEX_COLORS\n"
                }
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
        self.aabb
    }

    fn render_with_material(
        &self,
        material: &dyn Material,
        camera: &Camera,
        lights: &[&dyn Light],
    ) {
        // Update the instance buffers if required.
        self.update_instance_buffers(
            camera,
            material.material_type() == MaterialType::Transparent,
        );
        let instance_buffers = &self
            .instance_state
            .read()
            .expect("failed to acquire read access")
            .instance_buffers;

        let fragment_shader_source = material.fragment_shader_source(
            self.vertex_buffers.contains_key("color")
                || instance_buffers.contains_key("instance_color"),
            lights,
        );
        self.context
            .program(
                &self.vertex_shader_source(&fragment_shader_source, &instance_buffers),
                &fragment_shader_source,
                |program| {
                    material.use_uniforms(program, camera, lights);
                    self.draw(program, material.render_states(), camera);
                },
            )
            .expect("Failed compiling shader")
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
        let is_material_transparent = false; // is this correct? PostMaterial doesn't provide this.
        self.update_instance_buffers(camera, is_material_transparent);
        let instance_buffers = &self
            .instance_state
            .read()
            .expect("failed to acquire read access")
            .instance_buffers;

        let fragment_shader_source =
            material.fragment_shader_source(lights, color_texture, depth_texture);
        self.context
            .program(
                &self.vertex_shader_source(&fragment_shader_source, &instance_buffers),
                &fragment_shader_source,
                |program| {
                    material.use_uniforms(program, camera, lights, color_texture, depth_texture);
                    self.draw(program, material.render_states(), camera);
                },
            )
            .expect("Failed compiling shader")
    }
}

///
/// Defines the attributes for the instances of the model defined in [InstancedMesh] or [InstancedModel].
///
/// Each list of attributes must contain the same number of elements as the number of instances.
/// The attributes are applied to each instance before they are rendered.
/// The translation, rotation and scale is applied after the transformation applied to all instances (see [InstancedMesh::set_transformation]).
/// The texture transform is also applied after the texture transform applied to all instances (see [InstancedMesh::set_texture_transform]).
///
#[derive(Clone, Debug, Default)]
pub struct Instances {
    /// The transformations applied to each instance.
    pub transformations: Vec<Mat4>,
    /// The texture transform applied to the uv coordinates of each instance.
    pub texture_transforms: Option<Vec<Mat3>>,
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
            self.texture_transforms.as_ref().map(|b| b.len()),
            "texture transforms",
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
                .map(|p| Mat4::from_translation(p))
                .collect(),
            colors: points.colors,
            ..Default::default()
        }
    }
}
