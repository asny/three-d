use crate::core::*;
use crate::renderer::*;

pub struct TerrainPatch {
    context: Context,
    ix: i32,
    iy: i32,
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
        ix: i32,
        iy: i32,
        patch_size: f32,
        vertices_per_unit: u32,
    ) -> Self {
        let vertices_per_side = (patch_size + 1.0) as usize * vertices_per_unit as usize;
        let vertex_distance = 1.0 / vertices_per_unit as f32;
        let offset = vec2(ix as f32 * patch_size, iy as f32 * patch_size);
        let positions = Self::positions(height_map, offset, vertices_per_side, vertex_distance);
        let normals = Self::normals(
            height_map,
            offset,
            &positions,
            vertices_per_side,
            vertex_distance,
        );

        let positions_buffer = VertexBuffer::new_with_data(context, &positions);
        let normals_buffer = VertexBuffer::new_with_data(context, &normals);
        let index_buffer =
            ElementBuffer::new_with_data(context, &Self::indices(1, vertices_per_side));
        let coarse_index_buffer =
            ElementBuffer::new_with_data(context, &Self::indices(4, vertices_per_side));
        let very_coarse_index_buffer =
            ElementBuffer::new_with_data(context, &Self::indices(8, vertices_per_side));
        Self {
            context: context.clone(),
            ix,
            iy,
            index_buffer,
            coarse_index_buffer,
            very_coarse_index_buffer,
            positions_buffer,
            normals_buffer,
            patch_size,
        }
    }

    pub fn index(&self) -> (i32, i32) {
        (self.ix, self.iy)
    }

    fn index_buffer(&self, x0: i32, y0: i32) -> &ElementBuffer {
        let dist = (self.ix - x0).abs() + (self.iy - y0).abs();
        if dist > 4 {
            &self.very_coarse_index_buffer
        } else if dist > 8 {
            &self.coarse_index_buffer
        } else {
            &self.index_buffer
        }
    }

    fn indices(resolution: u32, vertices_per_side: usize) -> Vec<u32> {
        let mut indices: Vec<u32> = Vec::new();
        let stride = vertices_per_side as u32;
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
        vertices_per_side: usize,
        vertex_distance: f32,
    ) -> Vec<Vec3> {
        let mut data = vec![vec3(0.0, 0.0, 0.0); vertices_per_side * vertices_per_side];
        for r in 0..vertices_per_side {
            for c in 0..vertices_per_side {
                let vertex_id = r * vertices_per_side + c;
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
        vertices_per_side: usize,
        vertex_distance: f32,
    ) -> Vec<Vec3> {
        let mut data = vec![vec3(0.0, 0.0, 0.0); vertices_per_side * vertices_per_side];
        let h = vertex_distance;
        for r in 0..vertices_per_side {
            for c in 0..vertices_per_side {
                let vertex_id = r * vertices_per_side + c;
                let x = offset.x + r as f32 * vertex_distance;
                let z = offset.y + c as f32 * vertex_distance;
                let xp = if r == vertices_per_side - 1 {
                    height_map(x + h, z)
                } else {
                    positions[vertex_id + vertices_per_side][1]
                };
                let xm = if r == 0 {
                    height_map(x - h, z)
                } else {
                    positions[vertex_id - vertices_per_side][1]
                };
                let zp = if c == vertices_per_side - 1 {
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
                self.ix as f32 * self.patch_size,
                -self.patch_size,
                self.iy as f32 * self.patch_size,
            ),
            vec3(
                (self.ix + 1) as f32 * self.patch_size,
                self.patch_size,
                (self.iy + 1) as f32 * self.patch_size,
            ),
        ])
    }
}
