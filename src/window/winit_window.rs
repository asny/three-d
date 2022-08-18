#![allow(unsafe_code)]
use crate::core::{Context, Viewport};
use crate::window::*;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit::*;

///
/// Default window and event handler for easy setup.
///
pub struct Window {
    window: Option<winit::window::Window>,
    event_loop: Option<EventLoop<()>>,
    #[cfg(not(target_arch = "wasm32"))]
    glutin_context: Option<glutin::RawContext<glutin::PossiblyCurrent>>,
    #[cfg(target_arch = "wasm32")]
    closure: wasm_bindgen::closure::Closure<dyn FnMut(web_sys::Event)>,
    gl: MaybeHeadlessContext,
}

impl Window {
    /// function to create window on web platforms
    #[cfg(target_arch = "wasm32")]
    pub fn new(window_settings: WindowSettings) -> Result<Window, WindowError> {
        use std::sync::Arc;
        use wasm_bindgen::JsCast;
        use winit::platform::web::{WindowBuilderExtWebSys, WindowExtWebSys};

        let websys_window = web_sys::window().ok_or(WindowError::WindowCreation)?;
        let document = websys_window
            .document()
            .ok_or(WindowError::DocumentMissing)?;

        let event_loop = EventLoop::new();
        let canvas =
            if let Some(canvas) = window_settings.canvas {
                canvas
            } else {
                document.get_elements_by_tag_name("canvas").item(0)
            .expect("settings doesn't contain canvas and DOM doesn't have a canvas element either")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|e| WindowError::CanvasConvertFailed(format!("{:?}", e)))?
            };

        // create winit window
        let window = WindowBuilder::new()
            .with_title(window_settings.title)
            .with_canvas(Some(canvas))
            .with_prevent_default(true)
            .build(&event_loop)?;
        let canvas = window.canvas();

