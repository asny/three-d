use crate::core::*;
use crate::renderer::*;

use super::BaseMesh;

///
/// A triangle mesh [Geometry].
///
pub struct Mesh {
    base_mesh: BaseMesh,
    context: Context,
    aabb: AxisAlignedBoundingBox,
    transformation: Mat4,
    animation_transformation: Mat4,
    animation: Option<Box<dyn Fn(f32) -> Mat4 + Send + Sync>>,
}

impl Mesh {
    ///
    /// Creates a new triangle mesh from the given [CpuMesh].
    /// All data in the [CpuMesh] is transfered to the GPU, so make sure to remove all unnecessary data from the [CpuMesh] before calling this method.
    ///
    pub fn new(context: &Context, cpu_mesh: &CpuMesh) -> Self {
        let aabb = cpu_mesh.compute_aabb();
        Self {
            context: context.clone(),
            base_mesh: BaseMesh::new(context, cpu_mesh),
            aabb,
            transformation: Mat4::identity(),
            animation_transformation: Mat4::identity(),
            animation: None,
        }
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

    ///
    /// Returns the local to world transformation applied to this mesh.
    ///
    pub fn transformation(&self) -> Mat4 {
        self.transformation
    }

    ///
    /// Set the local to world transformation applied to this mesh.
    /// If any animation method is set using [Self::set_animation], the transformation from that method is applied before this transformation.
    ///
    pub fn set_transformation(&mut self, transformation: Mat4) {
        self.transformation = transformation;
    }

    ///
    /// Specifies a function which takes a time parameter as input and returns a transformation that should be applied to this mesh at the given time.
    /// To actually animate this mesh, call [Geometry::animate] at each frame which in turn evaluates the animation function defined by this method.
    /// This transformation is applied first, then the local to world transformation defined by [Self::set_transformation].
    ///
    pub fn set_animation(&mut self, animation: impl Fn(f32) -> Mat4 + Send + Sync + 'static) {
        self.animation = Some(Box::new(animation));
        self.animate(0.0);
    }

    ///
    /// Returns the number of vertices in this mesh.
    ///
    pub fn vertex_count(&self) -> u32 {
        self.base_mesh.positions.vertex_count()
    }

    ///
    /// Used for editing the triangle indices.
    /// Note: Changing this will possibly ruin the mesh.
    ///
    pub fn indices_mut(&mut self) -> &mut IndexBuffer {
        &mut self.base_mesh.indices
    }

    ///
    /// Used for editing the vertex positions.
    /// Note: Changing this will possibly ruin the mesh.
    ///
    pub fn positions_mut(&mut self) -> &mut VertexBuffer<Vec3> {
        &mut self.base_mesh.positions
    }

    ///
    /// Used for editing the vertex normals.
    /// Note: Changing this will possibly ruin the mesh.
    ///
    pub fn normals_mut(&mut self) -> &mut Option<VertexBuffer<Vec3>> {
        &mut self.base_mesh.normals
    }

    ///
    /// Used for editing the vertex uvs.
    /// Note: Changing this will possibly ruin the mesh.
    ///
    pub fn uvs_mut(&mut self) -> &mut Option<VertexBuffer<Vec2>> {
        &mut self.base_mesh.uvs
    }

    ///
    /// Used for editing the vertex tangents.
    /// Note: Changing this will possibly ruin the mesh.
    ///
    pub fn tangents_mut(&mut self) -> &mut Option<VertexBuffer<Vec4>> {
        &mut self.base_mesh.tangents
    }

    ///
    /// Used for editing the vertex colors.
    /// Note: Changing this will possibly ruin the mesh.
    ///
    pub fn colors_mut(&mut self) -> &mut Option<VertexBuffer<Vec4>> {
        &mut self.base_mesh.colors
    }
}

impl<'a> IntoIterator for &'a Mesh {
    type Item = &'a dyn Geometry;
    type IntoIter = std::iter::Once<&'a dyn Geometry>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self)
    }
}

impl Geometry for Mesh {
    fn aabb(&self) -> AxisAlignedBoundingBox {
        self.aabb
            .transformed(self.transformation * self.animation_transformation)
    }

    fn animate(&mut self, time: f32) {
        if let Some(animation) = &self.animation {
            self.animation_transformation = animation(time);
        }
    }

    fn draw(&self, viewer: &dyn Viewer, program: &Program, render_states: RenderStates) {
        let local2world = self.transformation * self.animation_transformation;
        if let Some(inverse) = local2world.invert() {
            program.use_uniform_if_required("normalMatrix", inverse.transpose());
        } else {
            // determinant is float zero
            return;
        }

        program.use_uniform("viewProjection", viewer.projection() * viewer.view());
        program.use_uniform("modelMatrix", local2world);

        self.base_mesh.draw(program, render_states, viewer);
    }

    fn vertex_shader_source(&self) -> String {
        self.base_mesh.vertex_shader_source()
    }

    fn id(&self) -> GeometryId {
        GeometryId::Mesh(
            self.base_mesh.normals.is_some(),
            self.base_mesh.tangents.is_some(),
            self.base_mesh.uvs.is_some(),
            self.base_mesh.colors.is_some(),
        )
    }

    fn render_with_material(
        &self,
        material: &dyn Material,
        viewer: &dyn Viewer,
        lights: &[&dyn Light],
    ) {
        render_with_material(&self.context, viewer, &self, material, lights);
    }

    fn render_with_effect(
        &self,
        material: &dyn Effect,
        viewer: &dyn Viewer,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) {
        render_with_effect(
            &self.context,
            viewer,
            self,
            material,
            lights,
            color_texture,
            depth_texture,
        )
    }
}
