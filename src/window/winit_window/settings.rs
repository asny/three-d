/// Selects the level of hardware graphics acceleration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HardwareAcceleration {
    /// Require graphics acceleration.
    Required,
    /// Prefer graphics acceleration, but fall back to software.
    Preferred,
    /// Do NOT use graphics acceleration.
    /// On some platforms (MacOS) this is ignored and treated the same as
    /// [Self::Preferred].
    /// On web, "willReadFrequently" is set to true.
    Off,
}

/// Settings controlling the behavior of the surface on where to draw, to present it on the screen.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub struct SurfaceSettings {
    /// Turn on vertical syncing, limiting the FPS to the display refresh rate.
    /// The default is true.
    /// On web this has no effect since vsync is always on.
    pub vsync: bool,
    /// Sets the number of bits in the depth buffer.
    /// A value of 0 means no depth buffer.
    /// The default value is 24.
    /// On web, this can only be off (0) or on (>0).
    pub depth_buffer: u8,
    /// Sets the number of bits in the stencil buffer.
    /// A value of 0 means no stencil buffer.
    /// The default value is 0.
    /// On web, this can only be off (0) or on (>0).
    pub stencil_buffer: u8,
    /// Set the level of the multisampling anti-aliasing (MSAA).
    /// Must be a power-of-two. Higher = more smooth edges.
    /// A value of 0 turns it off.
    /// The default value is 4.
    /// On web, this can only be off (0) or on (>0).
    /// The actual number of samples depends on browser settings.
    pub multisamples: u8,
    /// Specify whether or not hardware acceleration is preferred, required, or
    /// off. The default is [HardwareAcceleration::Preferred].
    pub hardware_acceleration: HardwareAcceleration,
}

impl Default for SurfaceSettings {
    fn default() -> Self {
        Self {
            vsync: true,
            depth_buffer: 24,
            stencil_buffer: 0,
            multisamples: 4,
            hardware_acceleration: HardwareAcceleration::Preferred,
        }
    }
}

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
    ///
    /// On web this has no effect.
    pub min_size: (u32, u32),
    /// The maximum size of the window (width, height). If None is specified, the window is maximized.
    ///
    /// On web this has no effect.
    pub max_size: Option<(u32, u32)>,
    /// Borderless mode.
    ///
    /// On web this has no effect.
    pub borderless: bool,
    /// An optional Canvas for using as winit window
    /// if this is None, the DOM (`index.html`) must contain a canvas element
    #[cfg(target_arch = "wasm32")]
    pub canvas: Option<web_sys::HtmlCanvasElement>,

    /// Settings related to the surface on where to draw.
    pub surface_settings: SurfaceSettings,
}
impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            title: "".to_string(),
            min_size: (2, 2),
            max_size: None,
            borderless: false,
            #[cfg(target_arch = "wasm32")]
            canvas: None,
            surface_settings: SurfaceSettings::default(),
        }
    }
}

impl std::ops::Deref for WindowSettings {
    type Target = SurfaceSettings;
    fn deref(&self) -> &Self::Target {
        &self.surface_settings
    }
}

impl std::ops::DerefMut for WindowSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.surface_settings
    }
}
