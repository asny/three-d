use crate::core::*;
use crate::renderer::*;

///
/// Similar to [Model], except it is possible to render many instances of the same model efficiently.
///
pub struct InstancedModel<M: Material> {
    context: Context,
    mesh: Mesh,
    instance_buffer1: InstanceBuffer,
    instance_buffer2: InstanceBuffer,
    instance_buffer3: InstanceBuffer,
    instance_tex_transform1: InstanceBuffer,
    instance_tex_transform2: InstanceBuffer,
    aabb_local: AxisAlignedBoundingBox,
    aabb: AxisAlignedBoundingBox,
    transformation: Mat4,
    instances: Vec<ModelInstance>,
    instance_count: u32,
    texture_transform: Mat3,
    /// The material applied to the instanced model
    pub material: M,
}

impl InstancedModel<ColorMaterial<Texture2D<u8>>> {
    ///
    /// Creates a new instanced 3D model with a triangle mesh as geometry and a default [ColorMaterial].
    /// The transformations are applied to each model instance before they are rendered.
    /// The model is rendered in as many instances as there are transformation matrices.
    ///
    pub fn new(
        context: &Context,
        instances: &[ModelInstance],
        cpu_mesh: &CpuMesh,
    ) -> ThreeDResult<Self> {
        Self::new_with_material(context, instances, cpu_mesh, ColorMaterial::default())
    }
}

impl<M: Material> InstancedModel<M> {
    pub fn new_with_material(
        context: &Context,
        instances: &[ModelInstance],
        cpu_mesh: &CpuMesh,
        material: M,
    ) -> ThreeDResult<Self> {
        let aabb = cpu_mesh.compute_aabb();
        let mut model = Self {
            context: context.clone(),
            mesh: Mesh::new(context, cpu_mesh)?,
            instance_buffer1: InstanceBuffer::new(context)?,
            instance_buffer2: InstanceBuffer::new(context)?,
            instance_buffer3: InstanceBuffer::new(context)?,
            instance_tex_transform1: InstanceBuffer::new(context)?,
            instance_tex_transform2: InstanceBuffer::new(context)?,
            aabb,
            aabb_local: aabb.clone(),
            transformation: Mat4::identity(),
            instances: Vec::new(),
            instance_count: 0,
            texture_transform: Mat3::identity(),
            material,
        };
        model.set_instances(instances);
        Ok(model)
    }

