use crate::geometries::mesh::Mesh;
use crate::objects::mesh_shader::MeshShader;
use crate::types::*;
use crate::camera::*;

pub struct Object {
    name: String,
    meshes: Vec<(Mesh, MeshShader)>
}

impl Object {
    pub fn new(name: String) -> Self {
        Object { name, meshes: Vec::new() }
    }

    pub fn add(&mut self, mesh: Mesh, material: MeshShader) {
        self.meshes.push((mesh, material));
    }

    pub fn render(&self, transformation: &Mat4, camera: &Camera) {
        for (mesh, material) in self.meshes.iter() {
            material.render(&mesh, transformation, camera);
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}