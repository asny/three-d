use crate::core::*;
use crate::renderer::*;

///
/// A 3D model consisting of a triangle mesh and any material that implements the `ForwardMaterial` trait.
///
#[derive(Clone)]
pub struct Model<M: ForwardMaterial> {
    context: Context,
    mesh: Mesh,
    aabb: AxisAlignedBoundingBox,
    aabb_local: AxisAlignedBoundingBox,
    transformation: Mat4,
    normal_transformation: Mat4,
    /// The material applied to the model
    pub material: M,
}

impl Model<ColorMaterial> {
    ///
    /// Creates a new 3D model with a triangle mesh as geometry and a default [ColorMaterial].
    ///
    pub fn new(context: &Context, cpu_mesh: &CPUMesh) -> ThreeDResult<Self> {
        Self::new_with_material(context, cpu_mesh, ColorMaterial::default())
    }
}

impl<M: ForwardMaterial> Model<M> {
    ///
    /// Creates a new 3D model with a triangle mesh as geometry and the given material.
    ///
    pub fn new_with_material(
        context: &Context,
        cpu_mesh: &CPUMesh,
        material: M,
    ) -> ThreeDResult<Self> {
        let mesh = Mesh::new(context, cpu_mesh)?;
        let aabb = cpu_mesh.compute_aabb();
        Ok(Self {
            mesh,
            aabb,
            aabb_local: aabb.clone(),
            transformation: Mat4::identity(),
            normal_transformation: Mat4::identity(),
            context: context.clone(),
            material,
        })
    }

    pub(in crate::renderer) fn set_transformation_2d(&mut self, transformation: Mat3) {
        self.set_transformation(Mat4::new(
            transformation.x.x,
            transformation.x.y,
            0.0,
            transformation.x.z,
            transformation.y.x,
            transformation.y.y,
            0.0,
            transformation.y.z,
            0.0,
            0.0,
            1.0,
            0.0,
            transformation.z.x,
            transformation.z.y,
            0.0,
            transformation.z.z,
        ));
    }
}

impl<M: ForwardMaterial> Geometry for Model<M> {
    fn aabb(&self) -> AxisAlignedBoundingBox {
        self.aabb
    }

    fn transformation(&self) -> Mat4 {
        self.transformation
    }
}

impl<M: ForwardMaterial> GeometryMut for Model<M> {
    fn set_transformation(&mut self, transformation: Mat4) {
        self.transformation = transformation;
        self.normal_transformation = self.transformation.invert().unwrap().transpose();
        let mut aabb = self.aabb_local.clone();
        aabb.transform(&self.transformation);
        self.aabb = aabb;
    }
}

impl<M: ForwardMaterial> Shadable for Model<M> {
    fn render_forward(
        &self,
        material: &dyn ForwardMaterial,
        camera: &Camera,
        lights: &Lights,
    ) -> ThreeDResult<()> {
        let fragment_shader_source =
            material.fragment_shader_source(self.mesh.color_buffer.is_some(), lights);
        self.context.program(
            &Mesh::vertex_shader_source(&fragment_shader_source),
            &fragment_shader_source,
            |program| {
                material.use_uniforms(program, camera, lights)?;
                self.mesh.use_attributes(program, camera.uniform_buffer())?;
                program.use_uniform_mat4("modelMatrix", &self.transformation)?;
                if program.requires_attribute("normal") {
                    program.use_uniform_mat4("normalMatrix", &self.normal_transformation)?;
                }
                if let Some(ref index_buffer) = self.mesh.index_buffer {
                    program.draw_elements(
                        material.render_states(),
                        camera.viewport(),
                        index_buffer,
                    );
                } else {
                    program.draw_arrays(
                        material.render_states(),
                        camera.viewport(),
                        self.mesh.position_buffer.count() as u32 / 3,
                    );
                }
                Ok(())
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
            &Mesh::vertex_shader_source(&fragment_shader_source),
            &fragment_shader_source,
            |program| {
                material.use_uniforms(program, camera, &Lights::default())?;
                self.mesh.use_attributes(program, camera.uniform_buffer())?;
                program.use_uniform_mat4("modelMatrix", &self.transformation)?;
                if program.requires_attribute("normal") {
                    program.use_uniform_mat4("normalMatrix", &self.normal_transformation)?;
                }
                if let Some(ref index_buffer) = self.mesh.index_buffer {
                    program.draw_elements(material.render_states(), viewport, index_buffer);
                } else {
                    program.draw_arrays(
                        material.render_states(),
                        viewport,
                        self.mesh.position_buffer.count() as u32 / 3,
                    );
                }
                Ok(())
            },
        )
    }
}

impl<M: ForwardMaterial> Object for Model<M> {
    fn render(&self, camera: &Camera, lights: &Lights) -> ThreeDResult<()> {
        self.render_forward(&self.material, camera, lights)
    }

    fn is_transparent(&self) -> bool {
        self.material.is_transparent()
    }
}
