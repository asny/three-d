use crate::core::*;
use crate::renderer::*;
use std::collections::HashMap;

///
/// Similar to [Mesh], except it is possible to render many instances of the same mesh efficiently.
///
pub struct InstancedMesh {
    context: Context,
    buffers: HashMap<String, VertexBuffer>,
    index_buffer: Option<ElementBuffer>,
    instance_buffer1: InstanceBuffer,
    instance_buffer2: InstanceBuffer,
    instance_buffer3: InstanceBuffer,
    instance_tex_transform: Option<(InstanceBuffer, InstanceBuffer)>,
    aabb_local: AxisAlignedBoundingBox,
    aabb: AxisAlignedBoundingBox,
    transformation: Mat4,
    instances: Instances,
    instance_count: u32,
    texture_transform: Mat3,
}

impl InstancedMesh {
    ///
    /// Creates a new 3D mesh from the given [CpuMesh].
    /// All data in the [CpuMesh] is transfered to the GPU, so make sure to remove all unnecessary data from the [CpuMesh] before calling this method.
    /// The mesh is rendered in as many instances as there are [Instance] structs given as input.
    /// The transformation and texture transform in [Instance] are applied to each instance before they are rendered.
    ///
    pub fn new(context: &Context, instances: &Instances, cpu_mesh: &CpuMesh) -> ThreeDResult<Self> {
        let aabb = cpu_mesh.compute_aabb();
        let mut model = Self {
            context: context.clone(),
            index_buffer: super::index_buffer_from_mesh(context, cpu_mesh)?,
            buffers: super::vertex_buffers_from_mesh(context, cpu_mesh)?,
            instance_buffer1: InstanceBuffer::new(context)?,
            instance_buffer2: InstanceBuffer::new(context)?,
            instance_buffer3: InstanceBuffer::new(context)?,
            instance_tex_transform: None,
            aabb,
            aabb_local: aabb.clone(),
            transformation: Mat4::identity(),
            instance_count: 0,
            instances: Instances::default(),
            texture_transform: Mat3::identity(),
        };
        model.set_instances(instances)?;
        Ok(model)
    }

    ///
    /// Returns the local to world transformation applied to all instances.
    ///
    pub fn transformation(&self) -> Mat4 {
        self.transformation
    }

    ///
    /// Set the local to world transformation applied to all instances.
    ///
    pub fn set_transformation(&mut self, transformation: Mat4) {
        self.transformation = transformation;
        self.update_aabb();
    }

    ///
    /// Get the texture transform applied to the uv coordinates of all of the instances.
    ///
    pub fn texture_transform(&mut self) -> &Mat3 {
        &self.texture_transform
    }

    ///
    /// Set the texture transform applied to the uv coordinates of all of the model instances.
    /// This is multiplied to the texture transform for each instance.
    ///
    pub fn set_texture_transform(&mut self, texture_transform: Mat3) {
        self.texture_transform = texture_transform;
    }

    /// Returns the number of instances that is rendered.
    pub fn instance_count(&mut self) -> u32 {
        self.instance_count
    }

    /// Use this if you only want to render instance 0 through to instance `instance_count`.
    /// This is the same as changing the instances using `set_instances`, except that it is faster since it doesn't update any buffers.
    /// `instance_count` will be set to the number of instances when they are defined by `set_instances`, so all instanced are rendered by default.
    pub fn set_instance_count(&mut self, instance_count: u32) {
        self.instance_count = instance_count.min(self.instances.count());
        self.update_aabb();
    }

    ///
    /// Returns all instances
    ///
    pub fn instances(&self) -> &Instances {
        &self.instances
    }

    ///
    /// Create an instance for each element with the given mesh and texture transforms.
    ///
    pub fn set_instances(&mut self, instances: &Instances) -> ThreeDResult<()> {
        #[cfg(debug_assertions)]
        instances.validate()?;
        self.instance_count = instances.count();
        self.instances = instances.clone();
        self.update_buffers()
    }

