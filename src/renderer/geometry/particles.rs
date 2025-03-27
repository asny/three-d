use super::BaseMesh;
use crate::core::*;
use crate::renderer::*;

///
/// Used for defining the attributes for each particle in a [ParticleSystem], for example its starting position and velocity.
///
/// Each list of attributes must contain the same number of elements as the number of particles.
///
#[derive(Clone, Debug, Default)]
pub struct Particles {
    /// Initial positions of each particle in world coordinates.
    pub start_positions: Vec<Vec3>,
    /// Initial velocities of each particle defined in the world coordinate system.
    pub start_velocities: Vec<Vec3>,
    /// The texture transform applied to the uv coordinates of each particle.
    pub texture_transforms: Option<Vec<Mat3>>,
    /// A custom color for each particle.
    pub colors: Option<Vec<Srgba>>,
}

impl Particles {
    ///
    /// Returns an error if the particle data is not valid.
    ///
    pub fn validate(&self) -> Result<(), RendererError> {
        let instance_count = self.count();
        let buffer_check = |length: Option<usize>, name: &str| -> Result<(), RendererError> {
            if let Some(length) = length {
                if length < instance_count as usize {
                    Err(RendererError::InvalidBufferLength(
                        name.to_string(),
                        instance_count as usize,
                        length,
                    ))?;
                }
            }
            Ok(())
        };

        buffer_check(
            self.texture_transforms.as_ref().map(|b| b.len()),
            "texture transforms",
        )?;
        buffer_check(self.colors.as_ref().map(|b| b.len()), "colors")?;
        buffer_check(Some(self.start_positions.len()), "start_positions")?;
        buffer_check(Some(self.start_velocities.len()), "start_velocities")?;

        Ok(())
    }

    /// Returns the number of particles.
    pub fn count(&self) -> u32 {
        self.start_positions.len() as u32
    }
}

///
/// Particle system that can be used to simulate effects such as fireworks, fire, smoke or water particles.
///
/// All particles are initialised with [Particles::start_positions] and [Particles::start_velocities] and a global [ParticleSystem::acceleration].
/// Then, when time passes, their position is updated based on
///
/// ```no_rust
/// new_position = start_position + start_velocity * time + 0.5 * acceleration * time * time
/// ```
///
/// The particles will only move if the [ParticleSystem::animate] is called every frame.
///
pub struct ParticleSystem {
    context: Context,
    base_mesh: BaseMesh,
    start_position: InstanceBuffer<Vec3>,
    start_velocity: InstanceBuffer<Vec3>,
    tex_transform: Option<(InstanceBuffer<Vec3>, InstanceBuffer<Vec3>)>,
    instance_color: Option<InstanceBuffer<Vec4>>,
    /// The acceleration applied to all particles defined in the world coordinate system.
    pub acceleration: Vec3,
    instance_count: u32,
    transformation: Mat4,
    time: f32,
}

impl ParticleSystem {
    ///
    /// Creates a new particle system with the given geometry and the given attributes for each particle.
    /// The acceleration is applied to all particles defined in the world coordinate system.
    ///
    pub fn new(
        context: &Context,
        particles: &Particles,
        acceleration: Vec3,
        cpu_mesh: &CpuMesh,
    ) -> Self {
        #[cfg(debug_assertions)]
        cpu_mesh.validate().expect("invalid cpu mesh");

        let mut particles_system = Self {
            context: context.clone(),
            base_mesh: BaseMesh::new(context, cpu_mesh),
            acceleration,
            instance_count: 0,
            transformation: Mat4::identity(),
            time: 0.0,
            start_position: InstanceBuffer::<Vec3>::new(context),
            start_velocity: InstanceBuffer::<Vec3>::new(context),
            tex_transform: None,
            instance_color: None,
        };
        particles_system.set_particles(particles);
        particles_system
    }

    ///
    /// Returns local to world transformation applied to the particle geometry before its position is updated as described in [ParticleSystem].
    ///
    pub fn transformation(&self) -> Mat4 {
        self.transformation
    }

    ///
    /// Set the local to world transformation applied to the particle geometry before its position is updated as described in [ParticleSystem].
    ///
    pub fn set_transformation(&mut self, transformation: Mat4) {
        self.transformation = transformation;
    }

