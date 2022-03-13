use crate::core::*;
use crate::window::*;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use glutin::*;

use thiserror::Error;
///
/// Error from the glutin window.
///
#[derive(Error, Debug)]
#[allow(missing_docs)]
pub enum WindowError {
    #[error("failed creating a new window")]
    WindowCreation(#[from] glutin::CreationError),
    #[error("failed creating a new context")]
    ContextCreation(#[from] glutin::ContextError),
    #[error("the number of MSAA samples must be a power of two")]
    InvalidNumberOfMSAASamples,
}

///
/// Default window and event handler for easy setup.
///
pub struct Window {
    windowed_context: ContextWrapper<PossiblyCurrent, window::Window>,
    event_loop: EventLoop<()>,
    gl: crate::Context,
}

impl Window {
    ///
    /// Constructs a new window with the given settings.
    ///
    pub fn new(mut settings: WindowSettings) -> ThreeDResult<Window> {
        let event_loop = EventLoop::new();
        let mut wc = Self::new_windowed_context(&settings, &event_loop);
        if wc.is_err() {
            settings.multisamples = 0;
            wc = Self::new_windowed_context(&settings, &event_loop);
        }

        let windowed_context = unsafe { wc?.make_current().unwrap() };

        let context = unsafe {
            glow::Context::from_loader_function(|s| {
                windowed_context.get_proc_address(s) as *const _
            })
        };
        Ok(Window {
            windowed_context,
            event_loop,
            gl: crate::core::Context::from_gl_context(std::rc::Rc::new(context)),
        })
    }

    fn new_windowed_context(
        settings: &WindowSettings,
        event_loop: &EventLoop<()>,
    ) -> ThreeDResult<WindowedContext<NotCurrent>> {
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

        Ok(ContextBuilder::new()
            .with_multisampling(settings.multisamples as u16)
            .with_vsync(settings.vsync)
            .build_windowed(window_builder, event_loop)?)
    }

    ///
    /// Start the main render loop which calls the `callback` closure each frame.
    ///
    pub fn render_loop<F: 'static + FnMut(FrameInput) -> FrameOutput>(
        self,
        mut callback: F,
    ) -> ThreeDResult<()> {
        let windowed_context = self.windowed_context;
        let mut last_time = std::time::Instant::now();
        let mut accumulated_time = 0.0;
        let mut events = Vec::new();
        let mut cursor_pos = None;
        let mut modifiers = Modifiers::default();
        let mut first_frame = true;
        let mut mouse_pressed = None;
        #[cfg(feature = "image-io")]
        let context = self.gl.clone();
        self.event_loop.run(move |event, _, control_flow| {
            match event {
                Event::LoopDestroyed => {
                    return;
                }
                Event::MainEventsCleared => {
                    windowed_context.window().request_redraw();
                }
                Event::RedrawRequested(_) => {
                    let now = std::time::Instant::now();
                    let duration = now.duration_since(last_time);
                    last_time = now;
                    let elapsed_time =
                        duration.as_secs() as f64 * 1000.0 + duration.subsec_nanos() as f64 * 1e-6;
                    accumulated_time += elapsed_time;

                    let (physical_width, physical_height): (u32, u32) =
                        windowed_context.window().inner_size().into();
                    let device_pixel_ratio = windowed_context.window().scale_factor();
                    let (width, height): (u32, u32) = windowed_context
                        .window()
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
                    };
                    first_frame = false;
                    events.clear();
                    let frame_output = callback(frame_input);
                    if frame_output.exit {
                        *control_flow = ControlFlow::Exit;
                    } else {
                        if frame_output.swap_buffers {
                            windowed_context.swap_buffers().unwrap();
                        }
                        if frame_output.wait_next_event {
                            *control_flow = ControlFlow::Wait;
                        } else {
                            *control_flow = ControlFlow::Poll;
                            windowed_context.window().request_redraw();
                        }
                    }

                    #[cfg(feature = "image-io")]
                    if let Some(ref path) = frame_output.screenshot {
                        let pixels = crate::Screen::read_color(
                            &context,
                            Viewport::new_at_origo(physical_width, physical_height),
                        )
                        .unwrap();
                        crate::Saver::save_pixels(path, &pixels, physical_width, physical_height)
                            .unwrap();
                    }
                }
                Event::WindowEvent { ref event, .. } => match event {
                    WindowEvent::Resized(physical_size) => {
                        windowed_context.resize(*physical_size);
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
                                        events.push(crate::Event::ModifiersChange { modifiers });
                                    }
                                }
                            }
                        }
                    }
                    WindowEvent::MouseWheel { delta, .. } => {
                        if let Some(position) = cursor_pos {
                            match delta {
                                glutin::event::MouseScrollDelta::LineDelta(x, y) => {
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
                                glutin::event::MouseScrollDelta::PixelDelta(delta) => {
                                    let d =
                                        delta.to_logical(windowed_context.window().scale_factor());
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
                        let p = position.to_logical(windowed_context.window().scale_factor());
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
    }

    ///
    /// Return the current logical size of the window.
    ///
    pub fn size(&self) -> ThreeDResult<(u32, u32)> {
        Ok(self
            .windowed_context
            .window()
            .inner_size()
            .to_logical::<f64>(self.windowed_context.window().scale_factor())
            .into())
    }

    ///
    /// Returns the current viewport of the window in physical pixels (the size of the [screen](crate::Screen)).
    ///
    pub fn viewport(&self) -> ThreeDResult<Viewport> {
        let (w, h): (u32, u32) = self.windowed_context.window().inner_size().into();
        Ok(Viewport::new_at_origo(w, h))
    }

    ///
    /// Returns the graphics context for this window.
    ///
    pub fn gl(&self) -> ThreeDResult<crate::Context> {
        Ok(self.gl.clone())
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
