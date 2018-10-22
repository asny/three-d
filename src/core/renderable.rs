use mesh::*;

pub trait Renderable
{
    fn indices(&self) -> Vec<u32>;

    fn attribute(&self, name: &str) -> Option<Attribute>;

    fn no_vertices(&self) -> usize;
}

impl Renderable for StaticMesh
{
    fn indices(&self) -> Vec<u32>
    {
        self.indices().clone()
    }

    fn attribute(&self, name: &str) -> Option<Attribute>
    {
        self.attribute(name)
    }

    fn no_vertices(&self) -> usize
    {
        self.no_vertices()
    }
}

impl Renderable for DynamicMesh
{
    fn indices(&self) -> Vec<u32>
    {
        let vertices: Vec<VertexID> = self.vertex_iterator().collect();
        let mut indices = Vec::with_capacity(self.no_faces() * 3);
        for face_id in self.face_iterator()
        {
            for walker in self.face_halfedge_iterator(&face_id) {
                let vertex_id = walker.vertex_id().unwrap();
                let index = vertices.iter().position(|v| v == &vertex_id).unwrap();
                indices.push(index as u32);
            }
        }
        indices
    }

    fn attribute(&self, name: &str) -> Option<Attribute>
    {
        match name {
            "position" => {
                let mut pos = Vec::with_capacity(self.no_vertices() * 3);
                for v3 in self.vertex_iterator().map(|ref vertex_id| self.position(vertex_id)) {
                    pos.push(v3.x); pos.push(v3.y); pos.push(v3.z);
                }
                Some(Attribute::new("position", 3, pos))
            },
            "normal" => {
                let mut nor = Vec::with_capacity(self.no_vertices() * 3);
                for vertex_id in self.vertex_iterator() {
                    if let Some(normal) = self.normal(&vertex_id)
                    {
                        nor.push(normal.x); nor.push(normal.y); nor.push(normal.z);
                    }
                    else { return None; }
                }
                Some(Attribute::new("normal", 3, nor))
            },
            _ => None
        }
    }

    fn no_vertices(&self) -> usize
    {
        self.no_vertices()
    }
}