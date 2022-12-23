use crate::*;
use std::ops::Deref;

include!("winit_window/settings.rs");
include!("winit_window/frame_io.rs");

pub struct Window {
    context: HeadlessContext,
    size: (u32, u32),
}

impl Window {
    pub fn new(
        window_settings: WindowSettings,
        surface_settings: SurfaceSettings,
    ) -> Result<Self, HeadlessError> {
        Ok(Self {
            context: HeadlessContext::new()?,
            size: window_settings.min_size,
        })
    }

    pub fn render_loop(self, mut callback: impl 'static + FnMut(FrameInput) -> FrameOutput) {
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
                    viewport: self.viewport(),
                    device_pixel_ratio: 1.0,
                    window_width: self.size.0,
                    window_height: self.size.1,
                    first_frame,
                    context: self.context.deref().clone(),
                });
                first_frame = false;
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        if let Ok(ref v) = std::env::var("THREE_D_SCREENSHOT") {
            let pixels = RenderTarget::screen(&self.gl, physical_width, physical_height)
                .read_color::<[u8; 4]>();
            use three_d_asset::io::Serialize;
            CpuTexture {
                data: TextureData::RgbaU8(pixels),
                width: physical_width,
                height: physical_height,
                ..Default::default()
            }
            .serialize(v)
            .unwrap()
            .save()
            .unwrap();
        }
    }

    ///
    /// Return the current logical size of the window.
    ///
    pub fn size(&self) -> (u32, u32) {
        self.size
    }

    ///
    /// Returns the current viewport of the window in physical pixels (the size of the screen [RenderTarget] which is returned from [FrameInput::screen]).
    ///
    pub fn viewport(&self) -> Viewport {
        Viewport::new_at_origo(self.size.0, self.size.1)
    }

    ///
    /// Returns the graphics context for this window.
    ///
    pub fn gl(&self) -> Context {
        self.context.deref().clone()
    }
}
