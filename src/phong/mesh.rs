
use crate::*;
use std::rc::Rc;

pub struct PhongForwardMesh {
    pub name: String,
    program_color_ambient: Rc<Program>,
    program_color_ambient_directional: Rc<Program>,
    program_texture_ambient: Rc<Program>,
    program_texture_ambient_directional: Rc<Program>,
    gpu_mesh: GPUMesh,
    pub material: PhongMaterial
}

impl PhongForwardMesh
{
    pub fn new(gl: &Gl, cpu_mesh: &CPUMesh, material: &PhongMaterial) -> Result<Self, Error>
    {
        Self::new_with_programs(gl, Self::program_color_ambient(gl)?,
                                Self::program_color_ambient_directional(gl)?,
                                Self::program_texture_ambient(gl)?,
                                Self::program_texture_ambient_directional(gl)?, cpu_mesh, material)
    }

    pub fn render_depth(&self, transformation: &Mat4, camera: &camera::Camera) -> Result<(), Error>
    {
        let program = match self.material.color_source {
            ColorSource::Color(_) => self.program_color_ambient.as_ref(),
            ColorSource::Texture(_) => self.program_texture_ambient.as_ref()
        };
        program.add_uniform_vec3("ambientLight.color", &vec3(0.0, 0.0, 0.0))?;
        program.add_uniform_float("ambientLight.intensity", &0.0)?;
        self.gpu_mesh.render(program, &self.material, transformation, camera, None)?;
        Ok(())
    }

    pub fn render_with_ambient(&self, transformation: &Mat4, camera: &camera::Camera, ambient_light: &AmbientLight) -> Result<(), Error>
    {
        let program = match self.material.color_source {
            ColorSource::Color(_) => self.program_color_ambient.as_ref(),
            ColorSource::Texture(_) => self.program_texture_ambient.as_ref()
        };
        program.add_uniform_vec3("ambientLight.color", &ambient_light.color())?;
        program.add_uniform_float("ambientLight.intensity", &ambient_light.intensity())?;
        self.gpu_mesh.render(program, &self.material, transformation, camera, None)?;
        Ok(())
    }

    pub fn render_with_ambient_and_directional(&self, transformation: &Mat4, camera: &camera::Camera, ambient_light: &AmbientLight, directional_light: &DirectionalLight) -> Result<(), Error>
    {
        let program = match self.material.color_source {
            ColorSource::Color(_) => self.program_color_ambient_directional.as_ref(),
            ColorSource::Texture(_) => self.program_texture_ambient_directional.as_ref()
        };
        program.add_uniform_vec3("ambientLight.color", &ambient_light.color())?;
        program.add_uniform_float("ambientLight.intensity", &ambient_light.intensity())?;
        program.add_uniform_vec3("eyePosition", &camera.position())?;
        program.use_texture(directional_light.shadow_map(), "shadowMap")?;
        program.use_uniform_block(directional_light.buffer(), "DirectionalLightUniform");
        self.gpu_mesh.render(program, &self.material, transformation, camera, None)?;
        Ok(())
    }

    pub(crate) fn program_color_ambient(gl: &Gl) -> Result<Rc<Program>, Error>
    {
        Ok(Rc::new(Program::from_source(gl, include_str!("shaders/mesh.vert"),
                                                                     &format!("{}\n{}",
                                                                              &include_str!("shaders/light_shared.frag"),
                                                                              &include_str!("shaders/colored_forward_ambient.frag")))?))
    }

    pub(crate) fn program_color_ambient_directional(gl: &Gl) -> Result<Rc<Program>, Error>
    {
        Ok(Rc::new(Program::from_source(gl, include_str!("shaders/mesh.vert"),
                                                                     &format!("{}\n{}",
                                                                              &include_str!("shaders/light_shared.frag"),
                                                                              &include_str!("shaders/colored_forward_ambient_directional.frag")))?))
    }

    pub(crate) fn program_texture_ambient(gl: &Gl) -> Result<Rc<Program>, Error>
    {
        Ok(Rc::new(Program::from_source(gl, include_str!("shaders/mesh.vert"),
                                                                       &format!("{}\n{}\n{}",
                                                                                include_str!("shaders/light_shared.frag"),
                                                                                include_str!("shaders/triplanar_mapping.frag"),
                                                                                include_str!("shaders/textured_forward_ambient.frag")))?))
    }

