
use crate::*;
use std::rc::Rc;

pub enum ColorSource {
    Color(Vec4),
    Texture(Rc<Texture2D>)
}

pub struct Mesh {
    pub name: String,
    program_color: Rc<Program>,
    program_texture: Rc<Program>,
    position_buffer: VertexBuffer,
    normal_buffer: VertexBuffer,
    index_buffer: Option<ElementBuffer>,
    uv_buffer: Option<VertexBuffer>,
    pub color: ColorSource,
    pub diffuse_intensity: f32,
    pub specular_intensity: f32,
    pub specular_power: f32
}

impl Mesh
{
    pub(crate) fn new_with_programs(gl: &Gl, program_color: Rc<Program>, program_texture: Rc<Program>,
                   cpu_mesh: &CPUMesh) -> Result<Self, Error>
    {
        let position_buffer = VertexBuffer::new_with_static_f32(gl, &cpu_mesh.positions)?;
        let normal_buffer = VertexBuffer::new_with_static_f32(gl,
              cpu_mesh.normals.as_ref().ok_or(Error::FailedToCreateMesh {message:
              "Cannot create a mesh without normals. Consider calling compute_normals on the CPUMesh before creating the mesh.".to_string()})?)?;
        let index_buffer = if let Some(ref ind) = cpu_mesh.indices { Some(ElementBuffer::new_with_u32(gl, ind)?) } else {None};
        let uv_buffer = if let Some(ref uvs) = cpu_mesh.uvs { Some(VertexBuffer::new_with_static_f32(gl, uvs)?) } else {None};

        let color = if let Some(ref img) = cpu_mesh.texture {
            use image::GenericImageView;
            ColorSource::Texture(Rc::new(texture::Texture2D::new_with_u8(&gl, Interpolation::Linear, Interpolation::Linear,
                                                                  Some(Interpolation::Linear), Wrapping::Repeat, Wrapping::Repeat,
                                                                  img.width(), img.height(), &img.to_bytes())?))
        } else {
            ColorSource::Color(cpu_mesh.color.map(|(r, g, b, a)| vec4(r, g, b, a)).unwrap_or(vec4(1.0, 1.0, 1.0, 1.0)))
        };

        Ok(Self { name: cpu_mesh.name.clone(), index_buffer, uv_buffer, position_buffer, normal_buffer,
            program_color, program_texture, color,
            diffuse_intensity: cpu_mesh.diffuse_intensity.unwrap_or(0.5),
            specular_intensity: cpu_mesh.specular_intensity.unwrap_or(0.2),
            specular_power: cpu_mesh.specular_power.unwrap_or(6.0) })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn render_with_lighting(&self, transformation: &Mat4, camera: &camera::Camera, light: &DirectionalLight) -> Result<(), Error>
    {
        let program = match self.color {
            ColorSource::Color(_) => self.program_color.as_ref(),
            ColorSource::Texture(_) => self.program_texture.as_ref()
        };
        program.add_uniform_vec3("eyePosition", &camera.position())?;
        program.use_texture(light.shadow_map(), "shadowMap")?;
        program.use_uniform_block(light.buffer(), "DirectionalLightUniform");
        self.render_internal(program, transformation, camera)?;
        Ok(())
    }

    fn render_internal(&self, program: &Program, transformation: &Mat4, camera: &camera::Camera) -> Result<(), Error>
    {
        program.add_uniform_float("diffuse_intensity", &self.diffuse_intensity)?;
        program.add_uniform_float("specular_intensity", &self.specular_intensity)?;
        program.add_uniform_float("specular_power", &self.specular_power)?;

        program.add_uniform_mat4("modelMatrix", &transformation)?;
        program.use_uniform_block(camera.matrix_buffer(), "Camera");
        program.add_uniform_mat4("normalMatrix", &transformation.invert().unwrap().transpose())?;

        match self.color {
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
    pub(crate) fn new_with_programs(mesh: Mesh, program_deferred_color: Rc<Program>, program_deferred_texture: Rc<Program>) -> Self
    {
        Self { mesh,program_deferred_color, program_deferred_texture }
    }

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
        let program = match self.mesh.color {
            ColorSource::Color(_) => self.program_deferred_color.as_ref(),
            ColorSource::Texture(_) => self.program_deferred_texture.as_ref()
        };
        self.mesh.render_internal(program, transformation, camera)?;
        Ok(())
    }
}