use crate::core::*;
use crate::renderer::*;

/// Renders a triangle mesh as a screen-space wireframe.
///
/// The wireframe is generated from per-vertex barycentric coordinates and drawn
/// on top of triangle faces in the fragment shader.
pub struct Wireframe {
    material: WireframeMaterial,
    indices: IndexBuffer,
    positions: VertexBuffer<Vec3>,
    barycentric: VertexBuffer<Vec3>,
    context: Context,
    aabb: AxisAlignedBoundingBox,
    transformation: Mat4,
}

impl Wireframe {
    /// Creates a new wireframe object from a CPU mesh.
    pub fn new(context: &Context, cpu_mesh: &CpuMesh, line_width: f32, line_color: Srgba) -> Self {
        let positions = VertexBuffer::new_with_data(context, &cpu_mesh.positions.to_f32());
        let barycentric = VertexBuffer::new_with_data(
            context,
            &(0..positions.count() / 3)
                .flat_map(|_| [vec3(1., 0., 0.), vec3(0., 1., 0.), vec3(0., 0., 1.)])
                .collect::<Vec<_>>(),
        );
        Self {
            material: material::WireframeMaterial {
                line_width,
                line_color,
            },
            indices: IndexBuffer::new(context, cpu_mesh),
            positions,
            barycentric,
            context: context.clone(),
            aabb: cpu_mesh.compute_aabb(),
            transformation: Mat4::identity(),
        }
    }

    /// Sets the model transformation applied when rendering.
    pub fn set_transformation(&mut self, transformation: Mat4) {
        self.transformation = transformation
    }

    /// Sets the wire thickness.
    pub fn set_wire_width(&mut self, width: f32) {
        self.material.line_width = width;
    }

    /// Sets the wire color.
    pub fn set_wire_color(&mut self, color: Srgba) {
        self.material.line_color = color;
    }
}

impl<'a> IntoIterator for &'a Wireframe {
    type Item = &'a dyn Object;
    type IntoIter = std::iter::Once<&'a dyn Object>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self)
    }
}

impl Object for Wireframe {
    fn render(&self, viewer: &dyn Viewer, lights: &[&dyn Light]) {
        if let Err(e) = render_with_material(&self.context, viewer, self, &self.material, lights) {
            panic!("{}", e.to_string());
        }
    }

    fn material_type(&self) -> MaterialType {
        self.material.material_type()
    }
}

impl Geometry for Wireframe {
    fn draw(&self, viewer: &dyn Viewer, program: &Program, render_states: RenderStates) {
        program.use_uniform("viewProjection", viewer.projection() * viewer.view());
        program.use_uniform("modelMatrix", self.transformation);
        program.use_vertex_attribute("position", &self.positions);
        program.use_vertex_attribute("barycentric", &self.barycentric);
        self.indices.draw(program, render_states, viewer);
    }

    fn vertex_shader_source(&self) -> String {
        include_str!("shaders/wireframe.vert").to_owned()
    }

    fn id(&self) -> GeometryId {
        GeometryId::Wireframe
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

    fn aabb(&self) -> AxisAlignedBoundingBox {
        self.aabb.transformed(self.transformation)
    }
}
