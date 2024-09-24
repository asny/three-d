#![allow(unsafe_code)]
use crate::core::{Context, CoreError, Viewport};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::*;

mod settings;
pub use settings::*;

mod frame_io;
pub use frame_io::*;

mod frame_input_generator;
pub use frame_input_generator::*;

mod windowed_context;
pub use windowed_context::*;

use thiserror::Error;
///
/// Error associated with a window.
///
#[cfg(not(target_arch = "wasm32"))]
#[derive(Error, Debug)]
#[allow(missing_docs)]
pub enum WindowError {
    #[error("glutin error")]
    GlutinError(#[from] glutin::error::Error),
    #[error("winit error")]
    WinitError(#[from] WinitError),
    #[error("error in three-d")]
    ThreeDError(#[from] CoreError),
    #[error("the number of MSAA samples must be a power of two")]
    InvalidNumberOfMSAASamples,
    #[error("it's not possible to create a graphics context/surface with the given settings")]
    SurfaceCreationError,
}

///
/// Error associated with a winit window.
///
#[derive(Error, Debug)]
#[allow(missing_docs)]
pub enum WinitError {
    #[error("os error")]
    OsError(#[from] winit::error::OsError),
    #[error("handle error")]
    HandleError(#[from] winit::raw_window_handle::HandleError),
    #[error("event loop error")]
    EventLoopError(#[from] winit::error::EventLoopError),
}

///
/// Error associated with a window.
///
#[cfg(target_arch = "wasm32")]
#[derive(Error, Debug)]
#[allow(missing_docs)]
pub enum WindowError {
    #[error("failed to create a new winit window")]
    WinitError(#[from] WinitError),
    #[error("failed creating a new window")]
    WindowCreation,
    #[error("failed to retrieve canvas from the window")]
    MissingCanvas,
    #[error("unable to get document from canvas")]
    DocumentMissing,
    #[error("unable to convert canvas to html canvas: {0}")]
    CanvasConvertFailed(String),
    #[error("unable to get webgl2 context for the given canvas, maybe the browser doesn't support WebGL2{0}")]
    WebGL2NotSupported(String),
    #[error("unable to get EXT_color_buffer_float extension for the given canvas, maybe the browser doesn't support EXT_color_buffer_float: {0}")]
    ColorBufferFloatNotSupported(String),
    #[error("unable to get OES_texture_float extension for the given canvas, maybe the browser doesn't support OES_texture_float: {0}")]
    OESTextureFloatNotSupported(String),
    #[error("error in three-d")]
    ThreeDError(#[from] CoreError),
}

///
/// Default window, context and event handling which uses [winit](https://crates.io/crates/winit).
///
/// To get full control over the creation of the [winit](https://crates.io/crates/winit) window, use [Window::from_winit_window].
/// To take control over everything, including the context creation and [winit](https://crates.io/crates/winit) event loop,
/// use [WindowedContext::from_winit_window] and [FrameInputGenerator].
///
pub struct Window {
    window: winit::window::Window,
    event_loop: EventLoop<()>,
    #[cfg(target_arch = "wasm32")]
    closure: wasm_bindgen::closure::Closure<dyn FnMut(web_sys::Event)>,
    gl: WindowedContext,
    #[allow(dead_code)]
    maximized: bool,
}

impl Window {
    ///
    /// Constructs a new Window with the given [settings].
    ///
    ///
    /// [settings]: WindowSettings
    pub fn new(window_settings: WindowSettings) -> Result<Self, WindowError> {
        Self::from_event_loop(
            window_settings,
            EventLoop::new().map_err(|e| WinitError::EventLoopError(e))?,
        )
    }

    /// Exactly the same as [`Window::new()`] except with the ability to supply
    /// an existing [`EventLoop`].
    pub fn from_event_loop(
        window_settings: WindowSettings,
        event_loop: EventLoop<()>,
    ) -> Result<Self, WindowError> {
        #[cfg(not(target_arch = "wasm32"))]
        let window_builder = {
            let window_builder = window::Window::default_attributes()
                .with_title(&window_settings.title)
                .with_min_inner_size(dpi::LogicalSize::new(
                    window_settings.min_size.0,
                    window_settings.min_size.1,
                ))
                .with_decorations(!window_settings.borderless);

            match (window_settings.initial_size, window_settings.max_size) {
                (Some((width, height)), Some((max_width, max_height))) => window_builder
                    .with_inner_size(dpi::LogicalSize::new(width as f64, height as f64))
                    .with_max_inner_size(dpi::LogicalSize::new(
                        max_width as f64,
                        max_height as f64,
                    )),
                (Some((width, height)), None) => window_builder
                    .with_inner_size(dpi::LogicalSize::new(width as f64, height as f64)),
                (None, Some((width, height))) => window_builder
                    .with_inner_size(dpi::LogicalSize::new(width as f64, height as f64))
                    .with_max_inner_size(dpi::LogicalSize::new(width as f64, height as f64)),
                (None, None) => window_builder.with_maximized(true),
            }
        };
        #[cfg(target_arch = "wasm32")]
        let window_builder = {
            use wasm_bindgen::JsCast;
            use winit::{dpi::LogicalSize, platform::web::WindowAttributesExtWebSys};

            let canvas = if let Some(canvas) = window_settings.canvas {
                canvas
            } else {
                web_sys::window()
                .ok_or(WindowError::WindowCreation)?
                .document()
                .ok_or(WindowError::DocumentMissing)?
                .get_elements_by_tag_name("canvas")
                .item(0)
                .expect(
                    "settings doesn't contain canvas and DOM doesn't have a canvas element either",
                )
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .map_err(|e| WindowError::CanvasConvertFailed(format!("{:?}", e)))?
            };

            let inner_size = window_settings
                .initial_size
                .or(window_settings.max_size)
                .map(|(width, height)| LogicalSize::new(width as f64, height as f64))
                .unwrap_or_else(|| {
                    let browser_window = canvas
                        .owner_document()
                        .and_then(|doc| doc.default_view())
                        .or_else(web_sys::window)
                        .unwrap();
                    LogicalSize::new(
                        browser_window.inner_width().unwrap().as_f64().unwrap(),
                        browser_window.inner_height().unwrap().as_f64().unwrap(),
                    )
                });

            window::Window::default_attributes()
                .with_title(window_settings.title)
                .with_canvas(Some(canvas))
                .with_inner_size(inner_size)
                .with_prevent_default(true)
        };

        let winit_window = event_loop
            .create_window(window_builder)
            .map_err(WinitError::OsError)?;
        winit_window.focus_window();
        Self::from_winit_window(
            winit_window,
            event_loop,
            window_settings.surface_settings,
            window_settings.max_size.is_none() && window_settings.initial_size.is_none(),
        )
    }

    ///
    /// Creates a new window from a [winit](https://crates.io/crates/winit) window and event loop with the given surface settings, giving the user full
    /// control over the creation of the window.
    /// This method takes ownership of the winit window and event loop, if this is not desired, use a [WindowedContext] or [HeadlessContext](crate::HeadlessContext) instead.
    ///
    pub fn from_winit_window(
        winit_window: window::Window,
        event_loop: EventLoop<()>,
        mut surface_settings: SurfaceSettings,
        maximized: bool,
    ) -> Result<Self, WindowError> {
        let mut gl = WindowedContext::from_winit_window(&winit_window, surface_settings);
        if gl.is_err() {
            surface_settings.multisamples = 0;
            gl = WindowedContext::from_winit_window(&winit_window, surface_settings);
        }

        #[cfg(target_arch = "wasm32")]
        let closure = {
            use wasm_bindgen::JsCast;
            use winit::platform::web::WindowExtWebSys;
            let closure =
                wasm_bindgen::closure::Closure::wrap(Box::new(move |event: web_sys::Event| {
                    event.prevent_default();
                }) as Box<dyn FnMut(_)>);
            winit_window
                .canvas()
                .ok_or(WindowError::MissingCanvas)?
                .add_event_listener_with_callback("contextmenu", closure.as_ref().unchecked_ref())
                .expect("failed to listen to canvas context menu");
            closure
        };

        Ok(Self {
            window: winit_window,
            event_loop,
            gl: gl?,
            #[cfg(target_arch = "wasm32")]
            closure,
            maximized,
        })
    }

    ///
    /// Start the main render loop which calls the `callback` closure each frame.
    ///
    pub fn render_loop<F: 'static + FnMut(FrameInput) -> FrameOutput>(self, mut callback: F) {
        let mut frame_input_generator = FrameInputGenerator::from_winit_window(&self.window);
        _ = self.event_loop.run(move |event, event_loop| match event {
            Event::LoopExiting => {
                #[cfg(target_arch = "wasm32")]
                {
                    use wasm_bindgen::JsCast;
                    use winit::platform::web::WindowExtWebSys;
                    if let Some(canvas) = self.window.canvas() {
                        canvas
                            .remove_event_listener_with_callback(
                                "contextmenu",
                                self.closure.as_ref().unchecked_ref(),
                            )
                            .unwrap();
                    }
                }
            }
            Event::AboutToWait => {
                self.window.request_redraw();
            }
            Event::WindowEvent { ref event, .. } => {
                frame_input_generator.handle_winit_window_event(event);
                match event {
                    WindowEvent::Resized(physical_size) => {
                        self.gl.resize(*physical_size);
                    }
                    WindowEvent::RedrawRequested => {
                        #[cfg(target_arch = "wasm32")]
                        if self.maximized || option_env!("THREE_D_SCREENSHOT").is_some() {
                            use winit::platform::web::WindowExtWebSys;

                            if let Some(html_canvas) = self.window.canvas() {
                                let browser_window = html_canvas
                                    .owner_document()
                                    .and_then(|doc| doc.default_view())
                                    .or_else(web_sys::window)
                                    .unwrap();
                                _ = self.window.request_inner_size(dpi::LogicalSize {
                                    width: browser_window.inner_width().unwrap().as_f64().unwrap(),
                                    height: browser_window
                                        .inner_height()
                                        .unwrap()
                                        .as_f64()
                                        .unwrap(),
                                });
                            }
                        }

                        let frame_input = frame_input_generator.generate(&self.gl);
                        let frame_output = callback(frame_input);
                        if frame_output.exit {
                            event_loop.exit();
                        } else {
                            if frame_output.swap_buffers
                                && option_env!("THREE_D_SCREENSHOT").is_none()
                            {
                                self.gl.swap_buffers().unwrap();
                            }
                            if frame_output.wait_next_event {
                                event_loop.set_control_flow(ControlFlow::Wait);
                            } else {
                                event_loop.set_control_flow(ControlFlow::Poll);
                                self.window.request_redraw();
                            }
                        }
                    }
                    // WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    //     self.gl.resize(**new_inner_size);
                    // }
                    WindowEvent::CloseRequested => event_loop.exit(),
                    _ => (),
                }
            }
            _ => (),
        });
    }

    ///
    /// Return the current logical size of the window.
    ///
    pub fn size(&self) -> (u32, u32) {
        self.window
            .inner_size()
            .to_logical::<f64>(self.window.scale_factor())
            .into()
    }

    ///
    /// Returns the current viewport of the window in physical pixels (the size of the screen returned from [FrameInput::screen]).
    ///
    pub fn viewport(&self) -> Viewport {
        let (w, h): (u32, u32) = self.window.inner_size().into();
        Viewport::new_at_origo(w, h)
    }

    ///
    /// Returns the device pixel ratio for this window.
    ///
    pub fn device_pixel_ratio(&self) -> f32 {
        self.window.scale_factor() as f32
    }

    ///
    /// Returns the graphics context for this window.
    ///
    pub fn gl(&self) -> Context {
        (*self.gl).clone()
    }
}