    pub(crate) fn program_texture_ambient_directional(gl: &Gl) -> Result<Rc<Program>, Error>
    {
        Ok(Rc::new(Program::from_source(gl, include_str!("shaders/mesh.vert"),
                                                                       &format!("{}\n{}\n{}",
                                                                                include_str!("shaders/light_shared.frag"),
                                                                                include_str!("shaders/triplanar_mapping.frag"),
                                                                                include_str!("shaders/textured_forward_ambient_directional.frag")))?))
    }

    pub(crate) fn new_with_programs(gl: &Gl, program_color_ambient: Rc<Program>, program_color_ambient_directional: Rc<Program>,
                                    program_texture_ambient: Rc<Program>, program_texture_ambient_directional: Rc<Program>,
                   cpu_mesh: &CPUMesh, material: &PhongMaterial) -> Result<Self, Error>
    {
        Ok(Self { name: cpu_mesh.name.clone(), gpu_mesh: GPUMesh::new(gl, cpu_mesh)?,
            program_color_ambient, program_color_ambient_directional, program_texture_ambient, program_texture_ambient_directional, material: material.clone() })
    }
}

pub struct PhongForwardInstancedMesh {
    pub name: String,
    program_color_ambient: Rc<Program>,
    program_color_ambient_directional: Rc<Program>,
    program_texture_ambient: Rc<Program>,
    program_texture_ambient_directional: Rc<Program>,
    gpu_mesh: InstancedGPUMesh,
    pub material: PhongMaterial
}

impl PhongForwardInstancedMesh
{
    pub fn new(gl: &Gl, transformations: &[Mat4], cpu_mesh: &CPUMesh, material: &PhongMaterial) -> Result<Self, Error>
    {
        Self::new_with_programs(gl, transformations, Self::program_color_ambient(gl)?,
                                Self::program_color_ambient_directional(gl)?,
                                Self::program_texture_ambient(gl)?,
                                Self::program_texture_ambient_directional(gl)?, cpu_mesh, material)
    }

    pub fn render_depth(&self, transformation: &Mat4, camera: &camera::Camera) -> Result<(), Error>
    {
        let program = match self.material.color_source {
            ColorSource::Color(_) => self.program_color_ambient.as_ref(),
            ColorSource::Texture(_) => self.program_texture_ambient.as_ref()
        };
        program.add_uniform_vec3("ambientLight.color", &vec3(0.0, 0.0, 0.0))?;
        program.add_uniform_float("ambientLight.intensity", &0.0)?;
        self.gpu_mesh.render(program, &self.material, transformation, camera)?;
        Ok(())
    }

    pub fn render_with_ambient(&self, transformation: &Mat4, camera: &camera::Camera, ambient_light: &AmbientLight) -> Result<(), Error>
    {
        let program = match self.material.color_source {
            ColorSource::Color(_) => self.program_color_ambient.as_ref(),
            ColorSource::Texture(_) => self.program_texture_ambient.as_ref()
        };
        program.add_uniform_vec3("ambientLight.color", &ambient_light.color())?;
        program.add_uniform_float("ambientLight.intensity", &ambient_light.intensity())?;
        self.gpu_mesh.render(program, &self.material, transformation, camera)?;
        Ok(())
    }

    pub fn render_with_ambient_and_directional(&self, transformation: &Mat4, camera: &camera::Camera, ambient_light: &AmbientLight, directional_light: &DirectionalLight) -> Result<(), Error>
    {
        let program = match self.material.color_source {
            ColorSource::Color(_) => self.program_color_ambient_directional.as_ref(),
            ColorSource::Texture(_) => self.program_texture_ambient_directional.as_ref()
        };
        program.add_uniform_vec3("ambientLight.color", &ambient_light.color())?;
        program.add_uniform_float("ambientLight.intensity", &ambient_light.intensity())?;
        program.add_uniform_vec3("eyePosition", &camera.position())?;
        program.use_texture(directional_light.shadow_map(), "shadowMap")?;
        program.use_uniform_block(directional_light.buffer(), "DirectionalLightUniform");
        self.gpu_mesh.render(program, &self.material, transformation, camera)?;
        Ok(())
    }

