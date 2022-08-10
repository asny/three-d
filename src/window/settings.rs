///
/// Window settings.
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowSettings {
    /// The title of the window.
    ///
    /// On web this has no effect.
    pub title: String,
    /// The minimum size of the window (width, height).
    pub min_size: (u32, u32),
    /// The maximum size of the window (width, height). If None is specified, the window is maximized.
    pub max_size: Option<(u32, u32)>,
    /// Whether VSync is enabled.
    ///
    /// On web this has no effect since VSync is always on.
    pub vsync: bool,
    /// Number of antialiasing samples.
    ///
    /// On web, this can only be off (0) or on (>0).
    /// The actual number of samples depends on browser settings.
    pub multisamples: u8,
    /// Borderless mode.
    ///
    /// No effect on web.
    pub borderless: bool,
    /// An optional Canvas for using as winit window
    /// if this is None, the DOM (`index.html`) must contain a canvas element
    #[cfg(target_arch = "wasm32")]
    pub canvas: Option<web_sys::HtmlCanvasElement>
}
impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            title: "".to_string(),
            min_size: (2, 2),
            max_size: None,
            vsync: true,
            multisamples: 4,
            borderless: false,
            #[cfg(target_arch = "wasm32")]
            canvas: None
        }
    }
}
