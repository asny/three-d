pub mod wireframe;
pub mod shaded_colored_mesh;
pub mod shaded_textured_mesh;
pub mod skybox;

pub use objects::wireframe::Wireframe as Wireframe;
pub use objects::shaded_colored_mesh::ShadedColoredMesh as ShadedColoredMesh;
pub use objects::shaded_textured_mesh::ShadedTexturedMesh as ShadedTexturedMesh;
pub use objects::skybox::Skybox as Skybox;