use crate::*;

pub struct Window {
    context: HeadlessContext,
}

impl Window {
    pub fn new(
        window_settings: WindowSettings,
        context_settings: ContextSettings,
    ) -> Result<Self, WindowError> {
        Ok(Self {
            context: HeadlessContext {},
        })
    }

    pub fn render_loop<F: 'static + FnMut(FrameInput) -> FrameOutput>(self, mut callback: F) {
        let exit_time = if let Ok(v) = std::env::var("THREE_D_EXIT") {
            v.parse::<f64>().unwrap()
        } else {
            3000.0
        };
        let mut last_time = std::time::Instant::now();
        let mut accumulated_time = 0.0;
        let mut first_frame = true;
        while exit_time > accumulated_time {
            let now = std::time::Instant::now();
            let duration = now.duration_since(last_time);
            if duration.as_millis() > 30 {
                last_time = now;
                let elapsed_time =
                    duration.as_secs() as f64 * 1000.0 + duration.subsec_nanos() as f64 * 1e-6;
                accumulated_time += elapsed_time;
                callback(FrameInput {
                    events: Vec::new(),
                    elapsed_time,
                    accumulated_time,
                    viewport: Viewport::new_at_origo(1024, 1024),
                    device_pixel_ratio: 1.0,
                    window_width: 1024,
                    window_height: 1024,
                    first_frame,
                    context: (*self.context).clone(),
                });
                first_frame = false;
            }
        }
    }

    ///
    /// Return the current logical size of the window.
    ///
    pub fn size(&self) -> (u32, u32) {
        (1024, 1024)
    }

    ///
    /// Returns the current viewport of the window in physical pixels (the size of the screen [RenderTarget] which is returned from [FrameInput::screen]).
    ///
    pub fn viewport(&self) -> Viewport {
        Viewport::new_at_origo(1024, 1024)
    }

    ///
    /// Returns the graphics context for this window.
    ///
    pub fn gl(&self) -> Context {
        (*self.context).clone()
    }
}
