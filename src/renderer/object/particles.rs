use crate::core::*;
use crate::renderer::*;

///
/// Shader program used for rendering [Particles](Particles). It has a fixed vertex shader and
/// customizable fragment shader for custom shading. Use this in combination with [Particles::render].
///
#[deprecated]
pub struct ParticlesProgram {
    program: Program,
}

impl ParticlesProgram {
    ///
    /// Creates a new program which can be used to render particles.
    /// The fragment shader code can use position (`in vec3 pos;`), normal (`in vec3 nor;`) and uv coordinates (`in vec2 uvs;`).
    ///
    pub fn new(context: &Context, fragment_shader_source: &str) -> Result<Self> {
        Ok(Self {
            program: Program::from_source(
                context,
                &Particles::vertex_shader_source(fragment_shader_source),
                fragment_shader_source,
            )?,
        })
    }
}

impl std::ops::Deref for ParticlesProgram {
    type Target = Program;

    fn deref(&self) -> &Program {
        &self.program
    }
}

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
/// Particle effect with fixed vertex shader and customizable fragment shader (see also [ParticlesProgram](ParticlesProgram)).
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
    pub time: f32,
}

impl Particles {
    ///
    /// Creates a new set of particles with geometry defined by the given cpu mesh.
    ///
    pub fn new(context: &Context, cpu_mesh: &CPUMesh) -> Result<Self> {
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

    ///
    /// Render all defined particles with the given [Program].
    /// Must be called in a render target render function,
    /// for example in the callback function of [Screen::write](crate::Screen::write).
    ///
    #[deprecated = "Use 'render_forward' instead"]
    pub fn render(
        &self,
        render_states: RenderStates,
        program: &Program,
        camera: &Camera,
        time: f32,
    ) -> Result<()> {
        program.use_uniform_mat4("modelMatrix", &self.transformation)?;
        program.use_uniform_vec3("acceleration", &self.acceleration)?;
        program.use_uniform_float("time", &time)?;
        program.use_uniform_block("Camera", camera.uniform_buffer());

        program.use_attribute_vec3_instanced("start_position", &self.start_position_buffer)?;
        program.use_attribute_vec3_instanced("start_velocity", &self.start_velocity_buffer)?;
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
            program.use_uniform_mat4(
                "normalMatrix",
                &self.transformation.invert().unwrap().transpose(),
            )?;
            program.use_attribute_vec3("normal", normal_buffer)?;
        }

        if let Some(ref index_buffer) = self.index_buffer {
            program.draw_elements_instanced(
                render_states,
                camera.viewport(),
                index_buffer,
                self.instance_count,
            );
        } else {
            program.draw_arrays_instanced(
                render_states,
                camera.viewport(),
                self.position_buffer.count() as u32 / 3,
                self.instance_count,
            );
        }
        Ok(())
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

impl Object for Particles {
    fn render_forward(&self, material: &dyn ForwardMaterial, camera: &Camera) -> Result<()> {
        let render_states = material.render_states(false);
        let fragment_shader_source = material.fragment_shader_source(false);
        self.context.program(
            &Particles::vertex_shader_source(&fragment_shader_source),
            &fragment_shader_source,
            |program| {
                material.bind(program, camera)?;
                self.render(render_states, program, camera, self.time)
            },
        )
    }

    fn render_deferred(
        &self,
        _material: &dyn DeferredMaterial,
        _camera: &Camera,
        _viewport: Viewport,
    ) -> Result<()> {
        unimplemented!()
    }

    fn transformation(&self) -> &Mat4 {
        &self.transformation
    }

    fn set_transformation(&mut self, transformation: Mat4) {
        self.transformation = transformation;
    }

    fn axis_aligned_bounding_box(&self) -> &AxisAlignedBoundingBox {
        &AxisAlignedBoundingBox::INFINITE // TODO: Compute bounding box
    }
}
