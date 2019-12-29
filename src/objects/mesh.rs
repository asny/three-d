
use crate::*;

#[derive(Debug)]
pub enum Error {
    Program(program::Error),
    Buffer(buffer::Error)
}

impl From<program::Error> for Error {
    fn from(other: program::Error) -> Self {
        Error::Program(other)
    }
}

impl From<buffer::Error> for Error {
    fn from(other: buffer::Error) -> Self {
        Error::Buffer(other)
    }
}

pub struct AxisAllignedBoundingBox {
    min: Vec3,
    max: Vec3
}

pub struct Mesh {
    name: String,
    position_buffer: StaticVertexBuffer,
    normal_buffer: StaticVertexBuffer,
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
    pub fn new(name: String, gl: &Gl, indices: &[u32], positions: &[f32], normals: &[f32]) -> Result<Self, Error>
    {
        let position_buffer = StaticVertexBuffer::new_with_vec3(gl, positions)?;
        let normal_buffer = StaticVertexBuffer::new_with_vec3(gl, normals)?;
        let index_buffer = ElementBuffer::new_with(gl, indices)?;

        let program = program::Program::from_source(&gl,
                                                    include_str!("shaders/mesh_shaded.vert"),
                                                    include_str!("shaders/shaded.frag"))?;

        Ok(Mesh { name, index_buffer, position_buffer, normal_buffer, program, aabb: compute_aabb(positions), color: vec3(1.0, 1.0, 1.0), texture: None,
            diffuse_intensity: 0.5, specular_intensity: 0.2, specular_power: 6.0 })
    }

    pub fn new_with_computed_normals(name: String, gl: &Gl, indices: &[u32], positions: &[f32]) -> Result<Self, Error>
    {
        Self::new(name, gl, indices, positions, &compute_normals(indices, positions))
    }

    pub fn update_positions(&mut self, positions: &[f32]) -> Result<(), Error>
    {
        self.position_buffer.clear();
        self.position_buffer.add(positions, 3);
        self.position_buffer.send_data();
        Ok(())
    }

    pub fn update_normals(&mut self, normals: &[f32]) -> Result<(), Error>
    {
        self.normal_buffer.clear();
        self.normal_buffer.add(normals, 3);
        self.normal_buffer.send_data();
        Ok(())
    }

    pub fn render(&self, transformation: &Mat4, camera: &camera::Camera)
    {
        self.program.cull(state::CullType::NONE);
        self.program.depth_test(state::DepthTestType::LEQUAL);
        self.program.depth_write(true);

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

        self.program.use_attribute_vec3_float(&self.position_buffer, "position", 0).unwrap();
        self.program.use_attribute_vec3_float(&self.normal_buffer, "normal", 0).unwrap();

        self.program.draw_elements(&self.index_buffer);
    }

    pub fn axis_aligned_bounding_box(&self) -> &AxisAllignedBoundingBox
    {
        &self.aabb
    }

    pub fn name(&self) -> &str {
        &self.name
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

fn compute_normals(indices: &[u32], positions: &[f32]) -> Vec<f32> {
    let mut normals = vec![0.0f32; positions.len() * 3];
    for face in 0..indices.len()/3 {
        let index0 = indices[face*3] as usize;
        let p0 = vec3(positions[index0*3], positions[index0*3+1], positions[index0*3+2]);
        let index1 = indices[face*3 + 1] as usize;
        let p1 = vec3(positions[index1*3], positions[index1*3+1], positions[index1*3+2]);
        let index2 = indices[face*3 + 2] as usize;
        let p2 = vec3(positions[index2*3], positions[index2*3+1], positions[index2*3+2]);

        let normal = (p1 - p0).cross(p2 - p0);
        normals[index0*3] += normal.x;
        normals[index0*3+1] += normal.y;
        normals[index0*3+2] += normal.z;
        normals[index1*3] += normal.x;
        normals[index1*3+1] += normal.y;
        normals[index1*3+2] += normal.z;
        normals[index2*3] += normal.x;
        normals[index2*3+1] += normal.y;
        normals[index2*3+2] += normal.z;
    }

    for i in 0..normals.len()/3 {
        let normal = vec3(normals[3*i], normals[3*i+1], normals[3*i+2]).normalize();
        normals[3*i] = normal.x;
        normals[3*i+1] = normal.y;
        normals[3*i+2] = normal.z;
    }
    normals
}