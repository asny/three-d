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
    current_transformation: Mat4,
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
            current_transformation: Mat4::identity(),
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
        self.current_transformation = transformation;
    }

    ///
    /// Specifies a function which takes a time parameter as input and returns a transformation that should be applied to this mesh at the given time.
    /// To actually animate this mesh, call [Geometry::animate] at each frame which in turn evaluates the animation function defined by this method.
    /// This transformation is applied first, then the local to world transformation defined by [Self::set_transformation].
    ///
    pub fn set_animation(&mut self, animation: impl Fn(f32) -> Mat4 + Send + Sync + 'static) {
        self.animation = Some(Box::new(animation));
    }

    ///
    /// Returns the number of vertices in this mesh.
    ///
    pub fn vertex_count(&self) -> u32 {
        self.base_mesh.positions.vertex_count()
    }

    /// Updates the vertex positions of the mesh.
    ///
    /// # Panics
    ///
    /// Panics if the number of positions does not match the number of vertices in the mesh.
    pub fn update_positions(&mut self, positions: &[Vector3<f32>]) {
        if positions.len() as u32 != self.vertex_count() {
            panic!("Failed updating positions: The number of positions {} does not match the number of vertices {} in the mesh.", positions.len(), self.vertex_count())
        }
        self.base_mesh.positions.fill(positions);
    }

    ///
    /// Updates the vertex normals of the mesh.
    ///
    /// # Panics
    ///
    /// Panics if the number of normals does not match the number of vertices in the mesh.
    pub fn update_normals(&mut self, normals: &[Vector3<f32>]) {
        if normals.len() as u32 != self.vertex_count() {
            panic!("Failed updating normals: The number of normals {} does not match the number of vertices {} in the mesh.", normals.len(), self.vertex_count())
        }

        if let Some(normal_buffer) = &mut self.base_mesh.normals {
            normal_buffer.fill(normals);
        } else {
            self.base_mesh.normals = Some(VertexBuffer::new_with_data(&self.context, normals));
        }
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
        let mut aabb = self.aabb;
        aabb.transform(&self.current_transformation);
        aabb
    }

    fn animate(&mut self, time: f32) {
        if let Some(animation) = &self.animation {
            self.current_transformation = self.transformation * animation(time);
        }
    }

    fn draw(
        &self,
        camera: &Camera,
        program: &Program,
        render_states: RenderStates,
        attributes: FragmentAttributes,
    ) {
        if attributes.normal {
            if let Some(inverse) = self.current_transformation.invert() {
                program.use_uniform_if_required("normalMatrix", inverse.transpose());
            } else {
                // determinant is float zero
                return;
            }
        }

        program.use_uniform("viewProjection", camera.projection() * camera.view());
        program.use_uniform("modelMatrix", self.current_transformation);

        self.base_mesh
            .draw(program, render_states, camera, attributes);
    }

    fn vertex_shader_source(&self, required_attributes: FragmentAttributes) -> String {
        format!(
            "{}{}{}{}{}{}",
            if required_attributes.normal {
                "#define USE_NORMALS\n"
            } else {
                ""
            },
            if required_attributes.tangents {
                "#define USE_TANGENTS\n"
            } else {
                ""
            },
            if required_attributes.uv {
                "#define USE_UVS\n"
            } else {
                ""
            },
            if required_attributes.color && self.base_mesh.colors.is_some() {
                "#define USE_VERTEX_COLORS\n"
            } else {
                ""
            },
            include_str!("../../core/shared.frag"),
            include_str!("shaders/mesh.vert"),
        )
    }

    fn id(&self, required_attributes: FragmentAttributes) -> u16 {
        let mut id = 0b1u16 << 15 | 0b1u16 << 4;
        if required_attributes.normal {
            id |= 0b1u16;
        }
        if required_attributes.tangents {
            id |= 0b1u16 << 1;
        }
        if required_attributes.uv {
            id |= 0b1u16 << 2;
        }
        if required_attributes.color && self.base_mesh.colors.is_some() {
            id |= 0b1u16 << 3;
        }
        id
    }

    fn render_with_material(
        &self,
        material: &dyn Material,
        camera: &Camera,
        lights: &[&dyn Light],
    ) {
        render_with_material(&self.context, camera, &self, material, lights);
    }

    fn render_with_effect(
        &self,
        material: &dyn Effect,
        camera: &Camera,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) {
        render_with_effect(
            &self.context,
            camera,
            self,
            material,
            lights,
            color_texture,
            depth_texture,
        )
    }
}
