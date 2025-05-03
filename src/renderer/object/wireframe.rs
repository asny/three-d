use crate::core::*;
use crate::renderer::*;

pub struct Wireframe{
    width: f32,
    positions: VertexBuffer<Vec3>,
    barycentric: VertexBuffer<Vec3>,
    context:  Context,
    aabb: AxisAlignedBoundingBox,
    transformation: Mat4,
}
impl Wireframe {
    pub fn new(context: &Context, mesh: &CpuMesh, width: f32) -> Self {
        let positions = VertexBuffer::new_with_data(
            context,
            &mesh.positions.to_f32()
        );
        let barycentric = VertexBuffer::new_with_data(
            context,
            &(0..positions.count()/3).flat_map(|_| [
                vec3(1., 0., 0.),
                vec3(0., 1., 0.),
                vec3(0., 0., 1.),
            ]).collect::<Vec<_>>()
        );
        Self {
            width,
            positions,
            barycentric,
            context: context.clone(),
            aabb: mesh.compute_aabb(),
            transformation: Mat4::identity(),
        }
    }
    pub fn transform_mut(&mut self) -> &mut Mat4 {
        &mut self.transformation
    }
    pub fn line_width_mut(&mut self) -> &mut f32 {
        &mut self.width
    }
}

struct WireframeMaterial(f32);

impl Object for Wireframe {
    fn render(&self, viewer: &dyn Viewer, lights: &[&dyn Light]) {
        render_with_material(&self.context, viewer, &self, WireframeMaterial(self.width), lights);
    }

    fn material_type(&self) -> MaterialType {
        WireframeMaterial(0.).material_type()
    }
}

impl Material for WireframeMaterial {
    fn fragment_shader_source(&self, lights: &[&dyn Light]) -> String {
        r#"
layout (location = 0) out vec4 outColor;

uniform float u_line_width = 0.5;

in vec3 bary;
in vec3 pos;

void main() {
    vec3 d = fwidth(bary);
    vec3 f = step(d * u_line_width, bary);
    float b = min(min(f.x, f.y), f.z);
    outColor = vec4(1.-b);
}
            "#.into()
    }

    fn id(&self) -> EffectMaterialId {
        //TODO!
        EffectMaterialId(0)
    }

    fn use_uniforms(&self, program: &Program, viewer: &dyn Viewer, lights: &[&dyn Light]) {
        program.use_uniform("u_line_width", self.0);
    }

    fn render_states(&self) -> RenderStates {
        RenderStates {
            depth_test: DepthTest::Always,
            blend: Blend::TRANSPARENCY,
            cull: Cull::None,
            ..Default::default()
        }
    }

    fn material_type(&self) -> MaterialType {
        MaterialType::Transparent
    }
}

impl Geometry for Wireframe {
    fn draw(&self, viewer: &dyn Viewer, program: &Program, render_states: RenderStates) {
        program.use_uniform("viewProjection", viewer.projection() * viewer.view());
        program.use_uniform("modelMatrix", self.transformation);
        program.use_vertex_attribute("position", &self.positions);
        program.use_vertex_attribute("barycentric", &self.barycentric);
        program.draw_arrays(
            render_states,
            viewer.viewport(),
            self.positions.count(),
        );
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
        "#.into()
    }

    fn id(&self) -> GeometryId {
        //TODO!
        GeometryId(0)
    }

    fn render_with_material(
        &self,
        _material: &dyn Material,
        _viewer: &dyn Viewer,
        _lights: &[&dyn Light],
    ) {
        panic!("Wireframes must be rendered with the built-in material.");
    }

    fn render_with_effect(
        &self,
        _material: &dyn Effect,
        _viewer: &dyn Viewer,
        _lights: &[&dyn Light],
        _color_texture: Option<ColorTexture>,
        _depth_texture: Option<DepthTexture>,
    ) {
        panic!("Wireframes do not support effects.");
    }

    fn aabb(&self) -> AxisAlignedBoundingBox {
        self.aabb.transformed(self.transformation)
    }
}
