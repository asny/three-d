use crate::core::*;
use crate::renderer::*;
use std::cell::RefCell;
use std::rc::Rc;

///
/// Similar to [Model], except it is possible to render many instances of the same model efficiently.
///
pub struct InstancedModel<M: ForwardMaterial> {
    context: Context,
    mesh: Mesh,
    aabb_local: RefCell<AxisAlignedBoundingBox>,
    aabb: RefCell<AxisAlignedBoundingBox>,
    transformation: RefCell<Mat4>,
    instances: RefCell<Vec<ModelInstance>>,
    texture_transform: RefCell<TextureTransform>,
    buffers_dirty: RefCell<bool>,
    instance_buffer1: RefCell<InstanceBuffer>,
    instance_buffer2: RefCell<InstanceBuffer>,
    instance_buffer3: RefCell<InstanceBuffer>,
    instance_buffer4: RefCell<InstanceBuffer>,
    /// The material applied to the instanced model
    pub material: Rc<M>,
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
        Self::new_with_material(
            context,
            instances,
            cpu_mesh,
            Rc::new(ColorMaterial::default()),
        )
    }
}

impl<M: ForwardMaterial> InstancedModel<M> {
    pub fn new_with_material(
        context: &Context,
        instances: &[ModelInstance],
        cpu_mesh: &CPUMesh,
        material: Rc<M>,
    ) -> ThreeDResult<Self> {
        let aabb = cpu_mesh.compute_aabb();
        let model = Self {
            context: context.clone(),
            mesh: Mesh::new(context, cpu_mesh)?,
            aabb: RefCell::new(aabb),
            aabb_local: RefCell::new(aabb.clone()),
            transformation: RefCell::new(Mat4::identity()),
            instances: RefCell::new(instances.to_vec()),
            texture_transform: RefCell::new(TextureTransform::default()),
            buffers_dirty: RefCell::new(true),
            instance_buffer1: RefCell::new(InstanceBuffer::new(context)?),
            instance_buffer2: RefCell::new(InstanceBuffer::new(context)?),
            instance_buffer3: RefCell::new(InstanceBuffer::new(context)?),
            instance_buffer4: RefCell::new(InstanceBuffer::new(context)?),
            material,
        };
        Ok(model)
    }

    pub fn texture_transform(&self) -> TextureTransform {
        (*self.texture_transform.borrow()).clone()
    }

    pub fn set_texture_transform(&self, texture_transform: TextureTransform) {
        let transform = &mut *self.texture_transform.borrow_mut();
        *transform = texture_transform;
        self.set_buffers_dirty(true);
    }

    ///
    /// Returns all instances
    ///
    pub fn instances(&self) -> Vec<ModelInstance> {
        (*self.instances.borrow()).to_vec()
    }

    ///
    /// Create an instance for each element with the given mesh and texture transforms.
    ///
    pub fn set_instances(&self, new_instances: &[ModelInstance]) {
        let instances = &mut *self.instances.borrow_mut();
        *instances = new_instances.to_vec();
        self.set_buffers_dirty(true);
    }

    ///
    /// Framework for future JIT updating additions.
    ///
    fn update(&self) -> ThreeDResult<()> {
        if *self.buffers_dirty.borrow() {
            self.update_buffers()?;
        }
        Ok(())
    }

    ///
    /// Updates instance transform and uv buffers.
    ///
    fn update_buffers(&self) -> ThreeDResult<()> {
        let mut row1 = Vec::new();
        let mut row2 = Vec::new();
        let mut row3 = Vec::new();
        let mut subt = Vec::new();
        let instances = &*self.instances.borrow();
        let instance_buffer1 = &mut *self.instance_buffer1.borrow_mut();
        let instance_buffer2 = &mut *self.instance_buffer2.borrow_mut();
        let instance_buffer3 = &mut *self.instance_buffer3.borrow_mut();
        let instance_buffer4 = &mut *self.instance_buffer4.borrow_mut();
        for instance in instances.iter() {
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
        instance_buffer1.fill_with_dynamic(&row1);
        instance_buffer2.fill_with_dynamic(&row2);
        instance_buffer3.fill_with_dynamic(&row3);
        instance_buffer4.fill_with_dynamic(&subt);
        self.update_aabb()?;
        self.set_buffers_dirty(false);
        Ok(())
    }

    fn set_buffers_dirty(&self, bool: bool) {
        let dirty = &mut self.buffers_dirty.borrow_mut();
        *dirty = bool
    }

    ///
    /// Updates aabb.
    ///
    fn update_aabb(&self) -> ThreeDResult<()> {
        let mut aabb = AxisAlignedBoundingBox::EMPTY;
        let instances = &*self.instances.borrow();
        let aabb_local = *self.aabb_local.borrow();
        let transformation = *self.transformation.borrow();
        for instance in instances.iter() {
            let mut aabb2 = aabb_local.clone();
            aabb2.transform(&(instance.mesh_transform * transformation));
            aabb.expand_with_aabb(&aabb2);
        }
        Ok(())
    }

    fn draw(
        &self,
        program: &Program,
        render_states: RenderStates,
        camera_buffer: &UniformBuffer,
        viewport: Viewport,
    ) -> ThreeDResult<()> {
        self.update()?;

        let transformation = *self.transformation.borrow();
        let texture_transform = *self.texture_transform.borrow();
        let instance_buffer1 = &*self.instance_buffer1.borrow();
        let instance_buffer2 = &*self.instance_buffer2.borrow();
        let instance_buffer3 = &*self.instance_buffer3.borrow();
        let instance_buffer4 = &*self.instance_buffer4.borrow();
        let instances = &*self.instances.borrow();

        program.use_uniform_block("Camera", camera_buffer);
        program.use_uniform_mat4("modelMatrix", &transformation)?;

        program.use_attribute_vec4_instanced("row1", &instance_buffer1)?;
        program.use_attribute_vec4_instanced("row2", &instance_buffer2)?;
        program.use_attribute_vec4_instanced("row3", &instance_buffer3)?;

        if program.requires_attribute("position") {
            program.use_attribute_vec3("position", &self.mesh.position_buffer)?;
        }
        if program.requires_attribute("uv_coordinates") {
            program.use_uniform_vec4("textureTransform", &texture_transform.to_vec4())?;
            program.use_attribute_vec4_instanced("subt", &instance_buffer4)?;
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
                instances.len() as u32,
            );
        } else {
            program.draw_arrays_instanced(
                render_states,
                viewport,
                self.mesh.position_buffer.count() as u32 / 3,
                instances.len() as u32,
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
        *self.aabb.borrow()
    }

    fn transformation(&self) -> Mat4 {
        *self.transformation.borrow()
    }
}

// &mut self is uncessary here, but needs to be removed at the trait level too.
impl<M: ForwardMaterial> GeometryMut for InstancedModel<M> {
    fn set_transformation(&mut self, new_transformation: Mat4) {
        let mut _transformation = *self.transformation.borrow_mut();
        _transformation = new_transformation;
        self.set_buffers_dirty(true);
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
        self.render_forward(&*self.material, camera, lights)
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
