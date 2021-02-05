
use crate::core::*;
use crate::objects::*;
use crate::phong::*;

pub struct PhongDeferredMesh {
    gl: Context,
    pub name: String,
    mesh: Mesh,
    pub material: PhongMaterial
}

impl PhongDeferredMesh {

    pub fn new(gl: &Context, cpu_mesh: &CPUMesh, material: &PhongMaterial) -> Result<Self, Error>
    {
        if cpu_mesh.normals.is_none() {
            Err(Error::FailedToCreateMesh {message:
              "Cannot create a mesh without normals. Consider calling compute_normals on the CPUMesh before creating the mesh.".to_string()})?
        }
        let mesh = Mesh::new(gl, cpu_mesh)?;
        unsafe {MESH_COUNT += 1;}
        Ok(Self {
            gl: gl.clone(),
            name: cpu_mesh.name.clone(),
            mesh,
            material: material.clone()
        })
    }

    pub fn new_meshes(gl: &Context, cpu_meshes: &[CPUMesh], materials: &[PhongMaterial]) -> Result<Vec<Self>, Error>
    {
        let mut meshes = Vec::new();
        for cpu_mesh in cpu_meshes {
            let material = cpu_mesh.material_name.as_ref().map(|material_name|
                materials.iter().filter(|m| &m.name == material_name).last()
                .map(|m| m.clone()).unwrap_or_else(|| PhongMaterial::default()))
                .unwrap_or_else(|| PhongMaterial::default());
            meshes.push(Self::new(gl,cpu_mesh, &material)?);
        }
        Ok(meshes)
    }

    pub fn render_depth(&self, render_states: RenderStates, viewport: Viewport, transformation: &Mat4, camera: &camera::Camera) -> Result<(), Error>
    {
        self.render_geometry(render_states, viewport, transformation, camera)
    }

    pub fn render_geometry(&self, render_states: RenderStates, viewport: Viewport, transformation: &Mat4, camera: &camera::Camera) -> Result<(), Error>
    {
        let program = match self.material.color_source {
            ColorSource::Color(_) => {
                unsafe {
                    if PROGRAM_COLOR.is_none()
                    {
                        PROGRAM_COLOR = Some(Mesh::create_program(&self.gl, &format!("{}\n{}",
                                                             include_str!("shaders/deferred_objects_shared.frag"),
                                                             include_str!("shaders/colored_deferred.frag")))?);
                    }
                    PROGRAM_COLOR.as_ref().unwrap()
                }
            },
            ColorSource::Texture(_) => {
                unsafe {
                    if PROGRAM_TEXTURE.is_none()
                    {
                        PROGRAM_TEXTURE = Some(Mesh::create_program(&self.gl, &format!("{}\n{}",
                                                             include_str!("shaders/deferred_objects_shared.frag"),
                                                             include_str!("shaders/textured_deferred.frag")))?);
                    }
                    PROGRAM_TEXTURE.as_ref().unwrap()
                }
            }
        };
        self.material.bind(program, self.mesh.has_uvs())?;
        self.mesh.render(program, render_states, viewport, transformation, camera)
    }
}

impl Drop for PhongDeferredMesh {

    fn drop(&mut self) {
        unsafe {
            MESH_COUNT -= 1;
            if MESH_COUNT == 0 {
                PROGRAM_COLOR = None;
                PROGRAM_TEXTURE = None;
            }
        }
    }
}

static mut PROGRAM_COLOR: Option<Program> = None;
static mut PROGRAM_TEXTURE: Option<Program> = None;
static mut MESH_COUNT: u32 = 0;