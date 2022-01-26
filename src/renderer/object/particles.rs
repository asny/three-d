use crate::core::*;
use crate::renderer::*;

///
/// Used to define the initial position and velocity of a particle in [Particles](Particles).
///
pub struct ParticleData {
    /// Initial position of the particle.
    pub start_position: Vec3,
    /// Initial velocity of the particle.
    pub start_velocity: Vec3,
}

///
/// Particle effect that can be rendered with any material.
///
/// Each particle is initialised with a position and velocity using the [update](Particles::update) function and a global acceleration.
/// Then when time passes, their position is updated based on
/// `new_position = start_position + start_velocity * time + 0.5 * acceleration * time * time`
///
pub struct Particles {
    context: Context,
    start_position_buffer: InstanceBuffer,
    start_velocity_buffer: InstanceBuffer,
    position_buffer: VertexBuffer,
    normal_buffer: Option<VertexBuffer>,
    uv_buffer: Option<VertexBuffer>,
    index_buffer: Option<ElementBuffer>,
    /// The acceleration applied to all particles. Default is gravity.
    pub acceleration: Vec3,
    instance_count: u32,
    transformation: Mat4,
    normal_transformation: Mat4,
    /// A time variable that should be updated each frame.
    pub time: f32,
}

impl Particles {
    ///
    /// Creates a new set of particles with geometry defined by the given cpu mesh.
    ///
    pub fn new(context: &Context, cpu_mesh: &CPUMesh) -> ThreeDResult<Self> {
        cpu_mesh.validate()?;
        let position_buffer = VertexBuffer::new_with_static(context, &cpu_mesh.positions)?;
        let normal_buffer = if let Some(ref normals) = cpu_mesh.normals {
            Some(VertexBuffer::new_with_static(context, normals)?)
        } else {
            None
        };
        let index_buffer = if let Some(ref indices) = cpu_mesh.indices {
            Some(match indices {
                Indices::U8(ind) => ElementBuffer::new_with(context, ind)?,
                Indices::U16(ind) => ElementBuffer::new_with(context, ind)?,
                Indices::U32(ind) => ElementBuffer::new_with(context, ind)?,
            })
        } else {
            None
        };
        let uv_buffer = if let Some(ref uvs) = cpu_mesh.uvs {
            Some(VertexBuffer::new_with_static(context, uvs)?)
        } else {
            None
        };

        Ok(Self {
            context: context.clone(),
            position_buffer,
            index_buffer,
            normal_buffer,
            uv_buffer,
            start_position_buffer: InstanceBuffer::new(context)?,
            start_velocity_buffer: InstanceBuffer::new(context)?,
            acceleration: vec3(0.0, -9.82, 0.0),
            instance_count: 0,
            transformation: Mat4::identity(),
            normal_transformation: Mat4::identity(),
            time: 0.0,
        })
    }

    ///
    /// Updates the particles with the given initial data.
    /// The list contain one entry for each particle.
    ///
    pub fn update(&mut self, data: &[ParticleData]) {
        let mut start_position = Vec::new();
        let mut start_velocity = Vec::new();
        for particle in data {
            start_position.push(particle.start_position.x);
            start_position.push(particle.start_position.y);
            start_position.push(particle.start_position.z);
            start_velocity.push(particle.start_velocity.x);
            start_velocity.push(particle.start_velocity.y);
            start_velocity.push(particle.start_velocity.z);
        }
        self.start_position_buffer
            .fill_with_dynamic(&start_position);
        self.start_velocity_buffer
            .fill_with_dynamic(&start_velocity);
        self.instance_count = data.len() as u32;
    }