    fn update_buffers(&mut self) -> ThreeDResult<()> {
        let mut row1 = Vec::new();
        let mut row2 = Vec::new();
        let mut row3 = Vec::new();
        for geometry_transform in self.instances.geometry_transforms.iter() {
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
        self.instance_buffer1.fill(&row1)?;
        self.instance_buffer2.fill(&row2)?;
        self.instance_buffer3.fill(&row3)?;

        if let Some(texture_transforms) = &self.instances.texture_transforms {
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
            self.instance_tex_transform = Some((
                InstanceBuffer::new_with_data(&self.context, &instance_tex_transform1)?,
                InstanceBuffer::new_with_data(&self.context, &instance_tex_transform2)?,
            ));
        }
        self.update_aabb();
        Ok(())
    }

    fn update_aabb(&mut self) {
        let mut aabb = AxisAlignedBoundingBox::EMPTY;
        for i in 0..self.instance_count as usize {
            let mut aabb2 = self.aabb_local.clone();
            aabb2.transform(&(self.instances.geometry_transforms[i] * self.transformation));
            aabb.expand_with_aabb(&aabb2);
        }
        self.aabb = aabb;
    }

    fn vertex_shader_source(&self, fragment_shader_source: &str) -> ThreeDResult<String> {
        let use_positions = fragment_shader_source.find("in vec3 pos;").is_some();
        let use_normals = fragment_shader_source.find("in vec3 nor;").is_some();
        let use_tangents = fragment_shader_source.find("in vec3 tang;").is_some();
        let use_uvs = fragment_shader_source.find("in vec2 uvs;").is_some();
        let use_colors = fragment_shader_source.find("in vec4 col;").is_some();
        Ok(format!(
            "#define INSTANCED\n{}{}{}{}{}{}{}{}",
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
                    Err(CoreError::MissingBitangent)?;
                }
                "#define USE_TANGENTS\n"
            } else {
                ""
            },
            if use_uvs { "#define USE_UVS\n" } else { "" },
            if use_colors {
                "#define USE_COLORS\n"
            } else {
                ""
            },
            if self.instances.texture_transforms.is_some() {
                "#define USE_INSTANCE_TEXTURE_TRANSFORMATION\n"
            } else {
                ""
            },
            include_str!("../../core/shared.frag"),
            include_str!("shaders/mesh.vert"),
        ))
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
    ) -> ThreeDResult<()> {
        let fragment_shader_source =
            material.fragment_shader_source(self.buffers.contains_key("color"), lights);
        self.context.program(
            &self.vertex_shader_source(&fragment_shader_source)?,
            &fragment_shader_source,
            |program| {
                material.use_uniforms(program, camera, lights)?;
                program.use_uniform("viewProjection", camera.projection() * camera.view())?;
                program.use_uniform("modelMatrix", &self.transformation)?;
                program.use_uniform_if_required("textureTransform", &self.texture_transform)?;
                program.use_uniform_if_required(
                    "normalMatrix",
                    &self.transformation.invert().unwrap().transpose(),
                )?;

                program.use_instance_attribute("row1", &self.instance_buffer1)?;
                program.use_instance_attribute("row2", &self.instance_buffer2)?;
                program.use_instance_attribute("row3", &self.instance_buffer3)?;

                if program.requires_attribute("uv_coordinates") {
                    if let Some((tex_transform_row1, tex_transform_row2)) =
                        &self.instance_tex_transform
                    {
                        program
                            .use_instance_attribute("tex_transform_row1", &tex_transform_row1)?;
                        program
                            .use_instance_attribute("tex_transform_row2", &tex_transform_row2)?;
                    }
                }

                for attribute_name in ["position", "normal", "tangent", "color", "uv_coordinates"] {
                    if program.requires_attribute(attribute_name) {
                        program.use_vertex_attribute(
                            attribute_name,
                            self.buffers
                                .get(attribute_name)
                                .ok_or(CoreError::MissingMeshBuffer(attribute_name.to_string()))?,
                        )?;
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
                        self.buffers.get("position").unwrap().vertex_count() as u32,
                        self.instance_count,
                    )
                }
            },
        )
    }
}

/// Defines an instance of the model defined in [InstancedMesh] or [InstancedModel].
#[derive(Clone, Debug)]
pub struct Instances {
    /// The local to world transformation applied to the positions of the model instance.
    pub geometry_transforms: Vec<Mat4>,
    /// The texture transform applied to the uv coordinates of the model instance.
    pub texture_transforms: Option<Vec<Mat3>>,
}

impl Instances {
    ///
    /// Returns an error if the instances is not valid.
    ///
    pub fn validate(&self) -> ThreeDResult<()> {
        let instance_count = self.geometry_transforms.len();
        let buffer_check = |length: Option<usize>, name: &str| -> ThreeDResult<()> {
            if let Some(length) = length {
                if length < instance_count {
                    Err(CoreError::InvalidBufferLength(
                        name.to_string(),
                        instance_count,
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

        Ok(())
    }

    /// Returns the number of instances.
    pub fn count(&self) -> u32 {
        self.geometry_transforms.len() as u32
    }
}

impl Default for Instances {
    fn default() -> Self {
        Self {
            geometry_transforms: vec![Mat4::identity()],
            texture_transforms: None,
        }
    }
}