    pub(crate) fn program_color_ambient(gl: &Gl) -> Result<Rc<Program>, Error>
    {
        Ok(Rc::new(Program::from_source(gl, include_str!("shaders/mesh_instanced.vert"),
                                                                     &format!("{}\n{}",
                                                                              &include_str!("shaders/light_shared.frag"),
                                                                              &include_str!("shaders/colored_forward_ambient.frag")))?))
    }

    pub(crate) fn program_color_ambient_directional(gl: &Gl) -> Result<Rc<Program>, Error>
    {
        Ok(Rc::new(Program::from_source(gl, include_str!("shaders/mesh_instanced.vert"),
                                                                     &format!("{}\n{}",
                                                                              &include_str!("shaders/light_shared.frag"),
                                                                              &include_str!("shaders/colored_forward_ambient_directional.frag")))?))
    }

    pub(crate) fn program_texture_ambient(gl: &Gl) -> Result<Rc<Program>, Error>
    {
        Ok(Rc::new(Program::from_source(gl, include_str!("shaders/mesh_instanced.vert"),
                                                                       &format!("{}\n{}\n{}",
                                                                                include_str!("shaders/light_shared.frag"),
                                                                                include_str!("shaders/triplanar_mapping.frag"),
                                                                                include_str!("shaders/textured_forward_ambient.frag")))?))
    }

    pub(crate) fn program_texture_ambient_directional(gl: &Gl) -> Result<Rc<Program>, Error>
    {
        Ok(Rc::new(Program::from_source(gl, include_str!("shaders/mesh_instanced.vert"),
                                                                       &format!("{}\n{}\n{}",
                                                                                include_str!("shaders/light_shared.frag"),
                                                                                include_str!("shaders/triplanar_mapping.frag"),
                                                                                include_str!("shaders/textured_forward_ambient_directional.frag")))?))
    }

    pub(crate) fn new_with_programs(gl: &Gl, transformations: &[Mat4], program_color_ambient: Rc<Program>, program_color_ambient_directional: Rc<Program>,
                                    program_texture_ambient: Rc<Program>, program_texture_ambient_directional: Rc<Program>,
                   cpu_mesh: &CPUMesh, material: &PhongMaterial) -> Result<Self, Error>
    {
        Ok(Self { name: cpu_mesh.name.clone(), gpu_mesh: InstancedGPUMesh::new(gl, transformations, cpu_mesh)?,
            program_color_ambient, program_color_ambient_directional, program_texture_ambient,
            program_texture_ambient_directional, material: material.clone() })
    }
}

pub struct PhongDeferredMesh {
    pub name: String,
    program_deferred_color: Rc<Program>,
    program_deferred_texture: Rc<Program>,
    gpu_mesh: GPUMesh,
    pub material: PhongMaterial
}

impl PhongDeferredMesh {

    pub fn new(gl: &Gl, cpu_mesh: &CPUMesh, material: &PhongMaterial) -> Result<Self, Error>
    {
        Ok(Self::new_with_programs(gl, cpu_mesh, material,
                                   Self::program_color(gl)?,
                                Self::program_textured(gl)?)?)
    }

    pub fn render_depth(&self, transformation: &Mat4, camera: &camera::Camera) -> Result<(), Error>
    {
        self.render_geometry(transformation, camera)
    }

    pub fn render_geometry(&self, transformation: &Mat4, camera: &camera::Camera) -> Result<(), Error>
    {
        let program = match self.material.color_source {
            ColorSource::Color(_) => self.program_deferred_color.as_ref(),
            ColorSource::Texture(_) => self.program_deferred_texture.as_ref()
        };
        self.gpu_mesh.render(program, &self.material, transformation, camera, None)
    }

    pub(crate) fn program_color(gl: &Gl) -> Result<Rc<Program>, Error>
    {
        Ok(Rc::new(Program::from_source(&gl,include_str!("shaders/mesh.vert"),
                                                              &format!("{}\n{}",
                                                             include_str!("shaders/deferred_objects_shared.frag"),
                                                             include_str!("shaders/colored_deferred.frag")))?))
    }

    pub(crate) fn program_textured(gl: &Gl) -> Result<Rc<Program>, Error>
    {
        Ok(Rc::new(Program::from_source(&gl,include_str!("shaders/mesh.vert"),
                                                    &format!("{}\n{}\n{}",
                                                             include_str!("shaders/deferred_objects_shared.frag"),
                                                             include_str!("shaders/triplanar_mapping.frag"),
                                                             include_str!("shaders/textured_deferred.frag")))?))
    }

