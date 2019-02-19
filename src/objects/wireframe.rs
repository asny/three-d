
use gl;
use crate::*;
use crate::objects::*;

pub struct Wireframe {
    edges: ShadedEdges,
    vertices: ShadedVertices
}

impl Wireframe
{
    pub fn new(gl: &gl::Gl, indices: &[u32], positions: &[f32], tube_radius: f32) -> Wireframe
    {
        let edges = ShadedEdges::new(&gl, indices, positions, tube_radius);
        let mut vertices = ShadedVertices::new(&gl, positions);
        vertices.scale = 2.0 * tube_radius;

        Wireframe {edges, vertices}
    }

    pub fn new_from_obj_source(gl: &gl::Gl, source: String, tube_radius: f32, translation: &Vec3) -> Wireframe
    {
        let objs = wavefront_obj::obj::parse(source).unwrap();
        let obj = objs.objects.first().unwrap();

        let mut positions = Vec::new();
        obj.vertices.iter().for_each(|v| {positions.push(v.x as f32); positions.push(v.y as f32); positions.push(v.z as f32);});
        let mut indices = Vec::new();
        for shape in obj.geometry.first().unwrap().shapes.iter() {
            match shape.primitive {
                wavefront_obj::obj::Primitive::Triangle(i0, i1, i2) => {
                    indices.push(i0.0 as u32);
                    indices.push(i1.0 as u32);
                    indices.push(i2.0 as u32);
                },
                _ => {}
            }
        }
        for i in 0..positions.len() {
            positions[i] += translation[i%3];
        }

        Self::new(&gl, &indices, &positions, tube_radius)
    }

    pub fn update_positions(&mut self, positions: &[f32])
    {
        self.vertices.update_positions(positions);
        self.edges.update_positions(positions);
    }

    pub fn render(&self, camera: &camera::Camera)
    {
        self.edges.render(camera);
        self.vertices.render(camera);
    }

    pub fn set_color(&mut self, color: &Vec3)
    {
        self.edges.color = *color;
        self.vertices.color = *color;
    }

    pub fn set_parameters(&mut self, diffuse_intensity: f32, specular_intensity: f32, specular_power: f32)
    {
        self.edges.diffuse_intensity = diffuse_intensity;
        self.edges.specular_intensity = specular_intensity;
        self.edges.specular_power = specular_power;
        self.vertices.diffuse_intensity = diffuse_intensity;
        self.vertices.specular_intensity = specular_intensity;
        self.vertices.specular_power = specular_power;
    }
}