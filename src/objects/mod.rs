pub mod shaded_vertices;
pub mod shaded_edges;
pub mod wireframe;
pub mod shaded_mesh;
pub mod skybox;

pub use objects::shaded_vertices::ShadedVertices as ShadedVertices;
pub use objects::shaded_edges::ShadedEdges as ShadedEdges;
pub use objects::wireframe::Wireframe as Wireframe;
pub use objects::shaded_mesh::ShadedMesh as ShadedMesh;
pub use objects::skybox::Skybox as Skybox;