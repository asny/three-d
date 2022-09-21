use crate::core::*;
use crate::renderer::*;

const VERTICES_PER_SIDE: usize = 33;

pub struct Water {
    context: Context,
    time: f64,
    index_buffer: ElementBuffer,
    position_buffer: VertexBuffer,
    aabb: AxisAlignedBoundingBox,
    side_length: f32,
    vertex_distance: f32,
}
impl Water {
    pub fn new(context: &Context, side_length: f32, vertex_distance: f32) -> Self {
        let index_buffer = Self::indices(context);
        let positions = Self::positions(vec2(0.0, 0.0), vertex_distance);
        let aabb = AxisAlignedBoundingBox::new_with_positions(&positions);
        Self {
            context: context.clone(),
            time: 0.0,
            index_buffer,
            position_buffer: VertexBuffer::new_with_data(context, &positions),
            aabb,
            side_length,
            vertex_distance,
        }
    }

    pub fn update(&mut self, time: f64) {
        self.time = time;
    }

    fn indices(context: &Context) -> ElementBuffer {
        let mut indices: Vec<u32> = Vec::new();
        let stride = VERTICES_PER_SIDE as u32;
        let max = stride - 1;
        for r in 0..max {
            for c in 0..max {
                indices.push(r + c * stride);
                indices.push(r + 1 + c * stride);
                indices.push(r + (c + 1) * stride);
                indices.push(r + (c + 1) * stride);
                indices.push(r + 1 + c * stride);
                indices.push(r + 1 + (c + 1) * stride);
            }
        }
        ElementBuffer::new_with_data(context, &indices)
    }

    fn positions(offset: Vec2, vertex_distance: f32) -> Vec<Vec3> {
        let mut data = vec![vec3(0.0, 0.0, 0.0); VERTICES_PER_SIDE * VERTICES_PER_SIDE];
        for r in 0..VERTICES_PER_SIDE {
            for c in 0..VERTICES_PER_SIDE {
                let vertex_id = r * VERTICES_PER_SIDE + c;
                let x = offset.x + r as f32 * vertex_distance;
                let z = offset.y + c as f32 * vertex_distance;
                data[vertex_id] = vec3(x, 0.0, z);
            }
        }
        data
    }
}

impl Geometry for Water {
    fn render_with_material(
        &self,
        material: &dyn Material,
        camera: &Camera,
        lights: &[&dyn Light],
    ) {
        let fragment_shader_source = material.fragment_shader_source(false, lights);
        self.context
            .program(
                &include_str!("shaders/water.vert"),
                &fragment_shader_source,
                |program| {
                    material.use_uniforms(program, camera, lights);
                    let transformation = Mat4::identity();
                    program.use_uniform("modelMatrix", &transformation);
                    program.use_uniform("projectionMatrix", camera.projection());
                    program.use_uniform("viewMatrix", camera.view());
                    program.use_uniform("time", &(self.time as f32 * 0.001));
                    let render_states = RenderStates {
                        blend: Blend::TRANSPARENCY,
                        ..Default::default()
                    };

                    program.use_vertex_attribute("position", &self.position_buffer);

                    program.draw_elements(render_states, camera.viewport(), &self.index_buffer);
                },
            )
            .unwrap();
    }

    fn aabb(&self) -> AxisAlignedBoundingBox {
        self.aabb
    }
}