    ///
    /// Set the particles attributes.
    ///
    pub fn set_particles(&mut self, particles: &Particles) {
        #[cfg(debug_assertions)]
        particles.validate().expect("invalid particles");
        self.instance_count = particles.count();

        self.start_position =
            InstanceBuffer::new_with_data(&self.context, &particles.start_positions);
        self.start_velocity =
            InstanceBuffer::new_with_data(&self.context, &particles.start_velocities);
        self.tex_transform = particles
            .texture_transforms
            .as_ref()
            .map(|texture_transforms| {
                let mut instance_tex_transform1 = Vec::new();
                let mut instance_tex_transform2 = Vec::new();
                for texture_transform in texture_transforms.iter() {
                    instance_tex_transform1.push(vec3(
                        texture_transform.x.x,
                        texture_transform.y.x,
                        texture_transform.z.x,
                    ));
                    instance_tex_transform2.push(vec3(
                        texture_transform.x.y,
                        texture_transform.y.y,
                        texture_transform.z.y,
                    ));
                }
                (
                    InstanceBuffer::new_with_data(&self.context, &instance_tex_transform1),
                    InstanceBuffer::new_with_data(&self.context, &instance_tex_transform2),
                )
            });
        self.instance_color = particles.colors.as_ref().map(|instance_colors| {
            InstanceBuffer::new_with_data(
                &self.context,
                &instance_colors
                    .iter()
                    .map(|c| c.to_linear_srgb())
                    .collect::<Vec<_>>(),
            )
        });
    }
}

impl<'a> IntoIterator for &'a ParticleSystem {
    type Item = &'a dyn Geometry;
    type IntoIter = std::iter::Once<&'a dyn Geometry>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self)
    }
}

impl Geometry for ParticleSystem {
    fn id(&self) -> GeometryId {
        GeometryId::ParticleSystem(
            self.base_mesh.normals.is_some(),
            self.base_mesh.tangents.is_some(),
            self.base_mesh.uvs.is_some(),
            self.base_mesh.colors.is_some(),
            self.instance_color.is_some(),
            self.tex_transform.is_some(),
        )
    }

    fn vertex_shader_source(&self) -> String {
        format!(
            "#define PARTICLES\n{}{}{}",
            if self.instance_color.is_some() {
                "#define USE_INSTANCE_COLORS\n"
            } else {
                ""
            },
            if self.tex_transform.is_some() {
                "#define USE_INSTANCE_TEXTURE_TRANSFORMATION\n"
            } else {
                ""
            },
            self.base_mesh.vertex_shader_source()
        )
    }

    fn draw(&self, viewer: &dyn Viewer, program: &Program, render_states: RenderStates) {
        if let Some(inverse) = self.transformation.invert() {
            program.use_uniform_if_required("normalMatrix", inverse.transpose());
        } else {
            // determinant is float zero
            return;
        }
        program.use_uniform("viewProjection", viewer.projection() * viewer.view());
        program.use_uniform("modelMatrix", self.transformation);
        program.use_uniform("acceleration", self.acceleration);
        program.use_uniform("time", self.time);

        program.use_instance_attribute("start_position", &self.start_position);
        program.use_instance_attribute("start_velocity", &self.start_velocity);

        if program.requires_attribute("tex_transform_row1") {
            if let Some((row1, row2)) = &self.tex_transform {
                program.use_instance_attribute("tex_transform_row1", row1);
                program.use_instance_attribute("tex_transform_row2", row2);
            }
        }

        if program.requires_attribute("instance_color") {
            if let Some(color) = &self.instance_color {
                program.use_instance_attribute("instance_color", color);
            }
        }

        self.base_mesh
            .draw_instanced(program, render_states, viewer, self.instance_count);
    }

    fn aabb(&self) -> AxisAlignedBoundingBox {
        AxisAlignedBoundingBox::INFINITE
    }

    fn render_with_material(
        &self,
        material: &dyn Material,
        viewer: &dyn Viewer,
        lights: &[&dyn Light],
    ) {
        if let Err(e) = render_with_material(&self.context, viewer, &self, material, lights) {
            panic!("{}", e.to_string());
        }
    }

    fn render_with_effect(
        &self,
        material: &dyn Effect,
        viewer: &dyn Viewer,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) {
        if let Err(e) = render_with_effect(
            &self.context,
            viewer,
            self,
            material,
            lights,
            color_texture,
            depth_texture,
        ) {
            panic!("{}", e.to_string());
        }
    }

    fn animate(&mut self, time: f32) {
        self.time = time;
    }
}
