use crate::core::*;
use crate::renderer::*;

///
/// Similar to [Model], except it is possible to render many instances of the same model efficiently.
///
pub struct InstancedModel<M: ForwardMaterial> {
    context: Context,
    mesh: Mesh,
    instance_count: u32,
    instance_buffer1: InstanceBuffer,
    instance_buffer2: InstanceBuffer,
    instance_buffer3: InstanceBuffer,
    aabb_local: AxisAlignedBoundingBox,
    aabb: AxisAlignedBoundingBox,
    transformation: Mat4,
    transformations: Vec<Mat4>,
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
        transformations: &[Mat4],
        cpu_mesh: &CPUMesh,
    ) -> ThreeDResult<Self> {
        Self::new_with_material(context, transformations, cpu_mesh, ColorMaterial::default())
    }
}

impl<M: ForwardMaterial> InstancedModel<M> {
    pub fn new_with_material(
        context: &Context,
        transformations: &[Mat4],
        cpu_mesh: &CPUMesh,
        material: M,
    ) -> ThreeDResult<Self> {
        let aabb = cpu_mesh.compute_aabb();
        let mut model = Self {
            context: context.clone(),
            instance_count: 0,
            mesh: Mesh::new(context, cpu_mesh)?,
            instance_buffer1: InstanceBuffer::new(context)?,
            instance_buffer2: InstanceBuffer::new(context)?,
            instance_buffer3: InstanceBuffer::new(context)?,
            aabb,
            aabb_local: aabb.clone(),
            transformation: Mat4::identity(),
            transformations: transformations.to_vec(),
            material,
        };
        model.update_transformations(transformations);
        model.update_aabb();
        Ok(model)
    }

    ///
    /// Updates the transformations applied to each model instance before they are rendered.
    /// The model is rendered in as many instances as there are transformation matrices.
    ///
    pub fn update_transformations(&mut self, transformations: &[Mat4]) {
        self.transformations = transformations.to_vec();
        self.instance_count = transformations.len() as u32;
        let mut row1 = Vec::new();
        let mut row2 = Vec::new();
        let mut row3 = Vec::new();
        for transform in transformations {
            row1.push(transform.x.x);
            row1.push(transform.y.x);
            row1.push(transform.z.x);
            row1.push(transform.w.x);

            row2.push(transform.x.y);
            row2.push(transform.y.y);
            row2.push(transform.z.y);
            row2.push(transform.w.y);

            row3.push(transform.x.z);
            row3.push(transform.y.z);
            row3.push(transform.z.z);
            row3.push(transform.w.z);
        }
        self.instance_buffer1.fill_with_dynamic(&row1);
        self.instance_buffer2.fill_with_dynamic(&row2);
        self.instance_buffer3.fill_with_dynamic(&row3);
        self.update_aabb();
    }

    pub fn transformations(&self) -> &[Mat4] {
        &self.transformations
    }

    fn update_aabb(&mut self) {
        let mut aabb = AxisAlignedBoundingBox::EMPTY;
        for transform in self.transformations.iter() {
            let mut aabb2 = self.aabb_local.clone();
            aabb2.transform(&(transform * self.transformation));
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
    fn render_with_material(
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

    fn render_forward(
        &self,
        material: &dyn ForwardMaterial,
        camera: &Camera,
        lights: &Lights,
    ) -> ThreeDResult<()> {
        self.render_with_material(material, camera, lights)
    }

    fn render_deferred(
        &self,
        material: &DeferredPhysicalMaterial,
        camera: &Camera,
        viewport: Viewport,
    ) -> ThreeDResult<()> {
        let lights = Lights::default();
        let fragment_shader_source =
            material.fragment_shader_source(self.mesh.color_buffer.is_some(), &lights);
        self.context.program(
            &Self::vertex_shader_source(&fragment_shader_source)?,
            &fragment_shader_source,
            |program| {
                material.use_uniforms(program, camera, &lights)?;
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
        self.render_with_material(&self.material, camera, lights)
    }

    fn is_transparent(&self) -> bool {
        self.material.is_transparent()
    }
}
