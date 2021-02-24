
use glutin::*;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use crate::window::frame_input;
use crate::{context, Modifiers};

#[derive(Debug)]
pub enum Error {
    WindowCreationError(glutin::CreationError),
    ContextError(glutin::ContextError)
}

impl From<glutin::CreationError> for Error {
    fn from(other: glutin::CreationError) -> Self {
        Error::WindowCreationError(other)
    }
}

impl From<glutin::ContextError> for Error {
    fn from(other: glutin::ContextError) -> Self {
        Error::ContextError(other)
    }
}

pub struct Window
{
    windowed_context: ContextWrapper<PossiblyCurrent, window::Window>,
    event_loop: EventLoop<()>,
    gl: crate::Context
}

impl Window
{
    pub fn new(title: &str, size: Option<(u32, u32)>) -> Result<Window, Error>
    {
        let event_loop = EventLoop::new();
        let mut wc = Self::new_windowed_context(title, size, true, &event_loop);
        if wc.is_err() {
            wc = Self::new_windowed_context(title, size, false, &event_loop);
        }

        let windowed_context = unsafe { wc?.make_current().unwrap() };
        let gl = context::Glstruct::load_with(|s| windowed_context.get_proc_address(s) as *const std::os::raw::c_void);
        Ok(Window { windowed_context, event_loop, gl})
    }

    fn new_windowed_context(title: &str, size: Option<(u32, u32)>, multisample: bool, event_loop: &EventLoop<()>) -> Result<WindowedContext<NotCurrent>, Error> {

        let window_builder =
            if let Some((width, height)) = size {
                WindowBuilder::new()
                    .with_title(title)
                    .with_inner_size(dpi::LogicalSize::new(width as f64, height as f64))
                    .with_resizable(false)
            } else {
                WindowBuilder::new()
                    .with_title(title)
                    .with_maximized(true)
                    .with_resizable(false)
            };

        if multisample {
            Ok(ContextBuilder::new()
                .with_multisampling(4)
                .build_windowed(window_builder, event_loop)?)
        } else {
            Ok(ContextBuilder::new()
                .build_windowed(window_builder, event_loop)?)
        }
    }