    pub fn texture_transform(&mut self) -> &Mat3 {
        &self.texture_transform
    }

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
        self.instance_count = instance_count.min(self.instances.len() as u32);
        self.update_aabb();
    }

    ///
    /// Returns all instances
    ///
    pub fn instances(&self) -> &[ModelInstance] {
        &self.instances
    }

    ///
    /// Create an instance for each element with the given mesh and texture transforms.
    ///
    pub fn set_instances(&mut self, instances: &[ModelInstance]) {
        self.instance_count = instances.len() as u32;
        self.instances = instances.to_vec();
        self.update_buffers();
    }

    fn update_buffers(&mut self) {
        let mut row1 = Vec::new();
        let mut row2 = Vec::new();
        let mut row3 = Vec::new();
        let mut instance_tex_transform1 = Vec::new();
        let mut instance_tex_transform2 = Vec::new();
        for instance in self.instances.iter() {
            row1.push(instance.geometry_transform.x.x);
            row1.push(instance.geometry_transform.y.x);
            row1.push(instance.geometry_transform.z.x);
            row1.push(instance.geometry_transform.w.x);

            row2.push(instance.geometry_transform.x.y);
            row2.push(instance.geometry_transform.y.y);
            row2.push(instance.geometry_transform.z.y);
            row2.push(instance.geometry_transform.w.y);

            row3.push(instance.geometry_transform.x.z);
            row3.push(instance.geometry_transform.y.z);
            row3.push(instance.geometry_transform.z.z);
            row3.push(instance.geometry_transform.w.z);

            instance_tex_transform1.push(instance.texture_transform.x.x);
            instance_tex_transform1.push(instance.texture_transform.y.x);
            instance_tex_transform1.push(instance.texture_transform.z.x);

            instance_tex_transform2.push(instance.texture_transform.x.y);
            instance_tex_transform2.push(instance.texture_transform.y.y);
            instance_tex_transform2.push(instance.texture_transform.z.y);
        }
        self.instance_buffer1.fill_with_dynamic(&row1);
        self.instance_buffer2.fill_with_dynamic(&row2);
        self.instance_buffer3.fill_with_dynamic(&row3);
        self.instance_tex_transform1
            .fill_with_dynamic(&instance_tex_transform1);
        self.instance_tex_transform2
            .fill_with_dynamic(&instance_tex_transform2);
        self.update_aabb();
    }

    fn update_aabb(&mut self) {
        let mut aabb = AxisAlignedBoundingBox::EMPTY;
        for i in 0..self.instance_count as usize {
            let mut aabb2 = self.aabb_local.clone();
            aabb2.transform(&(self.instances[i].geometry_transform * self.transformation));
            aabb.expand_with_aabb(&aabb2);
        }
        self.aabb = aabb;
    }

    fn draw(
        &self,
        program: &Program,
        render_states: RenderStates,
        camera_buffer: &UniformBuffer,
        viewport: Viewport,
    ) -> ThreeDResult<()> {
        program.use_uniform_block("Camera", camera_buffer);
        program.use_uniform_mat4("modelMatrix", &self.transformation)?;

        program.use_attribute_vec4_instanced("row1", &self.instance_buffer1)?;
        program.use_attribute_vec4_instanced("row2", &self.instance_buffer2)?;
        program.use_attribute_vec4_instanced("row3", &self.instance_buffer3)?;

        if program.requires_attribute("position") {
            program.use_attribute_vec3("position", &self.mesh.position_buffer)?;
        }
        if program.requires_attribute("uv_coordinates") {
            program.use_uniform_mat3("textureTransform", &self.texture_transform)?;
            program.use_attribute_vec3_instanced(
                "tex_transform_row1",
                &self.instance_tex_transform1,
            )?;
            program.use_attribute_vec3_instanced(
                "tex_transform_row2",
                &self.instance_tex_transform2,
            )?;
            let uv_buffer = self
                .mesh
                .uv_buffer
                .as_ref()
                .ok_or(CoreError::MissingMeshBuffer("uv coordinates".to_string()))?;
            program.use_attribute_vec2("uv_coordinates", uv_buffer)?;
        }
        if program.requires_attribute("normal") {
            let normal_buffer = self
                .mesh
                .normal_buffer
                .as_ref()
                .ok_or(CoreError::MissingMeshBuffer("normal".to_string()))?;
            program.use_attribute_vec3("normal", normal_buffer)?;
            if program.requires_attribute("tangent") {
                let tangent_buffer = self
                    .mesh
                    .tangent_buffer
                    .as_ref()
                    .ok_or(CoreError::MissingMeshBuffer("tangent".to_string()))?;
                program.use_attribute_vec4("tangent", tangent_buffer)?;
            }
        }
        if program.requires_attribute("color") {
            let color_buffer = self
                .mesh
                .color_buffer
                .as_ref()
                .ok_or(CoreError::MissingMeshBuffer("color".to_string()))?;
            program.use_attribute_vec4("color", color_buffer)?;
        }

        if let Some(ref index_buffer) = self.mesh.index_buffer {
            program.draw_elements_instanced(
                render_states,
                viewport,
                index_buffer,
                self.instance_count,
            );
        } else {
            program.draw_arrays_instanced(
                render_states,
                viewport,
                self.mesh.position_buffer.count() as u32 / 3,
                self.instance_count,
            );
        }
        Ok(())
    }

    fn vertex_shader_source(fragment_shader_source: &str) -> ThreeDResult<String> {
        Ok(format!(
            "#define INSTANCED\n{}",
            Model::<M>::vertex_shader_source(fragment_shader_source)?
        ))
    }
}

impl<M: Material> Geometry for InstancedModel<M> {
    fn aabb(&self) -> AxisAlignedBoundingBox {
        self.aabb
    }

    fn transformation(&self) -> Mat4 {
        self.transformation
    }

    fn render_with_material(
        &self,
        material: &dyn Material,
        camera: &Camera,
        lights: &[&dyn Light],
    ) -> ThreeDResult<()> {
        let fragment_shader_source =
            material.fragment_shader_source(self.mesh.color_buffer.is_some(), lights);
        self.context.program(
            &Self::vertex_shader_source(&fragment_shader_source)?,
            &fragment_shader_source,
            |program| {
                material.use_uniforms(program, camera, lights)?;
                self.draw(
                    program,
                    material.render_states(),
                    camera.uniform_buffer(),
                    camera.viewport(),
                )
            },
        )
    }
}

impl<M: Material> GeometryMut for InstancedModel<M> {
    fn set_transformation(&mut self, transformation: Mat4) {
        self.transformation = transformation;
        self.update_aabb();
    }
}

impl<M: Material> Object for InstancedModel<M> {
    fn render(&self, camera: &Camera, lights: &[&dyn Light]) -> ThreeDResult<()> {
        self.render_with_material(&self.material, camera, lights)
    }

    fn is_transparent(&self) -> bool {
        self.material.is_transparent()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ModelInstance {
    pub geometry_transform: Mat4,
    pub texture_transform: Mat3,
}

impl Default for ModelInstance {
    fn default() -> Self {
        Self {
            geometry_transform: Mat4::identity(),
            texture_transform: Mat3::identity(),
        }
    }
}
