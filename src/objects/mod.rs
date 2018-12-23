pub mod shaded_vertices;
pub mod shaded_edges;
pub mod wireframe;
pub mod shaded_mesh;
pub mod skybox;

pub use crate::objects::shaded_vertices::ShadedVertices as ShadedVertices;
pub use crate::objects::shaded_edges::ShadedEdges as ShadedEdges;
pub use crate::objects::wireframe::Wireframe as Wireframe;
pub use crate::objects::shaded_mesh::ShadedMesh as ShadedMesh;
pub use crate::objects::skybox::Skybox as Skybox;