        // get webgl context and verify extensions
        let webgl_context = canvas
            .get_context_with_context_options(
                "webgl2",
                &wasm_bindgen::JsValue::from_serde(&serde_json::json!({
                    "antialias": window_settings.multisamples > 0,
                }))
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
            .get_extension("OES_texture_float")
            .map_err(|e| WindowError::OESTextureFloatNotSupported(format!(": {:?}", e)))?;
        webgl_context
            .get_extension("OES_texture_float_linear")
            .map_err(|e| WindowError::OESTextureFloatNotSupported(format!(": {:?}", e)))?;
        let gl = crate::core::Context::from_gl_context(Arc::new(
            crate::context::Context::from_webgl2_context(webgl_context),
        ))?;

        let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |event: web_sys::Event| {
            event.prevent_default();
        }) as Box<dyn FnMut(_)>);
        canvas
            .add_event_listener_with_callback("contextmenu", closure.as_ref().unchecked_ref())
            .expect("failed to listen to canvas context menu");

        let window = Window {
            window: Some(window),
            event_loop: Some(event_loop),
            closure,
            gl: MaybeHeadlessContext::Haeded(gl),
        };

        Ok(window)
    }
    ///
    /// Constructs a new window with the given settings.
    ///
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new(mut settings: WindowSettings) -> Result<Window, WindowError> {
        if std::env::var("THREE_D_CI").is_ok() {
            return Ok(Window {
                window: None,
                event_loop: None,
                glutin_context: None,
                gl: HeadlessContext::new()?.into(),
            });
        } else {
            let event_loop = EventLoop::new();
            let mut wc = Self::new_windowed_context(&settings, &event_loop);
            if wc.is_err() {
                settings.multisamples = 0;
                wc = Self::new_windowed_context(&settings, &event_loop);
            }
            let windowed_context = unsafe { wc?.make_current().unwrap() };
            let context = unsafe {
                crate::context::Context::from_loader_function(|s| {
                    windowed_context.get_proc_address(s) as *const _
                })
            };
            // in future, we might just use winit for everything and build raw context from https://github.com/rust-windowing/glutin/blob/master/glutin_examples/examples/raw_context.rs
            let (glutin_context, winit_window) = unsafe { windowed_context.split() };
            Ok(Window {
                window: Some(winit_window),
                event_loop: Some(event_loop),
                #[cfg(not(target_arch = "wasm32"))]
                glutin_context: Some(glutin_context),
                gl: Context::from_gl_context(std::sync::Arc::new(context))?.into(),
            })
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    fn new_windowed_context(
        settings: &WindowSettings,
        event_loop: &EventLoop<()>,
    ) -> Result<glutin::WindowedContext<glutin::NotCurrent>, WindowError> {
        if settings.multisamples > 0 && !settings.multisamples.is_power_of_two() {
            Err(WindowError::InvalidNumberOfMSAASamples)?;
        }
        let borderless = settings.borderless;
        let window_builder = if let Some((width, height)) = settings.max_size {
            WindowBuilder::new()
                .with_title(&settings.title)
                .with_min_inner_size(dpi::LogicalSize::new(
                    settings.min_size.0,
                    settings.min_size.1,
                ))
                .with_inner_size(dpi::LogicalSize::new(width as f64, height as f64))
                .with_max_inner_size(dpi::LogicalSize::new(width as f64, height as f64))
                .with_decorations(!borderless)
        } else {
            WindowBuilder::new()
                .with_min_inner_size(dpi::LogicalSize::new(
                    settings.min_size.0,
                    settings.min_size.1,
                ))
                .with_title(&settings.title)
                .with_decorations(!borderless)
                .with_maximized(true)
        };

        Ok(glutin::ContextBuilder::new()
            .with_multisampling(settings.multisamples as u16)
            .with_vsync(settings.vsync)
            .build_windowed(window_builder, event_loop)?)
    }

    ///
    /// Start the main render loop which calls the `callback` closure each frame.
    ///
    pub fn render_loop<F: 'static + FnMut(FrameInput) -> FrameOutput>(self, mut callback: F) {
        if let Some(event_loop) = self.event_loop {
            let window = self.window.unwrap();
            #[cfg(not(target_arch = "wasm32"))]
            let glutin_context = self.glutin_context.unwrap();
            #[cfg(not(target_arch = "wasm32"))]
            let mut last_time = std::time::Instant::now();
            #[cfg(target_arch = "wasm32")]
            let mut last_time = instant::Instant::now();

            let mut accumulated_time = 0.0;
            let mut events = Vec::new();
            let mut cursor_pos = None;
            let mut modifiers = Modifiers::default();
            let mut first_frame = true;
            let mut mouse_pressed = None;
            let context = self.gl.clone();
            event_loop.run(move |event, _, control_flow| {
                match event {
                    Event::LoopDestroyed => {
                        #[cfg(target_arch = "wasm32")]
                        {
                            use wasm_bindgen::JsCast;
                            use winit::platform::web::WindowExtWebSys;
                            window
                                .canvas()
                                .remove_event_listener_with_callback(
                                    "contextmenu",
                                    self.closure.as_ref().unchecked_ref(),
                                )
                                .unwrap();
                        }
                        return;
                    }
                    Event::MainEventsCleared => {
                        window.request_redraw();
                    }
                    Event::RedrawRequested(_) => {
                        #[cfg(not(target_arch = "wasm32"))]
                        let now = std::time::Instant::now();
                        #[cfg(target_arch = "wasm32")]
                        let now = instant::Instant::now();

                        let duration = now.duration_since(last_time);
                        last_time = now;
                        let elapsed_time = duration.as_secs() as f64 * 1000.0
                            + duration.subsec_nanos() as f64 * 1e-6;
                        accumulated_time += elapsed_time;

                        #[cfg(target_arch = "wasm32")]
                        {
                            window.set_inner_size(winit::dpi::Size::Logical(
                                winit::dpi::LogicalSize {
                                    width: web_sys::window()
                                        .unwrap()
                                        .inner_width()
                                        .unwrap()
                                        .as_f64()
                                        .unwrap(),
                                    height: web_sys::window()
                                        .unwrap()
                                        .inner_height()
                                        .unwrap()
                                        .as_f64()
                                        .unwrap(),
                                },
                            ));
                        }

                        let (physical_width, physical_height): (u32, u32) =
                            window.inner_size().into();
                        let device_pixel_ratio = window.scale_factor();
                        let (width, height): (u32, u32) = window
                            .inner_size()
                            .to_logical::<f64>(device_pixel_ratio)
                            .into();
                        let frame_input = FrameInput {
                            events: events.clone(),
                            elapsed_time,
                            accumulated_time,
                            viewport: Viewport::new_at_origo(physical_width, physical_height),
                            window_width: width,
                            window_height: height,
                            device_pixel_ratio: device_pixel_ratio,
                            first_frame: first_frame,
                            context: context.clone(),
                        };
                        first_frame = false;
                        events.clear();
                        let mut frame_output = callback(frame_input);
                        #[cfg(not(target_arch = "wasm32"))]
                        if let Ok(ref v) = std::env::var("THREE_D_SCREENSHOT") {
                            let pixels =
                                RenderTarget::screen(&context, physical_width, physical_height)
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
                        if let Ok(v) = std::env::var("THREE_D_EXIT") {
                            if v.parse::<f64>().unwrap() < accumulated_time {
                                frame_output.exit = true;
                            }
                        }
                        if frame_output.exit {
                            *control_flow = ControlFlow::Exit;
                        } else {
                            if frame_output.swap_buffers {
                                #[cfg(not(target_arch = "wasm32"))]
                                glutin_context.swap_buffers().unwrap();
                            }
                            if frame_output.wait_next_event {
                                *control_flow = ControlFlow::Wait;
                            } else {
                                *control_flow = ControlFlow::Poll;
                                window.request_redraw();
                            }
                        }
                    }
                    Event::WindowEvent { ref event, .. } => match event {
                        #[cfg(not(target_arch = "wasm32"))]
                        WindowEvent::Resized(physical_size) => {
                            glutin_context.resize(*physical_size); // apparently some native contexts need to be resized manually to match window
                        }
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::KeyboardInput { input, .. } => {
                            if let Some(keycode) = input.virtual_keycode {
                                use event::VirtualKeyCode;
                                let state = input.state == event::ElementState::Pressed;
                                if let Some(kind) = translate_virtual_key_code(keycode) {
                                    events.push(if state {
                                        crate::Event::KeyPress {
                                            kind,
                                            modifiers,
                                            handled: false,
                                        }
                                    } else {
                                        crate::Event::KeyRelease {
                                            kind,
                                            modifiers,
                                            handled: false,
                                        }
                                    });
                                } else {
                                    if keycode == VirtualKeyCode::LControl
                                        || keycode == VirtualKeyCode::RControl
                                    {
                                        modifiers.ctrl = state;
                                        if !cfg!(target_os = "macos") {
                                            modifiers.command = state;
                                        }
                                        events.push(crate::Event::ModifiersChange { modifiers });
                                    } else if keycode == VirtualKeyCode::LAlt
                                        || keycode == VirtualKeyCode::RAlt
                                    {
                                        modifiers.alt = state;
                                        events.push(crate::Event::ModifiersChange { modifiers });
                                    } else if keycode == VirtualKeyCode::LShift
                                        || keycode == VirtualKeyCode::RShift
                                    {
                                        modifiers.shift = state;
                                        events.push(crate::Event::ModifiersChange { modifiers });
                                    } else if keycode == VirtualKeyCode::LWin
                                        || keycode == VirtualKeyCode::RWin
                                    {
                                        if cfg!(target_os = "macos") {
                                            modifiers.command = state;
                                            events
                                                .push(crate::Event::ModifiersChange { modifiers });
                                        }
                                    }
                                }
                            }
                        }
                        WindowEvent::MouseWheel { delta, .. } => {
                            if let Some(position) = cursor_pos {
                                match delta {
                                    winit::event::MouseScrollDelta::LineDelta(x, y) => {
                                        let line_height = 24.0; // TODO
                                        events.push(crate::Event::MouseWheel {
                                            delta: (
                                                (*x * line_height) as f64,
                                                (*y * line_height) as f64,
                                            ),
                                            position,
                                            modifiers,
                                            handled: false,
                                        });
                                    }
                                    winit::event::MouseScrollDelta::PixelDelta(delta) => {
                                        let d = delta.to_logical(window.scale_factor());
                                        events.push(crate::Event::MouseWheel {
                                            delta: (d.x, d.y),
                                            position,
                                            modifiers,
                                            handled: false,
                                        });
                                    }
                                }
                            }
                        }
                        WindowEvent::MouseInput { state, button, .. } => {
                            if let Some(position) = cursor_pos {
                                let button = match button {
                                    event::MouseButton::Left => Some(crate::MouseButton::Left),
                                    event::MouseButton::Middle => Some(crate::MouseButton::Middle),
                                    event::MouseButton::Right => Some(crate::MouseButton::Right),
                                    _ => None,
                                };
                                if let Some(b) = button {
                                    events.push(if *state == event::ElementState::Pressed {
                                        mouse_pressed = Some(b);
                                        crate::Event::MousePress {
                                            button: b,
                                            position,
                                            modifiers,
                                            handled: false,
                                        }
                                    } else {
                                        mouse_pressed = None;
                                        crate::Event::MouseRelease {
                                            button: b,
                                            position,
                                            modifiers,
                                            handled: false,
                                        }
                                    });
                                }
                            }
                        }
                        WindowEvent::CursorMoved { position, .. } => {
                            let p = position.to_logical(window.scale_factor());
                            let delta = if let Some(last_pos) = cursor_pos {
                                (p.x - last_pos.0, p.y - last_pos.1)
                            } else {
                                (0.0, 0.0)
                            };
                            events.push(crate::Event::MouseMotion {
                                button: mouse_pressed,
                                delta,
                                position: (p.x, p.y),
                                modifiers,
                                handled: false,
                            });
                            cursor_pos = Some((p.x, p.y));
                        }
                        WindowEvent::ReceivedCharacter(ch) => {
                            if is_printable_char(*ch) && !modifiers.ctrl && !modifiers.command {
                                events.push(crate::Event::Text(ch.to_string()));
                            }
                        }
                        WindowEvent::CursorEntered { .. } => {
                            events.push(crate::Event::MouseEnter);
                        }
                        WindowEvent::CursorLeft { .. } => {
                            mouse_pressed = None;
                            events.push(crate::Event::MouseLeave);
                        }
                        _ => (),
                    },
                    _ => (),
                }
            });
        } else {
            #[cfg(not(target_arch = "wasm32"))]
            {
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
                        let elapsed_time = duration.as_secs() as f64 * 1000.0
                            + duration.subsec_nanos() as f64 * 1e-6;
                        accumulated_time += elapsed_time;
                        callback(FrameInput {
                            events: Vec::new(),
                            elapsed_time,
                            accumulated_time,
                            viewport: self.viewport(),
                            device_pixel_ratio: 1.0,
                            window_width: self.size().0,
                            window_height: self.size().1,
                            first_frame,
                            context: self.gl().clone(),
                        });
                        first_frame = false;
                    }
                }
            }
        }
    }

    ///
    /// Return the current logical size of the window.
    ///
    pub fn size(&self) -> (u32, u32) {
        if let Some(ref window) = self.window {
            window
                .inner_size()
                .to_logical::<f64>(window.scale_factor())
                .into()
        } else {
            (1024, 1024)
        }
    }

    ///
    /// Returns the current viewport of the window in physical pixels (the size of the screen [RenderTarget] which is returned from [FrameInput::screen]).
    ///
    pub fn viewport(&self) -> Viewport {
        if let Some(ref window) = self.window {
            let (w, h): (u32, u32) = window.inner_size().into();
            Viewport::new_at_origo(w, h)
        } else {
            Viewport::new_at_origo(1024, 1024)
        }
    }

    ///
    /// Returns the graphics context for this window.
    ///
    pub fn gl(&self) -> Context {
        self.gl.clone()
    }
}

fn is_printable_char(chr: char) -> bool {
    let is_in_private_use_area = '\u{e000}' <= chr && chr <= '\u{f8ff}'
        || '\u{f0000}' <= chr && chr <= '\u{ffffd}'
        || '\u{100000}' <= chr && chr <= '\u{10fffd}';

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

/// A graphics context that may be headed or headless.
enum MaybeHeadlessContext {
    /// Headed graphics context.
    Haeded(Context),
    #[cfg(not(target_arch = "wasm32"))]
    /// Headless graphics context.
    Headless(HeadlessContext),
}

impl std::ops::Deref for MaybeHeadlessContext {
    type Target = Context;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Haeded(context) => context,
            #[cfg(not(target_arch = "wasm32"))]
            Self::Headless(context) => context,
        }
    }
}

impl From<Context> for MaybeHeadlessContext {
    fn from(context: Context) -> Self {
        Self::Haeded(context)
    }
}
#[cfg(not(target_arch = "wasm32"))]
impl From<HeadlessContext> for MaybeHeadlessContext {
    fn from(context: HeadlessContext) -> Self {
        Self::Headless(context)
    }
}
