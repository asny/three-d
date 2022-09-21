use crate::core::*;
use crate::renderer::*;

pub struct Water {
    context: Context,
    time: f64,
    index_buffer: ElementBuffer,
    position_buffer: VertexBuffer,
    aabb: AxisAlignedBoundingBox,
}
impl Water {
    pub fn new(context: &Context, side_length: f32, vertex_distance: f32) -> Self {
        let vertices_per_side = (side_length / vertex_distance).ceil() as u32 + 1;
        let index_buffer = Self::indices(context, vertices_per_side);
        let positions = Self::positions(side_length, vertex_distance, vertices_per_side);
        let aabb = AxisAlignedBoundingBox::new_with_positions(&positions);
        Self {
            context: context.clone(),
            time: 0.0,
            index_buffer,
            position_buffer: VertexBuffer::new_with_data(context, &positions),
            aabb,
        }
    }

    pub fn update(&mut self, time: f64) {
        self.time = time;
    }

    fn indices(context: &Context, vertices_per_side: u32) -> ElementBuffer {
        let mut indices: Vec<u32> = Vec::new();
        let stride = vertices_per_side;
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

    fn positions(side_length: f32, vertex_distance: f32, vertices_per_side: u32) -> Vec<Vec3> {
        let mut data = vec![vec3(0.0, 0.0, 0.0); (vertices_per_side * vertices_per_side) as usize];
        for r in 0..vertices_per_side {
            for c in 0..vertices_per_side {
                let vertex_id = r * vertices_per_side + c;
                let x = r as f32 * vertex_distance - 0.5 * side_length;
                let z = c as f32 * vertex_distance - 0.5 * side_length;
                data[vertex_id as usize] = vec3(x, 0.0, z);
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
