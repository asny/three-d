///
/// Window settings.
///
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct WindowSettings {
    /// Whether VSync is enabled.
    ///
    /// On web this has no effect since VSync is always on.
    pub vsync: bool,
    /// Number of antialiasing samples.
    ///
    /// On web, this can only be off (0) or on (>0).
    /// The actual number of samples depends on browser settings.
    pub multisamples: u8,
}
impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            vsync: true,
            multisamples: 4,
        }
    }
}
