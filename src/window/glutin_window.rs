
use glutin::*;
use crate::window::frame_input;
use crate::gl;

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
    gl_window: GlWindow,
    events_loop: EventsLoop,
    gl: crate::Gl
}

impl Window
{
    pub fn new_default(title: &str) -> Result<Window, Error>
    {
        Window::new(title, 1024, 512)
    }

    pub fn new(title: &str, width: u32, height: u32) -> Result<Window, Error>
    {
        let window = WindowBuilder::new()
            .with_title(title)
            .with_dimensions(dpi::LogicalSize::new(width as f64, height as f64));

        let events_loop = EventsLoop::new();

        let context = ContextBuilder::new().with_vsync(true);

        let gl_window = GlWindow::new(window, context, &events_loop)?;

        unsafe {
            gl_window.make_current()?;
        }
        let gl = gl::Glstruct::load_with(|s| gl_window.get_proc_address(s) as *const std::os::raw::c_void);
        Ok(Window {gl_window, events_loop, gl})
    }

    pub fn render_loop<F: 'static>(&mut self, mut callback: F) -> Result<(), Error>
        where F: FnMut(frame_input::FrameInput)
    {
        let mut last_time = std::time::Instant::now();
        let mut count = 0;
        let mut accumulated_time = 0.0;
        let mut error = Ok(());
        let mut exit = false;
        while error.is_ok() && !exit {
            let mut events = Vec::new();
            self.events_loop.poll_events(|event| {
                exit = Self::handle_window_close_events(&event);
                if let Some(e) = Self::map_event(&event)
                {
                    events.push(e);
                }
            });

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

            let (screen_width, screen_height) = self.framebuffer_size();
            let frame_input = frame_input::FrameInput {events, elapsed_time, screen_width, screen_height};
            callback(frame_input);
            error = self.gl_window.swap_buffers();
        }
        error?;
        Ok(())
    }

    pub fn framebuffer_size(&self) -> (usize, usize)
    {
        let t: (u32, u32) = self.gl_window.get_inner_size().unwrap().to_physical(self.gl_window.get_hidpi_factor()).into();
        (t.0 as usize, t.1 as usize)
    }

    pub fn size(&self) -> (usize, usize)
    {
        let t: (u32, u32) = self.gl_window.get_inner_size().unwrap().into();
        (t.0 as usize, t.1 as usize)
    }

    pub fn gl(&self) -> crate::Gl
    {
        self.gl.clone()
    }

    pub fn map_event(event: &Event) -> Option<frame_input::Event>
    {
        static mut CURSOR_POS: Option<(f64, f64)> = None;
        match event {
            Event::WindowEvent{ event, .. } => match event {
                WindowEvent::KeyboardInput {input, ..} => {
                    if let Some(keycode) = input.virtual_keycode {
                        let state = if input.state == ElementState::Pressed {frame_input::State::Pressed} else {frame_input::State::Released};
                        return Some(frame_input::Event::Key {state, kind: format!("{:?}", keycode)});
                    }
                },
                WindowEvent::MouseWheel {delta, ..} => {
                    match delta {
                        MouseScrollDelta::LineDelta(_, y) => {
                            return Some(frame_input::Event::MouseWheel { delta: *y as f64 });
                        },
                        MouseScrollDelta::PixelDelta(logical_position) => {
                            return Some(frame_input::Event::MouseWheel { delta: logical_position.y });
                        }
                    }
                },
                WindowEvent::MouseInput {state, button, ..} => {
                    if let Some(position) = unsafe {CURSOR_POS}
                    {
                        let state = if *state == ElementState::Pressed {frame_input::State::Pressed} else {frame_input::State::Released};
                        let button = match button {
                            MouseButton::Left => Some(frame_input::MouseButton::Left),
                            MouseButton::Middle => Some(frame_input::MouseButton::Middle),
                            MouseButton::Right => Some(frame_input::MouseButton::Right),
                            _ => None
                        };
                        return button.map(|b| frame_input::Event::MouseClick { state, button: b, position });
                    }
                },
                WindowEvent::CursorMoved {position, ..} => {
                    unsafe {
                        CURSOR_POS = Some((position.x, position.y));
                    }
                },
                _ => ()
            },
            Event::DeviceEvent{ event, .. } => match event {
                DeviceEvent::MouseMotion {delta} => {
                    return Some(frame_input::Event::MouseMotion {delta: *delta});
                },
                _ => {}
            }
            _ => ()
        }
        None
    }

    fn handle_window_close_events(event: &Event) -> bool
    {
        match event {
            Event::WindowEvent{ event, .. } => match event {
                WindowEvent::CloseRequested => true,
                WindowEvent::KeyboardInput {input, ..} => Some(VirtualKeyCode::Escape) == input.virtual_keycode,
                _ => false
            },
            _ => false
        }
    }
}
