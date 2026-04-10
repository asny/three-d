use crate::core::*;
use crate::renderer::*;

pub struct Wireframe {
    material: WireframeMaterial,
    positions: VertexBuffer<Vec3>,
    barycentric: VertexBuffer<Vec3>,
    context: Context,
    aabb: AxisAlignedBoundingBox,
    transformation: Mat4,
}

impl Wireframe {
    pub fn new(context: &Context, mesh: &CpuMesh, line_width: f32) -> Self {
        let positions = VertexBuffer::new_with_data(context, &mesh.positions.to_f32());
        let barycentric = VertexBuffer::new_with_data(
            context,
            &(0..positions.count() / 3)
                .flat_map(|_| [vec3(1., 0., 0.), vec3(0., 1., 0.), vec3(0., 0., 1.)])
                .collect::<Vec<_>>(),
        );
        Self {
            material: material::WireframeMaterial { line_width },
            positions,
            barycentric,
            context: context.clone(),
            aabb: mesh.compute_aabb(),
            transformation: Mat4::identity(),
        }
    }
    pub fn set_transformation(&mut self, transformation: Mat4) {
        self.transformation = transformation
    }
    pub fn set_line_width(&mut self, line_width: f32) {
        self.material.line_width = line_width;
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
        program.draw_arrays(render_states, viewer.viewport(), self.positions.count());
    }

    fn vertex_shader_source(&self) -> String {
        r#"
uniform mat4 viewProjection;
uniform mat4 modelMatrix;

in vec3 position;
in vec3 barycentric;

out vec3 pos;
out vec3 bary;

void main() {
    vec4 worldPos = modelMatrix * vec4(position, 1.);
    pos = worldPos.xyz;
    bary = barycentric;
    gl_Position = viewProjection * worldPos;
}
        "#
        .into()
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
