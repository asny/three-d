use crate::core::*;
use crate::io::*;
use ply_rs::{parser, ply};
use std::io::Cursor;
use std::path::Path;

impl Loaded {
    ///
    /// Deserialize a loaded .ply file into a [CpuPointCloud].
    ///
    pub fn ply(&mut self, path: impl AsRef<Path>) -> ThreeDResult<CpuPointCloud> {
        let p = parser::Parser::<ply::DefaultElement>::new();
        let bytes = self.get_bytes(path.as_ref())?;
        let _ply = p.read_ply(&mut Cursor::new(bytes));
        todo!()
        // Ok(CpuPointCloud {})
    }
}