    pub(crate) fn vertex_shader_source(fragment_shader_source: &str) -> String {
        let use_positions = fragment_shader_source.find("in vec3 pos;").is_some();
        let use_normals = fragment_shader_source.find("in vec3 nor;").is_some();
        let use_uvs = fragment_shader_source.find("in vec2 uvs;").is_some();
        format!("
                layout (std140) uniform Camera
                {{
                    mat4 viewProjection;
                    mat4 view;
                    mat4 projection;
                    vec3 position;
                    float padding;
                }} camera;

                uniform float time;
                uniform vec3 acceleration;

                in vec3 start_position;
                in vec3 start_velocity;

                uniform mat4 modelMatrix;
                in vec3 position;

                {} // Positions out
                {} // Normals in/out
                {} // UV coordinates in/out

                void main()
                {{
                    vec3 p = start_position + start_velocity * time + 0.5 * acceleration * time * time;
                    gl_Position = camera.projection * (camera.view * modelMatrix * vec4(p, 1.0) + vec4(position, 0.0));
                    {} // Position
                    {} // Normal
                    {} // UV coordinates
                }}
                ",
                if use_positions {"out vec3 pos;"} else {""},
                if use_normals {
                    "uniform mat4 normalMatrix;
                    in vec3 normal;
                    out vec3 nor;"
                    } else {""},
                if use_uvs {
                    "in vec2 uv_coordinates;
                    out vec2 uvs;"
                    } else {""},
                if use_positions {"pos = worldPosition.xyz;"} else {""},
                if use_normals { "nor = mat3(normalMatrix) * normal;" } else {""},
                if use_uvs { "uvs = uv_coordinates;" } else {""}
        )
    }
}

impl Geometry for Particles {
    fn aabb(&self) -> AxisAlignedBoundingBox {
        AxisAlignedBoundingBox::INFINITE
    }

    fn transformation(&self) -> Mat4 {
        self.transformation
    }
}

impl GeometryMut for Particles {
    fn set_transformation(&mut self, transformation: Mat4) {
        self.transformation = transformation;
        self.normal_transformation = self.transformation.invert().unwrap().transpose();
    }
}

impl Shadable for Particles {
    fn render_with_material<'a>(
        &self,
        material: &dyn Material,
        camera: &Camera,
        lights: &[&dyn Light],
    ) -> ThreeDResult<()> {
        let fragment_shader_source = material.fragment_shader_source(false, lights);
        self.context.program(
            &Particles::vertex_shader_source(&fragment_shader_source),
            &fragment_shader_source,
            |program| {
                material.use_uniforms(program, camera, lights)?;

                program.use_uniform_mat4("modelMatrix", &self.transformation)?;
                program.use_uniform_vec3("acceleration", &self.acceleration)?;
                program.use_uniform_float("time", &self.time)?;
                program.use_uniform_block("Camera", camera.uniform_buffer());

                program
                    .use_attribute_vec3_instanced("start_position", &self.start_position_buffer)?;
                program
                    .use_attribute_vec3_instanced("start_velocity", &self.start_velocity_buffer)?;
                if program.requires_attribute("position") {
                    program.use_attribute_vec3("position", &self.position_buffer)?;
                }
                if program.requires_attribute("uv_coordinates") {
                    let uv_buffer = self
                        .uv_buffer
                        .as_ref()
                        .ok_or(CoreError::MissingMeshBuffer("uv coordinate".to_string()))?;
                    program.use_attribute_vec2("uv_coordinates", uv_buffer)?;
                }
                if program.requires_attribute("normal") {
                    let normal_buffer = self
                        .normal_buffer
                        .as_ref()
                        .ok_or(CoreError::MissingMeshBuffer("normal".to_string()))?;
                    program.use_uniform_mat4("normalMatrix", &self.normal_transformation)?;
                    program.use_attribute_vec3("normal", normal_buffer)?;
                }

                if let Some(ref index_buffer) = self.index_buffer {
                    program.draw_elements_instanced(
                        material.render_states(),
                        camera.viewport(),
                        index_buffer,
                        self.instance_count,
                    );
                } else {
                    program.draw_arrays_instanced(
                        material.render_states(),
                        camera.viewport(),
                        self.position_buffer.count() as u32 / 3,
                        self.instance_count,
                    );
                }
                Ok(())
            },
        )
    }
}
