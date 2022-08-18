use crate::core::*;
use crate::renderer::*;
use std::collections::HashMap;

pub use three_d_asset::PointCloud as Points;

///
/// Similar to [Mesh], except it is possible to render many instances of the same mesh efficiently.
///
pub struct InstancedMesh {
    context: Context,
    vertex_buffers: HashMap<String, VertexBuffer>,
    instance_buffers: HashMap<String, InstanceBuffer>,
    index_buffer: Option<ElementBuffer>,
    aabb_local: AxisAlignedBoundingBox,
    aabb: AxisAlignedBoundingBox,
    transformation: Mat4,
    instance_transforms: Vec<Mat4>,
    instance_count: u32,
    texture_transform: Mat3,
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
            instance_buffers: HashMap::new(),
            aabb,
            aabb_local: aabb.clone(),
            transformation: Mat4::identity(),
            instance_count: 0,
            instance_transforms: Vec::new(),
            texture_transform: Mat3::identity(),
        };
        instanced_mesh.set_instances(instances);
        instanced_mesh
    }

    ///
    /// Creates a set of point geometries from the given [Points] data. Each point in the [Points] data is represented by the given [CpuMesh].
    /// The resulting geometry can then be visualized with any [Material].
    /// All data in [Points] is transfered to the GPU, so make sure to remove all unnecessary data from [Points] before calling this method.
    ///
    pub fn new_points(context: &Context, points: Points, point_mesh: &CpuMesh) -> Self {
        let instances = Instances {
            translations: points.positions.to_f32(),
            colors: points.colors.clone(),
            ..Default::default()
        };
        Self::new(context, &instances, point_mesh)
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
    /// This is the same as changing the instances using `set_instances`, except that it is faster since it doesn't update any buffers.
    /// `instance_count` will be set to the number of instances when they are defined by `set_instances`, so all instanced are rendered by default.
    pub fn set_instance_count(&mut self, instance_count: u32) {
        self.instance_count = instance_count.min(self.instance_transforms.len() as u32);
        self.update_aabb();
    }

    ///
    /// Update the instances.
    ///
    pub fn set_instances(&mut self, instances: &Instances) {
        #[cfg(debug_assertions)]
        instances.validate().expect("invalid instances");
        self.instance_count = instances.count();
        self.instance_buffers.clear();
        self.instance_transforms = (0..self.instance_count as usize)
            .map(|i| {
                Mat4::from_translation(instances.translations[i])
                    * instances
                        .rotations
                        .as_ref()
                        .map(|r| Mat4::from(r[i]))
                        .unwrap_or(Mat4::identity())
                    * instances
                        .scales
                        .as_ref()
                        .map(|s| Mat4::from_nonuniform_scale(s[i].x, s[i].y, s[i].z))
                        .unwrap_or(Mat4::identity())
            })
            .collect::<Vec<_>>();

        if instances.rotations.is_none() && instances.scales.is_none() {
            self.instance_buffers.insert(
                "instance_translation".to_string(),
                InstanceBuffer::new_with_data(&self.context, &instances.translations),
            );
        } else {
            let mut row1 = Vec::new();
            let mut row2 = Vec::new();
            let mut row3 = Vec::new();
            for geometry_transform in self.instance_transforms.iter() {
                row1.push(vec4(
                    geometry_transform.x.x,
                    geometry_transform.y.x,
                    geometry_transform.z.x,
                    geometry_transform.w.x,
                ));

                row2.push(vec4(
                    geometry_transform.x.y,
                    geometry_transform.y.y,
                    geometry_transform.z.y,
                    geometry_transform.w.y,
                ));

                row3.push(vec4(
                    geometry_transform.x.z,
                    geometry_transform.y.z,
                    geometry_transform.z.z,
                    geometry_transform.w.z,
                ));
            }

            self.instance_buffers.insert(
                "row1".to_string(),
                InstanceBuffer::new_with_data(&self.context, &row1),
            );
            self.instance_buffers.insert(
                "row2".to_string(),
                InstanceBuffer::new_with_data(&self.context, &row2),
            );
            self.instance_buffers.insert(
                "row3".to_string(),
                InstanceBuffer::new_with_data(&self.context, &row3),
            );
        }

        if let Some(texture_transforms) = &instances.texture_transforms {
            let mut instance_tex_transform1 = Vec::new();
            let mut instance_tex_transform2 = Vec::new();
            for texture_transform in texture_transforms.iter() {
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
            self.instance_buffers.insert(
                "tex_transform_row1".to_string(),
                InstanceBuffer::new_with_data(&self.context, &instance_tex_transform1),
            );
            self.instance_buffers.insert(
                "tex_transform_row2".to_string(),
                InstanceBuffer::new_with_data(&self.context, &instance_tex_transform2),
            );
        }
        if let Some(instance_colors) = &instances.colors {
            self.instance_buffers.insert(
                "instance_color".to_string(),
                InstanceBuffer::new_with_data(&self.context, &instance_colors),
            );
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

    fn vertex_shader_source(&self, fragment_shader_source: &str) -> String {
        let use_positions = fragment_shader_source.find("in vec3 pos;").is_some();
        let use_normals = fragment_shader_source.find("in vec3 nor;").is_some();
        let use_tangents = fragment_shader_source.find("in vec3 tang;").is_some();
        let use_uvs = fragment_shader_source.find("in vec2 uvs;").is_some();
        let use_colors = fragment_shader_source.find("in vec4 col;").is_some();
        format!(
            "{}{}{}{}{}{}{}{}{}",
            if self.instance_buffers.contains_key("instance_translation") {
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
                if self.instance_buffers.contains_key("instance_color")
                    && self.vertex_buffers.contains_key("color")
                {
                    "#define USE_COLORS\n#define USE_VERTEX_COLORS\n#define USE_INSTANCE_COLORS\n"
                } else if self.instance_buffers.contains_key("instance_color") {
                    "#define USE_COLORS\n#define USE_INSTANCE_COLORS\n"
                } else {
                    "#define USE_COLORS\n#define USE_VERTEX_COLORS\n"
                }
            } else {
                ""
            },
            if self.instance_buffers.contains_key("tex_transform_row1") {
                "#define USE_INSTANCE_TEXTURE_TRANSFORMATION\n"
            } else {
                ""
            },
            include_str!("../../core/shared.frag"),
            include_str!("shaders/mesh.vert"),
        )
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
        let fragment_shader_source = material.fragment_shader_source(
            self.vertex_buffers.contains_key("color")
                || self.instance_buffers.contains_key("instance_color"),
            lights,
        );
        self.context.program(
            &self.vertex_shader_source(&fragment_shader_source),
            &fragment_shader_source,
            |program| {
                material.use_uniforms(program, camera, lights);
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
                            self.instance_buffers
                            .get(attribute_name).expect(&format!("the render call requires the {} instance buffer which is missing on the given geometry", attribute_name))
                        );
                    }
                }

                if let Some(ref index_buffer) = self.index_buffer {
                    program.draw_elements_instanced(
                        material.render_states(),
                        camera.viewport(),
                        index_buffer,
                        self.instance_count,
                    )
                } else {
                    program.draw_arrays_instanced(
                        material.render_states(),
                        camera.viewport(),
                        self.vertex_buffers.get("position").unwrap().vertex_count() as u32,
                        self.instance_count,
                    )
                }
            },
        ).expect("Failed compiling shader")
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
    /// The translation applied to the positions of each instance.
    pub translations: Vec<Vec3>,
    /// The rotations applied to the positions of each instance.
    pub rotations: Option<Vec<Quat>>,
    /// The non-uniform scales applied to the positions of each instance.
    pub scales: Option<Vec<Vec3>>,
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
        buffer_check(self.rotations.as_ref().map(|b| b.len()), "rotations")?;
        buffer_check(self.scales.as_ref().map(|b| b.len()), "scales")?;
        buffer_check(self.colors.as_ref().map(|b| b.len()), "colors")?;
        buffer_check(Some(self.translations.len()), "translations")?;

        Ok(())
    }

    /// Returns the number of instances.
    pub fn count(&self) -> u32 {
        self.translations.len() as u32
    }
}