    pub(crate) fn new_with_programs(gl: &Gl, cpu_mesh: &CPUMesh, material: &PhongMaterial, program_deferred_color: Rc<Program>, program_deferred_texture: Rc<Program>) -> Result<Self, Error>
    {
        Ok(Self { name: cpu_mesh.name.clone(), gpu_mesh: GPUMesh::new(gl, cpu_mesh)?, material: material.clone(), program_deferred_color, program_deferred_texture })
    }
}

pub struct PhongDeferredInstancedMesh {
    pub name: String,
    program_deferred_color: Rc<Program>,
    program_deferred_texture: Rc<Program>,
    gpu_mesh: InstancedGPUMesh,
    pub material: PhongMaterial
}

impl PhongDeferredInstancedMesh
{
    pub fn new(gl: &Gl, transformations: &[Mat4], cpu_mesh: &CPUMesh, material: &PhongMaterial) -> Result<Self, Error>
    {
        Self::new_with_programs(gl, transformations, cpu_mesh, material,
                                   Self::program_color(gl)?,
                                Self::program_textured(gl)?)
    }

    pub fn update_transformations(&mut self, transformations: &[Mat4])
    {
        self.gpu_mesh.update_transformations(transformations);
    }

    pub fn render_depth(&self, transformation: &Mat4, camera: &camera::Camera) -> Result<(), Error>
    {
        self.render_geometry(transformation, camera)
    }

    pub fn render_geometry(&self, transformation: &Mat4, camera: &camera::Camera) -> Result<(), Error>
    {
        let program = match self.material.color_source {
            ColorSource::Color(_) => self.program_deferred_color.as_ref(),
            ColorSource::Texture(_) => self.program_deferred_texture.as_ref()
        };

        self.gpu_mesh.render(program, &self.material, transformation, camera)
    }

    pub(crate) fn program_color(gl: &Gl) -> Result<Rc<Program>, Error>
    {
        Ok(Rc::new(Program::from_source(&gl,
                                                    include_str!("shaders/mesh_instanced.vert"),
                                                    &format!("{}\n{}",
                                                             include_str!("shaders/deferred_objects_shared.frag"),
                                                             include_str!("shaders/colored_deferred.frag")))?))
    }

    pub(crate) fn program_textured(gl: &Gl) -> Result<Rc<Program>, Error>
    {
        Ok(Rc::new(Program::from_source(&gl,
                                                    include_str!("shaders/mesh_instanced.vert"),
                                                    &format!("{}\n{}\n{}",
                                                             include_str!("shaders/deferred_objects_shared.frag"),
                                                             include_str!("shaders/triplanar_mapping.frag"),
                                                             include_str!("shaders/textured_deferred.frag")))?))
    }

    pub(crate) fn new_with_programs(gl: &Gl, transformations: &[Mat4], cpu_mesh: &CPUMesh, material: &PhongMaterial,
                                    program_deferred_color: Rc<Program>, program_deferred_texture: Rc<Program>) -> Result<Self, Error>
    {
        Ok(Self { name: cpu_mesh.name.clone(),
            gpu_mesh: InstancedGPUMesh::new(gl, transformations, cpu_mesh)?,
            material: material.clone(), program_deferred_color, program_deferred_texture })
    }
}

struct InstancedGPUMesh {
    gpu_mesh: GPUMesh,
    instance_count: u32,
    instance_buffer1: VertexBuffer,
    instance_buffer2: VertexBuffer,
    instance_buffer3: VertexBuffer,
}

impl InstancedGPUMesh
{
    pub fn new(gl: &Gl, transformations: &[Mat4], cpu_mesh: &CPUMesh) -> Result<Self, Error>
    {
        let mut mesh = Self { instance_count: 0,
            instance_buffer1: VertexBuffer::new_with_dynamic_f32(gl, &[])?,
            instance_buffer2: VertexBuffer::new_with_dynamic_f32(gl, &[])?,
            instance_buffer3: VertexBuffer::new_with_dynamic_f32(gl, &[])?,
            gpu_mesh: GPUMesh::new(gl, cpu_mesh)?};
        mesh.update_transformations(transformations);
        Ok(mesh)
    }

