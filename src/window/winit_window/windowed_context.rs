use crate::Context;
use crate::SurfaceSettings;
use crate::WindowError;
use std::sync::Arc;
use winit::window::Window;

#[cfg(target_arch = "wasm32")]
mod inner {
    use crate::HardwareAcceleration;
    use serde::{Deserialize, Serialize};
    use wasm_bindgen::JsCast;
    use web_sys::HtmlCanvasElement;
    use winit::platform::web::WindowExtWebSys;

    use super::*;
    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize)]
    struct ContextOpt {
        pub antialias: bool,
        pub depth: bool,
        pub stencil: bool,
        pub willReadFrequently: bool,
        pub alpha: bool,
    }

    /// A context used for rendering
    pub struct WindowedContext {
        pub(super) context: Context,
        canvas: HtmlCanvasElement,
    }

    impl WindowedContext {
        /// Creates a new context from a [winit] window.
        pub fn from_winit_window(
            window: &Window,
            settings: SurfaceSettings,
        ) -> Result<Self, WindowError> {
            let canvas = window.canvas().ok_or(WindowError::CanvasMissing)?;
            Self::from_canvas(canvas, settings)
        }

        /// Creates a new context from a canvas.
        pub fn from_canvas(
            canvas: HtmlCanvasElement,
            settings: SurfaceSettings,
        ) -> Result<Self, WindowError> {
            // get webgl context and verify extensions
            let webgl_context = canvas
                .get_context_with_context_options(
                    "webgl2",
                    &serde_wasm_bindgen::to_value(&ContextOpt {
                        antialias: settings.multisamples > 0,
                        depth: settings.depth_buffer > 0,
                        stencil: settings.stencil_buffer > 0,
                        willReadFrequently: match settings.hardware_acceleration {
                            HardwareAcceleration::Required => false,
                            HardwareAcceleration::Preferred => false,
                            HardwareAcceleration::Off => true,
                        },
                        alpha: false,
                    })
                    .unwrap(),
                )
                .map_err(|e| WindowError::WebGL2NotSupported(format!(": {:?}", e)))?
                .ok_or(WindowError::WebGL2NotSupported("".to_string()))?
                .dyn_into::<web_sys::WebGl2RenderingContext>()
                .map_err(|e| WindowError::WebGL2NotSupported(format!(": {:?}", e)))?;
            webgl_context
                .get_extension("EXT_color_buffer_float")
                .map_err(|e| WindowError::ColorBufferFloatNotSupported(format!("{:?}", e)))?;
            webgl_context
                .get_extension("OES_texture_float_linear")
                .map_err(|e| WindowError::OESTextureFloatNotSupported(format!(": {:?}", e)))?;
            webgl_context
                .get_extension("OES_texture_half_float_linear")
                .map_err(|e| WindowError::OESTextureFloatNotSupported(format!(": {:?}", e)))?;

            Ok(Self {
                context: Context::from_gl_context(Arc::new(
                    crate::context::Context::from_webgl2_context(webgl_context),
                ))?,
                canvas,
            })
        }

        /// Resizes the context
        pub fn resize(&self, physical_size: winit::dpi::PhysicalSize<u32>) {
            self.canvas.set_width(physical_size.width as u32);
            self.canvas.set_height(physical_size.height as u32);
        }

        /// Make this context current. Needed when using multiple windows (contexts) on native.
        pub fn make_current(&self) -> Result<(), WindowError> {
            Ok(())
        }

        /// Swap buffers - should always be called after rendering.
        pub fn swap_buffers(&self) -> Result<(), WindowError> {
            Ok(())
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod inner {
    use glutin::{prelude::*, surface::*};

    use super::*;

    #[cfg(target_os = "linux")]
    fn glx_display_api_preference() -> glutin::display::DisplayApiPreference {
        glutin::display::DisplayApiPreference::Glx(Box::new(
            winit::platform::x11::register_xlib_error_hook,
        ))
    }

    ///
    /// A windowed graphics context, ie. a graphics context that is associated with a window.
    ///
    pub struct WindowedContext {
        pub(super) context: Context,
        surface: Surface<WindowSurface>,
        glutin_context: glutin::context::PossiblyCurrentContext,
    }

    impl WindowedContext {
        /// Creates a new windowed context from a [winit](https://crates.io/crates/winit) window.
        #[allow(unsafe_code)]
        pub fn from_winit_window(
            window: &Window,
            settings: SurfaceSettings,
        ) -> Result<Self, WindowError> {
            #[cfg(target_os = "windows")]
            use raw_window_handle::HasWindowHandle;

            // EGL is crossplatform and the official khronos way
            // but sometimes platforms/drivers may not have it, so we use back up options
            // where possible.
            // try egl and fallback to windows wgl. Windows is the only platform that
            // *requires* window handle to create display.
            #[cfg(target_os = "windows")]
            let preference = glutin::display::DisplayApiPreference::WglThenEgl(Some(
                window.window_handle()?.as_raw(),
            ));
            // try egl and fallback to x11 glx
            #[cfg(target_os = "linux")]
            let preference = glutin::display::DisplayApiPreference::EglThenGlx(Box::new(
                winit::platform::x11::register_xlib_error_hook,
            ));
            #[cfg(target_os = "macos")]
            let preference = glutin::display::DisplayApiPreference::Cgl;
            #[cfg(target_os = "android")]
            let preference = glutin::display::DisplayApiPreference::Egl;

            Self::from_winit_window_with_preference(window, settings, preference)
        }

        /// Creates a context using the X11 GLX backend instead of the default EGL-first preference.
        ///
        /// This can be useful when EGL display creation succeeds but EGL exposes no configuration
        /// compatible with the window. The default constructor only falls back to GLX if EGL
        /// display creation itself fails.
        #[cfg(target_os = "linux")]
        pub fn from_winit_window_with_glx(
            window: &Window,
            settings: SurfaceSettings,
        ) -> Result<Self, WindowError> {
            Self::from_winit_window_with_preference(window, settings, glx_display_api_preference())
        }

        #[allow(unsafe_code)]
        fn from_winit_window_with_preference(
            window: &Window,
            settings: SurfaceSettings,
            preference: glutin::display::DisplayApiPreference,
        ) -> Result<Self, WindowError> {
            if settings.multisamples > 0 && !settings.multisamples.is_power_of_two() {
                Err(WindowError::InvalidNumberOfMSAASamples)?;
            }
            use glutin::prelude::*;
            use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
            let raw_display_handle = window.display_handle()?.as_raw();
            let raw_window_handle = window.window_handle()?.as_raw();

            let gl_display =
                unsafe { glutin::display::Display::new(raw_display_handle, preference)? };
            let swap_interval = if settings.vsync {
                glutin::surface::SwapInterval::Wait(std::num::NonZeroU32::new(1).unwrap())
            } else {
                glutin::surface::SwapInterval::DontWait
            };

            let hardware_acceleration = match settings.hardware_acceleration {
                crate::HardwareAcceleration::Required => Some(true),
                crate::HardwareAcceleration::Preferred => None,
                crate::HardwareAcceleration::Off => Some(false),
            };
            let config_template = glutin::config::ConfigTemplateBuilder::new()
                .prefer_hardware_accelerated(hardware_acceleration)
                .with_depth_size(settings.depth_buffer);
            // we don't know if multi sampling option is set. so, check if its more than 0.
            let config_template = if settings.multisamples > 0 {
                config_template.with_multisampling(settings.multisamples)
            } else {
                config_template
            };
            let config_template = config_template
                .with_stencil_size(settings.stencil_buffer)
                .compatible_with_native_window(raw_window_handle)
                .build();
            // finds all valid configurations supported by this display that match the
            // config_template this is where we will try to get a "fallback" config if
            // we are okay with ignoring some native options required by user like multi
            // sampling, srgb, transparency etc..
            let config = unsafe {
                gl_display
                    .find_configs(config_template)?
                    .next()
                    .ok_or(WindowError::SurfaceCreationError)?
            };

            let context_attributes =
                glutin::context::ContextAttributesBuilder::new().build(Some(raw_window_handle));
            // for surface creation.
            let (width, height): (u32, u32) = window.inner_size().into();
            let width = std::num::NonZeroU32::new(width.max(1)).unwrap();
            let height = std::num::NonZeroU32::new(height.max(1)).unwrap();
            let surface_attributes =
                glutin::surface::SurfaceAttributesBuilder::<glutin::surface::WindowSurface>::new()
                    .build(raw_window_handle, width, height);
            // start creating the gl objects
            let gl_context = unsafe { gl_display.create_context(&config, &context_attributes)? };

            let gl_surface =
                unsafe { gl_display.create_window_surface(&config, &surface_attributes)? };
            let gl_context = gl_context.make_current(&gl_surface)?;
            gl_surface.set_swap_interval(&gl_context, swap_interval)?;

            Ok(Self {
                context: Context::from_gl_context(Arc::new(unsafe {
                    crate::context::Context::from_loader_function(|s| {
                        let s = std::ffi::CString::new(s)
                            .expect("failed to construct C string from string for gl proc address");

                        gl_display.get_proc_address(&s)
                    })
                }))?,
                glutin_context: gl_context,
                surface: gl_surface,
            })
        }

        /// Resizes the context
        pub fn resize(&self, physical_size: winit::dpi::PhysicalSize<u32>) {
            let width = std::num::NonZeroU32::new(physical_size.width.max(1)).unwrap();
            let height = std::num::NonZeroU32::new(physical_size.height.max(1)).unwrap();
            self.surface.resize(&self.glutin_context, width, height);
        }

        /// Make this context current. Needed when using multiple windows (contexts) on native.
        pub fn make_current(&self) -> Result<(), WindowError> {
            Ok(self.glutin_context.make_current(&self.surface)?)
        }

        /// Swap buffers - should always be called after rendering.
        pub fn swap_buffers(&self) -> Result<(), WindowError> {
            Ok(self.surface.swap_buffers(&self.glutin_context)?)
        }

        /// Enables or disabled vsync.
        pub fn set_vsync(&self, enabled: bool) -> Result<(), WindowError> {
            let swap_interval = if enabled {
                glutin::surface::SwapInterval::Wait(std::num::NonZeroU32::new(1).unwrap())
            } else {
                glutin::surface::SwapInterval::DontWait
            };
            Ok(self
                .surface
                .set_swap_interval(&self.glutin_context, swap_interval)?)
        }
    }

    #[cfg(all(test, target_os = "linux"))]
    mod tests {
        #[test]
        fn glx_preference_selects_glx_only() {
            assert!(matches!(
                super::glx_display_api_preference(),
                glutin::display::DisplayApiPreference::Glx(_)
            ));
        }
    }
}

pub use inner::*;

impl std::ops::Deref for WindowedContext {
    type Target = Context;

    fn deref(&self) -> &Self::Target {
        &self.context
    }
}
