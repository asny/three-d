
use crate::*;
use std::rc::Rc;

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
        if cpu_mesh.normals.is_none() {
            Err(Error::FailedToCreateMesh {message:
              "Cannot create a mesh without normals. Consider calling compute_normals on the CPUMesh before creating the mesh.".to_string()})?
        }
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

        bind_material(program, &self.material, self.gpu_mesh.has_uvs())?;
        self.gpu_mesh.render(program, transformation, camera)
    }

    pub(crate) fn program_color(gl: &Gl) -> Result<Rc<Program>, Error>
    {
        Ok(Rc::new(GPUMesh::create_program(gl, &format!("{}\n{}",
                                                             include_str!("shaders/deferred_objects_shared.frag"),
                                                             include_str!("shaders/colored_deferred.frag")))?))
    }

    pub(crate) fn program_textured(gl: &Gl) -> Result<Rc<Program>, Error>
    {
        Ok(Rc::new(GPUMesh::create_program(gl, &format!("{}\n{}",
                                                             include_str!("shaders/deferred_objects_shared.frag"),
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
        if cpu_mesh.normals.is_none() {
            Err(Error::FailedToCreateMesh {message:
              "Cannot create a mesh without normals. Consider calling compute_normals on the CPUMesh before creating the mesh.".to_string()})?
        }
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

        bind_material(program, &self.material, self.gpu_mesh.has_uvs())?;
        self.gpu_mesh.render(program, transformation, camera)
    }

    pub(crate) fn program_color(gl: &Gl) -> Result<Rc<Program>, Error>
    {
        Ok(Rc::new(InstancedGPUMesh::create_program(gl, &format!("{}\n{}",
                                                             include_str!("shaders/deferred_objects_shared.frag"),
                                                             include_str!("shaders/colored_deferred.frag")))?))
    }

    pub(crate) fn program_textured(gl: &Gl) -> Result<Rc<Program>, Error>
    {
        Ok(Rc::new(InstancedGPUMesh::create_program(gl, &format!("{}\n{}",
                                                             include_str!("shaders/deferred_objects_shared.frag"),
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

fn bind_material(program: &Program, material: &PhongMaterial, has_uvs: bool) -> Result<(), Error> {
    program.add_uniform_float("diffuse_intensity", &material.diffuse_intensity)?;
    program.add_uniform_float("specular_intensity", &material.specular_intensity)?;
    program.add_uniform_float("specular_power", &material.specular_power)?;

    match material.color_source {
        ColorSource::Color(ref color) => {
            program.add_uniform_vec4("color", color)?;
        },
        ColorSource::Texture(ref texture) => {
            if !has_uvs {
                Err(Error::FailedToCreateMesh {message:"Cannot use a texture as color source without uv coordinates.".to_string()})?;
            }
            program.use_texture(texture.as_ref(),"tex")?;
        }
    }
    Ok(())
}