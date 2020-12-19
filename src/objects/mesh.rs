
use crate::*;
use std::rc::Rc;

pub struct Mesh {
    pub name: String,
    program_color_ambient: Rc<Program>,
    program_color_ambient_directional: Rc<Program>,
    program_texture_ambient: Rc<Program>,
    program_texture_ambient_directional: Rc<Program>,
    position_buffer: VertexBuffer,
    normal_buffer: VertexBuffer,
    index_buffer: Option<ElementBuffer>,
    uv_buffer: Option<VertexBuffer>,
    pub material: Material
}

impl Mesh
{
    pub fn new(gl: &Gl, cpu_mesh: &CPUMesh, material: Material) -> Result<Self, Error>
    {
        Self::new_with_programs(gl, Self::program_color_ambient(gl)?,
                                Self::program_color_ambient_directional(gl)?,
                                Self::program_texture_ambient(gl)?,
                                Self::program_texture_ambient_directional(gl)?, cpu_mesh, material)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn render_with_ambient(&self, transformation: &Mat4, camera: &camera::Camera, ambient_light: &AmbientLight) -> Result<(), Error>
    {
        let program = match self.material.color_source {
            ColorSource::Color(_) => self.program_color_ambient.as_ref(),
            ColorSource::Texture(_) => self.program_texture_ambient.as_ref()
        };
        program.add_uniform_vec3("ambientLight.color", &ambient_light.color())?;
        program.add_uniform_float("ambientLight.intensity", &ambient_light.intensity())?;
        self.render_internal(program, transformation, camera)?;
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
        self.render_internal(program, transformation, camera)?;
        Ok(())
    }

    pub(crate) fn program_color_ambient(gl: &Gl) -> Result<Rc<Program>, Error>
    {
        Ok(Rc::new(Program::from_source(gl, include_str!("shaders/mesh.vert"),
                                                                     &format!("{}\n{}",
                                                                              &include_str!("../shaders/light_shared.frag"),
                                                                              &include_str!("shaders/colored_forward_ambient.frag")))?))
    }

    pub(crate) fn program_color_ambient_directional(gl: &Gl) -> Result<Rc<Program>, Error>
    {
        Ok(Rc::new(Program::from_source(gl, include_str!("shaders/mesh.vert"),
                                                                     &format!("{}\n{}",
                                                                              &include_str!("../shaders/light_shared.frag"),
                                                                              &include_str!("shaders/colored_forward_ambient_directional.frag")))?))
    }

    pub(crate) fn program_texture_ambient(gl: &Gl) -> Result<Rc<Program>, Error>
    {
        Ok(Rc::new(Program::from_source(gl, include_str!("shaders/mesh.vert"),
                                                                       &format!("{}\n{}\n{}",
                                                                                include_str!("../shaders/light_shared.frag"),
                                                                                include_str!("shaders/triplanar_mapping.frag"),
                                                                                include_str!("shaders/textured_forward_ambient.frag")))?))
    }

    pub(crate) fn program_texture_ambient_directional(gl: &Gl) -> Result<Rc<Program>, Error>
    {
        Ok(Rc::new(Program::from_source(gl, include_str!("shaders/mesh.vert"),
                                                                       &format!("{}\n{}\n{}",
                                                                                include_str!("../shaders/light_shared.frag"),
                                                                                include_str!("shaders/triplanar_mapping.frag"),
                                                                                include_str!("shaders/textured_forward_ambient_directional.frag")))?))
    }

    pub(crate) fn new_with_programs(gl: &Gl, program_color_ambient: Rc<Program>, program_color_ambient_directional: Rc<Program>,
                                    program_texture_ambient: Rc<Program>, program_texture_ambient_directional: Rc<Program>,
                   cpu_mesh: &CPUMesh, material: Material) -> Result<Self, Error>
    {
        let position_buffer = VertexBuffer::new_with_static_f32(gl, &cpu_mesh.positions)?;
        let normal_buffer = VertexBuffer::new_with_static_f32(gl,
              cpu_mesh.normals.as_ref().ok_or(Error::FailedToCreateMesh {message:
              "Cannot create a mesh without normals. Consider calling compute_normals on the CPUMesh before creating the mesh.".to_string()})?)?;
        let index_buffer = if let Some(ref ind) = cpu_mesh.indices { Some(ElementBuffer::new_with_u32(gl, ind)?) } else {None};
        let uv_buffer = if let Some(ref uvs) = cpu_mesh.uvs { Some(VertexBuffer::new_with_static_f32(gl, uvs)?) } else {None};

        Ok(Self { name: cpu_mesh.name.clone(), index_buffer, uv_buffer, position_buffer, normal_buffer,
            program_color_ambient, program_color_ambient_directional, program_texture_ambient, program_texture_ambient_directional, material })
    }

    fn render_internal(&self, program: &Program, transformation: &Mat4, camera: &camera::Camera) -> Result<(), Error>
    {
        program.add_uniform_float("diffuse_intensity", &self.material.diffuse_intensity.unwrap_or(0.5))?;
        program.add_uniform_float("specular_intensity", &self.material.specular_intensity.unwrap_or(0.2))?;
        program.add_uniform_float("specular_power", &self.material.specular_power.unwrap_or(6.0))?;

        program.add_uniform_mat4("modelMatrix", &transformation)?;
        program.use_uniform_block(camera.matrix_buffer(), "Camera");
        program.add_uniform_mat4("normalMatrix", &transformation.invert().unwrap().transpose())?;

        match self.material.color_source {
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

        if let Some(ref index_buffer) = self.index_buffer {
            program.draw_elements(index_buffer);
        } else {
            program.draw_arrays(self.position_buffer.count() as u32/3);
        }
        Ok(())
    }
}

pub struct DeferredMesh {
    mesh: Mesh,
    program_deferred_color: Rc<Program>,
    program_deferred_texture: Rc<Program>
}

impl DeferredMesh {
    pub fn name(&self) -> &str {
        self.mesh.name()
    }

    pub fn mesh(&self) -> &Mesh {
        &self.mesh
    }

    pub fn mesh_mut(&mut self) -> &mut Mesh {
        &mut self.mesh
    }

    pub fn render_geometry(&self, transformation: &Mat4, camera: &camera::Camera) -> Result<(), Error>
    {
        let program = match self.mesh.material.color_source {
            ColorSource::Color(_) => self.program_deferred_color.as_ref(),
            ColorSource::Texture(_) => self.program_deferred_texture.as_ref()
        };
        self.mesh.render_internal(program, transformation, camera)?;
        Ok(())
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

    pub(crate) fn new_with_programs(mesh: Mesh, program_deferred_color: Rc<Program>, program_deferred_texture: Rc<Program>) -> Self
    {
        Self { mesh,program_deferred_color, program_deferred_texture }
    }
}