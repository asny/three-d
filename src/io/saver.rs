
use std::path::Path;
use crate::io::*;

///
/// Functionality for saving resources. Only available on desktop at the moment.
///
pub struct Saver {}

impl Saver {
    ///
    /// Save the byte array as a file.
    ///
    pub fn save_file<P: AsRef<Path>>(path: P, bytes: &[u8]) -> Result<(), IOError>
    {
        let mut file = std::fs::File::create(path)?;
        use std::io::prelude::*;
        file.write_all(bytes)?;
        Ok(())
    }
}
