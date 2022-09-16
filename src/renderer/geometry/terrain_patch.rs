use crate::core::*;
use crate::renderer::*;

const VERTICES_PER_SIDE: usize = 65;

pub struct TerrainPatch {
    context: Context,
    index: (i32, i32),
    index_buffer: ElementBuffer,
    coarse_index_buffer: ElementBuffer,
    very_coarse_index_buffer: ElementBuffer,
    positions_buffer: VertexBuffer,
    normals_buffer: VertexBuffer,
    patch_size: f32,
}

impl TerrainPatch {
    pub fn new(
        context: &Context,
        height_map: &impl Fn(f32, f32) -> f32,
        index: (i32, i32),
        patch_size: f32,
    ) -> Self {
        let vertex_distance = patch_size / (VERTICES_PER_SIDE - 1) as f32;
        let offset = vec2(index.0 as f32 * patch_size, index.1 as f32 * patch_size);
        let positions = Self::positions(height_map, offset, vertex_distance);
        let normals = Self::normals(height_map, offset, &positions, vertex_distance);

        let positions_buffer = VertexBuffer::new_with_data(context, &positions);
        let normals_buffer = VertexBuffer::new_with_data(context, &normals);
        let index_buffer = ElementBuffer::new_with_data(context, &Self::indices(1));
        let coarse_index_buffer = ElementBuffer::new_with_data(context, &Self::indices(4));
        let very_coarse_index_buffer = ElementBuffer::new_with_data(context, &Self::indices(8));
        Self {
            context: context.clone(),
            index,
            index_buffer,
            coarse_index_buffer,
            very_coarse_index_buffer,
            positions_buffer,
            normals_buffer,
            patch_size,
        }
    }

    pub fn index(&self) -> (i32, i32) {
        self.index
    }

    fn index_buffer(&self, x0: i32, y0: i32) -> &ElementBuffer {
        let dist = (self.index.0 - x0).abs() + (self.index.1 - y0).abs();
        if dist > 4 {
            &self.very_coarse_index_buffer
        } else if dist > 8 {
            &self.coarse_index_buffer
        } else {
            &self.index_buffer
        }
    }

    fn indices(resolution: u32) -> Vec<u32> {
        let mut indices: Vec<u32> = Vec::new();
        let stride = VERTICES_PER_SIDE as u32;
        let max = (stride - 1) / resolution;
        for r in 0..max {
            for c in 0..max {
                indices.push(r * resolution + c * resolution * stride);
                indices.push(r * resolution + resolution + c * resolution * stride);
                indices.push(r * resolution + (c * resolution + resolution) * stride);
                indices.push(r * resolution + (c * resolution + resolution) * stride);
                indices.push(r * resolution + resolution + c * resolution * stride);
                indices.push(r * resolution + resolution + (c * resolution + resolution) * stride);
            }
        }
        indices
    }

    fn positions(
        height_map: &impl Fn(f32, f32) -> f32,
        offset: Vec2,
        vertex_distance: f32,
    ) -> Vec<Vec3> {
        let mut data = vec![vec3(0.0, 0.0, 0.0); VERTICES_PER_SIDE * VERTICES_PER_SIDE];
        for r in 0..VERTICES_PER_SIDE {
            for c in 0..VERTICES_PER_SIDE {
                let vertex_id = r * VERTICES_PER_SIDE + c;
                let x = offset.x + r as f32 * vertex_distance;
                let z = offset.y + c as f32 * vertex_distance;
                data[vertex_id] = vec3(x, height_map(x, z), z);
            }
        }
        data
    }

    fn normals(
        height_map: &impl Fn(f32, f32) -> f32,
        offset: Vec2,
        positions: &Vec<Vec3>,
        vertex_distance: f32,
    ) -> Vec<Vec3> {
        let mut data = vec![vec3(0.0, 0.0, 0.0); VERTICES_PER_SIDE * VERTICES_PER_SIDE];
        let h = vertex_distance;
        for r in 0..VERTICES_PER_SIDE {
            for c in 0..VERTICES_PER_SIDE {
                let vertex_id = r * VERTICES_PER_SIDE + c;
                let x = offset.x + r as f32 * vertex_distance;
                let z = offset.y + c as f32 * vertex_distance;
                let xp = if r == VERTICES_PER_SIDE - 1 {
                    height_map(x + h, z)
                } else {
                    positions[vertex_id + VERTICES_PER_SIDE][1]
                };
                let xm = if r == 0 {
                    height_map(x - h, z)
                } else {
                    positions[vertex_id - VERTICES_PER_SIDE][1]
                };
                let zp = if c == VERTICES_PER_SIDE - 1 {
                    height_map(x, z + h)
                } else {
                    positions[vertex_id + 1][1]
                };
                let zm = if c == 0 {
                    height_map(x, z - h)
                } else {
                    positions[vertex_id - 1][1]
                };
                let dx = xp - xm;
                let dz = zp - zm;
                data[vertex_id] = vec3(-dx, 2.0 * h, -dz).normalize();
            }
        }
        data
    }
}

impl Geometry for TerrainPatch {
    fn render_with_material(
        &self,
        material: &dyn Material,
        camera: &Camera,
        lights: &[&dyn Light],
    ) {
        let x0 = (camera.position().x / self.patch_size).floor() as i32;
        let y0 = (camera.position().z / self.patch_size).floor() as i32;
        let fragment_shader_source = material.fragment_shader_source(false, lights);
        self.context
            .program(
                &include_str!("shaders/terrain.vert"),
                &fragment_shader_source,
                |program| {
                    material.use_uniforms(program, camera, lights);
                    let transformation = Mat4::identity();
                    program.use_uniform("modelMatrix", &transformation);
                    program.use_uniform(
                        "viewProjectionMatrix",
                        &(camera.projection() * camera.view()),
                    );
                    program.use_uniform(
                        "normalMatrix",
                        &transformation.invert().unwrap().transpose(),
                    );
                    let render_states = RenderStates {
                        cull: Cull::Back,
                        ..Default::default()
                    };

                    program.use_vertex_attribute("position", &self.positions_buffer);
                    program.use_vertex_attribute("normal", &self.normals_buffer);

                    program.draw_elements(
                        render_states,
                        camera.viewport(),
                        &self.index_buffer(x0, y0),
                    );
                },
            )
            .unwrap();
    }

    fn aabb(&self) -> AxisAlignedBoundingBox {
        AxisAlignedBoundingBox::new_with_positions(&[
            vec3(
                self.index.0 as f32 * self.patch_size,
                -self.patch_size,
                self.index.1 as f32 * self.patch_size,
            ),
            vec3(
                (self.index.0 + 1) as f32 * self.patch_size,
                self.patch_size,
                (self.index.1 + 1) as f32 * self.patch_size,
            ),
        ])
    }
}