    pub fn render(&self, program: &Program, material: &PhongMaterial, transformation: &Mat4, camera: &camera::Camera) -> Result<(), Error>
    {
        program.use_attribute_vec4_float_divisor(&self.instance_buffer1, "row1", 1)?;
        program.use_attribute_vec4_float_divisor(&self.instance_buffer2, "row2", 1)?;
        program.use_attribute_vec4_float_divisor(&self.instance_buffer3, "row3", 1)?;
        self.gpu_mesh.render(program, material,transformation, camera, Some(self.instance_count))
    }

    pub fn update_transformations(&mut self, transformations: &[Mat4])
    {
        self.instance_count = transformations.len() as u32;
        let mut row1 = Vec::new();
        let mut row2 = Vec::new();
        let mut row3 = Vec::new();
        for transform in transformations {
            row1.push(transform.x.x);
            row1.push(transform.y.x);
            row1.push(transform.z.x);
            row1.push(transform.w.x);

            row2.push(transform.x.y);
            row2.push(transform.y.y);
            row2.push(transform.z.y);
            row2.push(transform.w.y);

            row3.push(transform.x.z);
            row3.push(transform.y.z);
            row3.push(transform.z.z);
            row3.push(transform.w.z);
        }
        self.instance_buffer1.fill_with_dynamic_f32(&row1);
        self.instance_buffer2.fill_with_dynamic_f32(&row2);
        self.instance_buffer3.fill_with_dynamic_f32(&row3);
    }
}

struct GPUMesh {
    position_buffer: VertexBuffer,
    normal_buffer: VertexBuffer,
    index_buffer: Option<ElementBuffer>,
    uv_buffer: Option<VertexBuffer>,
}

impl GPUMesh {
    fn new(gl: &Gl, cpu_mesh: &CPUMesh) -> Result<Self, Error>
    {
        let position_buffer = VertexBuffer::new_with_static_f32(gl, &cpu_mesh.positions)?;
        let normal_buffer = VertexBuffer::new_with_static_f32(gl,
              cpu_mesh.normals.as_ref().ok_or(Error::FailedToCreateMesh {message:
              "Cannot create a mesh without normals. Consider calling compute_normals on the CPUMesh before creating the mesh.".to_string()})?)?;
        let index_buffer = if let Some(ref ind) = cpu_mesh.indices { Some(ElementBuffer::new_with_u32(gl, ind)?) } else {None};
        let uv_buffer = if let Some(ref uvs) = cpu_mesh.uvs { Some(VertexBuffer::new_with_static_f32(gl, uvs)?) } else {None};

        Ok(GPUMesh {position_buffer, normal_buffer, index_buffer, uv_buffer})
    }

    fn render(&self, program: &Program, material: &PhongMaterial, transformation: &Mat4, camera: &camera::Camera, instances: Option<u32>) -> Result<(), Error>
    {
        program.add_uniform_float("diffuse_intensity", &material.diffuse_intensity)?;
        program.add_uniform_float("specular_intensity", &material.specular_intensity)?;
        program.add_uniform_float("specular_power", &material.specular_power)?;

        program.add_uniform_mat4("modelMatrix", &transformation)?;
        program.use_uniform_block(camera.matrix_buffer(), "Camera");
        program.add_uniform_mat4("normalMatrix", &transformation.invert().unwrap().transpose())?;

        match material.color_source {
            ColorSource::Color(ref color) => {
                program.add_uniform_vec4("color", color)?;
            },
            ColorSource::Texture(ref texture) => {
                program.use_texture(texture.as_ref(),"tex")?;
                if let Some(ref uv_buffer) = self.uv_buffer {
                    program.add_uniform_int("use_uvs", &1)?;
                    program.use_attribute_vec2_float(uv_buffer, "uv_coordinates")?;
                } else {
                    program.add_uniform_int("use_uvs", &0)?;
                }
            }
        }

        program.use_attribute_vec3_float(&self.position_buffer, "position")?;
        program.use_attribute_vec3_float(&self.normal_buffer, "normal")?;

        if let Some(instance_count) = instances {
            if let Some(ref index_buffer) = self.index_buffer {
                program.draw_elements_instanced(index_buffer, instance_count);
            } else {
                program.draw_arrays_instanced(self.position_buffer.count() as u32/3, instance_count);
            }
        }
        else {
            if let Some(ref index_buffer) = self.index_buffer {
                program.draw_elements(index_buffer);
            } else {
                program.draw_arrays(self.position_buffer.count() as u32/3);
            }
        }
        Ok(())
    }
}

