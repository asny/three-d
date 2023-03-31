#![allow(unsafe_code)]
use crate::control::*;
use crate::core::{Context, CoreError, Viewport};
use winit::event::{Event, TouchPhase, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit::*;

mod settings;
pub use settings::*;

mod frame_io;
pub use frame_io::*;

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
    WinitError(#[from] winit::error::OsError),
    #[error("error in three-d")]
    ThreeDError(#[from] CoreError),
    #[error("the number of MSAA samples must be a power of two")]
    InvalidNumberOfMSAASamples,
    #[error("it's not possible to create a graphics context/surface with the given settings")]
    SurfaceCreationError,
}

///
/// Error associated with a window.
///
#[cfg(target_arch = "wasm32")]
#[derive(Error, Debug)]
#[allow(missing_docs)]
pub enum WindowError {
    #[error("failed to create a new winit window")]
    WinitError(#[from] winit::error::OsError),
    #[error("failed creating a new window")]
    WindowCreation,
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
/// Window and event handling.
/// Use [Window::new] to create a new window or [Window::from_winit_window] which provides full control over the creation of the window.
///
pub struct Window<T: 'static + Clone> {
    window: winit::window::Window,
    event_loop: EventLoop<T>,
    #[cfg(target_arch = "wasm32")]
    closure: wasm_bindgen::closure::Closure<dyn FnMut(web_sys::Event)>,
    gl: WindowedContext,
    #[allow(dead_code)]
    maximized: bool,
}

impl Window<()> {
    ///
    /// Constructs a new Window with the given [settings].
    ///
    ///
    /// [settings]: WindowSettings
    pub fn new(window_settings: WindowSettings) -> Result<Window<()>, WindowError> {
        Self::from_event_loop(window_settings, EventLoop::new())
    }
}

impl<T: 'static + Clone> Window<T> {
    /// Exactly the same as [`Window::new()`] except with the ability to supply
    /// an existing [`EventLoop`]. Use the event loop's [proxy] to push custom
    /// events into the render loop (from any thread). Not available for web.
    ///
    /// [proxy]: winit::event_loop::EventLoopProxy
    #[cfg(not(target_arch = "wasm32"))]
    pub fn from_event_loop(
        window_settings: WindowSettings,
        event_loop: EventLoop<T>,
    ) -> Result<Self, WindowError> {
        let borderless = window_settings.borderless;
        let winit_window = if let Some((width, height)) = window_settings.max_size {
            WindowBuilder::new()
                .with_title(&window_settings.title)
                .with_min_inner_size(dpi::LogicalSize::new(
                    window_settings.min_size.0,
                    window_settings.min_size.1,
                ))
                .with_inner_size(dpi::LogicalSize::new(width as f64, height as f64))
                .with_max_inner_size(dpi::LogicalSize::new(width as f64, height as f64))
                .with_decorations(!borderless)
        } else {
            WindowBuilder::new()
                .with_min_inner_size(dpi::LogicalSize::new(
                    window_settings.min_size.0,
                    window_settings.min_size.1,
                ))
                .with_title(&window_settings.title)
                .with_decorations(!borderless)
                .with_maximized(true)
        }
        .build(&event_loop)?;
        Self::from_winit_window(
            winit_window,
            event_loop,
            window_settings.surface_settings,
            window_settings.max_size.is_none(),
        )
    }

    /// Exactly the same as [`Window::new()`] except with the ability to supply
    /// an existing [`EventLoop`]. Use the event loop's [proxy] to push custom
    /// events into the render loop (from any thread). Not available for web.
    ///
    /// [proxy]: winit::event_loop::EventLoopProxy
    #[cfg(target_arch = "wasm32")]
    pub fn from_event_loop(
        window_settings: WindowSettings,
        event_loop: EventLoop<T>,
    ) -> Result<Self, WindowError> {
        use wasm_bindgen::JsCast;
        use winit::{dpi::LogicalSize, platform::web::WindowBuilderExtWebSys};

        let websys_window = web_sys::window().ok_or(WindowError::WindowCreation)?;
        let document = websys_window
            .document()
            .ok_or(WindowError::DocumentMissing)?;

        let canvas = if let Some(canvas) = window_settings.canvas {
            canvas
        } else {
            document
                .get_elements_by_tag_name("canvas")
                .item(0)
                .expect(
                    "settings doesn't contain canvas and DOM doesn't have a canvas element either",
                )
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .map_err(|e| WindowError::CanvasConvertFailed(format!("{:?}", e)))?
        };

        let inner_size = window_settings
            .max_size
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

        let window_builder = WindowBuilder::new()
            .with_title(window_settings.title)
            .with_canvas(Some(canvas))
            .with_inner_size(inner_size)
            .with_prevent_default(true);

        let winit_window = window_builder.build(&event_loop)?;

        Self::from_winit_window(
            winit_window,
            event_loop,
            window_settings.surface_settings,
            window_settings.max_size.is_none(),
        )
    }

    ///
    /// Creates a new window from a [winit](https://crates.io/crates/winit) window and event loop with the given surface settings, giving the user full
    /// control over the creation of the window.
    /// This method takes ownership of the winit window and event loop, if this is not desired, use a [WindowedContext] or [HeadlessContext](crate::HeadlessContext) instead.
    ///
    pub fn from_winit_window(
        winit_window: window::Window,
        event_loop: EventLoop<T>,
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
    pub fn render_loop<F: 'static + FnMut(FrameInput<T>) -> FrameOutput>(self, mut callback: F) {
        let mut event_handler = EventHandler::new(&self.gl);
        self.event_loop.run(move |event, _, control_flow| {
            event_handler.handle_event(&event);
            match event {
                Event::LoopDestroyed => {
                    #[cfg(target_arch = "wasm32")]
                    {
                        use wasm_bindgen::JsCast;
                        use winit::platform::web::WindowExtWebSys;
                        self.window
                            .canvas()
                            .remove_event_listener_with_callback(
                                "contextmenu",
                                self.closure.as_ref().unchecked_ref(),
                            )
                            .unwrap();
                    }
                }
                Event::MainEventsCleared => {
                    self.window.request_redraw();
                }
                Event::RedrawRequested(_) => {
                    let frame_input = event_handler.resolve();
                    let frame_output = callback(frame_input);
                    if frame_output.exit {
                        *control_flow = ControlFlow::Exit;
                    } else {
                        if frame_output.swap_buffers {
                            #[cfg(not(target_arch = "wasm32"))]
                            self.gl.swap_buffers().unwrap();
                        }
                        if frame_output.wait_next_event {
                            *control_flow = ControlFlow::Wait;
                        } else {
                            *control_flow = ControlFlow::Poll;
                            self.window.request_redraw();
                        }
                    }

                    #[cfg(target_arch = "wasm32")]
                    if self.maximized {
                        use winit::platform::web::WindowExtWebSys;

                        let html_canvas = self.window.canvas();
                        let browser_window = html_canvas
                            .owner_document()
                            .and_then(|doc| doc.default_view())
                            .or_else(web_sys::window)
                            .unwrap();

                        self.window.set_inner_size(dpi::LogicalSize {
                            width: browser_window.inner_width().unwrap().as_f64().unwrap(),
                            height: browser_window.inner_height().unwrap().as_f64().unwrap(),
                        });
                    }
                }
                Event::WindowEvent { ref event, .. } => match event {
                    WindowEvent::Resized(physical_size) => {
                        self.gl.resize(*physical_size);
                    }
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    _ => (),
                },
                _ => (),
            }
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
    /// Returns the graphics context for this window.
    ///
    pub fn gl(&self) -> Context {
        (*self.gl).clone()
    }

    ///
    /// Returns an event loop proxy that can be used to send a `T` into the
    /// render loop using the proxy's [`send_event`] method. The event can be
    /// handled in the render loop by matching [`Event::UserEvent`].
    ///
    /// [`Event::UserEvent`]: crate::control::Event::UserEvent
    /// [`send_event`]: winit::event_loop::EventLoopProxy::send_event
    pub fn event_loop_proxy(&self) -> winit::event_loop::EventLoopProxy<T> {
        self.event_loop.create_proxy()
    }
}

fn is_printable_char(chr: char) -> bool {
    let is_in_private_use_area = ('\u{e000}'..='\u{f8ff}').contains(&chr)
        || ('\u{f0000}'..='\u{ffffd}').contains(&chr)
        || ('\u{100000}'..='\u{10fffd}').contains(&chr);

    !is_in_private_use_area && !chr.is_ascii_control()
}

fn translate_virtual_key_code(key: event::VirtualKeyCode) -> Option<crate::Key> {
    use event::VirtualKeyCode::*;

    Some(match key {
        Down => Key::ArrowDown,
        Left => Key::ArrowLeft,
        Right => Key::ArrowRight,
        Up => Key::ArrowUp,

        Escape => Key::Escape,
        Tab => Key::Tab,
        Back => Key::Backspace,
        Return => Key::Enter,
        Space => Key::Space,

        Insert => Key::Insert,
        Delete => Key::Delete,
        Home => Key::Home,
        End => Key::End,
        PageUp => Key::PageUp,
        PageDown => Key::PageDown,

        Key0 | Numpad0 => Key::Num0,
        Key1 | Numpad1 => Key::Num1,
        Key2 | Numpad2 => Key::Num2,
        Key3 | Numpad3 => Key::Num3,
        Key4 | Numpad4 => Key::Num4,
        Key5 | Numpad5 => Key::Num5,
        Key6 | Numpad6 => Key::Num6,
        Key7 | Numpad7 => Key::Num7,
        Key8 | Numpad8 => Key::Num8,
        Key9 | Numpad9 => Key::Num9,

        A => Key::A,
        B => Key::B,
        C => Key::C,
        D => Key::D,
        E => Key::E,
        F => Key::F,
        G => Key::G,
        H => Key::H,
        I => Key::I,
        J => Key::J,
        K => Key::K,
        L => Key::L,
        M => Key::M,
        N => Key::N,
        O => Key::O,
        P => Key::P,
        Q => Key::Q,
        R => Key::R,
        S => Key::S,
        T => Key::T,
        U => Key::U,
        V => Key::V,
        W => Key::W,
        X => Key::X,
        Y => Key::Y,
        Z => Key::Z,

        _ => {
            return None;
        }
    })
}
