use crate::core::*;
use crate::renderer::*;
use std::collections::HashMap;

use super::BaseMesh;

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
    pub colors: Option<Vec<Color>>,
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
    instance_buffers: HashMap<String, InstanceBuffer>,
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
            instance_buffers: HashMap::new(),
            acceleration,
            instance_count: 0,
            transformation: Mat4::identity(),
            time: 0.0,
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
        self.instance_buffers.clear();

        self.instance_buffers.insert(
            "start_position".to_string(),
            InstanceBuffer::new_with_data(&self.context, &particles.start_positions),
        );
        self.instance_buffers.insert(
            "start_velocity".to_string(),
            InstanceBuffer::new_with_data(&self.context, &particles.start_velocities),
        );
        if let Some(texture_transforms) = &particles.texture_transforms {
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
            self.instance_buffers.insert(
                "tex_transform_row1".to_string(),
                InstanceBuffer::new_with_data(&self.context, &instance_tex_transform1),
            );
            self.instance_buffers.insert(
                "tex_transform_row2".to_string(),
                InstanceBuffer::new_with_data(&self.context, &instance_tex_transform2),
            );
        }
        if let Some(instance_colors) = &particles.colors {
            self.instance_buffers.insert(
                "instance_color".to_string(),
                InstanceBuffer::new_with_data(&self.context, instance_colors),
            );
        }
    }

    fn draw(
        &self,
        program: &Program,
        render_states: RenderStates,
        camera: &Camera,
        attributes: FragmentAttributes,
    ) {
        if attributes.normal {
            if let Some(inverse) = self.transformation.invert() {
                program.use_uniform("normalMatrix", inverse.transpose());
            } else {
                // determinant is float zero
                return;
            }
        }
        program.use_uniform("viewProjection", camera.projection() * camera.view());
        program.use_uniform("modelMatrix", self.transformation);
        program.use_uniform("acceleration", self.acceleration);
        program.use_uniform("time", self.time);

        self.base_mesh.use_attributes(program, attributes);

        for attribute_name in [
            "start_position",
            "start_velocity",
            "tex_transform_row1",
            "tex_transform_row2",
            "instance_color",
        ] {
            if program.requires_attribute(attribute_name) {
                program.use_instance_attribute(
                    attribute_name,
                    self.instance_buffers
                    .get(attribute_name).unwrap_or_else(|| panic!("the render call requires the {} instance buffer which is missing on the given geometry", attribute_name))
                );
            }
        }
        self.base_mesh.draw_instanced(
            program,
            render_states,
            camera,
            attributes,
            self.instance_count,
        );
    }

    fn vertex_shader_source(&self, required_attributes: FragmentAttributes) -> String {
        format!(
            "#define PARTICLES\n{}{}{}{}{}{}{}",
            if required_attributes.normal {
                "#define USE_NORMALS\n"
            } else {
                ""
            },
            if required_attributes.tangents {
                "#define USE_TANGENTS\n"
            } else {
                ""
            },
            if required_attributes.uv {
                "#define USE_UVS\n"
            } else {
                ""
            },
            if self.instance_buffers.contains_key("instance_color")
                && self.base_mesh.colors.is_some()
            {
                "#define USE_VERTEX_COLORS\n#define USE_INSTANCE_COLORS\n"
            } else if self.instance_buffers.contains_key("instance_color") {
                "#define USE_INSTANCE_COLORS\n"
            } else if self.base_mesh.colors.is_some() {
                "#define USE_VERTEX_COLORS\n"
            } else {
                ""
            },
            if self.instance_buffers.contains_key("tex_transform_row1") {
                "#define USE_INSTANCE_TEXTURE_TRANSFORMATION\n"
            } else {
                ""
            },
            include_str!("../../core/shared.frag"),
            include_str!("shaders/mesh.vert"),
        )
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
    fn aabb(&self) -> AxisAlignedBoundingBox {
        AxisAlignedBoundingBox::INFINITE
    }

    fn render_with_material(
        &self,
        material: &dyn Material,
        camera: &Camera,
        lights: &[&dyn Light],
    ) {
        let fragment_shader = material.fragment_shader(lights);
        let vertex_shader_source = self.vertex_shader_source(fragment_shader.attributes);
        self.context
            .program(vertex_shader_source, fragment_shader.source, |program| {
                material.use_uniforms(program, camera, lights);
                self.draw(
                    program,
                    material.render_states(),
                    camera,
                    fragment_shader.attributes,
                );
            })
            .expect("Failed compiling shader");
    }

    fn render_with_post_material(
        &self,
        material: &dyn PostMaterial,
        camera: &Camera,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) {
        let fragment_shader = material.fragment_shader(lights, color_texture, depth_texture);
        let vertex_shader_source = self.vertex_shader_source(fragment_shader.attributes);
        self.context
            .program(vertex_shader_source, fragment_shader.source, |program| {
                material.use_uniforms(program, camera, lights, color_texture, depth_texture);
                self.draw(
                    program,
                    material.render_states(),
                    camera,
                    fragment_shader.attributes,
                );
            })
            .expect("Failed compiling shader");
    }

    fn animate(&mut self, time: f32) {
        self.time = time;
    }
}
