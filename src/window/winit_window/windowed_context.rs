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
    }

    impl WindowedContext {
        /// Creates a new context from a [winit] window.
        pub fn from_winit_window(
            window: &Window,
            settings: SurfaceSettings,
        ) -> Result<Self, WindowError> {
            let canvas = window.canvas();

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
            })
        }

        /// Resizes the context
        pub fn resize(&self, _physical_size: winit::dpi::PhysicalSize<u32>) {}

        /// Swap buffers - should always be called after rendering.
        pub fn swap_buffers(&self) -> Result<(), WindowError> {
            Ok(())
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod inner {
    use glutin::surface::*;

    use super::*;
    ///
    /// A windowed graphics context, ie. a graphics context that is associated with a window.
    /// For a graphics context that is not associated with a window, see [HeadlessContext](crate::HeadlessContext).
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
            if settings.multisamples > 0 && !settings.multisamples.is_power_of_two() {
                Err(WindowError::InvalidNumberOfMSAASamples)?;
            }
            use glutin::prelude::*;
            use raw_window_handle::*;
            let raw_display_handle = window.raw_display_handle();
            let raw_window_handle = window.raw_window_handle();

            // EGL is crossplatform and the official khronos way
            // but sometimes platforms/drivers may not have it, so we use back up options
            // where possible. TODO: check whether we can expose these options as
            // "features", so that users can select the relevant backend they want.

            // try egl and fallback to windows wgl. Windows is the only platform that
            // *requires* window handle to create display.
            #[cfg(target_os = "windows")]
            let preference =
                glutin::display::DisplayApiPreference::WglThenEgl(Some(raw_window_handle));
            // try egl and fallback to x11 glx
            #[cfg(target_os = "linux")]
            let preference = glutin::display::DisplayApiPreference::EglThenGlx(Box::new(
                winit::platform::x11::register_xlib_error_hook,
            ));
            #[cfg(target_os = "macos")]
            let preference = glutin::display::DisplayApiPreference::Cgl;
            #[cfg(target_os = "android")]
            let preference = glutin::display::DisplayApiPreference::Egl;

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

        /// Swap buffers - should always be called after rendering.
        pub fn swap_buffers(&self) -> Result<(), WindowError> {
            Ok(self.surface.swap_buffers(&self.glutin_context)?)
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
