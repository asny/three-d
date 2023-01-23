#![allow(deprecated)]
use crate::core::*;
use crate::renderer::*;
use std::collections::HashMap;

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
/// The particles will only move if the [ParticleSystem::time] variable is updated every frame.
///
pub struct ParticleSystem {
    context: Context,
    vertex_buffers: Vec<(String, VertexBuffer)>,
    instance_buffers: HashMap<String, InstanceBuffer>,
    index_buffer: Option<ElementBuffer>,
    /// The acceleration applied to all particles defined in the world coordinate system.
    pub acceleration: Vec3,
    instance_count: u32,
    transformation: Mat4,
    texture_transform: Mat3,
    /// A time variable that should be updated each frame.
    #[deprecated = "call the animate method each frame instead"]
    pub time: f32,
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
            index_buffer: super::index_buffer_from_mesh(context, cpu_mesh),
            vertex_buffers: super::vertex_buffers_from_mesh(context, cpu_mesh),
            instance_buffers: HashMap::new(),
            acceleration,
            instance_count: 0,
            transformation: Mat4::identity(),
            texture_transform: Mat3::identity(),
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
    /// Get the texture transform applied to the uv coordinates of all of the particles.
    ///
    #[deprecated]
    pub fn texture_transform(&self) -> &Mat3 {
        &self.texture_transform
    }

    ///
    /// Set the texture transform applied to the uv coordinates of all of the particles.
    /// This is applied before the texture transform for each particle.
    ///
    #[deprecated = "Set the texture transformation of Texture2DRef for a material instead"]
    pub fn set_texture_transform(&mut self, texture_transform: Mat3) {
        self.texture_transform = texture_transform;
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
                InstanceBuffer::new_with_data(&self.context, &instance_colors),
            );
        }
    }

    fn draw(&self, program: &Program, render_states: RenderStates, camera: &Camera) {
        program.use_uniform("viewProjection", camera.projection() * camera.view());
        program.use_uniform("modelMatrix", &self.transformation);
        program.use_uniform("acceleration", &self.acceleration);
        program.use_uniform("time", &self.time);
        program.use_uniform("textureTransform", &self.texture_transform);
        program.use_uniform(
            "normalMatrix",
            &self.transformation.invert().unwrap().transpose(),
        );

        for (attribute_name, buffer) in self.vertex_buffers.iter() {
            program.use_vertex_attribute(attribute_name, buffer);
        }

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
                    .get(attribute_name).expect(&format!("the render call requires the {} instance buffer which is missing on the given geometry", attribute_name))
                );
            }
        }

        if let Some(ref index_buffer) = self.index_buffer {
            program.draw_elements_instanced(
                render_states,
                camera.viewport(),
                index_buffer,
                self.instance_count,
            )
        } else {
            program.draw_arrays_instanced(
                render_states,
                camera.viewport(),
                self.vertex_buffers.first().unwrap().1.vertex_count() as u32,
                self.instance_count,
            )
        }
    }

    fn provided_attributes(&self) -> FragmentAttributes {
        FragmentAttributes {
            position: true,
            normal: self.vertex_buffers.iter().any(|(n, _)| n == "normal"),
            tangents: self.vertex_buffers.iter().any(|(n, _)| n == "normal")
                && self.vertex_buffers.iter().any(|(n, _)| n == "tangent"),
            uv: self
                .vertex_buffers
                .iter()
                .any(|(n, _)| n == "uv_coordinates"),
            color: self.vertex_buffers.iter().any(|(n, _)| n == "color")
                || self.instance_buffers.contains_key("instance_color"),
        }
    }
    fn program(&self, fragment_shader_source: String, callback: impl FnOnce(&Program)) {
        let vertex_shader_source = format!(
            "#define PARTICLES\n{}{}{}{}{}{}{}{}{}",
            if self.instance_buffers.contains_key("instance_translation") {
                "#define USE_INSTANCE_TRANSLATIONS\n"
            } else {
                "#define USE_INSTANCE_TRANSFORMS\n"
            },
            if true { "#define USE_POSITIONS\n" } else { "" },
            if true { "#define USE_NORMALS\n" } else { "" },
            if true { "#define USE_TANGENTS\n" } else { "" },
            if true { "#define USE_UVS\n" } else { "" },
            if self.instance_buffers.contains_key("instance_color") {
                "#define USE_COLORS\n#define USE_VERTEX_COLORS\n#define USE_INSTANCE_COLORS\n"
            } else if self.instance_buffers.contains_key("instance_color") {
                "#define USE_COLORS\n#define USE_INSTANCE_COLORS\n"
            } else {
                "#define USE_COLORS\n#define USE_VERTEX_COLORS\n"
            },
            if self.instance_buffers.contains_key("tex_transform_row1") {
                "#define USE_INSTANCE_TEXTURE_TRANSFORMATION\n"
            } else {
                ""
            },
            include_str!("../../core/shared.frag"),
            include_str!("shaders/mesh.vert"),
        );
        self.context
            .program(vertex_shader_source, fragment_shader_source, callback)
            .expect("Failed compiling shader")
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
        let fragment_shader = material
            .fragment_shader_source(self.provided_attributes(), lights)
            .unwrap_or_else(|e| panic!("{}", e));
        self.program(fragment_shader.source, |program| {
            material.use_uniforms(program, camera, lights);
            self.draw(program, material.render_states(), camera);
        });
    }

    fn render_with_post_material(
        &self,
        material: &dyn PostMaterial,
        camera: &Camera,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) {
        let fragment_shader = material
            .fragment_shader_source(
                self.provided_attributes(),
                lights,
                color_texture,
                depth_texture,
            )
            .unwrap_or_else(|e| panic!("{}", e));
        self.program(fragment_shader.source, |program| {
            material.use_uniforms(program, camera, lights, color_texture, depth_texture);
            self.draw(program, material.render_states(), camera);
        });
    }

    fn animate(&mut self, time: f32) {
        self.time = time;
    }
}
