
use crate::*;
use std::rc::Rc;
use std::cell::RefCell;

pub struct AxisAllignedBoundingBox {
    pub min: Vec3,
    pub max: Vec3
}

impl AxisAllignedBoundingBox {
    pub fn add(&self, other: &AxisAllignedBoundingBox) -> AxisAllignedBoundingBox {
        return AxisAllignedBoundingBox {
            min: vec3(f32::min(self.min.x, other.min.x), f32::min(self.min.y, other.min.y), f32::min(self.min.z, other.min.z)),
            max: vec3(f32::max(self.max.x, other.max.x), f32::max(self.max.y, other.max.y), f32::max(self.max.z, other.max.z))
        }
    }
}

pub struct Mesh {
    position_buffer: VertexBuffer,
    normal_buffer: VertexBuffer,
    index_buffer: ElementBuffer,
    program: program::Program,
    aabb: AxisAllignedBoundingBox,
    pub color: Vec3,
    pub texture: Option<texture::Texture2D>,
    pub diffuse_intensity: f32,
    pub specular_intensity: f32,
    pub specular_power: f32
}

impl Mesh
{
    pub fn new(gl: &Gl, indices: &[u32], positions: &[f32], normals: &[f32]) -> Result<Self, Error>
    {
        let position_buffer = VertexBuffer::new_with_static_f32(gl, positions)?;
        let normal_buffer = VertexBuffer::new_with_static_f32(gl, normals)?;
        let index_buffer = ElementBuffer::new_with_u32(gl, indices)?;

        let program = program::Program::from_source(&gl,
                                                    include_str!("shaders/mesh_shaded.vert"),
                                                    include_str!("shaders/shaded.frag"))?;

        Ok(Mesh { index_buffer, position_buffer, normal_buffer, program, aabb: compute_aabb(positions), color: vec3(1.0, 1.0, 1.0), texture: None,
            diffuse_intensity: 0.5, specular_intensity: 0.2, specular_power: 6.0 })
    }

    pub fn from_file(gl: &Gl, path: &'static str) -> Rc<RefCell<Mesh>> {
        let mesh = Rc::new(RefCell::new(Self::new(gl, &[], &[], &[]).unwrap()));
        CPUMesh::from_file_with_mapping(path, mesh.clone(), |cpu_mesh, mesh| {
            mesh.borrow_mut().update(&cpu_mesh.indices, &cpu_mesh.positions, &cpu_mesh.normals).unwrap();
        });
        mesh
    }

    pub fn update(&mut self, indices: &[u32], positions: &[f32], normals: &[f32]) -> Result<(), Error> {
        self.update_indices(indices)?;
        self.update_positions(positions)?;
        self.update_normals(normals)?;
        Ok(())
    }

    pub fn update_indices(&mut self, indices: &[u32]) -> Result<(), Error>
    {
        self.index_buffer.fill_with_u32(indices);
        Ok(())
    }

    pub fn update_positions(&mut self, positions: &[f32]) -> Result<(), Error>
    {
        self.position_buffer.fill_with_static_f32(positions);
        Ok(())
    }

    pub fn update_normals(&mut self, normals: &[f32]) -> Result<(), Error>
    {
        self.normal_buffer.fill_with_static_f32(normals);
        Ok(())
    }

    pub fn render(&self, transformation: &Mat4, camera: &camera::Camera)
    {
        self.program.add_uniform_float("diffuse_intensity", &self.diffuse_intensity).unwrap();
        self.program.add_uniform_float("specular_intensity", &self.specular_intensity).unwrap();
        self.program.add_uniform_float("specular_power", &self.specular_power).unwrap();

        if let Some(ref tex) = self.texture
        {
            self.program.add_uniform_int("use_texture", &1).unwrap();
            self.program.use_texture(tex,"tex").unwrap();
        }
        else {
            self.program.add_uniform_int("use_texture", &0).unwrap();
            self.program.add_uniform_vec3("color", &self.color).unwrap();
        }

        self.program.add_uniform_mat4("modelMatrix", &transformation).unwrap();
        self.program.use_uniform_block(camera.matrix_buffer(), "Camera");

        self.program.add_uniform_mat4("normalMatrix", &transformation.invert().unwrap().transpose()).unwrap();

        self.program.use_attribute_vec3_float(&self.position_buffer, "position").unwrap();
        self.program.use_attribute_vec3_float(&self.normal_buffer, "normal").unwrap();

        self.program.draw_elements(&self.index_buffer);
    }

    pub fn axis_aligned_bounding_box(&self) -> &AxisAllignedBoundingBox
    {
        &self.aabb
    }
}

fn compute_aabb(positions: &[f32]) -> AxisAllignedBoundingBox {

    let mut aabb = AxisAllignedBoundingBox {min: vec3(std::f32::INFINITY, std::f32::INFINITY, std::f32::INFINITY),
        max: vec3(std::f32::NEG_INFINITY, std::f32::NEG_INFINITY, std::f32::NEG_INFINITY)};

    for i in 0..positions.len() {
        match i%3 {
            0 => {
                aabb.min.x = f32::min(positions[i], aabb.min.x);
                aabb.max.x = f32::max(positions[i], aabb.max.x);
            },
            1 => {
                aabb.min.y = f32::min(positions[i], aabb.min.y);
                aabb.max.y = f32::max(positions[i], aabb.max.y);
            },
            2 => {
                aabb.min.z = f32::min(positions[i], aabb.min.z);
                aabb.max.z = f32::max(positions[i], aabb.max.z);
            },
            _ => {unreachable!()}
        };
    }
    aabb
}