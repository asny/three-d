use crate::core::*;
use crate::renderer::*;

pub struct Sprites {
    context: Context,
    mesh: Mesh,
    center_buffer: InstanceBuffer,
    instance_count: u32,
    transformation: Mat4,
}

impl Sprites {
    pub fn new(context: &Context, centers: &[f32]) -> ThreeDResult<Self> {
        let center_buffer = InstanceBuffer::new_with_dynamic(context, centers)?;
        Ok(Self {
            context: context.clone(),
            mesh: Mesh::new(context, &CpuMesh::square())?,
            center_buffer,
            instance_count: centers.len() as u32,
            transformation: Mat4::identity(),
        })
    }
}

impl Geometry for Sprites {
    fn render_with_material(
        &self,
        material: &dyn Material,
        camera: &Camera,
        lights: &[&dyn Light],
    ) -> ThreeDResult<()> {
        let fragment_shader_source =
            material.fragment_shader_source(self.mesh.color_buffer.is_some(), lights);
        self.context.program(
            &include_str!("shaders/sprites.vert"),
            &fragment_shader_source,
            |program| {
                let render_states = RenderStates {
                    blend: Blend::TRANSPARENCY,
                    cull: Cull::Back,
                    ..Default::default()
                };
                material.use_uniforms(program, camera, lights)?;
                program.use_uniform_block("Camera", camera.uniform_buffer());
                program.use_attribute_vec3("position", &self.mesh.position_buffer)?;
                program
                    .use_attribute_vec2("uv_coordinate", self.mesh.uv_buffer.as_ref().unwrap())?;
                program.use_attribute_vec3_instanced("center", &self.center_buffer)?;
                program.draw_elements_instanced(
                    render_states,
                    camera.viewport(),
                    self.mesh.index_buffer.as_ref().unwrap(),
                    self.instance_count,
                );
                Ok(())
            },
        )
    }

    fn aabb(&self) -> AxisAlignedBoundingBox {
        AxisAlignedBoundingBox::INFINITE
    }

    fn transformation(&self) -> Mat4 {
        self.transformation
    }
}

impl GeometryMut for Sprites {
    fn set_transformation(&mut self, transformation: Mat4) {
        self.transformation = transformation;
    }
}
