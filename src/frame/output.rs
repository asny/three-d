///
/// Output from the rendering to the window each frame.
///
#[derive(Clone, Debug)]
pub struct FrameOutput {
    ///
    /// Closes the window and stops the renderloop if this is true.
    /// Only relevant on desktop, ignored on web.
    ///
    pub exit: bool,

    ///
    /// Swaps the back and front buffer if this is true.
    /// Set this to true if something have been rendered this frame and you want to display it.
    /// Set it to false if nothing have been rendered this frame, for example if nothing has changed,
    /// and you want to reuse the image from an old frame.
    /// Currently ignored on web, since it does not use double buffering.
    ///
    pub swap_buffers: bool,

    ///
    /// Takes a screenshot if this is set to some path and saves it at the given location.
    /// Only works on desktop, will be ignored on web.
    ///
    pub screenshot: Option<std::path::PathBuf>,

    ///
    /// Whether to stop the render loop until next event.
    /// Only works on desktop, will be ignored on web.
    ///
    pub wait_next_event: bool,
}

impl Default for FrameOutput {
    fn default() -> Self {
        Self {
            exit: false,
            swap_buffers: true,
            screenshot: None,
            wait_next_event: false,
        }
    }
}
