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
        self.base_mesh.indices.vertex_count()
    }

    ///
    /// Used for editing the triangle indices.
    /// Note: Changing this will possibly ruin the mesh.
    ///
    pub fn indices_mut(&mut self) -> &mut IndexBuffer {
        &mut self.base_mesh.indices
    }

    ///
    /// Update the vertex positions of the mesh.
    /// Returns an error if the number of positions does not match the number of vertices in the mesh.
    ///
    pub fn set_positions(&mut self, positions: &[Vec3]) -> Result<(), RendererError> {
        if positions.len() as u32 != self.vertex_count() {
            Err(RendererError::InvalidBufferLength(
                "Position".to_string(),
                self.vertex_count() as usize,
                positions.len(),
            ))?;
        }
        self.base_mesh.positions.fill(positions);
        self.aabb = AxisAlignedBoundingBox::new_with_positions(positions);
        Ok(())
    }

    ///
    /// Partially update the vertex positions of the mesh.
    /// Returns an error if the number of positions plus the offset is larger than the number of vertices in the mesh.
    ///
    pub fn set_positions_partially(
        &mut self,
        offset: u32,
        positions: &[Vec3],
    ) -> Result<(), RendererError> {
        if offset + positions.len() as u32 > self.vertex_count() {
            Err(RendererError::InvalidBufferLength(
                "Position".to_string(),
                self.vertex_count() as usize,
                offset as usize + positions.len(),
            ))?;
        }
        self.base_mesh.positions.fill_subset(offset, positions);
        self.aabb.expand(positions);
        Ok(())
    }

    ///
    /// Used for editing the vertex positions.
    /// Note: Changing this will possibly ruin the mesh.
    ///
    #[deprecated = "use set_positions and set_positions_partially instead"]
    pub fn positions_mut(&mut self) -> &mut VertexBuffer<Vec3> {
        &mut self.base_mesh.positions
    }

    ///
    /// Update the vertex normals of the mesh.
    /// Returns an error if the number of normals does not match the number of vertices in the mesh.
    ///
    pub fn set_normals(&mut self, normals: &[Vec3]) -> Result<(), RendererError> {
        if normals.len() as u32 != self.vertex_count() {
            Err(RendererError::InvalidBufferLength(
                "Normal".to_string(),
                self.vertex_count() as usize,
                normals.len(),
            ))?;
        }
        if let Some(buffer) = self.base_mesh.normals.as_mut() {
            buffer.fill(normals);
        } else {
            self.base_mesh.normals = Some(VertexBuffer::new_with_data(&self.context, normals));
        }
        Ok(())
    }

    ///
    /// Partially update the vertex normals of the mesh.
    /// Returns an error if the number of normals plus the offset is larger than the number of vertices in the mesh
    /// or if the normal buffer is missing.
    ///
    pub fn set_normals_partially(
        &mut self,
        offset: u32,
        normals: &[Vec3],
    ) -> Result<(), RendererError> {
        if offset + normals.len() as u32 > self.vertex_count() {
            Err(RendererError::InvalidBufferLength(
                "Normal".to_string(),
                self.vertex_count() as usize,
                offset as usize + normals.len(),
            ))?;
        }
        if let Some(buffer) = self.base_mesh.normals.as_mut() {
            buffer.fill_subset(offset, normals);
        } else {
            Err(RendererError::PartialUpdateFailedMissingBuffer(
                "Normal".to_string(),
            ))?;
        }
        Ok(())
    }

    ///
    /// Used for editing the vertex normals.
    /// Note: Changing this will possibly ruin the mesh.
    ///
    #[deprecated = "use set_normals and set_normals_partially instead"]
    pub fn normals_mut(&mut self) -> &mut Option<VertexBuffer<Vec3>> {
        &mut self.base_mesh.normals
    }

    ///
    /// Update the vertex UVs of the mesh.
    /// Returns an error if the number of UVs does not match the number of vertices in the mesh.
    ///
    pub fn set_uvs(&mut self, uvs: &[Vec2]) -> Result<(), RendererError> {
        if uvs.len() as u32 != self.vertex_count() {
            Err(RendererError::InvalidBufferLength(
                "UV".to_string(),
                self.vertex_count() as usize,
                uvs.len(),
            ))?;
        }
        if let Some(buffer) = self.base_mesh.uvs.as_mut() {
            buffer.fill(uvs);
        } else {
            self.base_mesh.uvs = Some(VertexBuffer::new_with_data(&self.context, uvs));
        }
        Ok(())
    }

    ///
    /// Partially update the vertex UVs of the mesh.
    /// Returns an error if the number of UVs plus the offset is larger than the number of vertices in the mesh
    /// or if the UV buffer is missing.
    ///
    pub fn set_uvs_partially(&mut self, offset: u32, uvs: &[Vec2]) -> Result<(), RendererError> {
        if offset + uvs.len() as u32 > self.vertex_count() {
            Err(RendererError::InvalidBufferLength(
                "UV".to_string(),
                self.vertex_count() as usize,
                offset as usize + uvs.len(),
            ))?;
        }
        if let Some(buffer) = self.base_mesh.uvs.as_mut() {
            buffer.fill_subset(offset, uvs);
        } else {
            Err(RendererError::PartialUpdateFailedMissingBuffer(
                "UV".to_string(),
            ))?;
        }
        Ok(())
    }

    ///
    /// Used for editing the vertex uvs.
    /// Note: Changing this will possibly ruin the mesh.
    ///
    #[deprecated = "use set_uvs and set_uvs_partially instead"]
    pub fn uvs_mut(&mut self) -> &mut Option<VertexBuffer<Vec2>> {
        &mut self.base_mesh.uvs
    }

    ///
    /// Update the vertex tangents of the mesh.
    /// Returns an error if the number of tangents does not match the number of vertices in the mesh.
    ///
    pub fn set_tangents(&mut self, tangents: &[Vec4]) -> Result<(), RendererError> {
        if tangents.len() as u32 != self.vertex_count() {
            Err(RendererError::InvalidBufferLength(
                "Tangents".to_string(),
                self.vertex_count() as usize,
                tangents.len(),
            ))?;
        }
        if let Some(buffer) = self.base_mesh.tangents.as_mut() {
            buffer.fill(tangents);
        } else {
            self.base_mesh.tangents = Some(VertexBuffer::new_with_data(&self.context, tangents));
        }
        Ok(())
    }

    ///
    /// Partially update the vertex tangents of the mesh.
    /// Returns an error if the number of tangents plus the offset is larger than the number of vertices in the mesh
    /// or if the tangent buffer is missing.
    ///
    pub fn set_tangents_partially(
        &mut self,
        offset: u32,
        tangents: &[Vec4],
    ) -> Result<(), RendererError> {
        if offset + tangents.len() as u32 > self.vertex_count() {
            Err(RendererError::InvalidBufferLength(
                "Tangent".to_string(),
                self.vertex_count() as usize,
                offset as usize + tangents.len(),
            ))?;
        }
        if let Some(buffer) = self.base_mesh.tangents.as_mut() {
            buffer.fill_subset(offset, tangents);
        } else {
            Err(RendererError::PartialUpdateFailedMissingBuffer(
                "Tangent".to_string(),
            ))?;
        }
        Ok(())
    }

    ///
    /// Used for editing the vertex tangents.
    /// Note: Changing this will possibly ruin the mesh.
    ///
    #[deprecated = "use set_tangents and set_tangents_partially instead"]
    pub fn tangents_mut(&mut self) -> &mut Option<VertexBuffer<Vec4>> {
        &mut self.base_mesh.tangents
    }

    ///
    /// Update the vertex colors of the mesh.
    /// Returns an error if the number of colors does not match the number of vertices in the mesh.
    ///
    pub fn set_colors(&mut self, colors: &[Vec4]) -> Result<(), RendererError> {
        if colors.len() as u32 != self.vertex_count() {
            Err(RendererError::InvalidBufferLength(
                "Color".to_string(),
                self.vertex_count() as usize,
                colors.len(),
            ))?;
        }
        if let Some(buffer) = self.base_mesh.colors.as_mut() {
            buffer.fill(colors);
        } else {
            self.base_mesh.colors = Some(VertexBuffer::new_with_data(&self.context, colors));
        }
        Ok(())
    }

    ///
    /// Partially update the vertex colors of the mesh.
    /// Returns an error if the number of colors plus the offset is larger than the number of vertices in the mesh
    /// or if the colors buffer is missing.
    ///
    pub fn set_colors_partially(
        &mut self,
        offset: u32,
        colors: &[Vec4],
    ) -> Result<(), RendererError> {
        if offset + colors.len() as u32 > self.vertex_count() {
            Err(RendererError::InvalidBufferLength(
                "Color".to_string(),
                self.vertex_count() as usize,
                offset as usize + colors.len(),
            ))?;
        }
        if let Some(buffer) = self.base_mesh.colors.as_mut() {
            buffer.fill_subset(offset, colors);
        } else {
            Err(RendererError::PartialUpdateFailedMissingBuffer(
                "Color".to_string(),
            ))?;
        }
        Ok(())
    }

    ///
    /// Used for editing the vertex colors.
    /// Note: Changing this will possibly ruin the mesh.
    ///
    #[deprecated = "use set_colors and set_colors_partially instead"]
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
        if let Err(e) = render_with_material(&self.context, viewer, &self, material, lights) {
            panic!("{}", e.to_string());
        }
    }

    fn render_with_effect(
        &self,
        material: &dyn Effect,
        viewer: &dyn Viewer,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) {
        if let Err(e) = render_with_effect(
            &self.context,
            viewer,
            self,
            material,
            lights,
            color_texture,
            depth_texture,
        ) {
            panic!("{}", e.to_string());
        }
    }
}
