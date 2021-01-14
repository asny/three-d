
use crate::*;
use std::rc::Rc;

pub struct PhongForwardPipeline {
    gl: Gl,
    depth_texture: Option<Texture2D>,
    mesh_color_ambient_program: Rc<Program>,
    mesh_color_ambient_directional_program: Rc<Program>,
    mesh_texture_ambient_program: Rc<Program>,
    mesh_texture_ambient_directional_program: Rc<Program>,
    mesh_instanced_color_ambient_program: Rc<Program>,
    mesh_instanced_color_ambient_directional_program: Rc<Program>,
    mesh_instanced_texture_ambient_program: Rc<Program>,
    mesh_instanced_texture_ambient_directional_program: Rc<Program>,
}

impl PhongForwardPipeline {

    pub fn new(gl: &Gl) -> Result<Self, Error>
    {
        Ok(Self {
            gl: gl.clone(),
            depth_texture: Some(Texture2D::new(gl, 1, 1,
                    Interpolation::Nearest, Interpolation::Nearest, None, Wrapping::ClampToEdge,
                    Wrapping::ClampToEdge, Format::Depth32F)?),
            mesh_color_ambient_program: PhongForwardMesh::program_color_ambient(gl)?,
            mesh_color_ambient_directional_program: PhongForwardMesh::program_color_ambient_directional(gl)?,
            mesh_texture_ambient_program: PhongForwardMesh::program_texture_ambient(gl)?,
            mesh_texture_ambient_directional_program: PhongForwardMesh::program_texture_ambient_directional(gl)?,
            mesh_instanced_color_ambient_program: PhongForwardInstancedMesh::program_color_ambient(gl)?,
            mesh_instanced_color_ambient_directional_program: PhongForwardInstancedMesh::program_color_ambient_directional(gl)?,
            mesh_instanced_texture_ambient_program: PhongForwardInstancedMesh::program_texture_ambient(gl)?,
            mesh_instanced_texture_ambient_directional_program: PhongForwardInstancedMesh::program_texture_ambient_directional(gl)?
        })
    }

    pub fn depth_pass<F: FnOnce() -> Result<(), Error>>(&mut self, width: usize, height: usize, render_scene: F) -> Result<(), Error>
    {
        state::depth_write(&self.gl, true);
        state::depth_test(&self.gl, state::DepthTestType::LessOrEqual);
        state::cull(&self.gl, state::CullType::None);
        state::blend(&self.gl, state::BlendType::None);

        self.depth_texture = Some(Texture2D::new(&self.gl, width, height,
                    Interpolation::Nearest, Interpolation::Nearest, None, Wrapping::ClampToEdge,
                    Wrapping::ClampToEdge, Format::Depth32F)?);
        RenderTarget::write_to_depth(&self.gl,0, 0, width, height,Some(1.0),self.depth_texture.as_ref(), render_scene)?;
        Ok(())
    }

    pub fn render_to_screen<F: FnOnce() -> Result<(), Error>>(&self, width: usize, height: usize, render_scene: F) -> Result<(), Error>
    {
        Ok(Screen::write(&self.gl, 0, 0, width, height,
                         Some(&vec4(0.0, 0.0, 0.0, 1.0)),
                         Some(1.0),
                         render_scene)?)
    }

    pub fn depth_texture(&self) -> &Texture2D
    {
        &self.depth_texture.as_ref().unwrap()
    }

    pub fn new_material(&self, cpu_material: &CPUMaterial) -> Result<PhongMaterial, Error>
    {
        PhongMaterial::new(&self.gl, cpu_material)
    }

    pub fn new_mesh(&self, cpu_mesh: &CPUMesh, material: &PhongMaterial) -> Result<PhongForwardMesh, Error>
    {
        Ok(PhongForwardMesh::new_with_programs(&self.gl,
                  self.mesh_color_ambient_program.clone(),
                  self.mesh_color_ambient_directional_program.clone(),
                  self.mesh_texture_ambient_program.clone(),
                  self.mesh_texture_ambient_directional_program.clone(), cpu_mesh, material)?)
    }

    pub fn new_meshes(&self, cpu_meshes: &Vec<CPUMesh>, cpu_materials: &Vec<CPUMaterial>) -> Result<Vec<PhongForwardMesh>, Error>
    {
        let materials = cpu_materials.iter().map(|m| PhongMaterial::new(&self.gl, m).unwrap()).collect::<Vec<PhongMaterial>>();
        let mut meshes = Vec::new();
        for cpu_mesh in cpu_meshes {
            let material = cpu_mesh.material_name.as_ref().map(|material_name|
                materials.iter().filter(|m| &m.name == material_name).last()
                .map(|m| m.clone()).unwrap_or_else(|| PhongMaterial::default()))
                .unwrap_or_else(|| PhongMaterial::default());
            meshes.push(self.new_mesh(cpu_mesh, &material)?);
        }
        Ok(meshes)
    }

    pub fn new_instanced_mesh(&self, transformations: &[Mat4], cpu_mesh: &CPUMesh, material: &PhongMaterial) -> Result<PhongForwardInstancedMesh, Error>
    {
        PhongForwardInstancedMesh::new_with_programs(&self.gl, transformations,
                  self.mesh_instanced_color_ambient_program.clone(),
                  self.mesh_instanced_color_ambient_directional_program.clone(),
                  self.mesh_instanced_texture_ambient_program.clone(),
                  self.mesh_instanced_texture_ambient_directional_program.clone(), cpu_mesh, material)
    }
}
