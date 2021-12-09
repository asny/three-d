use crate::core::*;
use crate::renderer::*;
use std::rc::Rc;

///
/// A 3D model consisting of a triangle mesh and any material that implements the `Material` trait.
///
#[derive(Clone)]
pub struct Model<M: Material> {
    context: Context,
    mesh: Rc<Mesh>,
    aabb: AxisAlignedBoundingBox,
    aabb_local: AxisAlignedBoundingBox,
    transformation: Mat4,
    texture_transform: TextureTransform,
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

impl<M: Material> Model<M> {
    ///
    /// Creates a new 3D model with a triangle mesh as geometry and the given material.
    ///
    pub fn new_with_material(
        context: &Context,
        cpu_mesh: &CPUMesh,
        material: M,
    ) -> ThreeDResult<Self> {
        let mesh = Rc::new(Mesh::new(context, cpu_mesh)?);
        let aabb = cpu_mesh.compute_aabb();
        Ok(Self {
            mesh,
            aabb,
            aabb_local: aabb.clone(),
            transformation: Mat4::identity(),
            texture_transform: TextureTransform::default(),
            context: context.clone(),
            material,
        })
    }

    pub fn texture_transform(&mut self) -> &TextureTransform {
        &self.texture_transform
    }

    pub fn set_texture_transform(&mut self, texture_transform: TextureTransform) {
        self.texture_transform = texture_transform;
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

impl<M: Material> Geometry for Model<M> {
    fn aabb(&self) -> AxisAlignedBoundingBox {
        self.aabb
    }

    fn transformation(&self) -> Mat4 {
        self.transformation
    }
}

impl<M: Material> GeometryMut for Model<M> {
    fn set_transformation(&mut self, transformation: Mat4) {
        self.transformation = transformation;
        let mut aabb = self.aabb_local.clone();
        aabb.transform(&self.transformation);
        self.aabb = aabb;
    }
}

impl<M: Material> Shadable for Model<M> {
    fn render_with_material(
        &self,
        material: &dyn Material,
        camera: &Camera,
        lights: &Lights,
    ) -> ThreeDResult<()> {
        let fragment_shader_source =
            material.fragment_shader_source(self.mesh.color_buffer.is_some(), lights);
        self.context.program(
            &Mesh::vertex_shader_source(&fragment_shader_source)?,
            &fragment_shader_source,
            |program| {
                material.use_uniforms(program, camera, lights)?;
                self.mesh.draw(
                    program,
                    material.render_states(),
                    camera.uniform_buffer(),
                    camera.viewport(),
                    &self.transformation,
                    &self.texture_transform,
                )
            },
        )
    }

    fn render_forward(
        &self,
        material: &dyn Material,
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
            &Mesh::vertex_shader_source(&fragment_shader_source)?,
            &fragment_shader_source,
            |program| {
                material.use_uniforms(program, camera, &lights)?;
                self.mesh.draw(
                    program,
                    material.render_states(),
                    camera.uniform_buffer(),
                    viewport,
                    &self.transformation,
                    &self.texture_transform,
                )
            },
        )
    }
}

impl<M: Material> Object for Model<M> {
    fn render(&self, camera: &Camera, lights: &Lights) -> ThreeDResult<()> {
        self.render_with_material(&self.material, camera, lights)
    }

    fn is_transparent(&self) -> bool {
        self.material.is_transparent()
    }
}
