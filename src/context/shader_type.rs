use crate::context::consts;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32 /* GLenum */)]
pub enum ShaderType {
    #[cfg(not(target_arch = "wasm32"))]
    Compute = consts::COMPUTE_SHADER,
    Vertex = consts::VERTEX_SHADER,
    #[cfg(not(target_arch = "wasm32"))]
    TessControl = consts::TESS_CONTROL_SHADER,
    #[cfg(not(target_arch = "wasm32"))]
    TessEvaluation = consts::TESS_EVALUATION_SHADER,
    #[cfg(not(target_arch = "wasm32"))]
    Geometry = consts::GEOMETRY_SHADER,
    Fragment = consts::FRAGMENT_SHADER,
}
