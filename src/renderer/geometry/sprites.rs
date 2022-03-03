use crate::core::*;
use crate::renderer::*;

pub struct Sprites {
    context: Context,
    position_buffer: VertexBuffer<f32>,
    uv_buffer: VertexBuffer<f32>,
    center_buffer: InstanceBuffer,
    instance_count: u32,
    transformation: Mat4,
}

impl Sprites {
    pub fn new(context: &Context, centers: &[f32]) -> ThreeDResult<Self> {
        let position_buffer = VertexBuffer::new_with(
            &context,
            &[
                vec3(-1.0, -1.0, 0.0),
                vec3(1.0, -1.0, 0.0),
                vec3(1.0, 1.0, 0.0),
                vec3(1.0, 1.0, 0.0),
                vec3(-1.0, 1.0, 0.0),
                vec3(-1.0, -1.0, 0.0),
            ],
        )?;
        let uv_buffer = VertexBuffer::new_with(
            &context,
            &[
                vec2(0.0, 0.0),
                vec2(1.0, 0.0),
                vec2(1.0, 1.0),
                vec2(1.0, 1.0),
                vec2(0.0, 1.0),
                vec2(0.0, 0.0),
            ],
        )?;
        Ok(Self {
            context: context.clone(),
            position_buffer,
            uv_buffer,
            center_buffer: InstanceBuffer::new_with_dynamic(context, centers)?,
            instance_count: centers.len() as u32 / 3,
            transformation: Mat4::identity(),
        })
    }

    ///
    /// Returns the local to world transformation applied to all sprites.
    ///
    pub fn transformation(&self) -> Mat4 {
        self.transformation
    }

    ///
    /// Set the local to world transformation applied to all sprites.
    ///
    pub fn set_transformation(&mut self, transformation: Mat4) {
        self.transformation = transformation;
    }

    pub fn set_centers(&mut self, centers: &[f32]) {
        self.instance_count = centers.len() as u32 / 3;
        self.center_buffer.fill_with_dynamic(centers);
    }
}

impl Geometry for Sprites {
    fn render_with_material(
        &self,
        material: &dyn Material,
        camera: &Camera,
        lights: &[&dyn Light],
    ) -> ThreeDResult<()> {
        let fragment_shader_source = material.fragment_shader_source(false, lights);
        self.context.program(
            &include_str!("shaders/sprites.vert"),
            &fragment_shader_source,
            |program| {
                material.use_uniforms(program, camera, lights)?;
                program.use_uniform_block("Camera", camera.uniform_buffer());
                program.use_uniform("transformation", self.transformation)?;
                program.use_attribute_vec3("position", &self.position_buffer)?;
                program.use_attribute_vec2("uv_coordinate", &self.uv_buffer)?;
                program.use_attribute_vec3_instanced("center", &self.center_buffer)?;
                program.draw_arrays_instanced(
                    material.render_states(),
                    camera.viewport(),
                    6,
                    self.instance_count,
                );
                Ok(())
            },
        )
    }

    fn aabb(&self) -> AxisAlignedBoundingBox {
        AxisAlignedBoundingBox::INFINITE
    }
}