    pub fn render_loop<F: 'static>(self, mut callback: F) -> Result<(), Error>
        where F: FnMut(frame_input::FrameInput)
    {
        let windowed_context = self.windowed_context;
        let mut last_time = std::time::Instant::now();
        let mut count = 0;
        let mut accumulated_time = 0.0;
        let mut events = Vec::new();
        let mut cursor_pos = None;
        let mut modifiers = Modifiers::default();
        self.event_loop.run(move |event, _, control_flow| {
                *control_flow = ControlFlow::Poll;
                match event {
                    Event::LoopDestroyed => {
                        return;
                    }
                    Event::MainEventsCleared => {
                        let now = std::time::Instant::now();
                        let duration = now.duration_since(last_time);
                        last_time = now;
                        let elapsed_time = duration.as_secs() as f64 * 1000.0 + duration.subsec_nanos() as f64 * 1e-6;
                        accumulated_time += elapsed_time;
                        count += 1;
                        if accumulated_time > 1000.0 {
                            println!("FPS: {}", count as f64 / (accumulated_time * 0.001));
                            count = 0;
                            accumulated_time = 0.0;
                        }

                        let (physical_width, physical_height): (u32, u32) = windowed_context.window().inner_size().into();
                        let device_pixel_ratio = windowed_context.window().scale_factor();
                        let (width, height): (u32, u32) = windowed_context.window().inner_size().to_logical::<f64>(device_pixel_ratio).into();
                        let frame_input = frame_input::FrameInput {
                            events: events.clone(),
                            elapsed_time,
                            accumulated_time,
                            viewport: crate::Viewport::new_at_origo(physical_width as usize, physical_height as usize),
                            window_width: width as usize,
                            window_height: height as usize,
                            device_pixel_ratio: device_pixel_ratio as usize
                        };
                        events.clear();
                        callback(frame_input);
                        windowed_context.swap_buffers().unwrap();
                    }
                    Event::WindowEvent { ref event, .. } => match event {
                        WindowEvent::Resized(physical_size) => {
                            windowed_context.resize(*physical_size);
                        }
                        WindowEvent::CloseRequested | WindowEvent::Destroyed => {
                            *control_flow = ControlFlow::Exit
                        },
                        WindowEvent::KeyboardInput {input, ..} => {
                            if let Some(keycode) = input.virtual_keycode {
                                use event::VirtualKeyCode;
                                if keycode == VirtualKeyCode::Escape {
                                    *control_flow = ControlFlow::Exit;
                                }
                                let state = if input.state == event::ElementState::Pressed {frame_input::State::Pressed} else {frame_input::State::Released};
                                if let Some(kind) = translate_virtual_key_code(keycode) {
                                    events.push(frame_input::Event::Key {state, kind, modifiers});
                                } else {
                                    if keycode == VirtualKeyCode::LControl || keycode == VirtualKeyCode::RControl {
                                        modifiers.ctrl = state;
                                        if !cfg!(target_os = "macos") {
                                            modifiers.command = state;
                                        }
                                    } else if keycode == VirtualKeyCode::LAlt || keycode == VirtualKeyCode::RAlt {
                                        modifiers.alt = state;
                                    } else if keycode == VirtualKeyCode::LShift || keycode == VirtualKeyCode::RShift {
                                        modifiers.shift = state;
                                    } else if keycode == VirtualKeyCode::LWin || keycode == VirtualKeyCode::RWin {
                                        if cfg!(target_os = "macos")
                                        {
                                            modifiers.command = state;
                                        }
                                    }
                                }
                            }
                        },
                        WindowEvent::MouseWheel {delta, ..} => {
                            if let Some(position) = cursor_pos
                            {
                                match delta {
                                    event::MouseScrollDelta::LineDelta(_, y) => {
                                        events.push(frame_input::Event::MouseWheel { delta: *y as f64, position });
                                    },
                                    event::MouseScrollDelta::PixelDelta(logical_position) => {
                                        events.push(frame_input::Event::MouseWheel { delta: logical_position.y, position });
                                    }
                                }
                            }
                        },
                        WindowEvent::MouseInput {state, button, ..} => {
                            if let Some(position) = cursor_pos
                            {
                                let state = if *state == event::ElementState::Pressed {frame_input::State::Pressed} else {frame_input::State::Released};
                                let button = match button {
                                    event::MouseButton::Left => Some(frame_input::MouseButton::Left),
                                    event::MouseButton::Middle => Some(frame_input::MouseButton::Middle),
                                    event::MouseButton::Right => Some(frame_input::MouseButton::Right),
                                    _ => None
                                };
                                if let Some(b) = button {
                                    events.push(frame_input::Event::MouseClick { state, button: b, position, modifiers });
                                }
                            }
                        },
                        WindowEvent::CursorMoved {position, ..} => {
                            let p = position.to_logical(windowed_context.window().scale_factor());
                            let delta = if let Some(last_pos) = cursor_pos {
                                (p.x - last_pos.0, p.y - last_pos.1)
                            } else {(0.0, 0.0)};
                            events.push(frame_input::Event::MouseMotion { delta, position: (p.x, p.y) });
                            cursor_pos = Some((p.x, p.y));
                        },
                        _ => (),
                    },
                    _ => (),
                }
            });
    }

    pub fn size(&self) -> (usize, usize)
    {
        let t: (u32, u32) = self.windowed_context.window().inner_size().to_logical::<f64>(self.windowed_context.window().scale_factor()).into();
        (t.0 as usize, t.1 as usize)
    }

    pub fn viewport(&self) -> crate::Viewport {
        let (w, h): (u32, u32) = self.windowed_context.window().inner_size().into();
        crate::Viewport::new_at_origo(w as usize, h as usize)
    }

    pub fn gl(&self) -> crate::Context
    {
        self.gl.clone()
    }
}

fn translate_virtual_key_code(key: event::VirtualKeyCode) -> Option<frame_input::Key> {
    use event::VirtualKeyCode::*;
    use frame_input::Key;

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
