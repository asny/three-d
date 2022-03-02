use crate::core::*;
use crate::renderer::*;

///
/// A 3D model consisting of a triangle mesh and any material that implements the `Material` trait.
///
pub struct Model<M: Material> {
    mesh: Mesh,
    /// The material applied to the model
    pub material: M,
}

impl Model<ColorMaterial<std::rc::Rc<Texture2D<u8>>>> {
    ///
    /// Creates a new 3D model with a triangle mesh as geometry and a default [ColorMaterial].
    ///
    pub fn new(context: &Context, cpu_mesh: &CpuMesh) -> ThreeDResult<Self> {
        Self::new_with_material(context, cpu_mesh, ColorMaterial::default())
    }
}

impl<M: Material> Model<M> {
    ///
    /// Creates a new 3D model with a triangle mesh as geometry and the given material.
    ///
    pub fn new_with_material(
        context: &Context,
        cpu_mesh: &CpuMesh,
        material: M,
    ) -> ThreeDResult<Self> {
        Ok(Self {
            mesh: Mesh::new(context, cpu_mesh)?,
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

impl<M: Material> Geometry for Model<M> {
    fn aabb(&self) -> AxisAlignedBoundingBox {
        self.mesh.aabb()
    }

    fn transformation(&self) -> Mat4 {
        self.mesh.transformation()
    }

    fn render_with_material(
        &self,
        material: &dyn Material,
        camera: &Camera,
        lights: &[&dyn Light],
    ) -> ThreeDResult<()> {
        self.mesh.render_with_material(material, camera, lights)
    }
}

impl<M: Material> GeometryMut for Model<M> {
    fn set_transformation(&mut self, transformation: Mat4) {
        self.mesh.set_transformation(transformation);
    }
}

impl<M: Material> Object for Model<M> {
    fn render(&self, camera: &Camera, lights: &[&dyn Light]) -> ThreeDResult<()> {
        self.render_with_material(&self.material, camera, lights)
    }

    fn is_transparent(&self) -> bool {
        self.material.is_transparent()
    }
}
/*
impl<M: Material + Clone> Clone for Model<M> {
    fn clone(&self) -> Self {
        Self {
            context: self.context.clone(),
            mesh: self.mesh.clone(),
            material: self.material.clone(),
        }
    }
}*/
