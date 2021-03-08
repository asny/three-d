
use crate::math::*;
use crate::definition::*;
use crate::core::*;
use crate::camera::*;

///
/// Shader program used for rendering [Particles](Particles).
/// The fragment shader code can use position (`in vec3 pos;`) normal (`in vec3 nor;`) and uv coordinates (`in vec2 uvs;`).
///
pub struct ParticlesProgram {
    program: Program,
    use_normals: bool,
    use_uvs: bool,
}

impl ParticlesProgram {
    pub fn new(context: &Context, fragment_shader_source: &str) -> Result<Self, Error> {
        let use_positions = fragment_shader_source.find("in vec3 pos;").is_some();
        let use_normals = fragment_shader_source.find("in vec3 nor;").is_some();
        let use_uvs = fragment_shader_source.find("in vec2 uvs;").is_some();
        let vertex_shader_source = &format!("
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
        );

        let program = Program::from_source(context, vertex_shader_source, fragment_shader_source)?;
        Ok(Self {program, use_normals, use_uvs})
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
    pub start_position: Vec3,
    pub start_velocity: Vec3
}

///
/// Particle effect with fixed vertex shader and customizable fragment shader (see also [ParticlesProgram](ParticlesProgram)).
///
/// Each particle is initialised with a position and velocity using the [update](Particles::update) function and a global acceleration.
/// Then when time passes, their position is updated based on
/// `new_position = start_position + start_velocity * time + 0.5 * acceleration * time * time`
///
pub struct Particles {
    start_position_buffer: VertexBuffer,
    start_velocity_buffer: VertexBuffer,
    position_buffer: VertexBuffer,
    normal_buffer: Option<VertexBuffer>,
    uv_buffer: Option<VertexBuffer>,
    index_buffer: Option<ElementBuffer>,
    pub acceleration: Vec3,
    instance_count: u32
}

impl Particles {
    pub fn new(context: &Context, cpu_mesh: &CPUMesh, acceleration: &Vec3) -> Result<Self, Error>
    {
        let position_buffer = VertexBuffer::new_with_static_f32(context, &cpu_mesh.positions)?;
        let normal_buffer = if let Some(ref normals) = cpu_mesh.normals { Some(VertexBuffer::new_with_static_f32(context, normals)?) } else {None};
        let index_buffer = if let Some(ref ind) = cpu_mesh.indices { Some(ElementBuffer::new_with_u32(context, ind)?) } else {None};
        let uv_buffer = if let Some(ref uvs) = cpu_mesh.uvs { Some(VertexBuffer::new_with_static_f32(context, uvs)?) } else {None};

        Ok(Self {
            position_buffer, index_buffer, normal_buffer, uv_buffer,
            start_position_buffer: VertexBuffer::new_with_dynamic_f32(context, &[])?,
            start_velocity_buffer: VertexBuffer::new_with_dynamic_f32(context, &[])?,
            acceleration: *acceleration,
            instance_count: 0
        })
    }

    ///
    /// Updates the particles with the given initial data.
    /// The list contain one entry for each particle.
    ///
    pub fn update(&mut self, data: &[ParticleData])
    {
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
        self.start_position_buffer.fill_with_dynamic_f32(&start_position);
        self.start_velocity_buffer.fill_with_dynamic_f32(&start_velocity);
        self.instance_count = data.len() as u32;
    }

    ///
    /// Render all defined particles with the given [ParticlesProgram](ParticlesProgram).
    ///
    pub fn render(&self, program: &ParticlesProgram, render_states: RenderStates, viewport: Viewport, transformation: &Mat4, camera: &Camera, time: f32) -> Result<(), Error>
    {
        program.add_uniform_mat4("modelMatrix", &transformation)?;
        program.add_uniform_vec3("acceleration", &self.acceleration)?;
        program.add_uniform_float("time", &time)?;
        program.use_uniform_block(camera.matrix_buffer(), "Camera");

        program.use_attribute_vec3_divisor(&self.start_position_buffer, "start_position", 1)?;
        program.use_attribute_vec3_divisor(&self.start_velocity_buffer, "start_velocity", 1)?;
        program.use_attribute_vec3(&self.position_buffer, "position")?;
        if program.use_uvs {
            let uv_buffer = self.uv_buffer.as_ref().ok_or(
                Error::FailedToCreateMesh {message: "The particles shader program needs uv coordinates, but the mesh does not have any.".to_string()})?;
            program.use_attribute_vec2(uv_buffer, "uv_coordinates")?;
        }
        if program.use_normals {
            let normal_buffer = self.normal_buffer.as_ref().ok_or(
                Error::FailedToCreateMesh {message: "The particles shader program needs normals, but the mesh does not have any. Consider calculating the normals on the CPUMesh.".to_string()})?;
            program.add_uniform_mat4("normalMatrix", &transformation.invert().unwrap().transpose())?;
            program.use_attribute_vec3(normal_buffer, "normal")?;
        }

        if let Some(ref index_buffer) = self.index_buffer {
            program.draw_elements_instanced(render_states, viewport,index_buffer, self.instance_count);
        } else {
            program.draw_arrays_instanced(render_states, viewport,self.position_buffer.count() as u32/3, self.instance_count);
        }
        Ok(())
    }
}