use crate::core::*;
use crate::renderer::*;

///
/// Similar to [Model], except it is possible to render many instances of the same model efficiently.
///
pub struct InstancedModel<M: ForwardMaterial> {
    context: Context,
    mesh: Mesh,
    instance_buffer1: InstanceBuffer,
    instance_buffer2: InstanceBuffer,
    instance_buffer3: InstanceBuffer,
    instance_buffer4: InstanceBuffer,
    aabb_local: AxisAlignedBoundingBox,
    aabb: AxisAlignedBoundingBox,
    transformation: Mat4,
    instances: Vec<ModelInstance>,
    texture_transform: TextureTransform,
    /// The material applied to the instanced model
    pub material: M,
}

impl InstancedModel<ColorMaterial> {
    ///
    /// Creates a new instanced 3D model with a triangle mesh as geometry and a default [ColorMaterial].
    /// The transformations are applied to each model instance before they are rendered.
    /// The model is rendered in as many instances as there are transformation matrices.
    ///
    pub fn new(
        context: &Context,
        instances: &[ModelInstance],
        cpu_mesh: &CPUMesh,
    ) -> ThreeDResult<Self> {
        Self::new_with_material(context, instances, cpu_mesh, ColorMaterial::default())
    }
}

impl<M: ForwardMaterial> InstancedModel<M> {
    pub fn new_with_material(
        context: &Context,
        instances: &[ModelInstance],
        cpu_mesh: &CPUMesh,
        material: M,
    ) -> ThreeDResult<Self> {
        let aabb = cpu_mesh.compute_aabb();
        let mut model = Self {
            context: context.clone(),
            mesh: Mesh::new(context, cpu_mesh)?,
            instance_buffer1: InstanceBuffer::new(context)?,
            instance_buffer2: InstanceBuffer::new(context)?,
            instance_buffer3: InstanceBuffer::new(context)?,
            instance_buffer4: InstanceBuffer::new(context)?,
            aabb,
            aabb_local: aabb.clone(),
            transformation: Mat4::identity(),
            instances: instances.to_vec(),
            texture_transform: TextureTransform::default(),
            material,
        };
        model.update_buffers();
        Ok(model)
    }

    pub fn texture_transform(&mut self) -> &TextureTransform {
        &self.texture_transform
    }

    pub fn set_texture_transform(&mut self, texture_transform: TextureTransform) {
        self.texture_transform = texture_transform;
    }

    ///
    /// Updates instance transform and uv buffers and aabb on demand.
    ///
    fn update_buffers(&mut self) {
        let instances = self.instances.as_slice();
        let mut row1 = Vec::new();
        let mut row2 = Vec::new();
        let mut row3 = Vec::new();
        let mut subt = Vec::new();
        for instance in instances {
            row1.push(instance.mesh_transform.x.x);
            row1.push(instance.mesh_transform.y.x);
            row1.push(instance.mesh_transform.z.x);
            row1.push(instance.mesh_transform.w.x);

            row2.push(instance.mesh_transform.x.y);
            row2.push(instance.mesh_transform.y.y);
            row2.push(instance.mesh_transform.z.y);
            row2.push(instance.mesh_transform.w.y);

            row3.push(instance.mesh_transform.x.z);
            row3.push(instance.mesh_transform.y.z);
            row3.push(instance.mesh_transform.z.z);
            row3.push(instance.mesh_transform.w.z);

            subt.push(instance.texture_transform.offset_x);
            subt.push(instance.texture_transform.offset_y);
            subt.push(instance.texture_transform.scale_x);
            subt.push(instance.texture_transform.scale_y);
        }
        self.instance_buffer1.fill_with_dynamic(&row1);
        self.instance_buffer2.fill_with_dynamic(&row2);
        self.instance_buffer3.fill_with_dynamic(&row3);
        self.instance_buffer4.fill_with_dynamic(&subt);
        self.update_aabb();
    }

    ///
    /// Retrieve all instances
    ///
    pub fn get_instances(&mut self) -> Vec<ModelInstance> {
        self.instances.clone()
    }

    ///
    /// Create an instance for each element with the given mesh and texture transforms.
    ///
    pub fn set_instances(&mut self, instances: Vec<ModelInstance>) {
        self.instances = instances.to_vec();
        self.update_buffers();
    }

    fn update_aabb(&mut self) {
        let mut aabb = AxisAlignedBoundingBox::EMPTY;
        for instance in self.instances.iter() {
            let mut aabb2 = self.aabb_local.clone();
            aabb2.transform(&(instance.mesh_transform * self.transformation));
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
            program.use_uniform_vec4("textureTransform", &self.texture_transform.to_vec4())?;
            program.use_attribute_vec4_instanced("subt", &self.instance_buffer4)?;
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
                self.instances.len() as u32,
            );
        } else {
            program.draw_arrays_instanced(
                render_states,
                viewport,
                self.mesh.position_buffer.count() as u32 / 3,
                self.instances.len() as u32,
            );
        }
        Ok(())
    }

    fn vertex_shader_source(fragment_shader_source: &str) -> ThreeDResult<String> {
        Ok(format!(
            "#define INSTANCED\n{}",
            Mesh::vertex_shader_source(fragment_shader_source)?
        ))
    }
}

impl<M: ForwardMaterial> Geometry for InstancedModel<M> {
    fn aabb(&self) -> AxisAlignedBoundingBox {
        self.aabb
    }

    fn transformation(&self) -> Mat4 {
        self.transformation
    }
}

impl<M: ForwardMaterial> GeometryMut for InstancedModel<M> {
    fn set_transformation(&mut self, transformation: Mat4) {
        self.transformation = transformation;
        self.update_aabb();
    }
}

impl<M: ForwardMaterial> Shadable for InstancedModel<M> {
    fn render_forward(
        &self,
        material: &dyn ForwardMaterial,
        camera: &Camera,
        lights: &Lights,
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

    fn render_deferred(
        &self,
        material: &dyn DeferredMaterial,
        camera: &Camera,
        viewport: Viewport,
    ) -> ThreeDResult<()> {
        let fragment_shader_source =
            material.fragment_shader_source_deferred(self.mesh.color_buffer.is_some());
        self.context.program(
            &Self::vertex_shader_source(&fragment_shader_source)?,
            &fragment_shader_source,
            |program| {
                material.use_uniforms(program, camera, &Lights::default())?;
                self.draw(
                    program,
                    material.render_states(),
                    camera.uniform_buffer(),
                    viewport,
                )
            },
        )
    }
}

impl<M: ForwardMaterial> Object for InstancedModel<M> {
    fn render(&self, camera: &Camera, lights: &Lights) -> ThreeDResult<()> {
        self.render_forward(&self.material, camera, lights)
    }

    fn is_transparent(&self) -> bool {
        self.material.is_transparent()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ModelInstance {
    pub mesh_transform: Mat4,
    pub texture_transform: TextureTransform,
}

impl Default for ModelInstance {
    fn default() -> Self {
        Self {
            mesh_transform: Mat4::identity(),
            texture_transform: TextureTransform::default(),
        }
    }
}
